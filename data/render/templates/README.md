<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

 <a href="https://analysis-tools.dev/">
   <img alt="Analysis Tools Website" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/redesign.svg" />
 </a>

This repository lists **static analysis tools** for all programming languages, build tools, config files and more. The focus is on tools which improve code quality such as linters and formatters.
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

[![Website](https://img.shields.io/badge/Website-Online-2B5BAE)](https://analysis-tools.dev)
![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)
[![Links](https://github.com/analysis-tools-dev/static-analysis/actions/workflows/links.yml/badge.svg)](https://github.com/analysis-tools-dev/static-analysis/actions/workflows/links.yml)

## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
   <tr>
      <td>
         <a href="https://bugprove.com">
            <picture >
               <source width="200px" media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/bugprove-dark.svg">
               <img width="200px" alt="BugProve" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/bugprove-light.svg">
            </picture>
         </a>
      </td>
      <td>
         <a href="https://www.betterscan.io">
            <picture >
               <source width="200px" media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/betterscan-dark.svg">
               <img width="200px" alt="Betterscan" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/betterscan-light.svg">
            </picture>
         </a>
      </td>
      <td>
         <a href="https://www.pixee.ai/">
            <picture >
               <source width="200px" media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/pixee-light.png">
               <img width="200px" alt="Pixee" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/pixee-dark.png">
            </picture>
         </a>
      </td>
      <td>
         <a href="https://coderabbit.ai">
            <img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/code-rabbit.svg" />
         </a>
      </td>
      <td>
         <a href="https://semgrep.dev/">
            <img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/semgrep.svg" />
         </a>
      </td>
      <td>
         <a href="https://offensive360.com/">
            <img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/offensive360.png" />
         </a>
      </td>
   </tr>
</table>

If you also want to support this project, head over to our [Github sponsors page](https://github.com/sponsors/analysis-tools-dev).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

## Table of Contents

#### [Programming Languages](#programming-languages-1)

{% for (language, _) in linters %}
- [{{ language.name }}](#{{ language.value }})
  {%- endfor %}

#### [Multiple Languages](#multiple-languages-1)

#### [Other](#other-1)
<details>
 <summary>Show Other</summary>
{% for (tag, _) in others %}
- [{{ tag.name }}](#{{ tag.value }})
  {%- endfor %}
</details>

---

## Programming Languages

{%- for (language, linters) in linters %}

<a name="{{ language.value }}" />
<h2>{{ language.name }}</h2>

{% for linter in linters %}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() && linter.deprecated.unwrap() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} — {{ linter.description }}
{% endfor %}

{%- endfor %}

## Multiple languages

{% for linter in multi %}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() && linter.deprecated.unwrap() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} — {{ linter.description }}
{% endfor %}

## Other

{% for (tag, others) in others %}

<a name="{{ tag.value }}" />
<h2>{{ tag.name }}</h2>

{% for other in others %}
- [{{ other.name }}]({{ other.homepage }}){% if other.discussion.is_some() %} [:information_source:](<{{other.discussion.as_ref().unwrap()}}>){% endif %}{% if other.deprecated.is_some() && other.deprecated.unwrap() %} :warning:{% endif %}{% if other.license == "proprietary" %} :copyright:{% endif %} — {{ other.description }}
{% endfor %}

{%- endfor %}

## More Collections

- [Clean code linters](https://github.com/collections/clean-code-linters) — A collection of linters in github collections
- [Code Quality Checker Tools For PHP Projects](https://github.com/collections/code-quality-in-php) — A collection of PHP linters in github collections
- [go-tools](https://github.com/dominikh/go-tools) — A collection of tools and libraries for working with Go code, including linters and static analysis
- [linters](https://github.com/mcandre/linters) — An introduction to static code analysis
- [OWASP Source Code Analysis Tools](https://owasp.org/www-community/Source_Code_Analysis_Tools) — List of tools maintained by the Open Web Application Security Project
- [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) — A reviewed list of useful PHP static analysis tools
- [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) — A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.


Title image [Designed by Freepik](https://www.freepik.com).
