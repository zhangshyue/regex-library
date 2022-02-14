install:
	rustup --version || (echo "You need to install rust, go to https://rustup.rs/"; exit 1)
	pip3 --version || (echo "You need to install python3 and pip3"; exit 1)
	git submodule init
	git submodule update --recursive --remote
	make install-generalizability
	make install-extraction
	make install-output

install-generalizability:
	cd generalizability && cargo build --release
	cd ..

install-extraction:
	cd extraction && cargo build --release
	cd ..

install-output:
	cd output && pip3 install -r requirements.txt
	cp output/root_pb2
	cd ..