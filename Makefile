SRC = $(shell find src/ -type f)
NAME = $(shell  pwd | xargs -n1 basename)
EXECUTIBLE = target/release/${NAME}
TARGET = /usr/local/bin

build: ${EXECUTIBLE}

${EXECUTIBLE}: ${SRC}
	cargo build --release

install: build
	cp ${EXECUTIBLE} ${TARGET}
