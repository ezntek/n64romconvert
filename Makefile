.PHONY: clean

CLEAN_DIRNAMES := target/debug target/release
CLEAN_DIRS := $(strip $(foreach dir,$(CLEAN_DIRNAMES),$(wildcard $(dir))))
UNINSTALL_FILENAMES := $(wildcard $(HOME)/.local/bin/n64rom*)
UNINSTALL_FILES := $(strip $(foreach file, $(UNINSTALL_FILENAMES),$(wildcard $(file))))

export PATH := $(HOME)/.local/bin:$(PATH)

docs:
	cargo doc --no-deps

all: make_localbin build install

install:
	cp target/release/n64romconvert target/release/n64romtype $(HOME)/.local/bin/

build:
	cargo build --release

make_localbin:
	[ -d $(HOME)/.local/bin ] || mkdir -p $(HOME)/.local/bin

clean: 
ifneq (,$(CLEAN_DIRS))
	rm -r $(CLEAN_DIRS)
endif

uninstall:
ifneq (,$(UNINSTALL_FILES))
	rm -r $(UNINSTALL_FILES)
endif