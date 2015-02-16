# See https://github.com/klutzy/rust-windows/blob/master/examples/Makefile

RUSTC?=rustc.exe
RUST_OPTS=
CODEGEN_OPTS=-C link-args="res/winman.obj -Wl,--subsystem,windows"
WIN32_SRC=win32/mod.rs win32/constants.rs win32/win32_macros.rs win32/types.rs win32/window.rs win32/wstr.rs
APP_SRC=app/mod.rs app/window.rs app/dummy.rs app/hotkey.rs
SRC=winman.rs $(WIN32_SRC) $(APP_SRC)

.SUFFIXES:

.PHONY: all
all: winman.exe

winman.exe: $(SRC)
	$(RUSTC) -o $@ winman.rs $(CODEGEN_OPTS) $(RUST_OPTS)

.PHONY: clean
clean:
	rm -rf $(EXE)