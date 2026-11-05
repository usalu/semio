import { expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, readlinkSync, symlinkSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import {createRequire} from "node:module";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import corpus from "./🔣️.json";
const semver:{gte(actual:string,required:string):boolean}=createRequire(import.meta.url)("semver");


const root = findWorkspaceRoot(import.meta.dir);
const bootstrap = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh"), "utf8");
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("Bootstrap toolchain tests require SEMIO_TEST_ARTIFACT_DIR");
const generated = resolve(output, `bt-${process.pid}`);
mkdirSync(generated, { recursive: true });
const body = (name: string, next: string) => bootstrap.slice(bootstrap.indexOf(name + "() {"), bootstrap.indexOf(next + "() {"));
const execute = (script: string, env: Record<string, string> = {}) => spawnSync("bash", ["-c", script], { encoding: "utf8", env: { ...process.env, ...env } });

test("bootstrap version ordering is portable and agrees with the neutral corpus and semver", () => {
  expect(new Ajv().validate({ type: "array", items: { type: "object", required: ["actual", "required", "expected"], properties: { actual: { type: "string" }, required: { type: "string" }, expected: { type: "boolean" } }, additionalProperties: false } }, corpus.versions)).toBe(true);
  for (const row of corpus.versions) {
    expect(semver.gte(row.actual, row.required)).toBe(row.expected);
    const result = execute(body("version_at_least", "ensure_uv") + '\nsort() { return 2; }\nversion_at_least "$ACTUAL" "$REQUIRED"', { ACTUAL: row.actual, REQUIRED: row.required });
    expect(result.status === 0, JSON.stringify(row)).toBe(row.expected);
  }
});

test("Linux native packages include compiler and ICU development inputs", () => {
  for (const row of corpus.linuxPackages) {
    const result = execute(body("linux_toolchain_packages", "install_linux_packages") + '\nlinux_toolchain_packages "$MANAGER"', { MANAGER: row.manager });
    expect(result.status).toBe(0);
    for (const required of row.required) expect(result.stdout.trim().split("\n")).toContain(required);
  }
});

test("Rust bootstrap keeps shell profiles untouched", () => {
  const installer = join(generated, "rustup-init"), args = join(generated, "args.txt"), toolchainArgs = join(generated, "toolchain-args.txt");
  writeFileSync(installer, '#!/usr/bin/env bash\nprintf "%s\\n" "$@" > "$SEMIO_TEST_ARGUMENTS"\n');
  const script = body("ensure_rust_toolchain", "ensure_node") + `
log() { :; }
host_pin() { printf "{}"; }
pin_field() { printf "test"; }
fetch_pinned() { cp "$SEMIO_TEST_INSTALLER" "$2"; }
command() { if [ "$1" = "-v" ] && [ "$2" = rustup ]; then return 1; fi; builtin command "$@"; }
rustup() { printf "%s\\n" "$@" > "$SEMIO_TEST_TOOLCHAIN_ARGUMENTS"; }
ensure_rust_toolchain
`;
  const result = execute(script, { REPO_ROOT: root, CARGO_HOME: join(generated, "cargo"), RUSTUP_HOME: join(generated, "rustup"), TMPDIR: generated, SEMIO_TEST_INSTALLER: installer, SEMIO_TEST_ARGUMENTS: args, SEMIO_TEST_TOOLCHAIN_ARGUMENTS: toolchainArgs });
  expect(result.status, result.stderr).toBe(0);
  const actual = readFileSync(args, "utf8").trim().split("\n");
  for (const argument of corpus.rustupRequiredArguments) expect(actual).toContain(argument);
  expect(readFileSync(toolchainArgs, "utf8").trim().split("\n")).toEqual(corpus.rustupToolchainArguments);
  expect(bootstrap).toContain("unset RUSTUP_TOOLCHAIN");
});

test("Windows Rust bootstrap keeps profiles and the pinned installer untouched", () => {
  const windows = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"), "utf8");
  const installer = windows.match(/^\s*& \$installer (.+) \| Out-Host$/mu)?.[1]?.split(/\s+/u);
  expect(installer).toBeDefined();
  for (const argument of corpus.rustupRequiredArguments) expect(installer).toContain(argument);
  expect(windows).toContain("Remove-Item Env:RUSTUP_TOOLCHAIN -ErrorAction SilentlyContinue");
  const toolchain = windows.match(/-FilePath \$rustupPath -ArgumentList @\(([^)]+)\) -WorkingDirectory \$repoRoot/u)?.[1];
  expect(toolchain).toBeDefined();
  const args = JSON.parse("[" + toolchain + "]");
  expect(new Ajv().validate({ type: "array", items: { type: "string" } }, args)).toBe(true);
  expect(args).toEqual(corpus.rustupToolchainArguments);
  expect(windows).toContain("function Get-PinnedGoDirectory");
  expect(windows).toContain(".cache\\b\\t\\m\\native\\go\\");
  expect(windows).toContain('Get-FirstCommandPath @("go.exe")');
  expect(windows).toContain('Get-FirstCommandPath @("uvx.exe")');
  expect(windows).toContain('Get-FirstCommandPath @("npm.cmd", "npm.exe")');
  expect(windows).toContain('Join-Path $nodeRoot "npm.cmd"');
  expect(windows).toContain('Get-FirstCommandPath @("node.exe")');
  expect(windows).toContain("Split-Path -Parent $nodeFound");
  expect(windows).toContain("Split-Path -Parent $uvFound");

});

test("container graphics has a Vulkan loader and software ICD", () => {
  const dockerfile = readFileSync(join(root, ".devcontainer/Dockerfile"), "utf8");
  const install = dockerfile.match(/RUN apt-get update && apt-get install -y --no-install-recommends ([\s\S]*?)&& apt-get clean/u)?.[1];
  expect(install).toBeDefined();
  const packages = install!.replaceAll("\\", "").split(/\s+/u).filter(Boolean);
  for (const required of corpus.imageRequiredPackages) {
    expect(packages).toContain(required);
    expect(new Ajv().validate({ type: "array", contains: { const: required } }, packages)).toBe(true);
  }
});

test("container dependency projections stay on the Source filesystem", () => {
  const source = readFileSync(join(root, ".devcontainer/devcontainer.json"), "utf8");
  const config = Bun.JSONC.parse(source);
  if(!config||typeof config!=="object"||!("mounts" in config)||!Array.isArray(config.mounts)||!config.mounts.every(value=>typeof value==="string")||!("onCreateCommand" in config)||(typeof config.onCreateCommand!=="string"&&(!Array.isArray(config.onCreateCommand)||!config.onCreateCommand.every(value=>typeof value==="string")))||!("workspaceFolder" in config)||typeof config.workspaceFolder!=="string")throw Error("Declared Source projection configuration required");
  expect(config).toEqual(JSON.parse(source));
  const fixture = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧬️schema/🔣️.json"), "utf8"));
  expect(new Ajv().validate(schema.$defs.SourceDependencyProjection, { sourceEndpointBasename: fixture.sourceEndpointBasename })).toBe(true);
  const targets = config.mounts.map((mount: string) => Object.fromEntries(mount.split(",").map(pair => pair.split("=")))).map((mount: Record<string, string>) => mount.target);
  for (const target of targets) {
    const basename = target.split("/").at(-1);
    expect(basename).not.toBe(fixture.sourceEndpointBasename);
    expect(new Ajv().validate({ not: { const: fixture.sourceEndpointBasename } }, basename)).toBe(true);
  }
  expect(config.onCreateCommand).not.toContain(config.workspaceFolder + "/" + fixture.sourceEndpointBasename);
});

test("user-local pkg-config matches the neutral corpus without a system package", () => {
  expect(new Ajv().validate({type:"object", required:["version","modversion","cflags","libs","prefix","libsOnlyL","libsOnlyLdir","cflagsOnlyI","definedPrefix"], properties:{version:{type:"string"}, modversion:{type:"string"}, cflags:{type:"string"}, libs:{type:"string"}, prefix:{type:"string"}, libsOnlyL:{type:"string"}, libsOnlyLdir:{type:"string"}, cflagsOnlyI:{type:"string"}, definedPrefix:{type:"string"}}, additionalProperties:false}, corpus.pkgConfig)).toBe(true);
  const start = bootstrap.indexOf("cat > \"$LOCAL_BIN/pkg-config\" <<'EOF'\n");
  const end = bootstrap.indexOf("\nEOF\n  chmod 755 \"$LOCAL_BIN/pkg-config\"", start);
  expect(start).toBeGreaterThan(0);
  expect(end).toBeGreaterThan(start);
  const script = join(generated, "pkg-config");
  writeFileSync(script, bootstrap.slice(start + "cat > \"$LOCAL_BIN/pkg-config\" <<'EOF'\n".length, end) + "\n", {mode:0o755});
  const tree = join(generated, "pc");
  mkdirSync(tree, {recursive:true});
  writeFileSync(join(tree, "base.pc"), "prefix=/opt/base\nName: Base\nVersion: 1.0.0\nCflags: -I${prefix}/include\nLibs: -L${prefix}/lib -lbase\n");
  writeFileSync(join(tree, "demo.pc"), "prefix=/opt/demo\nlibdir=${prefix}/lib\nName: Demo\nVersion: 1.2.3\nRequires: base\nCflags: -I${prefix}/include\nLibs: -L${libdir} -ldemo\n");
  const run = (args: string[]) => spawnSync(script, args, {encoding:"utf8", env:{...process.env, PKG_CONFIG_LIBDIR: tree, PKG_CONFIG_PATH: ""}});
  expect(run(["--version"]).stdout.trim()).toBe(corpus.pkgConfig.version);
  expect(run(["--modversion", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.modversion);
  expect(run(["--cflags", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.cflags);
  expect(run(["--libs", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.libs);
  expect(run(["--variable=prefix", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.prefix);
  expect(run(["--exists", "demo"]).status).toBe(0);
  expect(run(["--exists", "missing"]).status).not.toBe(0);
  writeFileSync(join(tree, "newer.pc"), "Name: Newer\nVersion: 2.0.0\nRequires: base >= 2\nLibs: -lnewer\n");
  writeFileSync(join(tree, "older.pc"), "Name: Older\nVersion: 1.0.0\nRequires: base >= 1.0, base = 1.0.0\nLibs: -lolder\n");
  expect(run(["--exists", "newer"]).status).not.toBe(0);
  expect(run(["--exists", "older"]).status).toBe(0);
  expect(run(["--libs", "older"]).stdout.trim()).toBe("-L/opt/base/lib -lbase -lolder");
  writeFileSync(join(tree, "multi.pc"), "Name: Multi\nVersion: 1\nCflags.private: -DPRIVATE\nCflags: -I/opt/multi/include\nCflags: -DMULTI\nLibs: -L/opt/multi/lib\nLibs: -lmulti\nLibs.private: -lmultiprivate\n");
  expect(run(["--cflags", "multi"]).stdout.trim()).toBe("-I/opt/multi/include -DMULTI");
  expect(run(["--libs", "multi"]).stdout.trim()).toBe("-L/opt/multi/lib -lmulti");
  expect(run(["--static", "--cflags", "multi"]).stdout.trim()).toBe("-DPRIVATE -I/opt/multi/include -DMULTI");
  expect(run(["--static", "--libs", "multi"]).stdout.trim()).toBe("-L/opt/multi/lib -lmulti -lmultiprivate");


  writeFileSync(join(tree, "shared.pc"), "Name: Shared\nVersion: 1.0.0\nRequires.private: missingprivate\nCflags: -I/opt/shared/include\nLibs: -L/opt/shared/lib -lshared\nLibs.private: -lprivate\n");
  expect(run(["--exists", "shared"]).status).toBe(0);
  expect(run(["--cflags", "shared"]).stdout.trim()).toBe("-I/opt/shared/include");
  expect(run(["--libs", "shared"]).stdout.trim()).toBe("-L/opt/shared/lib -lshared");
  expect(run(["--static", "--exists", "shared"]).status).not.toBe(0);
  writeFileSync(join(tree, "missingprivate.pc"), "Name: MissingPrivate\nVersion: 1.0.0\nLibs: -lmissingprivate\n");
  expect(run(["--static", "--exists", "shared"]).status).toBe(0);
  expect(run(["--static", "--libs", "shared"]).stdout.trim()).toBe("-lmissingprivate -L/opt/shared/lib -lshared -lprivate");
  expect(run(["--libs", "shared"]).stdout.trim()).toBe("-L/opt/shared/lib -lshared");

  expect(readFileSync(script, "utf8")).toContain("/usr/lib/x86_64-linux-gnu/pkgconfig");
  expect(readFileSync(script, "utf8")).toContain("/usr/lib/aarch64-linux-gnu/pkgconfig");
  expect(readFileSync(script, "utf8")).toContain("/usr/local/lib/pkgconfig");
  expect(readFileSync(script, "utf8")).toContain("/opt/homebrew/lib/pkgconfig");

  expect(run(["--print-errors", "--short-errors", "--exists", "demo"]).status).toBe(0);
  expect(run(["--libs-only-l", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.libsOnlyL);
  expect(run(["--libs-only-L", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.libsOnlyLdir);
  expect(run(["--cflags-only-I", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.cflagsOnlyI);
  expect(run(["--define-variable=prefix=" + corpus.pkgConfig.definedPrefix, "--variable=prefix", "demo"]).stdout.trim()).toBe(corpus.pkgConfig.definedPrefix);
  expect(bootstrap).toContain("sudo -n \"$@\"");
  expect(bootstrap).toContain("ensure_user_cc");
  expect(bootstrap).toContain("host_pin zig");
  const pkgHome = join(generated, "pkg-home");
  const pkgBin = join(generated, "pkg-bin");
  const pkgSystem = join(generated, "pkg-system");
  mkdirSync(pkgBin, {recursive: true});
  mkdirSync(pkgSystem, {recursive: true});
  writeFileSync(join(pkgBin, "pkg-config"), "#!/bin/sh\nif [ \"$1\" = --version ]; then printf '0.0.0\\n'; exit 0; fi\nexit 1\n", {mode: 0o755});
  const pkgBody = body("ensure_user_pkg_config", "report_missing_tools") + "\nensure_user_pkg_config\n";
  const refreshedPkg = execute(pkgBody, {HOME: pkgHome, LOCAL_BIN: pkgBin, PATH: `${pkgBin}:/usr/bin:/bin`});
  expect(refreshedPkg.status, refreshedPkg.stderr).toBe(0);
  expect(spawnSync(join(pkgBin, "pkg-config"), ["--version"], {encoding: "utf8"}).stdout.trim()).toBe(corpus.pkgConfig.version);
  writeFileSync(join(pkgSystem, "pkg-config"), "#!/bin/sh\nprintf '1.8.1\\n'\n", {mode: 0o755});
  const keptSystem = execute(pkgBody, {HOME: pkgHome, LOCAL_BIN: join(generated, "pkg-empty"), PATH: `${pkgSystem}:/usr/bin:/bin`});
  expect(keptSystem.status, keptSystem.stderr).toBe(0);
  expect(existsSync(join(generated, "pkg-empty", "pkg-config"))).toBe(false);
  const current = join(generated, "pkg-current");
  mkdirSync(current, {recursive: true});
  writeFileSync(join(current, "pkg-config"), "#!/bin/sh\nprintf '0.7.0\\n'\n# kept\n", {mode: 0o755});
  const keptCurrent = execute(pkgBody, {HOME: pkgHome, LOCAL_BIN: current, PATH: `${current}:/usr/bin:/bin`});
  expect(keptCurrent.status, keptCurrent.stderr).toBe(0);
  expect(readFileSync(join(current, "pkg-config"), "utf8")).toContain("# kept");
  const stale = join(generated, "pkg-stale");
  mkdirSync(stale, {recursive: true});
  writeFileSync(join(stale, "pkg-config"), "#!/bin/sh\nprintf '0.6.0\\n'\n# stale\n", {mode: 0o755});
  const replaced = execute(pkgBody, {HOME: pkgHome, LOCAL_BIN: stale, PATH: `${stale}:/usr/bin:/bin`});
  expect(replaced.status, replaced.stderr).toBe(0);
  expect(readFileSync(join(stale, "pkg-config"), "utf8")).not.toContain("# stale");
  expect(spawnSync(join(stale, "pkg-config"), ["--version"], {encoding: "utf8"}).stdout.trim()).toBe(corpus.pkgConfig.version);

});

test("user tools are linked into the local bin and musl glibc pins are refused", () => {
  const start = bootstrap.indexOf("link_user_tool() {");
  const end = bootstrap.indexOf("\nensure_launcher() {", start);
  expect(start).toBeGreaterThan(0);
  expect(end).toBeGreaterThan(start);
  const bin = join(generated, "bin"), source = join(generated, "repo");
  mkdirSync(bin, {recursive: true});
  writeFileSync(source, "#!/bin/sh\nprintf linked\\n\n", {mode: 0o755});
  const result = execute(bootstrap.slice(start, end) + `\nLOCAL_BIN="$BIN"\nlink_user_tool "$SOURCE" repo\nlink_user_tool "$MISSING" cargo\n`, {BIN: bin, SOURCE: source, MISSING: join(generated, "missing")});
  expect(result.status, result.stderr).toBe(0);
  expect(readFileSync(join(bin, "repo"), "utf8")).toContain("linked");
  expect(existsSync(join(bin, "cargo"))).toBe(false);
  const main = bootstrap.slice(bootstrap.indexOf("#region 🔖️Main"));
  expect(main.indexOf("host_is_musl")).toBeLessThan(main.indexOf("ensure_native_toolchain"));
  expect(main).toContain("exit 1");
  expect(bootstrap).toContain('link_user_tool "$CARGO_HOME/bin/rustc" rustc');
  expect(bootstrap).toContain('link_user_tool "$LAUNCHER_BIN/repo" repo');
  expect(bootstrap).toContain('partition="$REPO_ROOT/.cache/b/t/m/native/go/$version/$platform-$arch-${sha:0:16}/bin/go"');
  const goBin = join(generated, "go-bin"), repo = join(generated, "go-repo");
  const partition = join(repo, ".cache/b/t/m/native/go/9/linux-x64-abcdefabcdefabcd/bin");
  mkdirSync(partition, {recursive: true});
  writeFileSync(join(partition, "go"), "#!/bin/sh\nprintf go-pin\\n\n", {mode: 0o755});
  writeFileSync(join(partition, "gofmt"), "#!/bin/sh\nprintf fmt\\n\n", {mode: 0o755});
  const linked = execute(bootstrap.slice(start, end) + `
LOCAL_BIN="$BIN"
REPO_ROOT="$REPO"
host_pin() { printf '%s' '{"tool":"go","version":"9","sha256":"abcdefabcdefabcdffffffffffffffffffffffffffffffffffffffffffffffff","platform":"linux","arch":"x64"}'; }
pin_field() { case "$2" in version) printf 9 ;; sha256) printf abcdefabcdefabcdffffffffffffffffffffffffffffffffffffffffffffffff ;; esac; }
host_platform() { printf linux; }
host_arch() { printf x64; }
link_pinned_go
`, {BIN: goBin, REPO: repo});
  expect(linked.status, linked.stderr).toBe(0);
  expect(readFileSync(join(goBin, "go"), "utf8")).toContain("go-pin");
  expect(readFileSync(join(goBin, "gofmt"), "utf8")).toContain("fmt");
  expect(main).not.toContain("Open a new shell");
});

test("Linux session persistence keeps the Playwright cache inside the checkout", () => {
  const start = bootstrap.indexOf("persist_linux_session() {");
  const end = bootstrap.indexOf("\nif host_is_musl; then", start);
  expect(start).toBeGreaterThan(0);
  expect(end).toBeGreaterThan(start);
  const home = join(generated, "session-home"), repo = join(generated, "session-repo");
  mkdirSync(home, {recursive: true});
  const result = execute(bootstrap.slice(start, end) + "\npersist_linux_session\n", {HOME: home, XDG_CONFIG_HOME: join(home, "config"), REPO_ROOT: repo, CMAKE_PRESET: ""});
  expect(result.status, result.stderr).toBe(0);
  const file = readFileSync(join(home, "config", "environment.d", "semio.conf"), "utf8");
  const playwright = `${repo}/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`;
  expect(file).toContain("CMAKE_PRESET=linux-prototype");
  expect(file).toContain(`PLAYWRIGHT_BROWSERS_PATH="${playwright}"`);
  expect(file).toContain("DOTNET_CLI_TELEMETRY_OPTOUT=1");
  expect(existsSync(playwright)).toBe(true);
  expect(file.includes("/home/deck")).toBe(false);
  expect(file.includes("/Users/")).toBe(false);
});

test("user-local Zig wrappers target host glibc unless a target is already set", () => {
  const home = join(generated, "zig-home"), bin = join(generated, "zig-bin"), data = join(generated, "zig-data");
  const zig = join(data, "semio", "zig", "v0.14.1", "zig"), recorded = join(generated, "zig-args");
  mkdirSync(join(data, "semio", "zig", "v0.14.1"), {recursive: true});
  writeFileSync(zig, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$SEMIO_TEST_ZIG_ARGS\"\n", {mode: 0o755});
  const script = body("ensure_user_cc", "ensure_user_pkg_config") + `
log() { :; }
host_pin() { printf '%s\\n' '{"version":"0.14.1"}'; }
pin_field() { printf '%s\\n' 0.14.1; }
ensure_user_cc
`;
  const result = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: bin, PATH: "/usr/bin:/bin", SEMIO_TEST_ZIG_ARGS: recorded});
  expect(result.status, result.stderr).toBe(0);
  const machine = spawnSync("uname", ["-m"], {encoding: "utf8"}).stdout.trim();
  const triple = machine === "x86_64" || machine === "amd64" ? "x86_64-linux-gnu" : machine === "aarch64" || machine === "arm64" ? "aarch64-linux-gnu" : "";
  const wrapper = join(bin, "c++");
  expect(readFileSync(wrapper, "utf8")).toContain(triple || "exec");
  expect(readFileSync(wrapper, "utf8")).toContain("x86_64-macos");
  expect(readFileSync(wrapper, "utf8")).toContain("aarch64-macos");
  const darwinStart = bootstrap.indexOf("\n  Darwin)\n");
  const darwinEnd = bootstrap.indexOf("\n  Linux)\n", darwinStart);
  expect(darwinStart).toBeGreaterThan(0);
  expect(darwinEnd).toBeGreaterThan(darwinStart);
  const darwin = bootstrap.slice(darwinStart, darwinEnd);
  expect(darwin).toContain("ensure_user_cc");
  expect(darwin).toContain("ensure_user_pkg_config");

  expect(readlinkSync(join(bin, "zig"))).toBe(zig);
  writeFileSync(wrapper, readFileSync(wrapper, "utf8") + "\n# kept\n");
  const kept = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: bin, PATH: `${bin}:/usr/bin:/bin`, SEMIO_TEST_ZIG_ARGS: recorded});
  expect(kept.status, kept.stderr).toBe(0);
  expect(readFileSync(wrapper, "utf8")).toContain("# kept");
  expect(readlinkSync(join(bin, "zig"))).toBe(zig);

  const plain = spawnSync(wrapper, ["-dumpmachine"], {encoding: "utf8", env: {...process.env, SEMIO_TEST_ZIG_ARGS: recorded}});
  expect(plain.status, plain.stderr).toBe(0);
  const plainArgs = readFileSync(recorded, "utf8").trim().split("\n");
  if (triple) expect(plainArgs.slice(0, 3)).toEqual(["c++", "-target", triple]);
  const explicit = spawnSync(wrapper, ["-target", "aarch64-linux-musl", "-dumpmachine"], {encoding: "utf8", env: {...process.env, SEMIO_TEST_ZIG_ARGS: recorded}});
  expect(explicit.status, explicit.stderr).toBe(0);
  expect(readFileSync(recorded, "utf8").trim().split("\n")).toEqual(["c++", "-target", "aarch64-linux-musl", "-dumpmachine"]);
});

test("stale user-local Zig wrappers gain a host glibc target and system compilers stay", () => {
  const home = join(generated, "refresh-home"), bin = join(generated, "refresh-bin"), data = join(generated, "refresh-data"), system = join(generated, "refresh-system");
  const zig = join(data, "semio", "zig", "v0.14.1", "zig");
  mkdirSync(join(data, "semio", "zig", "v0.14.1"), {recursive: true});
  mkdirSync(bin, {recursive: true});
  mkdirSync(system, {recursive: true});
  writeFileSync(zig, "#!/bin/sh\nexit 0\n", {mode: 0o755});
  for (const name of ["cc", "c++"]) writeFileSync(join(bin, name), "#!/bin/sh\nexec /old/zig " + (name === "cc" ? "cc" : "c++") + " \"$@\"\n", {mode: 0o755});
  const script = body("ensure_user_cc", "ensure_user_pkg_config") + `
log() { :; }
host_pin() { printf '%s\\n' '{"version":"0.14.1"}'; }
pin_field() { printf '%s\\n' 0.14.1; }
ensure_user_cc
`;
  const refreshed = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: bin, PATH: `${bin}:/usr/bin:/bin`});
  expect(refreshed.status, refreshed.stderr).toBe(0);
  expect(readFileSync(join(bin, "c++"), "utf8")).toContain("-target");
  for (const name of ["cc", "c++"]) writeFileSync(join(bin, name), "#!/bin/sh\nexec /old/zig " + name + " -target aarch64-macos \"$@\"\n# wrong-os\n", {mode: 0o755});
  const wrongOs = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: bin, PATH: `${bin}:/usr/bin:/bin`});
  expect(wrongOs.status, wrongOs.stderr).toBe(0);
  expect(readFileSync(join(bin, "c++"), "utf8")).not.toContain("# wrong-os");
  const hostTriple = spawnSync("uname", ["-m"], {encoding: "utf8"}).stdout.trim() === "aarch64" || spawnSync("uname", ["-m"], {encoding: "utf8"}).stdout.trim() === "arm64" ? "aarch64-linux-gnu" : "x86_64-linux-gnu";
  expect(readFileSync(join(bin, "c++"), "utf8")).toContain(hostTriple);
  const selected = join(generated, "selected-compiler");
  mkdirSync(selected, {recursive: true});
  const selectedDriver = '#!/bin/sh\nexec /vendor/clang -target aarch64-macos "$@"\n';
  for (const name of ["cc", "c++"]) writeFileSync(join(selected, name), selectedDriver, {mode: 0o755});
  const preserved = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: selected, PATH: `${selected}:/usr/bin:/bin`});
  expect(preserved.status, preserved.stderr).toBe(0);
  for (const name of ["cc", "c++"]) expect(readFileSync(join(selected, name), "utf8")).toBe(selectedDriver);
  for (const name of ["cc", "c++"]) writeFileSync(join(system, name), "#!/bin/sh\nprintf system\\n\n", {mode: 0o755});
  const kept = execute(script, {HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: join(generated, "refresh-empty"), PATH: `${system}:/usr/bin:/bin`});
  expect(kept.status, kept.stderr).toBe(0);
  expect(readFileSync(join(system, "cc"), "utf8")).toContain("system");
  expect(existsSync(join(generated, "refresh-empty", "cc"))).toBe(false);
  const outerBin = join(generated, "outer-bin"), outerData = join(generated, "outer-data"), outerSystem = join(generated, "outer-system");
  mkdirSync(join(outerData, "semio", "zig", "v0.14.1"), {recursive: true});
  mkdirSync(outerBin, {recursive: true});
  mkdirSync(outerSystem, {recursive: true});
  writeFileSync(join(outerData, "semio", "zig", "v0.14.1", "zig"), "#!/bin/sh\nexit 0\n", {mode: 0o755});
  for (const name of ["cc", "c++"]) writeFileSync(join(outerBin, name), "#!/bin/sh\nexec /old/zig " + name + " \"$@\"\n", {mode: 0o755});
  const outer = body("ensure_user_cc", "ensure_user_pkg_config") + body("ensure_native_toolchain", "ensure_rust_toolchain") + `
log() { :; }
host_pin() { printf '%s\n' '{"version":"0.14.1"}'; }
pin_field() { printf '%s\n' 0.14.1; }
ensure_user_pkg_config() { :; }
report_missing_tools() { :; }
uname() { if [ "$1" = -s ]; then printf Linux; else command uname "$@"; fi; }
ensure_native_toolchain
`;
  const throughSetup = execute(outer, {HOME: join(generated, "outer-home"), XDG_DATA_HOME: outerData, LOCAL_BIN: outerBin, PATH: `${outerBin}:/usr/bin:/bin`});
  expect(throughSetup.status, throughSetup.stderr).toBe(0);
  expect(readFileSync(join(outerBin, "c++"), "utf8")).toContain("-target");
  for (const name of ["cc", "c++"]) writeFileSync(join(outerSystem, name), "#!/bin/sh\nprintf system\n\n", {mode: 0o755});
  const throughSystem = execute(outer, {HOME: join(generated, "outer-home"), XDG_DATA_HOME: outerData, LOCAL_BIN: join(generated, "outer-empty"), PATH: `${outerSystem}:/usr/bin:/bin`});
  expect(throughSystem.status, throughSystem.stderr).toBe(0);
  expect(existsSync(join(generated, "outer-empty", "cc"))).toBe(false);
});

test("Windows debugger setup continues when install privileges are missing", () => {
  const windows = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"), "utf8");
  expect(windows).toContain('run", "@semio-tech/repo-developer-tools:setup-debugger"');
  expect(windows).toContain("Native debugger setup needs noninteractive install privileges or a working LLDB already on PATH. Setup continues.");
  expect(windows.indexOf("setup-debugger")).toBeGreaterThan(windows.indexOf('run", "workspace:setup"'));
});

test("Linux lightningcss follows ldd and treats a missing glibc report as glibc", async () => {
  const { linuxNativeLibcV1, linuxHostLibcV1 } = await import('../../../../📚️library/🎛️commands/🔧️setup/🟦️.ts');
  expect(linuxNativeLibcV1("ldd (Ubuntu GLIBC 2.39-0ubuntu8.4) 2.39")).toBe("gnu");
  expect(linuxNativeLibcV1("")).toBe("gnu");
  expect(linuxNativeLibcV1("musl libc (x86_64)\nVersion 1.2.5")).toBe("musl");
  expect(linuxHostLibcV1()).toBe("gnu");
});

test("pinned Node links npm when the matching node has no npm", () => {
  const version = JSON.parse(readFileSync(join(root, "package.json"), "utf8")).engines.node as string;
  const home = join(generated, "node-home"), data = join(generated, "node-data"), bin = join(data, "semio", "node", `v${version}`, "bin"), local = join(home, ".local", "bin"), kept = join(generated, "node-kept");
  mkdirSync(bin, { recursive: true });
  mkdirSync(kept, { recursive: true });
  for (const name of ["node", "npm", "npx", "corepack"]) writeFileSync(join(bin, name), "#!/bin/sh\n", { mode: 0o755 });
  const head = body("ensure_node", "dotnet_sdk_pin") + `
log() { :; }
host_pin() { printf 'pin called\\n' >&2; return 1; }
node() { printf 'v%s\\n' "$SEMIO_NODE_VERSION"; }
`;
  const result = execute(head + "ensure_node\n", { REPO_ROOT: root, HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: local, PATH: "/usr/bin:/bin", SEMIO_NODE_VERSION: version });
  expect(result.status, result.stderr).toBe(0);
  expect(result.stderr).not.toContain("pin called");
  for (const name of ["node", "npm", "npx", "corepack"]) expect(readFileSync(join(local, name), "utf8")).toBe("#!/bin/sh\n");
  const skip = execute(head + "npm() { :; }\nensure_node\n", { REPO_ROOT: root, HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: kept, PATH: "/usr/bin:/bin", SEMIO_NODE_VERSION: version });
  expect(skip.status, skip.stderr).toBe(0);
  expect(existsSync(join(kept, "npm"))).toBe(false);
  const nodeElsewhere = join(generated, "node-elsewhere"), nodeLinked = join(generated, "node-linked");
  mkdirSync(nodeElsewhere, {recursive: true});
  for (const name of ["node", "npm", "npx", "corepack"]) writeFileSync(join(nodeElsewhere, name), "#!/bin/sh\nprintf '" + (name === "node" ? "v" + version : name) + "\\n'\n", {mode: 0o755});
  const nodeLinkedResult = execute(body("ensure_node", "dotnet_sdk_pin") + "\nlog() { :; }\nhost_pin() { printf 'pin called\\n' >&2; return 1; }\nensure_node\n", { REPO_ROOT: root, HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: nodeLinked, PATH: `${nodeElsewhere}:/usr/bin:/bin`, SEMIO_NODE_VERSION: version });
  expect(nodeLinkedResult.status, nodeLinkedResult.stderr).toBe(0);
  expect(nodeLinkedResult.stderr).not.toContain("pin called");
  for (const name of ["node", "npm", "npx", "corepack"]) expect(readlinkSync(join(nodeLinked, name))).toBe(join(nodeElsewhere, name));

});

test("pinned uv installs uvx when uv is already new enough", () => {
  const home = join(generated, "uv-home"), local = join(home, "bin"), archive = join(generated, "uv.tar.gz"), packed = join(generated, "uv-pack", "pkg");
  mkdirSync(packed, { recursive: true });
  for (const name of ["uv", "uvx"]) writeFileSync(join(packed, name), "#!/bin/sh\nprintf '" + name + "'\n", { mode: 0o755 });
  const packedTar = spawnSync("tar", ["-czf", archive, "-C", join(generated, "uv-pack"), "pkg"], { encoding: "utf8" });
  expect(packedTar.status, packedTar.stderr).toBe(0);
  const head = body("ensure_uv", "run_privileged") + `
log() { :; }
host_pin() { printf '{}'; }
pin_field() { printf '0.11.6'; }
fetch_pinned() { cp "$SEMIO_TEST_ARCHIVE" "$2"; }
`;
  const installed = execute(head + "uv() { printf 'uv 0.11.6\\n'; }\nensure_uv\n", { REPO_ROOT: root, HOME: home, LOCAL_BIN: local, TMPDIR: generated, SEMIO_TEST_ARCHIVE: archive });
  expect(installed.status, installed.stderr).toBe(0);
  for (const name of ["uv", "uvx"]) expect(readFileSync(join(local, name), "utf8")).toContain(name);
  const kept = join(generated, "uv-kept");
  mkdirSync(kept, { recursive: true });
  const skip = execute(head + "fetch_pinned() { printf 'fetched\\n' >&2; return 1; }\nuv() { printf 'uv 0.11.6\\n'; }\nuvx() { :; }\nversion_at_least() { return 0; }\nensure_uv\n", { REPO_ROOT: root, HOME: home, LOCAL_BIN: kept, TMPDIR: generated, SEMIO_TEST_ARCHIVE: archive });
  expect(skip.status, skip.stderr).toBe(0);
  expect(skip.stderr).not.toContain("fetched");
  expect(existsSync(join(kept, "uvx"))).toBe(false);
  const elsewhere = join(generated, "uv-elsewhere"), linked = join(generated, "uv-linked");
  mkdirSync(elsewhere, {recursive: true});
  for (const name of ["uv", "uvx"]) writeFileSync(join(elsewhere, name), "#!/bin/sh\nprintf '" + name + " 0.11.6\\n'\n", {mode: 0o755});
  const linkedResult = execute(head + "version_at_least() { return 0; }\nfetch_pinned() { printf 'fetched\\n' >&2; return 1; }\nensure_uv\n", { REPO_ROOT: root, HOME: home, LOCAL_BIN: linked, PATH: `${elsewhere}:/usr/bin:/bin`, TMPDIR: generated, SEMIO_TEST_ARCHIVE: archive });
  expect(linkedResult.status, linkedResult.stderr).toBe(0);
  expect(linkedResult.stderr).not.toContain("fetched");
  expect(readlinkSync(join(linked, "uv"))).toBe(join(elsewhere, "uv"));
  expect(readlinkSync(join(linked, "uvx"))).toBe(join(elsewhere, "uvx"));

});

test("an existing .NET SDK is linked into the local bin", () => {
  const sdk = JSON.parse(readFileSync(join(root, "global.json"), "utf8")).sdk.version as string;
  const source = join(generated, "dotnet-source"), local = join(generated, "dotnet-local");
  mkdirSync(source, { recursive: true });
  writeFileSync(join(source, "dotnet"), "#!/bin/sh\nif [ \"$1\" = --list-sdks ]; then printf '%s [/sdk]\\n' \"$SEMIO_DOTNET_SDK\"; fi\n", { mode: 0o755 });
  const script = body("dotnet_sdk_pin", "link_user_tool") + `
log() { :; }
pin_row() { printf 'pin called\\n' >&2; return 1; }
ensure_dotnet
`;
  const result = execute(script, { REPO_ROOT: root, HOME: join(generated, "dotnet-home"), LOCAL_BIN: local, PATH: `${source}:${process.env.PATH}`, SEMIO_DOTNET_SDK: sdk });
  expect(result.status, result.stderr).toBe(0);
  expect(result.stderr).not.toContain("pin called");
  expect(readFileSync(join(local, "dotnet"), "utf8")).toContain("--list-sdks");
});

test("Rust commands already on PATH are linked into the local bin", () => {
  const start = bootstrap.indexOf("link_user_tool() {");
  const end = bootstrap.indexOf("\nensure_launcher() {", start);
  const cargo = join(generated, "rust-cargo", "bin"), other = join(generated, "rust-other"), local = join(generated, "rust-local");
  mkdirSync(cargo, {recursive: true});
  mkdirSync(other, {recursive: true});
  for (const name of ["rustup", "cargo", "rustc"]) writeFileSync(join(other, name), "#!/bin/sh\nprintf '" + name + "'\n", {mode: 0o755});
  const result = execute(bootstrap.slice(start, end) + "\nlink_installed_rust\n", {LOCAL_BIN: local, CARGO_HOME: join(generated, "rust-cargo"), PATH: `${other}:${process.env.PATH}`});
  expect(result.status, result.stderr).toBe(0);
  for (const name of ["rustup", "cargo", "rustc"]) expect(readFileSync(join(local, name), "utf8")).toContain(name);
});

test("an incomplete pinned Node tree is replaced so npm exists", () => {
  const version = JSON.parse(readFileSync(join(root, "package.json"), "utf8")).engines.node as string;
  const home = join(generated, "node-incomplete-home"), data = join(generated, "node-incomplete-data"), bin = join(data, "semio", "node", `v${version}`, "bin"), local = join(home, ".local", "bin");
  mkdirSync(bin, { recursive: true });
  writeFileSync(join(bin, "node"), "#!/bin/sh\nprintf node\\n", { mode: 0o755 });
  const script = body("unpack_tar_xz", "dotnet_sdk_pin") + `
log() { :; }
host_pin() { printf '{}'; }
pin_field() { printf '%s' "$SEMIO_NODE_VERSION"; }
fetch_pinned() { :; }
tar() {
  mkdir -p "$SEMIO_NODE_BIN"
  printf '#!/bin/sh\\nprintf node\\n' > "$SEMIO_NODE_BIN/node"
  printf '#!/bin/sh\\nprintf npm\\n' > "$SEMIO_NODE_BIN/npm"
  chmod 755 "$SEMIO_NODE_BIN/node" "$SEMIO_NODE_BIN/npm"
}
node() { printf 'v%s\\n' "$SEMIO_NODE_VERSION"; }
ensure_node
`;
  const result = execute(script, { REPO_ROOT: root, HOME: home, XDG_DATA_HOME: data, LOCAL_BIN: local, PATH: "/usr/bin:/bin", SEMIO_NODE_VERSION: version, SEMIO_NODE_BIN: bin });
  expect(result.status, result.stderr).toBe(0);
  expect(readFileSync(join(local, "npm"), "utf8")).toContain("npm");
});

test("Go already on PATH is linked into the local bin", () => {
  const start = bootstrap.indexOf("link_user_tool() {");
  const end = bootstrap.indexOf("\nensure_launcher() {", start);
  const other = join(generated, "go-other"), local = join(generated, "go-local"), repo = join(generated, "go-empty-repo");
  mkdirSync(other, {recursive: true});
  for (const name of ["go", "gofmt"]) writeFileSync(join(other, name), "#!/bin/sh\nprintf '" + name + "'\n", {mode: 0o755});
  const result = execute(bootstrap.slice(start, end) + `
LOCAL_BIN="$BIN"
REPO_ROOT="$REPO"
host_pin() { printf '%s' '{"tool":"go","version":"9","sha256":"abcdefabcdefabcdffffffffffffffffffffffffffffffffffffffffffffffff","platform":"linux","arch":"x64"}'; }
pin_field() { case "$2" in version) printf 9 ;; sha256) printf abcdefabcdefabcdffffffffffffffffffffffffffffffffffffffffffffffff ;; esac; }
host_platform() { printf linux; }
host_arch() { printf x64; }
link_pinned_go
`, {BIN: local, REPO: repo, PATH: `${other}:${process.env.PATH}`, HOME: join(generated, "go-home"), XDG_DATA_HOME: join(generated, "go-data")});
  expect(result.status, result.stderr).toBe(0);
  expect(readFileSync(join(local, "go"), "utf8")).toContain("go");
  expect(readFileSync(join(local, "gofmt"), "utf8")).toContain("gofmt");
});

test("an xz archive unpacks with Python when tar cannot read xz", () => {
  const directory = join(generated, "xz-python");
  const archive = join(directory, "sample.tar.xz");
  const dest = join(directory, "out");
  mkdirSync(directory, {recursive: true});
  const created = spawnSync("python3", ["-c", [
    "import io, lzma, os, tarfile",
    "archive = os.environ['A']",
    "payload = b'#!/bin/sh\\nprintf npm\\n'",
    "buffer = io.BytesIO()",
    "with tarfile.open(fileobj=buffer, mode='w') as tar:",
    "    info = tarfile.TarInfo('node-v1/bin/npm')",
    "    info.size = len(payload)",
    "    info.mode = 0o755",
    "    tar.addfile(info, io.BytesIO(payload))",
    "with lzma.open(archive, 'wb') as output:",
    "    output.write(buffer.getvalue())",
  ].join("\n")], {env: {...process.env, A: archive}});
  expect(created.status, created.stderr?.toString()).toBe(0);
  const result = execute(body("unpack_tar_xz", "ensure_node") + `
tar() { return 1; }
unpack_tar_xz "$ARCHIVE" "$DEST"
`, {ARCHIVE: archive, DEST: dest});
  expect(result.status, result.stderr).toBe(0);
  expect(readFileSync(join(dest, "bin", "npm"), "utf8")).toContain("npm");
});

test("pinned downloads use Python when curl and SHA-256 tools are missing", async () => {
  const directory = join(generated, "py-https");
  const key = join(directory, "key.pem");
  const cert = join(directory, "cert.pem");
  const dest = join(directory, "tool");
  mkdirSync(directory, {recursive: true});
  const rejected = execute(body("verify_sha256", "fetch_pinned") + `
command() { if [ "$1" = "-v" ] && [ "$2" = curl ]; then return 1; fi; builtin command "$@"; }
log() { :; }
download_verified "http://127.0.0.1/tool" abcdef "$DEST"
`, {DEST: dest});
  expect(rejected.status).not.toBe(0);
  expect(existsSync(dest)).toBe(false);
  const made = spawnSync("openssl", ["req", "-x509", "-newkey", "rsa:2048", "-keyout", key, "-out", cert, "-days", "1", "-nodes", "-subj", "/CN=127.0.0.1", "-addext", "subjectAltName=IP:127.0.0.1"], {encoding: "utf8"});
  expect(made.status, made.stderr).toBe(0);
  const server = spawn("python3", ["-c", [
    "import http.server, os, ssl",
    "class Handler(http.server.BaseHTTPRequestHandler):",
    "    def do_GET(self):",
    "        body = b'pinned-bytes'",
    "        self.send_response(200)",
    "        self.send_header('Content-Length', str(len(body)))",
    "        self.end_headers()",
    "        self.wfile.write(body)",
    "    def log_message(self, format, *args):",
    "        return",
    "server = http.server.HTTPServer(('127.0.0.1', 0), Handler)",
    "context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)",
    "context.minimum_version = ssl.TLSVersion.TLSv1_2",
    "context.load_cert_chain(os.environ['CERT'], os.environ['KEY'])",
    "server.socket = context.wrap_socket(server.socket, server_side=True)",
    "print(server.server_address[1], flush=True)",
    "server.serve_forever()",
  ].join("\n")], {env: {...process.env, CERT: cert, KEY: key}, stdio: ["ignore", "pipe", "pipe"]});
  const port = await new Promise<string>((resolvePort, rejectPort) => {
    let buffer = "";
    const fail = () => rejectPort(new Error(server.stderr?.read()?.toString() ?? "https server exited"));
    server.once("exit", fail);
    server.stdout?.on("data", (chunk) => {
      buffer += chunk.toString();
      const line = buffer.split("\n")[0]?.trim();
      if (line) {
        server.removeListener("exit", fail);
        resolvePort(line);
      }
    });
  });
  try {
    const digest = spawnSync("python3", ["-c", "import hashlib; print(hashlib.sha256(b'pinned-bytes').hexdigest())"], {encoding: "utf8"}).stdout.trim();
    const result = execute(body("verify_sha256", "fetch_pinned") + `
command() { if [ "$1" = "-v" ] && { [ "$2" = curl ] || [ "$2" = sha256sum ] || [ "$2" = shasum ]; }; then return 1; fi; builtin command "$@"; }
log() { :; }
download_verified "https://127.0.0.1:$PORT/tool" "$DIGEST" "$DEST"
`, {DEST: dest, DIGEST: digest, PORT: port, SSL_CERT_FILE: cert});
    expect(result.status, result.stderr).toBe(0);
    expect(readFileSync(dest, "utf8")).toBe("pinned-bytes");
  } finally {
    server.kill();
  }
});

test("pinned uv unpacks with Python when tar cannot read gzip", () => {
  const home = join(generated, "uv-gz-home"), local = join(home, "bin"), archive = join(generated, "uv-gz.tar.gz"), packed = join(generated, "uv-gz-pack", "pkg");
  mkdirSync(packed, {recursive: true});
  for (const name of ["uv", "uvx"]) writeFileSync(join(packed, name), "#!/bin/sh\nprintf '" + name + "'\n", {mode: 0o755});
  const packedTar = spawnSync("tar", ["-czf", archive, "-C", join(generated, "uv-gz-pack"), "pkg"], {encoding: "utf8"});
  expect(packedTar.status, packedTar.stderr).toBe(0);
  const result = execute(body("ensure_uv", "run_privileged") + `
log() { :; }
host_pin() { printf '{}'; }
pin_field() { printf '0.11.6'; }
fetch_pinned() { cp "$SEMIO_TEST_ARCHIVE" "$2"; }
tar() { return 1; }
uv() { printf 'uv 0.0.0\\n'; }
ensure_uv
`, {REPO_ROOT: root, HOME: home, LOCAL_BIN: local, TMPDIR: generated, SEMIO_TEST_ARCHIVE: archive});
  expect(result.status, result.stderr).toBe(0);
  for (const name of ["uv", "uvx"]) expect(readFileSync(join(local, name), "utf8")).toContain(name);
});

test("the .NET installer downloads through Python when curl is missing", async () => {
  const directory = join(generated, "dotnet-curl");
  const key = join(directory, "key.pem");
  const cert = join(directory, "cert.pem");
  const home = join(directory, "home");
  const local = join(directory, "local");
  const pathDir = join(directory, "path");
  const installer = join(directory, "dotnet-install.sh");
  const payload = join(home, ".dotnet", "payload");
  mkdirSync(pathDir, {recursive: true});
  for (const source of ["/usr/bin", "/bin"]) {
    for (const name of readdirSync(source)) {
      if (name === "curl" || existsSync(join(pathDir, name))) continue;
      try { symlinkSync(join(source, name), join(pathDir, name)); } catch {}
    }
  }
  writeFileSync(installer, "#!/bin/bash\nset -eu\nmkdir -p \"$DOTNET_ROOT\"\ncurl --retry 2 --retry-delay 0 --connect-timeout 15 -fsSL -o \"$DOTNET_ROOT/payload\" \"$SEMIO_TEST_URL\"\ncurl -sI \"$SEMIO_TEST_URL\" | grep -qi content-length\nprintf '#!/bin/sh\\n' > \"$DOTNET_ROOT/dotnet\"\nchmod 755 \"$DOTNET_ROOT/dotnet\"\n", {mode: 0o755});
  const made = spawnSync("openssl", ["req", "-x509", "-newkey", "rsa:2048", "-keyout", key, "-out", cert, "-days", "1", "-nodes", "-subj", "/CN=127.0.0.1", "-addext", "subjectAltName=IP:127.0.0.1"], {encoding: "utf8"});
  expect(made.status, made.stderr).toBe(0);
  const server = spawn("python3", ["-c", [
    "import http.server, os, ssl",
    "class Handler(http.server.BaseHTTPRequestHandler):",
    "    def do_GET(self):",
    "        body = b'dotnet-sdk'",
    "        self.send_response(200)",
    "        self.send_header('Content-Length', str(len(body)))",
    "        self.end_headers()",
    "        self.wfile.write(body)",
    "    def do_HEAD(self):",
    "        self.send_response(200)",
    "        self.send_header('Content-Length', '10')",
    "        self.end_headers()",
    "    def log_message(self, format, *args):",
    "        return",
    "server = http.server.HTTPServer(('127.0.0.1', 0), Handler)",
    "context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)",
    "context.minimum_version = ssl.TLSVersion.TLSv1_2",
    "context.load_cert_chain(os.environ['CERT'], os.environ['KEY'])",
    "server.socket = context.wrap_socket(server.socket, server_side=True)",
    "print(server.server_address[1], flush=True)",
    "server.serve_forever()",
  ].join("\n")], {env: {...process.env, CERT: cert, KEY: key}, stdio: ["ignore", "pipe", "pipe"]});
  const port = await new Promise<string>((resolvePort, rejectPort) => {
    let buffer = "";
    const fail = () => rejectPort(new Error(String(server.stderr?.read() ?? "https server exited")));
    server.once("exit", fail);
    server.stdout?.on("data", (chunk) => {
      buffer += chunk.toString();
      const line = buffer.split("\n")[0]?.trim();
      if (line) {
        server.removeListener("exit", fail);
        resolvePort(line);
      }
    });
  });
  try {
    const result = execute(body("dotnet_sdk_pin", "link_user_tool") + `
log() { :; }
pin_row() { printf 'row'; }
fetch_pinned() { cp "$SEMIO_TEST_INSTALLER" "$2"; }
ensure_dotnet
`, {REPO_ROOT: root, HOME: home, LOCAL_BIN: local, PATH: pathDir, SSL_CERT_FILE: cert, SEMIO_TEST_INSTALLER: installer, SEMIO_TEST_URL: `https://127.0.0.1:${port}/sdk`});
    expect(result.status, result.stderr).toBe(0);
    expect(readFileSync(payload, "utf8")).toBe("dotnet-sdk");
    expect(existsSync(join(local, "dotnet"))).toBe(true);
  } finally {
    server.kill();
  }
});

test("an xz archive keeps relative symlinks that stay inside the destination", () => {
  const directory = join(generated, "xz-link");
  const archive = join(directory, "node.tar.xz");
  const escaped = join(directory, "escape.tar.xz");
  const dest = join(directory, "out");
  const outside = join(directory, "escape-out");
  mkdirSync(directory, {recursive: true});
  const writeArchive = (path: string, link: string) => {
    const created = spawnSync("python3", ["-c", [
      "import io, lzma, os, tarfile",
      "archive, link = os.environ['A'], os.environ['L']",
      "payload = b'npm-cli'",
      "buffer = io.BytesIO()",
      "with tarfile.open(fileobj=buffer, mode='w') as tar:",
      "    info = tarfile.TarInfo('node-v1/lib/npm-cli.js')",
      "    info.size = len(payload)",
      "    info.mode = 0o755",
      "    tar.addfile(info, io.BytesIO(payload))",
      "    linked = tarfile.TarInfo('node-v1/bin/npm')",
      "    linked.type = tarfile.SYMTYPE",
      "    linked.linkname = link",
      "    tar.addfile(linked)",
      "with lzma.open(archive, 'wb') as output:",
      "    output.write(buffer.getvalue())",
    ].join("\n")], {env: {...process.env, A: path, L: link}});
    expect(created.status, created.stderr?.toString()).toBe(0);
  };
  writeArchive(archive, "../lib/npm-cli.js");
  writeArchive(escaped, "../../outside");
  const unpack = body("unpack_tar_xz", "ensure_node") + `
tar() { return 1; }
unpack_tar_xz "$ARCHIVE" "$DEST"
`;
  const kept = execute(unpack, {ARCHIVE: archive, DEST: dest});
  expect(kept.status, kept.stderr).toBe(0);
  expect(spawnSync("readlink", [join(dest, "bin", "npm")], {encoding: "utf8"}).stdout.trim()).toBe("../lib/npm-cli.js");
  expect(readFileSync(join(dest, "lib", "npm-cli.js"), "utf8")).toBe("npm-cli");
  const rejected = execute(unpack, {ARCHIVE: escaped, DEST: outside});
  expect(rejected.status).not.toBe(0);
  expect(existsSync(join(outside, "bin", "npm"))).toBe(false);
});

test("a gzip SDK archive unpacks when tar cannot read gzip", () => {
  const directory = join(generated, "tar-gz"), bin = join(directory, "bin"), archive = join(directory, "sdk.tar.gz"), dest = join(directory, "out");
  mkdirSync(bin, {recursive: true});
  writeFileSync(join(bin, "tar"), "#!/bin/sh\nexit 1\n", {mode: 0o755});
  const created = spawnSync("python3", ["-c", [
    "import gzip, io, os, tarfile",
    "payload = b'sdk'",
    "buffer = io.BytesIO()",
    "with tarfile.open(fileobj=buffer, mode='w') as tar:",
    "    info = tarfile.TarInfo('dotnet')",
    "    info.size = len(payload)",
    "    info.mode = 0o755",
    "    tar.addfile(info, io.BytesIO(payload))",
    "with gzip.open(os.environ['A'], 'wb') as output:",
    "    output.write(buffer.getvalue())",
  ].join("\n")], {env: {...process.env, A: archive}});
  expect(created.status, created.stderr?.toString()).toBe(0);
  const result = execute(body("provide_tar_gzip", "provide_curl") + `
provide_tar_gzip
mkdir -p "$DEST"
tar -xzf "$ARCHIVE" -C "$DEST"
`, {PATH: `${bin}:${process.env.PATH}`, ARCHIVE: archive, DEST: dest});
  expect(result.status, result.stderr).toBe(0);
  expect(readFileSync(join(dest, "dotnet"), "utf8")).toBe("sdk");
});

test("owned C driver translates Rust vendors into the pinned backend target grammar without changing arguments",()=>{
 const cases:readonly {input:readonly string[];target:string;rest:readonly string[]}[]=JSON.parse(readFileSync(new URL("./compiler-targets.json",import.meta.url),"utf8"));
 expect(new Ajv().validate({type:"array",minItems:9,items:{type:"object",additionalProperties:false,required:["input","target","rest"],properties:{input:{type:"array",items:{type:"string"}},target:{type:"string",pattern:"^[a-zA-Z0-9_]+-[a-zA-Z0-9_.]+(?:-[a-zA-Z0-9_.]+)?$"},rest:{type:"array",items:{type:"string"}}}}},cases)).toBe(true);
 const compilerData=join(generated,"compiler data"),compilerBin=join(generated,"compiler bin"),backend=join(compilerData,"semio/zig/v0.14.1/zig");
 mkdirSync(join(compilerData,"semio/zig/v0.14.1"),{recursive:true});mkdirSync(compilerBin,{recursive:true});
 writeFileSync(backend,'#!/bin/sh\nprintf "%s\\n" "$@"\n',{mode:0o755});
 const quote=(value:string)=>"'"+value.replaceAll("'","'\\''")+"'";
 for(const row of cases){
  const script=body("ensure_user_cc","ensure_user_pkg_config")+'\nlog(){ :; }\nhost_pin(){ printf "{}"; }\npin_field(){ printf "0.14.1"; }\ncommand(){ if [ "$1" = "-v" ] && { [ "$2" = cc ] || [ "$2" = c++ ]; }; then return 1; fi; builtin command "$@"; }\nensure_user_cc\n"$LOCAL_BIN/cc" '+row.input.map(quote).join(" ");
  const result=execute(script,{LOCAL_BIN:compilerBin,XDG_DATA_HOME:compilerData});
  expect(result.status,result.stderr).toBe(0);expect(result.stdout.trim().split("\n"),JSON.stringify(row)).toEqual(["cc","-target",row.target,...row.rest]);
 }
 for(const name of ["cc","c++","gcc","g++"]){
  const missing=spawnSync(join(compilerBin,name),["--target"],{encoding:"utf8"});
  expect(missing.status).toBe(2);expect(missing.stderr).toContain("C compiler target requires a value");
 }
});
