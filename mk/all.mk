.PHONY: all run watch
all:
	cargo build
run:
	cargo run -- lib/$(APP).ini
watch:
	cargo watch -x 'run -- lib/$(APP).ini'
