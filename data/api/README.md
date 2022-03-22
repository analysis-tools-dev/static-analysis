# API

This directory contains machine-readable JSON files of all static analysis
tools in the repo. The files can be used to create your own API endpoints from
the data, for running more complicated queries on the command-line with `jq`, or
for using them inside a Jupyter notebook.

The data format is subject to change as we use it for rendering the [website].

The files in this directory are not meant to be edited directly. Instead, update
the `.yml` files in the `/data/tools` directory and render the JSON again by
calling `make render` from the root directory.

[website]: https://analysis-tools.dev
