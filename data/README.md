This folder contains the source data for the list.

- `tools/` - one YAML file per tool
- `tags.yml` - all valid tags that can be used in tool entries
- `api/` - generated JSON output (do not edit manually)

To add a tool, create a file in `tools/` following the format described in [CONTRIBUTING.md](../CONTRIBUTING.md).

The Rust code that renders the list lives in [`ci/`](../ci/).