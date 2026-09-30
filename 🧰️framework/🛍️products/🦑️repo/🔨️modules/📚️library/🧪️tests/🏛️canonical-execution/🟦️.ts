import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseToml, stringify as stringifyToml } from "@iarna/toml";
import ts from "typescript";
import { canonicalArchitectureEnvironment, cargoDirectories } from "../../⚡️caching/🦀️cargo/🟦️.ts";
import { orchestratorBudgetOpts } from "../../🏃️process/🟦️.ts";

const libraryRoot = resolve(import.meta.dir, "../..");
const vectors = JSON.parse(readFileSync(join(libraryRoot, "🧫️fixtures/🏛️canonical-execution/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(libraryRoot, "🧬️schema/🏛️canonical-execution/🔣️.json"), "utf8"));
const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(artifactRoot, { recursive: true });

test("closed canonical execution vectors admit only declared environment inputs", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...vectors, unexpected: true })).toBe(false);
  const unknown = structuredClone(vectors);
  unknown.cases[0].environment.UNDECLARED = "value";
  expect(validate(unknown)).toBe(false);
});

for (const row of vectors.cases) {
  test(`canonical Cargo environment: ${row.name}`, () => {
    const root = resolve(artifactRoot, "portable-workspace");
    const input = Object.freeze({
      ...row.environment,
      ...(row.target.mode === "relative" ? { CARGO_TARGET_DIR: join(...row.target.segments) } : {}),
      ...(row.target.mode === "absolute" ? { CARGO_TARGET_DIR: resolve(root, ...row.target.segments) } : {}),
      SEMIO_UNDEFINED: undefined,
    });
    const before = { ...input };
    const actual = canonicalArchitectureEnvironment(root, input);
    const target = resolve(root, ...row.expectedTarget);
    expect(actual).toEqual({ ...before, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: target });
    expect(input).toEqual(before);
    expect(actual).not.toBe(input);
    expect(Object.keys(actual).sort()).toEqual([...new Set([...Object.keys(input), "CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"])].sort());
  });
}

test("explicit empty environment never captures an ambient secret", () => {
  const key = "SEMIO_CANONICAL_ENV_AMBIENT_SENTINEL";
  const previous = process.env[key];
  process.env[key] = "fixture-only-ambient-sentinel";
  try {
    expect(Object.hasOwn(canonicalArchitectureEnvironment(artifactRoot, {}), key)).toBe(false);
  } finally {
    if (previous === undefined) delete process.env[key];
    else process.env[key] = previous;
  }
});

test("the actual root caller preserves canonical Cargo authority through process options", () => {
  const source = ts.createSourceFile("📜️script.ts", readFileSync(join(process.cwd(), "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const owner = source.statements.find((node): node is ts.ClassDeclaration => ts.isClassDeclaration(node) && node.name?.text === "VerifyScript");
  const method = owner?.members.find((node): node is ts.MethodDeclaration => ts.isMethodDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "runCanonicalArchitecture");
  if (!method) throw new Error("canonical root execution owner is absent");
  const code = ts.transpileModule(`class Invocation { root = repoRoot; ${method.getText(source)} } new Invocation().runCanonicalArchitecture();`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const invoke = new Function("repoRoot", "process", "runCmd", "canonicalArchitectureEnvironment", "orchestratorBudgetOpts", "semioNxParallelFlag", "console", code);
  for (const row of vectors.cases) {
    const root = resolve(artifactRoot, "portable-workspace");
    const env = { ...row.environment, ...(row.target.mode === "relative" ? { CARGO_TARGET_DIR: join(...row.target.segments) } : {}), ...(row.target.mode === "absolute" ? { CARGO_TARGET_DIR: resolve(root, ...row.target.segments) } : {}) };
    let observed: { env: Record<string, string | undefined> } | undefined;
    invoke(root, { env }, (_command: string, _args: string[], options: typeof observed) => { observed = options; }, canonicalArchitectureEnvironment, orchestratorBudgetOpts, () => [], { log() {} });
    expect(observed?.env.CARGO_TARGET_DIR).toBe(resolve(root, ...row.expectedTarget));
    expect(observed?.env.CARGO_BUILD_BUILD_DIR).toBe(resolve(root, ...row.expectedTarget));
  }
});

test("Cargo metadata and independent TOML parsing confirm isolated target and build roots", () => {
  const root = realpathSync(mkdtempSync(join(artifactRoot, "canonical-cargo-oracle-")));
  try {
    mkdirSync(join(root, "src"));
    mkdirSync(join(root, ".cargo"));
    writeFileSync(join(root, "Cargo.toml"), '[package]\nname = "canonical_environment_oracle"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n');
    writeFileSync(join(root, "src", "lib.rs"), "");
    const config = stringifyToml({ build: { "target-dir": "configured-target", "build-dir": "configured-build" } });
    writeFileSync(join(root, ".cargo", "config.toml"), config);
    const oracle = parseToml(config) as { build: { "target-dir": string; "build-dir": string } };
    expect(cargoDirectories(root, {})).toEqual({ target: resolve(root, oracle.build["target-dir"]), build: resolve(root, oracle.build["build-dir"]) });
    const toolchain = Bun.TOML.parse(readFileSync(join(process.cwd(), "rust-toolchain.toml"), "utf8")) as { toolchain: { channel: string } };
    for (const row of vectors.cases) {
      const input = {
        ...row.environment,
        ...(row.target.mode === "relative" ? { CARGO_TARGET_DIR: join(...row.target.segments) } : {}),
        ...(row.target.mode === "absolute" ? { CARGO_TARGET_DIR: resolve(root, ...row.target.segments) } : {}),
      };
      const isolated = canonicalArchitectureEnvironment(root, input);
      const target = resolve(root, ...row.expectedTarget);
      expect(cargoDirectories(root, isolated)).toEqual({ target, build: target });
      const { CARGO_TARGET_DIR: _target, CARGO_BUILD_TARGET_DIR: _secondary, CARGO_BUILD_BUILD_DIR: _build, ...ambient } = process.env;
      const cargo = spawnSync("cargo", ["metadata", "--no-deps", "--offline", "--format-version", "1"], {
        cwd: root, encoding: "utf8", timeout: 30_000,
        env: { ...ambient, CARGO_TARGET_DIR: isolated.CARGO_TARGET_DIR, CARGO_BUILD_BUILD_DIR: isolated.CARGO_BUILD_BUILD_DIR, RUSTUP_TOOLCHAIN: toolchain.toolchain.channel },
      });
      expect(cargo.status, cargo.stderr).toBe(0);
      expect(JSON.parse(cargo.stdout).target_directory).toBe(target);
    }
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}, { timeout: 120_000 });
