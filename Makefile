.PHONY: render
render:
	cd data/render && cargo run -- --tags ../../data/tags.yml --tools ../../data/tools --out README.md
