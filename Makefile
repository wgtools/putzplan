all: package

run: build
	cargo run --release

build:
	cargo build --release

package: build
	mkdir -p ./output
	cp -r ./templates ./output
	cp -r ./static ./output
	cp ./target/release/putzplan ./output
	zip -r release.zip ./output

