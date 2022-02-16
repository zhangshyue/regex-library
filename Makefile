install:
	pip3 install tqdm
	pip3 install statistics
	python3 -m pip install -U pip
	python3 -m pip install -U matplotlib
	pip3 install -r requirements.txt
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
	cd ..