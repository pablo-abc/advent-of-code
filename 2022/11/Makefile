compile:
	rustc --out-dir build main.rs

run-test:
	make compile && cat test | ./build/main

run-1:
	make compile && cat input | ./build/main

run-2:
	make compile && cat input | ./build/main --lcm
