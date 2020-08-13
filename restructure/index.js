const fs = require("fs");
const yaml = require("js-yaml");

const Bottleneck = require("bottleneck/es5");
const fetch = require("node-fetch");

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
  }
  console.log(`Missing license for ${tool.name} ${tool.source}`);
  return undefined;
};

const getType = (tool) => {
  let type = [];
  if (tool.tags.includes("service")) {
    type.push("service");
  }
  if (tool.tags.includes("cli")) {
    type.push("cli");
  }
  if (tool.tags.includes("ide")) {
    type.push("ide-plugin");
  }

  return [];
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

  let tags = data
    .map((x) => x.tags)
    .reduce((x, y) => {
      return x.concat(y);
    }, [])
    .filter((item, i, ar) => ar.indexOf(item) === i)
    .filter((x) => {
      if (x == "ide") {
        return false;
      }
      return true;
    })
    .sort();

  const results = await Promise.all(
    data.map(async (tool) => {
      let x = {
        name: tool.name,
        category: getCategory(tool),
        tags: tool.tags ? tool.tags.filter((t) => tags.includes(t)).sort() : [],
      };
      if (tool.deprecated) {
        x.deprecated = true;
      }
      let license = await getLicense(tool);
      if (license) {
        x.license = license;
      }
      let type = getType(tool);
      if (type) {
        x.type = type;
      }
      if (tool.source) {
        x.source = tool.source;
      }
      if (tool.homepage) {
        x.homepage = tool.homepage;
      }
      if (tool.description) {
        x.description = tool.description;
      }

      return x;
    })
  );

  fs.rmdirSync("../data/tools", { recursive: true });
  fs.mkdirSync("../data/tools");
  results.forEach((tool) => {
    fs.writeFileSync(
      `../data/tools/${slugify(tool.name)}.yml`,
      yaml.dump(tool)
    );
  });
};

try {
  run();
} catch (e) {
  console.log(e);
}
