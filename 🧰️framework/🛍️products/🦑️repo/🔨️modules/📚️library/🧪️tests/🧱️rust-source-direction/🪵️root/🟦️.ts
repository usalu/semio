import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { inspectRustSourceInputs } from "../../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";

const library = resolve(import.meta.dir, "../../..");
const fixture = JSON.parse(readFileSync(join(library, "🧫️fixtures/🧱️rust-source-direction/🪵️root/🔣️.json"), "utf8")) as { cases: readonly { id: string; mode: string; expected: { state: string; reason: string | null } }[]; freshness: { identity: string; expected: string; cancellation: string }; native: readonly { id: string; markers: readonly string[] }[] };
const schema = JSON.parse(readFileSync(join(library, "🧬️schema/🧱️rust-source-direction/🪵️root/🔣️.json"), "utf8"));

test("standalone root contract is closed with unique physical identities", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(fixture.cases.length);
});

test("standalone input inspection refuses root links before ordinary target reads", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Standalone Rust input roots require caller-owned artifacts");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-input-roots-")));
  try {
    for (const directory of ["physical", "physical/child", "other/nested", "other/physical"]) mkdirSync(join(root, directory), { recursive: true });
    for (const directory of ["physical", "physical/child"]) writeFileSync(join(root, directory, "leaf.rs"), 'pub const OWNER: &str = "declared";\n');
    writeFileSync(join(root, "other/physical/leaf.rs"), 'pub const OWNER: &str = "foreign";\n');
    writeFileSync(join(root, "file"), "root-file");
    symlinkSync(join(root, "physical"), join(root, "root-link"), process.platform === "win32" ? "junction" : "dir");
    symlinkSync(join(root, "other/nested"), join(root, "prefix-link"), process.platform === "win32" ? "junction" : "dir");
    const paths: Record<string, string> = { regular: join(root, "physical"), relative: relative(process.cwd(), join(root, "physical")), "root-link": join(root, "root-link"), "ancestor-link": join(root, "root-link/child"), file: join(root, "file"), missing: join(root, "missing"), dot: root + "/./physical", parent: root + "/other/../physical", "cancelled-link": root + "/prefix-link/../physical", empty: "", nul: root + "\0", "drive-relative": "C:root" };
    const rows = fixture.cases.map(row => ({ ...row, input: paths[row.mode]! }));
    const script = "const fs=require(\"node:fs\"),path=require(\"node:path\");const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>{const value=row.input;if(!value||value.includes(\"\\0\")||/^[A-Za-z]:(?:$|[^\\\\/])/.test(value)||value.split(/[\\\\/]/).some(part=>part===\".\"||part===\"..\"))return {state:\"rejected\",reason:\"unsafe-root\"};const root=path.resolve(value),parts=[];for(let current=root;;current=path.dirname(current)){parts.push(current);if(current===path.dirname(current))break;}for(const current of parts.reverse()){let node;try{node=fs.lstatSync(current);}catch(error){if(error.code===\"ENOENT\")return {state:\"rejected\",reason:\"missing-root\"};throw error;}if(node.isSymbolicLink())return {state:\"rejected\",reason:\"linked-root\"};if(!node.isDirectory())return {state:\"rejected\",reason:\"non-directory-root\"};}return {state:\"admitted\",reason:null};})));";
    const oracle = Bun.spawn(["node", "-e", script, JSON.stringify(rows)], { stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, status] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
    expect(status, stderr).toBe(0);
    const references = JSON.parse(stdout) as readonly { state: string; reason: string | null }[];
    for (const [index, row] of rows.entries()) expect(references[index], row.id).toEqual(row.expected);
    const markerOracle = Bun.spawn(["node", "-e", 'const fs=require("node:fs");console.log(JSON.stringify(JSON.parse(process.argv[1]).map(path=>fs.readFileSync(path+"/leaf.rs","utf8").match(/OWNER: &str = "([^"\\n]+)"/)[1])));', JSON.stringify(fixture.native.map(row => rows.find(value => value.id === row.id)!.input))], { stdout: "pipe", stderr: "pipe" });
    const [markers, markerErrors, markerStatus] = await Promise.all([new Response(markerOracle.stdout).text(), new Response(markerOracle.stderr).text(), markerOracle.exited]);
    expect(markerStatus, markerErrors).toBe(0);
    const physicalMarkers = JSON.parse(markers) as readonly string[];
    for (const [index, row] of fixture.native.entries()) {
      const input = rows.find(value => value.id === row.id)!.input, source = join(root, row.id + ".rs"), binary = join(root, row.id + (process.platform === "win32" ? ".exe" : ".bin"));
      writeFileSync(source, "include!(" + JSON.stringify(input + "/leaf.rs") + "); fn main(){println!(\"{}\",OWNER);}\n");
      const compiler = Bun.spawn(["rustc", "--edition=2021", "--crate-name", "root_oracle", source, "-o", binary], { cwd: root, stdout: "pipe", stderr: "pipe" });
      const [compiled, errors, code] = await Promise.all([new Response(compiler.stdout).text(), new Response(compiler.stderr).text(), compiler.exited]);
      expect(code, compiled + errors).toBe(0);
      const child = Bun.spawn([binary], { cwd: root, stdout: "pipe", stderr: "pipe" });
      expect(await child.exited).toBe(0);
      const actualMarker = (await new Response(child.stdout).text()).trim();
      expect(actualMarker, row.id).toBe(physicalMarkers[index]!);
      expect(row.markers.includes(actualMarker), row.id).toBe(true);
    }
    for (const row of rows) for (const targets of [[], [{ to: "leaf.rs", directories: [], reference: { kind: "include" as const, path: "leaf.rs", line: 1 } }]]) {
      let actual: { state: string; reason: string | null };
      try { expect(await inspectRustSourceInputs(row.input, targets, new Set(["leaf.rs"]))).toEqual([]); actual = { state: "admitted", reason: null }; }
      catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        actual = { state: "rejected", reason: message.includes("unnormalized safe root") ? "unsafe-root" : message.includes("linked root") ? "linked-root" : message.includes("root directory kind") ? "non-directory-root" : message.includes("unavailable root") ? "missing-root" : "unexpected" };
      }
      expect(actual, row.id).toEqual(row.expected);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("standalone roots are reobserved and cancellation precedes target admission", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Standalone Rust input roots require caller-owned artifacts");
  mkdirSync(output, { recursive: true });
  const parent = realpathSync(mkdtempSync(join(output, "rust-input-root-reuse-"))), root = join(parent, "root"), other = join(parent, "other");
  try {
    mkdirSync(root); mkdirSync(other);
    expect(await inspectRustSourceInputs(root, [], new Set())).toEqual([]);
    renameSync(root, join(parent, "retained"));
    symlinkSync(other, root, process.platform === "win32" ? "junction" : "dir");
    expect(await inspectRustSourceInputs(root, [], new Set()).then(() => "admitted", error => error.message.includes("linked root") ? "linked-root" : "unexpected"), fixture.freshness.identity).toBe(fixture.freshness.expected);
    expect(await inspectRustSourceInputs(other, [], new Set(), () => { throw new Error("checked-before-targets"); }).then(() => "admitted", error => error.message), "cancellation").toBe(fixture.freshness.cancellation);
  } finally { rmSync(parent, { recursive: true, force: true }); }
});
