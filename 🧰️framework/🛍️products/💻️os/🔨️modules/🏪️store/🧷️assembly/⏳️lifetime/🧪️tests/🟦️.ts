import { expect, test } from "bun:test";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync } from "node:fs";
import { createHash } from "node:crypto";
import { isAbsolute, join } from "node:path";



const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as { contract: string; scenarios: { id: string; baselineAccepted: boolean; accepted: boolean; diagnostic: string | null }[] };
const storeUrl = new URL("../../../🦀️.rs", import.meta.url);
const source = readFileSync(storeUrl, "utf8");
const providers = readFileSync(new URL("../🧫️fixtures/🧪️registry-providers/🦀️.rs", import.meta.url), "utf8");
const baseline = readFileSync(new URL("../🧫️fixtures/🧾️original-api/🦀️.rs", import.meta.url), "utf8");
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output || !isAbsolute(output)) throw Error("SEMIO_TEST_ARTIFACT_DIR must name the caller-owned absolute artifact directory");
mkdirSync(output, { recursive: true });
const directory = mkdtempSync(join(output, "assembly-lifetime-"));
const hash = (bytes: string): string => createHash("sha256").update(bytes).digest("hex");

/** 🧷️ Extracts the actual owning declarations without substituting a lifetime model. */
function section(start: string, end: string): string {
  const first = source.indexOf(start);
  expect(first).toBeGreaterThanOrEqual(0);
  expect(source.lastIndexOf(start)).toBe(first);
  const last = source.indexOf(end, first);
  expect(last).toBeGreaterThan(first);
  return source.slice(first, last);
}

const assembly = readFileSync(new URL("../../../../../../../🔨️modules/🧬️schema/📇️registry/🧷️assembly/🦀️.rs", import.meta.url), "utf8");
const current = "mod semio_framework_schema_registry { pub mod assembly {\n" + assembly + "\n}}\n" + section("/// 🧷️ All writable store registries", "/// 🔬️ Verifies all staged store rows") + "\n";

/** 🦀️ Runs one bounded native compiler with explicitly adjacent data and provider stubs. */
function compile(api: string, client: string, id: string, stage: string) {
  const input = join(directory, id + "-" + stage + ".rs");
  const binary = join(directory, id + "-" + stage + (process.platform === "win32" ? ".exe" : ""));
  writeFileSync(input, providers + api + "\n" + client);
  const args = ["--edition=2021", "--crate-name", "assembly_lifetime_probe", input, "--emit=link,dep-info", "-o", binary];
  const result = Bun.spawnSync(["rustc", ...args], { cwd: directory, stdout: "pipe", stderr: "pipe", timeout: 20000 });
  const stderr = result.stderr.toString();
  writeFileSync(join(directory, id + "-" + stage + "-compiler.log"), result.stdout.toString() + stderr);
  const codes = [...new Set([...stderr.matchAll(/error\[(E\d+)\]/g)].map((row) => row[1]))];
  console.log(`[assembly-lifetime] scenario=${id} stage=${stage} exit=${result.exitCode} diagnostics=${codes.join(",")}`);
  return { exitCode: result.exitCode, stderr, codes, binary, inputSHA: hash(readFileSync(input, "utf8")) };
}

test("assembly lifetime schema is closed under owned and independent validation", () => {
  const foreign = { ...fixture, foreign: true };
  expect(new Set(fixture.scenarios.map((row) => row.id)).size).toBe(4);
});

test("assembly lifetime baseline retains the exact original API and registry scope", () => {
  expect(hash(baseline)).toBe("0ad696eb7431112abb41d2876b7eb83ed938f1909747efe5442de9ea8bc84018");
  expect(baseline).toContain("Result<ArtifactAssemblyStoreRegistryGuards, ArtifactAssemblyStoreRegistryError>");
  expect(current).toContain("_assembly: &'assembly semio_framework_schema_registry::assembly::Transaction");
  expect(current).toContain("Result<ArtifactAssemblyStoreRegistryGuards<'assembly>, ArtifactAssemblyStoreRegistryError>");
});

for (const row of fixture.scenarios) test("actual Store assembly lifetime: " + row.id, () => {
  const client = readFileSync(new URL("../🧫️fixtures/🧑️client/" + row.id + "/🦀️.rs", import.meta.url), "utf8");
  const original = compile(baseline, client, row.id, "original");
  expect(original.exitCode, original.stderr).toBe(0);
  const currentClient = readFileSync(new URL("../🧫️fixtures/🧑️client/" + row.id + "/📇️registry/🦀️.rs", import.meta.url), "utf8");
  const owned = compile(current, currentClient, row.id, "current");
  expect(owned.exitCode === 0, owned.stderr).toBe(row.accepted);
  if (row.diagnostic) expect(owned.codes).toContain(row.diagnostic);
  let runtime: { exitCode: number; stdout: string } | null = null;
  if (row.accepted) {
    const result = Bun.spawnSync([owned.binary], { cwd: directory, stdout: "pipe", stderr: "pipe", timeout: 10000 });
    runtime = { exitCode: result.exitCode, stdout: result.stdout.toString() };
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(runtime.stdout).toContain("released registry guards before the barrier");
  }
  expect(hash(readFileSync(storeUrl, "utf8"))).toBe(hash(source));
  writeFileSync(join(directory, row.id + "-receipt.json"), JSON.stringify({ id: row.id, storeSHA: hash(source), assemblySHA: hash(assembly), clientSHA: hash(client), currentClientSHA: hash(currentClient), original, owned, runtime, scope: "Actual Store declarations with adjacent value/error/provider stubs; compiler lifetime and valid disposal runtime only" }, null, 2));
}, 60000);
