# Thank you for contributing

We welcome pull requests for static analysis tools that meet the requirements
below. **Please verify all criteria before submitting a tool.** If a tool does
not qualify yet, wait until it does rather than opening a pull request or issue.

### Requirements

Before submitting, each tool must

- have existed for at least six months
- have at least 20 stars on GitHub
- have more than one human contributor

These requirements apply regardless of whether a tool was built with AI
assistance. This is a curated list, not a launch or marketing channel. Please do
not open a pull request to promote a tool or leave one open while it gains stars,
contributors, or enough history to qualify.

The CI bot will politely close pull requests when it verifies that a tool does
not meet the criteria. You are welcome to submit a new pull request once **all**
criteria are met. If a check cannot be verified automatically (for example, for
a non-GitHub or proprietary tool), it requires manual review rather than
automatic closure. Meeting the minimum criteria does not guarantee inclusion.

### Format

⚠️ **The main `README.md` is just a rendered version of the data. Do not edit it
manually.**

To add a new tool, please create a file in the `data/tools` directory like
`data/tools/<toolname>.yml`. Feel free to check out a few other YAML files in
that directory to see how it should look like.

- Make each tool description as precise as possible.  Please limit the
  description to **500 characters**.
- Add a license. If it's a proprietary tool, use `license: proprietary`.
- Please add as many tags as possible. You can choose from the tags in
  `data/tags.yml`. If a tool does not match any existing tag, feel free to add a
  new tag but also add it to `data/tags.yml`.
- For AI-related tools, add `ai-generated-code` if the tool analyzes
  AI-generated code, and add `uses-llm` if it invokes an LLM or other model
  while analyzing code.

Finally, create a pull request with all your changes. You can call `make
render` to check for errors before.  This is optional, because it will also be
done when creating a pull request.

### How to mark a tool as unmaintained/deprecated

Sometimes a tool becomes unmaintained and there's nothing wrong with that.  
After all, a tool can still be very valuable to the community - even without
frequent updates.  
However, since it is one of the goals of this project to allow people to make an
informed decision on what is the best tool for the job, we are marking
unmaintained or deprecated tools after a while.
[Here](https://github.com/mre/awesome-static-analysis/issues/223) is a nice
discussion about why we think this is necessary. If you find a tool, which is
unmaintained, please add `deprecated: true` to the entry in `data/tools/` and
create a pull request in which you provide an objective explanation as to why
you think the tool should be marked deprecated. Every deprecation will be
handled on a case-by-case basis.

**Thanks for helping out!** :tada:
