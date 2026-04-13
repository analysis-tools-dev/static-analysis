{{ marker }}
## Contributing criteria check

{% if reports.is_empty() %}
No new tool files detected in `data/tools/`. Nothing to check.
{% else %}
{% for report in reports %}
### [{{ report.status() }}] `{{ report.name }}`

{% if let Some(src) = report.source.as_ref() %}
Source: {{ src }}

{% endif %}
{% if let Some(note) = report.note.as_ref() %}
> **Note:** {{ note }}

{% endif %}
| Criterion | Result |
|---|---|
| Stars (min 20) | {{ report.stars.symbol() }} {{ report.stars.message() }} |
| Contributors (min 2) | {{ report.contributors.symbol() }} {{ report.contributors.message() }} |
| Age (min 3 months) | {{ report.age.symbol() }} {{ report.age.message() }} |

{% endfor %}
---

{% if any_failures %}
One or more tools do not meet the [contributing criteria](CONTRIBUTING.md) yet. We will keep this PR open. Feel free to update it once the thresholds are met.
{% else %}
All criteria passed. Thank you for your contribution.
{% endif %}
{% endif %}