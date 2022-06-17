
# * HTTP SERVER
# * Shell
# * Lib
run-%:
	cargo run -p $*
build-%:
	cargo build -p $*
check-%:
	cargo check -p $*
test-%:
	cargo test -p $*

workspace-build:
	cargo build

# * Target cleaning

cls-target:
	rm -rf ./target
