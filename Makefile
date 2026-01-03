.PHONY: test test-01 test-02 test-03 test-04 test-05 test-06 run clean

test:
	cargo test

test-01:
	cargo test --test lesson_01

test-02:
	cargo test --test lesson_02

test-03:
	cargo test --test lesson_03

test-04:
	cargo test --test lesson_04

test-05:
	cargo test --test lesson_05

test-06:
	cargo test --test lesson_06

run:
	cargo run

clean:
	cargo clean
