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
| Age (min 6 months) | {{ report.age.symbol() }} {{ report.age.message() }} |

{% endfor %}
---

{% if should_close %}
Thank you for sharing your tool! One or more tools do not yet meet the [contribution criteria](https://github.com/analysis-tools-dev/static-analysis/blob/master/CONTRIBUTING.md), as shown above, so we are closing this pull request.

Please wait until all criteria are met before submitting a tool. We do not keep pull requests open while tools become eligible, and this list is not a launch or marketing channel. You are welcome to submit a new pull request once all criteria are met. Thank you for understanding!
{% else if any_failures %}
Thank you for your contribution. One or more criteria could not be verified automatically, so this pull request needs manual review and will not be closed automatically. Please provide evidence for the unverified [contribution criteria](https://github.com/analysis-tools-dev/static-analysis/blob/master/CONTRIBUTING.md), or retry the check if the GitHub API was unavailable.
{% else %}
All tool eligibility criteria passed. Thank you for your contribution.
{% endif %}
{% endif %}