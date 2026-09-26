import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { cpSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { decodeCargoDepInfo, foreignDepInfoPaths, invalidateForeignUnits, scanBuildDirProvenance, type CargoDepInfoFile } from "../../🦀️cargo/🧾️provenance/🟦️.ts";

const moduleRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(moduleRoot, "../../../../../..");
const fixture = JSON.parse(readFileSync(join(moduleRoot, "🧫️fixtures/🧾️cargo-provenance/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(moduleRoot, "🧬️schema/🧾️cargo-provenance/🔣️.json"), "utf8"));
const bytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../g)!.map((pair) => Number.parseInt(pair, 16)));

/** 🐍️ Python's own `ntpath`/`posixpath` containment, independent of the TypeScript classifier. */
const PYTHON_CONTAINMENT_ORACLE = `import json,ntpath,posixpath,sys
cases=json.loads(sys.argv[1]); out=[]
for case in cases:
 p=ntpath if case['platform']=='win32' else posixpath
 fold=(lambda v: p.normcase(p.normpath(v))) if case['platform']=='win32' else p.normpath
 def inside(path,root):
  path,root=fold(path),fold(root)
  if path.startswith(root.rstrip(p.sep)+p.sep) or path==root.rstrip(p.sep): return True
  return False
 out.append([x for x in case['paths'] if p.isabs(x) and not any(inside(x,r) for r in case['roots'])])
print(json.dumps(out))
`;

test("validates the language-neutral cargo provenance fixture", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

test("decodes Cargo dep-info exactly and refuses every other layout", () => {
  for (const row of fixture.decode) {
    if (row.error) expect(() => decodeCargoDepInfo(bytes(row.hex)), row.name).toThrow(row.error);
    else expect(decodeCargoDepInfo(bytes(row.hex)), row.name).toEqual(row.files);
  }
});

test("classifies foreign sources under POSIX and Windows path rules, agreeing with Python's ntpath/posixpath", () => {
  const actual = fixture.classify.map((row: { platform: NodeJS.Platform; roots: string[]; paths: string[] }) =>
    foreignDepInfoPaths(row.paths.map((path): CargoDepInfoFile => ({ base: "build", path })), row.roots, row.platform),
  );
  expect(actual).toEqual(fixture.classify.map((row: { foreign: string[] }) => row.foreign));
  const python = Bun.which("python3") ?? Bun.which("python");
  expect(python, "python3 is part of the workspace toolchain").toBeTruthy();
  const oracle = spawnSync(python!, ["-c", PYTHON_CONTAINMENT_ORACLE, JSON.stringify(fixture.classify)], { encoding: "utf8" });
  expect(oracle.status, oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(actual);
});

test("a second checkout poisons a shared build-dir, the gate names it and the repair makes Cargo rebuild", { timeout: 240_000 }, () => {
  const toolchain = Bun.TOML.parse(readFileSync(join(repoRoot, "rust-toolchain.toml"), "utf8")) as { toolchain: { channel: string } };
  const root = realpathSync(mkdtempSync(join(tmpdir(), "semio-cargo-provenance-")));
  try {
    const first = join(root, "first"), second = join(root, "second"), build = join(root, "build");
    mkdirSync(join(first, "crate", "src"), { recursive: true });
    mkdirSync(join(first, "shared"), { recursive: true });
    writeFileSync(join(first, "crate", "Cargo.toml"), '[package]\nname = "poison"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n');
    writeFileSync(join(first, "crate", "src", "lib.rs"), '#[path = "../../shared/mod.rs"]\npub mod shared;\n');
    writeFileSync(join(first, "shared", "mod.rs"), "pub fn answer() -> u32 { 42 }\n");
    cpSync(first, second, { recursive: true });
    const cargo = (checkout: string) =>
      spawnSync("cargo", ["-Zchecksum-freshness", "-Zbuild-dir-new-layout", "check", "--quiet"], {
        cwd: join(checkout, "crate"),
        encoding: "utf8",
        env: { ...process.env, RUSTUP_TOOLCHAIN: toolchain.toolchain.channel, CARGO_BUILD_BUILD_DIR: build, CARGO_TARGET_DIR: join(root, "target"), CARGO_INCREMENTAL: "0" },
      });
    expect(cargo(first).status).toBe(0);
    const unit = readdirSync(join(build, "debug", "build", "poison"))[0]!;
    const decoded = decodeCargoDepInfo(readFileSync(join(build, "debug", "build", "poison", unit, "fingerprint", "dep-lib-poison")));
    const rustcChecksums = [...readFileSync(join(build, "debug", "build", "poison", unit, "out", `poison-${unit}.d`), "utf8").matchAll(/^# checksum:(\S+) file_len:\d+ /gm)].map((match) => match[1]).sort();
    expect(decoded.map((file) => file.checksum).sort(), "rustc's own dep-info names the same checksums").toEqual(rustcChecksums);
    writeFileSync(join(second, "shared", "mod.rs"), 'pub fn answer() -> u32 { "not a number" }\n');
    expect(cargo(second).status, "the shared unit is Fresh for the second checkout although its source no longer compiles").toBe(0);
    const trusted = [second, build, join(root, "target")];
    const poisoned = scanBuildDirProvenance(build, trusted, process.platform, new AbortController().signal);
    expect(poisoned.foreign.flatMap((entry) => entry.foreign).map((path) => resolve(path))).toEqual([join(first, "shared", "mod.rs")]);
    expect(invalidateForeignUnits(poisoned.foreign, new AbortController().signal)).toBe(1);
    const rebuilt = cargo(second);
    expect(rebuilt.status).not.toBe(0);
    expect(rebuilt.stderr).toContain("E0308");
    writeFileSync(join(second, "shared", "mod.rs"), "pub fn answer() -> u32 { 7 }\n");
    expect(cargo(second).status).toBe(0);
    expect(scanBuildDirProvenance(build, trusted, process.platform, new AbortController().signal).foreign).toEqual([]);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("a cancelled scan stops before reading another package", () => {
  const build = mkdtempSync(join(tmpdir(), "semio-cargo-provenance-cancel-"));
  try {
    mkdirSync(join(build, "debug", "build", "poison", "0000", "fingerprint"), { recursive: true });
    const controller = new AbortController();
    controller.abort(new Error("cancelled by the law"));
    expect(() => scanBuildDirProvenance(build, [build], process.platform, controller.signal)).toThrow("cancelled by the law");
  } finally {
    rmSync(build, { recursive: true, force: true });
  }
});
