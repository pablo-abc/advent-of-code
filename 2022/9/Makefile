compile:
	rustc --out-dir build main.rs

run-test:
	make compile && cat test | ./build/main

run-test2:
	make compile && cat test2 | ./build/main

run-1:
	make compile && cat input | ./build/main 2

run-2:
	make compile && cat input | ./build/main 10
