# See https://github.com/klutzy/rust-windows/blob/master/examples/Makefile

RUSTC?=rustc.exe
RUST_OPTS=
CODEGEN_OPTS=-C link-args="-Wl,--subsystem,windows"
WIN32_SRC=win32/mod.rs win32/constants.rs win32/types.rs win32/window.rs win32/wstr.rs
SRC=winman.rs $(WIN32_SRC)

.SUFFIXES:

.PHONY: all
all: winman.exe

winman.exe: $(SRC)
	$(RUSTC) -o $@ winman.rs $(CODEGEN_OPTS) $(RUST_OPTS)

.PHONY: clean
clean:
	rm -rf $(EXE)