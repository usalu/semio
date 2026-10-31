import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
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
});

test("Windows Rust bootstrap keeps profiles and the pinned installer untouched", () => {
  const windows = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"), "utf8");
  const installer = windows.match(/^\s*& \$installer (.+) \| Out-Host$/mu)?.[1]?.split(/\s+/u);
  expect(installer).toBeDefined();
  for (const argument of corpus.rustupRequiredArguments) expect(installer).toContain(argument);
  const toolchain = windows.match(/-FilePath \$rustupPath -ArgumentList @\(([^)]+)\) -WorkingDirectory \$repoRoot/u)?.[1];
  expect(toolchain).toBeDefined();
  const args = JSON.parse("[" + toolchain + "]");
  expect(new Ajv().validate({ type: "array", items: { type: "string" } }, args)).toBe(true);
  expect(args).toEqual(corpus.rustupToolchainArguments);
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
