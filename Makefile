.PHONY: all rust clean

all: rust example

rust:
	cargo build

example: example.c
	gcc -g -Wall -Werror -Ltarget/debug -lcrate_b -lcrate_a -Wl,-rpath=target/debug -o $@ $^

clean:
	cargo clean && rm -f example
