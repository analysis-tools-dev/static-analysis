.PHONY: render
render:
	cargo run --manifest-path data/render/Cargo.toml -- --tags data/tags.yml --tools data/tools --out README.md
