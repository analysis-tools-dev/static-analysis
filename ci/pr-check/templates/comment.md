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
{% if let Some(domain) = report.domain.as_ref() %}| Homepage domain age (min 6 months; [RDAP record](https://rdap.org/domain/{{ domain }})) | {{ report.age.symbol() }} {{ report.age.message() }} |
{% else %}| Age (min 6 months) | {{ report.age.symbol() }} {{ report.age.message() }} |
{% endif %}

{% endfor %}
---

{% if should_close %}
Thank you for sharing your tool! One or more tools do not yet meet the [contribution criteria](https://github.com/analysis-tools-dev/static-analysis/blob/master/CONTRIBUTING.md), as shown above, so we are closing this pull request.

You are welcome to submit a new pull request once all criteria are met. Thank you for your contribution!
{% else if any_failures %}
Thank you for your contribution. One or more criteria could not be verified automatically, so this pull request needs manual review and will not be closed automatically. Please provide evidence for the unverified [contribution criteria](https://github.com/analysis-tools-dev/static-analysis/blob/master/CONTRIBUTING.md), or retry the check if an external API was unavailable.
{% else %}
All tool eligibility criteria passed. Thank you for your contribution.
{% endif %}
{% endif %}