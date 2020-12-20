<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

 <a href="http://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.png" />
 </a>

This repository lists **static analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)

## What is Static Analysis?

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

> The most important thing I have done as a programmer in recent years is to aggressively pursue static code analysis. Even more valuable than the hundreds of serious bugs I have prevented with it is the change in mindset about the way I view software reliability and code quality. — [John Carmack (Creator of Doom)](https://www.gamasutra.com/view/news/128836/InDepth_Static_Code_Analysis.php)

## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
  <tr>
    <td><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepcode.svg" /></a></td>
    <td><a href="https://deepsource.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepsource.png" /></a></td>
    <td><a href="https://www.viva64.com/free-license"><img height="100px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/pvs-studio.svg" /></a></td>
    <td><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codescene.png" /></a></td>
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

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    {% for (language, _) in linters -%}
      <li><a href="#{{ language.tag }}">{{ language.name }}</a></li>
    {% endfor -%}
  </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

{% for (tag, _) in others -%}
- [{{ tag.name }}](#{{ tag.tag }})
  {% endfor %}

---

## Programming Languages

{%- for (language, linters) in linters %}

<h2 id="{{ language.tag }}">{{ language.name }}</h2>

{% for linter in linters -%}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

{%- endfor %}

## Multiple languages

{% for linter in multi -%}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

## Other

{% for (tag, others) in others %}

<h2 id="{{ tag.tag }}">{{ tag.name }}</h2>

{% for other in others -%}
- [{{ other.name }}]({{ other.homepage }}){% if other.discussion.is_some() %} [:information_source:](<{{other.discussion.as_ref().unwrap()}}>){% endif %}{% if other.deprecated.is_some() %} :warning:{% endif %}{% if other.license == "proprietary" %} :copyright:{% endif %} - {{ other.description }}
{% endfor %}

{%- endfor %}

## More Collections

- [go-tools](https://github.com/dominikh/go-tools) - A collection of tools and libraries for working with Go code, including linters and static analysis
- [Clean code linters](https://github.com/collections/clean-code-linters) - A collection of linters in github collections
- [Code Quality Checker Tools For PHP Projects](https://github.com/collections/code-quality-in-php) - A collection of PHP linters in github collections
- [linters](https://github.com/mcandre/linters) - An introduction to static code analysis
- [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) - A reviewed list of useful PHP static analysis tools
- [Tools for C/C++](https://www.peerlyst.com/posts/a-list-of-static-analysis-tools-for-c-c-peerlyst?utm_source=twitter&utm_medium=social&utm_content=peerlyst_post&utm_campaign=peerlyst_resources) - A list of static analysis tools for C/C++
- [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) - A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).
