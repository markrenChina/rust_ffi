all:demo

demo:main.c
	cargo build --release
	gcc -o demo main.c  -I/usr/include -L./target/release -ldemo

clean:
	rm demo