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
	cp ./Rocket.toml ./output
	cp ./gen_keys.sh ./output
	zip -r release.zip ./output

