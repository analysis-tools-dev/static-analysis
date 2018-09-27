.PHONY: install
install:
	pip install mkdocs==1.0.4
	pip install mkdocs-material==3.0.4
	pip install Markdown==3.0.1

.PHONY: link
link:
	ln -sf $(CURDIR)/README.md $(CURDIR)/docs/index.md
	ln -sf $(CURDIR)/awesome.png $(CURDIR)/docs/awesome.png

.PHONY: serve
serve: link
	mkdocs serve

.PHONY: deploy
deploy: link
	mkdocs gh-deploy

