<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

![Logo](awesome.png)

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

> The most important thing I have done as a programmer in recent years is to aggressively pursue static code analysis. Even more valuable than the hundreds of serious bugs I have prevented with it is the change in mindset about the way I view software reliability and code quality. — [John Carmack (Creator of Doom)](https://www.gamasutra.com/view/news/128836/InDepth_Static_Code_Analysis.php) 

![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)

This is a collection of static analysis tools and code quality checkers. Pull requests are very welcome!

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 6 months, or the repo was archived.

Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

# Table of Contents

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

# Programming Languages

{%- for (language, linters) in linters %}

<h2 id="{{ language.tag }}">{{ language.name }}</h2>

{% for linter in linters -%}
- {% if linter.source.is_some() %}{{ linter.source.as_ref().unwrap()|format_badge }}{%endif%}[{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:]({{linter.discussion.as_ref().unwrap()}}){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.proprietary.is_some() %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

{%- endfor %}

# Multiple languages

{% for linter in multi -%}
- {% if linter.source.is_some() %}{{ linter.source.as_ref().unwrap()|format_badge }}{%endif%}[{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:]({{linter.discussion.as_ref().unwrap()}}){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.proprietary.is_some() %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

# Other

{% for (tag, others) in others %}

<h2 id="{{ tag.tag }}">{{ tag.name }}</h2>

{% for other in others -%}
- {% if other.source.is_some() %}{{ other.source.as_ref().unwrap()|format_badge }}{%endif%}[{{ other.name }}]({{ other.homepage }}){% if other.discussion.is_some() %} [:information_source:]({{other.discussion.as_ref().unwrap()}}){% endif %}{% if other.deprecated.is_some() %} :warning:{% endif %}{% if other.proprietary.is_some() %} :copyright:{% endif %} - {{ other.description }}
{% endfor %}

{%- endfor %}

# More collections

- [go-tools](https://github.com/dominikh/go-tools) - A collection of tools and libraries for working with Go code, including linters and static analysis
- [linters](https://github.com/mcandre/linters/tree/b044f0628c4a96dfea869cf61e0e96cf4c49cf6b) - An introduction to static code analysis
- [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) -  A reviewed list of useful PHP static analysis tools
- [Tools for C/C++](https://www.peerlyst.com/posts/a-list-of-static-analysis-tools-for-c-c-peerlyst?utm_source=twitter&utm_medium=social&utm_content=peerlyst_post&utm_campaign=peerlyst_resources) - A list of static analysis tools for C/C++
- [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) -  A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).
