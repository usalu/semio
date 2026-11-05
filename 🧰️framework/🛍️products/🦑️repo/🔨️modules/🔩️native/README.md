# 🔩️ Native

The zero-touch bootstrap: the scripts that bring a fresh checkout to a working toolchain on
devcontainer, native windows, native macos and native linux, and the build lines the other modules
produce their binaries with.

## 📦️ Packages

None — this module carries `🥾️bootstrap` scripts rather than a language package.

## 🧪️ Tests

The repository-owned `repo:test-runtime-bootstrap` suite exercises six cold/warm native-entry and refusal cases through recording shell functions. `repo:test-devcontainer-lifecycle` exercises start/attach behavior through recording hosts. These bounded contracts complement actual setup runs on each supported host.

Human development starts with `repo dashboard` after native installation. The container post-create entry invokes the owned shell bootstrap, which installs the native entry before running repository setup. The Windows bootstrap uses the same repository setup control.

`repo ./📜️script.ts setup native rust` prepares the host C/C++ compiler and the pinned Rust toolchain. On Windows it verifies the pinned Visual Studio bootstrapper before launching it, repairs an existing installation when its required linker is missing, and checks linker availability after installation. With `SEMIO_TICKET_DIR` selected, installer temporary files stay physically inside that ticket’s generated directory; an owned ASCII junction or DOS alias accommodates native installers. Windows still displays its normal operating-system elevation dialog when installation requires it. The full `setup native` command retains the complete host bootstrap.
