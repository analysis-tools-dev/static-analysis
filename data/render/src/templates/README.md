![Logo](awesome.png)

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)

This is a collection of static analysis tools and code quality checkers. Pull requests are very welcome!

* :copyright: stands for proprietary software. All other tools are Open Source.
* :warning: indicates that the community does not recommend to use this tool for
  new projects anymore as it is outdated or no longer maintained.

Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

# Table of Contents

#### [Programming Languages](#programming-languages-1)

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    {% for language in categories.languages -%}
    {% if catalog.linters is containing(language.name) -%}
    <li><a href="#{{ language.tag }}">{{ language.name }}</a></li>
    {% endif -%}
    {% endfor -%}
  </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

{% for other in categories.other -%}
- [{{ other.name }}](#{{ other.tag }})
{% endfor %}

---

# Programming Languages
{% for language, map in catalog.linters %}
{% for tag, linters in map -%}
<h2 id="{{ tag }}">{{ language }}</h2>

{% for linter in linters -%}
* [{{linter.name }}]({{linter.url | safe }})
  {%- if linter.deprecated %} :warning:{% endif %} {% if linter.proprietary %}:copyright: {% endif %}- {{linter.description | safe }}
{% endfor -%}
{% endfor -%}
{% endfor %}

# Multiple languages

{% for linter in catalog.multi -%}

* [{{linter.name }}]({{linter.url | safe }})
  {%- if linter.deprecated %} :warning:{% endif %} {% if linter.proprietary %}:copyright: {% endif %}- {{linter.description | safe }}
{% endfor %}

# Other
{% for category, map in catalog.others %}
{% for tag, others in map -%}
<h2 id="{{ tag }}">{{ category }}</h2>

{% for other in others -%}
* [{{ other.name }}]({{ other.url | safe }}) - {{ other.description | safe }}
{% endfor -%}
{% endfor -%}
{% endfor %}

# More collections

* [go-tools](https://github.com/dominikh/go-tools) - A collection of tools and libraries for working with Go code, including linters and static analysis
* [linters](https://github.com/mcandre/linters) - An introduction to static code analysis
* [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) -  A reviewed list of useful PHP static analysis tools
* [Tools for C/C++](https://www.peerlyst.com/posts/a-list-of-static-analysis-tools-for-c-c-peerlyst?utm_source=twitter&utm_medium=social&utm_content=peerlyst_post&utm_campaign=peerlyst_resources) - A list of static analysis tools for C/C++
* [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) -  A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).