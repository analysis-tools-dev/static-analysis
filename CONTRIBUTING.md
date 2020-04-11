# How to add a new tool to the list

Please feel free to open a pull request if you know of a code analysis tool that
is not mentioned here.  
If you're in doubt if a tool is a good fit for the list, **don't open an issue,
but create a pull request right away** because that's easier to handle. Thanks!
:smiley:

### Requirements

Each tool on the list should be 
* actively maintained (more than one contributor)
* actively used (have **more than 20 stars on Github or similar impact**)
* relatively mature (project exists for at least three months)

### Format

The main `README.md` is just a rendered version of the data. To add a new tool,
please edit `data/data.yml`.

- Make each tool description as precise as possible.  
  Please limit the description to **500 characters**.
- Please keep the tools in alphabetical order.
- By default, we assume that the tool is open source.
  If a tool is proprietary, add `proprietary: true`.
- Please add as many categories as possible. You can choose from the categories
  in `data/categories.yml` If a tool does not match any existing category, feel
  free to start a new category.

Finally, create a pull request with all your changes.
You can call `make render` to check for errors before.  
This is optional, because it will also be done when creating 
a pull request.

# How to mark a tool as unmaintained/deprecated

Sometimes a tool becomes unmaintained and there's nothing wrong with that.  
After all, a tool can still be very valuable to the community - even without
frequent updates.  
However, since it is one of the goals of this project to allow people to make an
informed decision on what is the best tool for the job, we are marking
unmaintained or deprecated tools.
[Here](https://github.com/mre/awesome-static-analysis/issues/223) is a nice
discussion about why we think this is necessary. If you find a tool, which is
unmaintained, please add `deprecated: true` to the entry in `data/data.yml` and
create a pull request in which you provide an objective explanation as to why
you think the tool should be marked deprecated. Every deprecation will be
handled on a case-by-case basis.

**Thanks for helping out!** :tada:
