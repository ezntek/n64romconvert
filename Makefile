export PATH := $(HOME)/.local/bin

all: build install make_localbin

build:
	cargo build --release

install:
	cp target/release/n64romconvert

make_localbin:
ifneq (,$(wildcard $(HOME)/.local/bin))
	mkdir $(HOME)/.local/bin
endif