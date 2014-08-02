# See https://github.com/klutzy/rust-windows/blob/master/examples/Makefile

RUSTC?=rustc.exe
RUST_OPTS=
CODEGEN_OPTS=-C link-args="-Wl,--subsystem,windows"
SRC=winman.rs
EXE=$(patsubst %.rs, %.exe, $(SRC))

.SUFFIXES:

.PHONY: all
all: $(EXE)

%.exe: %.rs
	$(RUSTC) -o $@ $< $(CODEGEN_OPTS) $(RUST_OPTS)

.PHONY: clean
clean:
	rm -rf $(EXE)