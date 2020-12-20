.PHONY: render
render:
	cargo run --manifest-path data/render/Cargo.toml -- --tags data/tags.yml --tools data/tools --out README.md

.PHONY: render-skip-deprecated
render-skip-deprecated:
	cargo run --manifest-path data/render/Cargo.toml -- --tags data/tags.yml --tools data/tools --out README.md --skip-deprecated
