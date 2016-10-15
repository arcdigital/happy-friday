all:
	cargo build --release
	strip target/release/happy-friday
	zip -j happyFridayLambda.zip target/release/happy-friday index.js
