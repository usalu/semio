import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve,relative } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readRustSourceClosureV1, rustCfgMatchesV1, selectRustSourceV1 } from "../🟦️.ts";

const corpus = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { cases: { name: string; cfg: string[]; source: string; expected: string; oracle?: boolean }[]; closures:{name:string;cfg:string[];entry:string;files:Record<string,string>;expected:{path:string;kind:"source"|"embedded"}[]}[] };
const normalize = (source: string) => source.replace(/\s+/gu, " ").trim();
test("portable cfg source corpus is closed and validated by Ajv", () => {
  const validate = new Ajv().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/schema.json"), "utf8")));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, hidden: true })).toBe(false);
});
for (const row of corpus.cases) test(row.name, () => expect(normalize(selectRustSourceV1(row.source, row.cfg))).toBe(row.expected));
test("malformed conditional expressions fail closed", () => {
  for (const condition of ["not(test, unix)", "feature", "all(test", "feature = 42"]) expect(() => selectRustSourceV1(`#[cfg(${condition})] mod unsafe_mount;`, [])).toThrow();
});
test("a cfg-false attribute with nothing to own fails closed", () => {
  for (const source of ["fn f() { #[cfg(test)] }", "struct S { a: u8, #[cfg(test)] }", "#[cfg(test)]"]) expect(() => selectRustSourceV1(source, [])).toThrow("Unsupported cfg-owned Rust item");
});
const expand = (directory: string, source: string, cfg: string[]) => {
  const file = join(directory, "oracle.rs"), flags = cfg.flatMap(value => ["--cfg", value.includes("=") ? value.replace(/=(.*)$/u, '="$1"') : value]);
  writeFileSync(file, source);
  const run = spawnSync(process.env.RUSTC ?? "rustc", ["-Zunpretty=expanded", "--edition", "2021", "--crate-type", "lib", "--crate-name", "oracle", "-Awarnings", ...flags, file], { encoding: "utf8", env: { ...process.env, RUSTC_BOOTSTRAP: "1" } });
  if (run.status !== 0) throw Error(`rustc rejected ${JSON.stringify(source)}: ${run.stderr}`);
  return run.stdout;
};
test("independent rustc cfg expansion agrees with the selected source of every oracle vector", () => {
  const directory = mkdtempSync(join(tmpdir(), "rust-cfg-oracle-"));
  try {
    const rows = corpus.cases.filter(row => row.oracle);
    expect(rows.length).toBeGreaterThanOrEqual(20);
    for (const row of rows) expect(expand(directory, selectRustSourceV1(row.source, row.cfg), []), row.name).toBe(expand(directory, row.source, row.cfg));
  } finally { rmSync(directory, { recursive: true, force: true }); }
}, 120_000);
test("independent SQLite Boolean evaluation agrees with nested cfg predicates", () => {
  const database = new Database(":memory:");
  try {
    for (const cfg of [[], ["test"], ["unix"], ["unix", "feature=engine"], ["target_env=p2"]]) {
      const row = database.query("SELECT (?1 AND NOT ?2) OR ?3 AS selected").get(Number(cfg.includes("unix")), Number(cfg.includes("feature=engine")), Number(cfg.includes("target_env=p2"))) as { selected: number };
      expect(rustCfgMatchesV1('any(all(unix, not(feature = "engine")), target_env = "p2")', cfg)).toBe(Boolean(row.selected));
    }
  } finally { database.close(); }
});
test("production source DAG selects cfg mounts, excludes tests and includes external module sources", async () => {
  const root = resolve("virtual-workspace");
  const fixture = new Map<string, string>([
    [join(root, "package/lib.rs"), '#[cfg(test)] mod tests; #[path="../shared/leaf.rs"] mod leaf; mod inline { #[cfg(feature="engine")] mod engine; }'],
    [join(root, "shared/leaf.rs"), 'include_str!("vocabulary.json"); pub struct Leaf;'],
    [join(root, "shared/vocabulary.json"), '{"version":1}'],
    [join(root, "package/inline/engine.rs"), "pub struct Engine;"],
    [join(root, "package/tests.rs"), "this source must never be parsed"],
  ]);
  const read = async (path: string) => fixture.has(path) ? new TextEncoder().encode(fixture.get(path)!) : null;
  const portable = await readRustSourceClosureV1(["package/lib.rs"], root, [], read);
  const engine = await readRustSourceClosureV1(["package/lib.rs"], root, ["feature=engine"], read);
  expect(portable.map(row => row.path)).toEqual(["package/lib.rs", "shared/leaf.rs", "shared/vocabulary.json"]);
  expect(engine.map(row => row.path)).toContain("package/inline/engine.rs");
  expect(portable.every(row => /^[a-f0-9]{64}$/u.test(row.digest))).toBe(true);
  expect(portable.find(row => row.path.endsWith("vocabulary.json"))!.kind).toBe("embedded");
  await expect(readRustSourceClosureV1(["package/lib.rs"], join(root, "package"), [], read)).rejects.toThrow();
});
test("a unit owns manifest-relative inputs and the token output it emits, while OUT_DIR and unowned closures stay refused", async () => {
  const root = resolve("virtual-owner"), bytes = new Uint8Array([0, 159, 146, 150, 10, 10]);
  const fixture = new Map<string, string | Uint8Array>([
    [join(root, "crate/src/lib.rs"), 'pub mod lifetime { include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shared/lifetime.rs")); } pub static ICC: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/profile.icc"));'],
    [join(root, "shared/lifetime.rs"), "pub struct Lifetime;\npub fn live() {}"],
    [join(root, "assets/profile.icc"), bytes],
    [join(root, "derive/lib.rs"), 'use quote::quote; pub fn expand(path: &str) -> proc_macro2::TokenStream { quote! { const _: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #path)); } }'],
    [join(root, "generated/lib.rs"), 'include!(concat!(env!("OUT_DIR"), "/bindings.rs"));'],
  ]);
  const read = async (path: string) => { const value = fixture.get(path); return value === undefined ? null : typeof value === "string" ? new TextEncoder().encode(value) : value; };
  const emitted: string[] = [], owner = (directory: string) => ({ manifestDirectory: join(root, directory), generated: (file: string, output: { inputs: readonly { kind: string }[] }) => emitted.push(file + ":" + output.inputs.map(input => input.kind).join(",")) });
  const owned = await readRustSourceClosureV1(["crate/src/lib.rs"], root, [], read, owner("crate"));
  expect(owned.map(row => [row.path, row.kind, row.activeLines])).toEqual([["assets/profile.icc", "embedded", 0], ["crate/src/lib.rs", "source", 1], ["shared/lifetime.rs", "source", 2]]);
  expect(owned.find(row => row.kind === "embedded")!.bytes).toBe(bytes.byteLength);
  await expect(readRustSourceClosureV1(["crate/src/lib.rs"], root, [], read)).rejects.toThrow("Generated Rust input requires compiler ownership");
  expect((await readRustSourceClosureV1(["derive/lib.rs"], root, [], read, owner("derive"))).map(row => row.path)).toEqual(["derive/lib.rs"]);
  expect(emitted).toEqual(["derive/lib.rs:include_str"]);
  await expect(readRustSourceClosureV1(["derive/lib.rs"], root, [], read)).rejects.toThrow("explicit generated-input observer");
  await expect(readRustSourceClosureV1(["generated/lib.rs"], root, [], read, owner("generated"))).rejects.toThrow("Generated Rust input requires compiler ownership");
});

for(const row of corpus.closures)test(row.name,async()=>{
 const root=resolve("virtual-source-role"),read=async(path:string)=>{const local=relative(root,path).replaceAll("\\","/");return Object.hasOwn(row.files,local)?new TextEncoder().encode(row.files[local]!):null;},inputs=await readRustSourceClosureV1([row.entry],root,row.cfg,read);
 expect(inputs.map(({path,kind})=>({path,kind}))).toEqual(row.expected);
 if(row.name==="inactive inner file Source")expect(inputs[0]!.activeLines).toBe(0);
 console.log("[DEBUG] Exact authored Rust Source membership "+row.name);
});
