# How to add a new tool to the list

Please feel free to open a pull request if you know of a code analysis tool that is not mentioned here.  
If you're in doubt if a tool is a good fit for the list, **don't open an issue, but create a pull request right away** because that's easier to handle. Thanks! :smiley:

### Requirements

Each tool on the list should be 
* actively maintained
* actively used (have more than ten Stars on Github or similar impact)
* relatively mature (project exists for at least one month)

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


# Deprecating tools

Sometimes it happens that you come across a tool, which might not be a good fit for the list anymore. For example, it has not received any updates in a very long time.
In such a case, please remember that the tool can still be very valuable to the community - it might just not be a good fit for your specific use-case.  

If you still find that a tool should be avoided, please create a pull request which adds an `:information_source:` sign and provide an objective explanation as to why you think the tool should be marked. If possible, please also mention the project maintainers there, so we can find a solution together.

Please keep in mind that behind every project there are human beings which put a lot of effort into building and maintaining a project, so be nice and constructive.

**Thanks for helping out!** :tada:
