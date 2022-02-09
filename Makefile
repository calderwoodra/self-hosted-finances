# Define required macros here
SHELL = /bin/sh

server:
	cd backend/ && cargo-watch -x run

psql:
	psql -U db-finance-user -d self-hosted-finances

cmd:
	cd cli/ && cargo-watch -x build
