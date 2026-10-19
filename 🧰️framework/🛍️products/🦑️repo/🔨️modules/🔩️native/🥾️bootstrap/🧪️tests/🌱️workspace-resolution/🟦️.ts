import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { lstatSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import findUp from "find-up";
import Ajv from "ajv";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import corpus from "@semio-tech/process/routing/workspace/testing/fixtures.json";
import bootstrapCorpus from "../🧫️fixtures/🔣️.json";
import schema from "@semio-tech/process/routing/workspace/schema.json";

const root = findWorkspaceRoot(import.meta.dir);
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("Workspace identity tests require explicit SEMIO_TEST_ARTIFACT_DIR");
const generated = resolve(output, `wb-${process.pid}`);
mkdirSync(generated, { recursive: true });

function oracle(start: string, boundary: string): string | null {
  try {
    return findUp.sync(directory => {
      const markers = ["📋️project.json", "package.json"].map(name => join(directory, name));
      if (markers.every(path => lstatSync(path, { throwIfNoEntry: false })?.isFile())) {
        const workspaces = JSON.parse(readFileSync(markers[1]!, "utf8")).workspaces;
        if (Array.isArray(workspaces) || workspaces !== null && typeof workspaces === "object") return directory;
      }
      return directory === boundary ? findUp.stop : undefined;
    }, { cwd: start, type: "directory" }) ?? null;
  } catch { return null; }
}

function materialize(index: number, rows = corpus.cases): string {
  const row = rows[index]!;
  const directory = join(generated, String(index), ...(row.expected === null ? Array(32).fill("a") : []));
  mkdirSync(join(directory, row.start), { recursive: true });
  for (const [name, content] of Object.entries(row.files)) {
    const path = join(directory, name);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, content);
  }
  return directory;
}

test("pre-runtime native bootstrap resolves the neutral workspace corpus", () => {
  const bootstrap = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap");
  const shell = readFileSync(join(bootstrap, "🐚️.sh"), "utf8").split('REPO_ROOT="$(resolve_repo_root)"')[0]!;
  const powershell = readFileSync(join(bootstrap, "🔵️.ps1"), "utf8").split("function Get-RepoRoot {")[1]!.split("function Join-HomePath")[0]!;
  const hasPowerShell = spawnSync("pwsh", ["-NoProfile", "-Command", "exit 0"]).status === 0;
  const rows = [...corpus.cases, ...bootstrapCorpus.cases];
  expect(new Ajv({ strict: false }).validate(schema, bootstrapCorpus)).toBe(true);
  for (const [index, row] of rows.entries()) {
    const directory = materialize(index, rows), start = join(directory, row.start);
    const expected = row.expected === null ? null : resolve(directory, row.expected);
    expect(oracle(start, directory), row.id).toBe(expected);
    const env = { ...process.env, SEMIO_WORKSPACE_ROOT: "", COMPOSE_REPO_ROOT: "", SEMIO_BOOTSTRAP_TEST_START: start };
    const result = spawnSync("bash", ["-c", shell + '\nresolve_repo_root "$SEMIO_BOOTSTRAP_TEST_START"'], { env, encoding: "utf8" });
    expect(result.error, row.id).toBeUndefined();
    expect(result.status === 0 ? result.stdout.trim() : null, `Bash ${row.id}: ${result.stderr}`).toBe(expected);
    if (hasPowerShell) {
      const result = spawnSync("pwsh", ["-NoProfile", "-Command", '$ErrorActionPreference="Stop";function Get-RepoRoot {' + powershell + '\nGet-RepoRoot -Start $env:SEMIO_BOOTSTRAP_TEST_START'], { env, encoding: "utf8" });
      expect(result.status === 0 ? result.stdout.trim() : null, `PowerShell ${row.id}: ${result.stderr}`).toBe(expected);
    }
  }
});


test("native bootstrap validates explicit workspace ownership and rejects marker links", () => {
  const bootstrap = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap");
  const shell = readFileSync(join(bootstrap, "🐚️.sh"), "utf8").split('REPO_ROOT="$(resolve_repo_root)"')[0]!;
  const directory = materialize(0), child = join(directory, "domain/package");
  const execute = (override: string) => spawnSync("bash", ["-c", shell + '\nresolve_repo_root "$SEMIO_BOOTSTRAP_TEST_START"'], { env: { ...process.env, SEMIO_WORKSPACE_ROOT: override, SEMIO_BOOTSTRAP_TEST_START: child }, encoding: "utf8" });
  expect(execute(directory).stdout).toBe(directory);
  expect(execute(child).status).toBe(1);
  const linked = join(generated, "linked"), start = join(linked, ...Array(32).fill("a"));
  mkdirSync(start, { recursive: true });
  symlinkSync(join(directory, "package.json"), join(linked, "package.json"));
  writeFileSync(join(linked, "📋️project.json"), "{}");
  const result = spawnSync("bash", ["-c", shell + '\nresolve_repo_root "$SEMIO_BOOTSTRAP_TEST_START"'], { env: { ...process.env, SEMIO_WORKSPACE_ROOT: linked, SEMIO_BOOTSTRAP_TEST_START: start }, encoding: "utf8" });
  expect(result.status).toBe(1);
});
