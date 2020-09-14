const fs = require("fs");
const yaml = require("js-yaml");
const normalizeUrl = require("normalize-url");
require("make-promises-safe");

// https://stackoverflow.com/questions/63123579
const any = require("promise.any");
any.shim(); // will be a no-op if not needed

const Bottleneck = require("bottleneck/es5");
const fetch = require("node-fetch");
const util = require("util");
const exec = util.promisify(require("child_process").exec);

const limiter = new Bottleneck({
  maxConcurrent: 5,
  minTime: 300,
});
const throttledFetch = limiter.wrap(fetch);

const slugify = (text) => {
  return text
    .toString()
    .toLowerCase()
    .replace(/\s+/g, "-") // Replace spaces with -
    .replace(/[^\w\-]+/g, "") // Remove all non-word chars
    .replace(/\-\-+/g, "-") // Replace multiple - with single -
    .replace(/^-+/, "") // Trim - from start of text
    .replace(/-+$/, ""); // Trim - from end of text
};

const getGithubStats = async (url) => {
  try {
    if (!url) {
      return {};
    }
    if (!url.includes("github.com")) {
      return {};
    }
    const match = url.match(/.*github.com\/(.*)\/(.*).*/);
    if (!match) {
      return {};
    }

    let headers = { accept: "application/vnd.github.preview" };
    if (process.env.GITHUB_USERNAME && process.env.GITHUB_TOKEN) {
      headers.Authorization =
        "Basic " +
        Buffer.from(
          process.env.GITHUB_USERNAME + ":" + process.env.GITHUB_TOKEN
        ).toString("base64");
    }
    const res = await throttledFetch(
      `https://api.github.com/repos/${match[1]}/${match[2]}`,
      { headers }
    );
    return await res.json();
  } catch (err) {
    console.log(err);
    return {};
  }
};

const licenseRegex = /License\: (.*)$/gm;

const matchEsotericLicenses = (licenseText) => {
  let licenseMap = new Map();
  licenseMap.set("Brakeman Public Use License", "Brakeman Public Use License");
  licenseMap.set(
    `Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
  <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
  option`,
    "Apache-2.0/MIT"
  );
  licenseMap.set(
    "Licensed under the Apache License, Version 2.0",
    "Apache-2.0"
  );
  licenseMap.set("Copyright (c) MMXV jden jason@denizac.org", "ISC");
  licenseMap.set(
    "CeCILL-C FREE SOFTWARE LICENSE AGREEMENT",
    "CeCILL-C license"
  );
  licenseMap.set("LibVCS4j", "MIT/GPL-3.0/LGPL-3.0");
  licenseMap.set("Checker Framework developers", "GPL with Classpath exception / MIT License")

  const licenseTextSanitized = licenseText.replace(/\s+/g, "").toLowerCase();
  // console.log(`Checking esoteric license ${licenseTextSanitized}`);
  for (const [needle, license] of licenseMap.entries()) {
    let needleSanitized = needle.replace(/\s+/g, "").toLowerCase();
    // console.log(needleSanitized, license);
    if (licenseTextSanitized.indexOf(needleSanitized) > -1) {
      console.log(`Found ${license}`);
      return license;
    }
  }
};

const getLicense = async (tool) => {
  if (tool.proprietary) {
    return "proprietary";
  }
  if (tool.license) {
    return tool.license;
  }
  if (tool.source.includes("github")) {
    const stats = await getGithubStats(tool.source);
    if (stats.license && stats.license.name) {
      return stats.license.name;
    }

    const licenseTextAsync = await Promise.any(
      [
        "master/LICENSE",
        "master/LICENSE.MD",
        "master/LICENSE.md",
        "master/LICENSE.txt",
        "master/license",
        "master/LICENSE-MIT",
        "master/README",
        "main/LICENSE",
        "main/LICENSE.MD",
        "main/LICENSE.md",
        "main/LICENSE.txt",
        "main/license",
        "main/LICENSE-MIT",
        "main/README",
        "master/COPYRIGHT",
        "latest/LICENSE-ClassGraph.txt",
        "master/COPYING",
        "master/LICENSE.TXT",
        "dev/LICENSE",
        "master/LICENSE_1_0.txt",
        "master/LICENCE.md",
        "master/LICENSE.BSD",
        "master/LICENCE.txt",
        "spotbugs/license.txt",
        "gh-pages/LICENSE",
        "develop/license.txt",
        "master/licence.txt",
        "master/License.txt",
        "master/MIT-LICENSE",
        "develop/LICENSE",
        "master/LICENSE-CECILL-C.txt",
        "main/COPYING",
        "master/COPYING.txt",
      ].map(async (file) => {
        const url = `${tool.source.replace(
          "github.com",
          "raw.githubusercontent.com"
        )}/${file}`;
        console.log(url);
        let response = await fetch(url);
        if (response.status != 200) {
          console.log(response.status);
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        // console.log(url);
        return await response.text();
      })
    ).catch((e) => {
      throw new Error(
        `Cannot find license file for tool ${tool.source} ${e.message}`
      );
    });
    const licenseText = await licenseTextAsync;
    fs.writeFileSync(`license-${tool.name}`, licenseText);
    try {
      const { stdout, stderr } = await exec(
        `askalono identify license-${tool.name}`,
        {
          shell: true,
        }
      );
      const license = stdout.match(licenseRegex);
      if (license !== undefined && license.length > 0) {
        return license[0]
          .replace("License: ", "")
          .replace(" (original text)", "");
      }
    } catch (e) {
      console.log(e);
      return matchEsotericLicenses(licenseText);
    } finally {
      fs.unlinkSync(`license-${tool.name}`);
    }
  }
  // Halt and catch fire
  throw new Error(`Missing license for ${tool.name} (${tool.source})`);
};

const getTypes = (tool) => {
  let types = [];
  if (tool.tags.includes("service")) {
    types.push("service");
  }
  if (tool.tags.includes("cli")) {
    types.push("cli");
  }
  if (tool.tags.includes("ide")) {
    types.push("ide-plugin");
  }
  if (types.length == 0) {
    types.push("cli");
  }

  return types;
};

const getCategory = (tool) => {
  let category = [];
  if (tool.description.includes("format")) {
    category.push("formatter");
  }
  if (tool.tags.includes("formatter")) {
    category.push("formatter");
  }

  if (tool.tags.includes("meta")) {
    category.push("meta");
  } else if (tool.description.toLowerCase().includes("lint")) {
    category.push("linter");
  } else if (tool.description.toLowerCase().includes("checks")) {
    category.push("linter");
  }
  if (category.length == 0) {
    category.push("linter");
  }

  return category.filter((item, i, ar) => ar.indexOf(item) === i);
};

const run = async () => {
  let fileContents = fs.readFileSync("../data/tools.yml", "utf8");
  let data = yaml.safeLoad(fileContents);

  data = data.filter((tool) => {
    const path = `../data/tools/${slugify(tool.name)}.yml`;
    if (fs.existsSync(path)) {
      console.log(`Found file ${path}. Skipping.`);
      return false;
    }
    return true;
  });

  let tags = data
    .map((x) => x.tags)
    .reduce((x, y) => {
      return x.concat(y);
    }, [])
    .filter((item, i, ar) => ar.indexOf(item) === i)
    .filter((x) => {
      if (x == "ide" || x == "formatter") {
        return false;
      }
      return true;
    })
    .sort();

  // fs.rmdirSync("../data/tools", { recursive: true });
  // fs.mkdirSync("../data/tools");
  const results = await Promise.all(
    data.map(async (tool) => {
      let x = {
        name: tool.name,
        categories: getCategory(tool),
        tags: tool.tags ? tool.tags.filter((t) => tags.includes(t)).sort() : [],
      };
      if (tool.deprecated) {
        x.deprecated = true;
      }
      let license = await getLicense(tool);
      if (license) {
        x.license = license;
      }
      let types = getTypes(tool);
      if (types) {
        x.types = types;
      }
      if (tool.source) {
        x.source = normalizeUrl(tool.source, {
          sortQueryParameters: false,
          stripWWW: false,
        });
      }
      if (tool.homepage) {
        x.homepage = normalizeUrl(tool.homepage, {
          sortQueryParameters: false,
          stripWWW: false,
        });
      }
      if (tool.description) {
        x.description = tool.description;
      }
      const path = `../data/tools/${slugify(x.name)}.yml`;
      fs.writeFileSync(path, yaml.dump(x));
      return x;
    })
  );
};

run();
