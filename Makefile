# Define required macros here
SHELL = /bin/sh

server:
	cd backend/ && cargo-watch -x run
