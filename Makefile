export PATH := $(HOME)/.local/bin:$(PATH)

all: make_localbin build install

make_localbin:
ifeq($(wildcard $(HOME)/.local/bin),)
	mkdir $(HOME)/.local/bin
endif

build:
	cargo build --release

clean:
ifneq($(wildcard ./target),)
	rm ./target
endif

install:
	$(shell cp -i target/release/n64rom{convert,type,convert-gui} $(HOME)/.local/bin/)

uninstall:
	rm $(HOME)/.local/bin/n64rom*

remove: uninstall
