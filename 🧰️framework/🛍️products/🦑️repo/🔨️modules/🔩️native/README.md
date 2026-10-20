# 🔩️ Native

The zero-touch bootstrap: the scripts that bring a fresh checkout to a working toolchain on
devcontainer, native windows, native macos and native linux, and the build lines the other modules
produce their binaries with.

## 📦️ Packages

None — this module carries `🥾️bootstrap` scripts rather than a language package.

## 🧪️ Tests

The repository-owned `repo:test-runtime-bootstrap` suite exercises six cold/warm native-entry and refusal cases through recording shell functions. `repo:test-devcontainer-lifecycle` exercises start/attach behavior through recording hosts. These bounded contracts complement actual setup runs on each supported host.

Human development starts with `repo dashboard` after native installation. The container post-create entry invokes the owned shell bootstrap, which installs the native entry before running repository setup. The Windows bootstrap uses the same repository setup control.
