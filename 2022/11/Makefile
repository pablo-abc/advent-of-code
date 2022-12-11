compile:
	rustc --out-dir build main.rs

run-test:
	make compile && cat test | ./build/main

run:
	make compile && cat input | ./build/main
