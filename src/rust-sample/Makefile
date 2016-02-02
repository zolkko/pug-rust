all: src/lib.rs
	rm -Rf ./target
	cargo rustc --release -- \
		-C link-args=-L/System/Library/Frameworks/Python.framework/Versions/Current/lib/python2.7/ \
		-lpython2.7
	cp ./target/release/libsample.dylib ./sample.so

