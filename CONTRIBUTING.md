# How to add a new tool to the list

Please feel free to open a pull request if you know of a code analysis tool that is not mentioned here.  
If you're in doubt if a tool is a good fit for the list, **don't open an issue, but create a pull request right away** because that's easier to handle. Thanks! :smiley:

### Requirements

Each tool on the list should be 
* actively maintained (more than one contributor)
* actively used (have **more than 20 stars on Github or similar impact**)
* relatively mature (project exists for at least three months)

### Format

```Markdown
* [Project Name](https://github.com/author/project-name) - A short, but meaningful description. Maximum two lines long.
```
e.g.:

```Markdown
* [CMetrics](https://github.com/MetricsGrimoire/CMetrics) [OSS] - Measures size and complexity for C files
```

for **proprietary tools**, please add a "copyright emoji" (`:copyright:`):  

```Markdown
* [Functor Prevent](http://www.functor.se/products/prevent/) :copyright: - Static code analysis for C code.
```

### Ordering

Please make sure to keep the alphabetical, case-insensitive ordering of the
tools.

### Description length

Make each tool description as precise as possible.  
Please limit the description to **200 characters**.

### Categories

If you can, please limit yourself to only one category.  
This way, all tools get treated fairly and the list is easier to read.


# How to mark a tool as unmaintained/deprecated

Sometimes it happens that a tool becomes unmaintained and there's nothing wrong
with that.  
After all, a tool can still be very valuable to the community - even without
frequent updates.  
However, since it is one of the goals of this project to allow people to make an
informed decision on what is the best tool for the job, we are marking
unmaintained or deprecated tools with a :warning: (`:warning:`) sign.
This sign indicates that the community does not recommend to use this tool for
new projects anymore.

[Here](https://github.com/mre/awesome-static-analysis/issues/223) is a nice
discussion about why we think this is necessary. If you find a tool, which is
unmaintained, please create a pull request which adds the `:warning:` sign and
provide an objective explanation as to why you think the tool should be marked.
Every deprecation will be handled on a case-by-case basis.


**Thanks for helping out!** :tada:
