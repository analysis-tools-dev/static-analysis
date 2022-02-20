.PHONY: render
render:
	cargo run --manifest-path data/render/Cargo.toml -- --tags data/tags.yml --tools data/tools --md-out README.md --json-out data/api/tools.json

.PHONY: render-skip-deprecated
render-skip-deprecated:
	cargo run --manifest-path data/render/Cargo.toml -- --tags data/tags.yml --tools data/tools --md-out README.md --json-out data/api/tools.json --skip-deprecated