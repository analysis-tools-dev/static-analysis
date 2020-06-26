.PHONY: render
render:
	(cd data/render && cargo run -- ../../data/tags.yml ../../data/tools.yml) > README.md
