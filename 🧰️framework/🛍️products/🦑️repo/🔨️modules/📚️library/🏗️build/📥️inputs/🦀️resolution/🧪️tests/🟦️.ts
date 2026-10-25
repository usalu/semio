/** 🦀️ The Cargo resolution identity agrees with its language-agnostic fixture, an independent Python implementation, two TOML parsers and the policy; a crate's key moves exactly with what the crate compiles with. */
import {expect, test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {readFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {parse as iarna} from "@iarna/toml";
import Ajv from "ajv";
import {CargoResolutionErrorV1, type CargoLeafV1, type CargoReceiptStoreV1, cargoResolutionPartsV1, flushCargoReceiptsV1, canonicalJsonV1, composeCargoResolutionV1, createCargoResolutionContextV1, crateIdentityV1, readCargoResolutionPolicyV1, toolchainIdentityV1, workspaceKeyViolationV1, workspaceKeyViolationsV1, cargoResolutionTextV1} from "../🟦️.ts";
import {decodeCargoReceiptV1, encodeCargoReceiptV1, cargoReceiptNameV1} from "../💾️receipt/🟨️.mjs";
import policyDocument from "../🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type: "json"};

const here = import.meta.dir, policy = readCargoResolutionPolicyV1(policyDocument);
const tree = (text: string) => Bun.TOML.parse(text) as Record<string, unknown>;
const apply = (root: any, path: (string | number)[], value: unknown) => {
  let node = root;
  for (const part of path.slice(0, -1)) node = node[part];
  const last = path[path.length - 1]!;
  if (value === null) delete node[last];
  else if (Array.isArray(value) && value[0] === "$push") node[last].push(...value.slice(1));
  else node[last] = value;
};
const digestOf = (value: unknown) => createHash("sha256").update(canonicalJsonV1(value)).digest("hex");
const hostOf = (trees: Record<string, unknown>) => ({read: async (path: string) => (trees[path] ? {value: trees[path] as Record<string, unknown>, sha: digestOf(trees[path])} : null), digest: async (path: string) => (trees[path] ? digestOf(trees[path]) : null)});
const context = (trees: Record<string, unknown>, store?: CargoReceiptStoreV1) => createCargoResolutionContextV1(hostOf(trees), policy, store);
const queries = fixture.queries.map((query) => [query.slice(0, -2), query.endsWith(":t")] as [string, boolean]);
const identities = async (trees: Record<string, unknown>) => { const ctx = context(trees); return Object.fromEntries(await Promise.all(queries.map(async ([crate, tests]) => [`${crate}:${tests ? "t" : "b"}`, await crateIdentityV1(ctx, crate, tests)]))); };
const oracle = JSON.parse(spawnSync(testPythonV1()??"python3", ["-I", join(here, "🐍️oracle/🐍️.py"), join(here, "🔣️.json"), join(dirname(here), "🔣️.json")], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024}).stdout);

test("the fixture and the policy satisfy their closed schemas", () => {
  const ajv = new Ajv({strict: false});
  expect(ajv.validate(fixtureSchema, fixture), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, policyDocument), JSON.stringify(ajv.errors)).toBe(true);
});

test("the stored trees are what two independent TOML parsers read from the stored files", () => {
  for (const [path, text] of Object.entries(fixture.files)) {
    expect(tree(text), path).toEqual(fixture.trees[path as keyof typeof fixture.trees] as never);
    expect(JSON.parse(JSON.stringify(iarna(text))), `iarna ${path}`).toEqual(JSON.parse(JSON.stringify(fixture.trees[path as keyof typeof fixture.trees])));
  }
});

test("canonical JSON sorts keys by bytes and agrees with the Python oracle", () => {
  fixture.canonical.forEach((row, index) => {
    expect(canonicalJsonV1(row.value)).toBe(row.text);
    expect(oracle.canonical[index]).toBe(row.text);
  });
});

test("every crate identity equals the frozen digest and the Python oracle's", async () => {
  expect(await identities(fixture.trees)).toEqual(fixture.expect.base);
  expect(oracle.base).toEqual(fixture.expect.base);
  expect(new Set(Object.values(fixture.expect.base)).size).toBeGreaterThan(5);
  const tool = await toolchainIdentityV1(context(fixture.trees));
  expect(tool).toEqual(fixture.expect.tool);
  expect(oracle.tool).toEqual(fixture.expect.tool);
  expect(composeCargoResolutionV1({lock: "L", workspace: fixture.expect.base["a:t"]!, ...tool})).toBe(fixture.expect.composed);
});

test("an edit moves exactly the crates that compile with what it changed", async () => {
  for (const edit of fixture.edits) {
    const trees: Record<string, any> = structuredClone(fixture.trees);
    for (const [path, value] of edit.set) apply(trees[edit.path], path as (string | number)[], value);
    const after = await identities(trees), changed = Object.keys(after).filter((key) => after[key] !== fixture.expect.base[key as keyof typeof fixture.expect.base]);
    expect(changed, `${edit.id}: ${edit.note}`).toEqual(edit.changed);
    expect(oracle.edits[edit.id], `${edit.id} oracle`).toEqual(edit.changed);
  }
});

test("toolchain and configuration move with their meaning, never with comments, ordering or artifact locations", async () => {
  for (const edit of fixture.toolEdits) {
    const trees: Record<string, any> = structuredClone(fixture.trees);
    trees[edit.path] = tree(edit.text);
    const tool = await toolchainIdentityV1(context(trees));
    expect(tool.toolchain !== fixture.expect.tool.toolchain || tool.config !== fixture.expect.tool.config, edit.id).toBe(edit.changed);
  }
});

test("a path cycle, a path leaving the repository and a workspace without a table are refused", async () => {
  for (const row of fixture.refusals) {
    let message: string | undefined;
    try { await crateIdentityV1(context(row.trees), row.crate, row.tests); } catch (error) { expect(error).toBeInstanceOf(CargoResolutionErrorV1); message = (error as Error).message; }
    expect(message, row.id).toBeDefined();
    expect({cycle: /cycle/u, escape: /escapes the repository/u, workspace: /names no \[workspace\]/u}[row.error]!.test(message!), `${row.id}: ${message}`).toBe(true);
    expect(oracle.refusals[row.id]).toBe(row.error);
  }
});

test("whole-workspace Rust and schema keys are recognised like the Python oracle does, and a recorded command exempts its own", () => {
  fixture.keys.forEach((row, index) => {
    expect(workspaceKeyViolationV1(policy, row.input) ?? null, row.input).toBe(row.violation);
    expect(oracle.keys[index]).toBe(row.violation);
  });
  for (const row of fixture.exemptCases) expect(workspaceKeyViolationsV1(policy, row.inputs, row.command)).toEqual(row.violations);
});

test("the policy never lists a key it also lets a task declare, and its messages exist in every language", () => {
  for (const literal of policy.workspaceKeys.literals) expect(workspaceKeyViolationV1(policy, `{workspaceRoot}/${literal}`)).toBe(literal);
  for (const tail of policy.workspaceKeys.tails) expect(workspaceKeyViolationV1(policy, `{workspaceRoot}/**/${tail}`)).toBe(`**/${tail}`);
  for (const language of policy.messages.languages) for (const key of Object.keys(policy.messages.texts) as (keyof typeof policy.messages.texts)[]) expect(cargoResolutionTextV1(policy, key, language, {subject: "s", keys: "k", crate: "c"}).length, `${key}/${language}`).toBeGreaterThan(8);
  expect(cargoResolutionTextV1(policy, "workspaceKey", "de", {subject: "p:t", keys: "Cargo.lock", crate: ""})).toContain("p:t");
});

test("on the real repository every crate resolves, none escapes, and the identity moves with a single inherited entry", async () => {
  if (process.env.SEMIO_RESOLUTION_REPO !== "1") return;
  const root = (() => { for (let at = here; ; at = dirname(at)) { try { readFileSync(join(at, "nx.json")); return at; } catch { if (dirname(at) === at) throw new Error("workspace root not found"); } } })();
  const cache = new Map<string, Record<string, unknown> | null>();
  const read = async (path: string) => { if (!cache.has(path)) { try { cache.set(path, Bun.TOML.parse(readFileSync(join(root, path), "utf8")) as Record<string, unknown>); } catch { cache.set(path, null); } } return cache.get(path)!; };
  const ctx = createCargoResolutionContextV1({read: async (path) => { const value = await read(path); return value ? {value, sha: digestOf(value)} : null; }, digest: async (path) => { const value = await read(path); return value ? digestOf(value) : null; }}, policy);
  const rootManifest = (await read("Cargo.toml")) as any;
  const members: string[] = rootManifest.workspace.members.slice(0, 40);
  const ids = new Set<string>();
  for (const member of members) ids.add(await crateIdentityV1(ctx, member, true));
  expect(ids.size).toBeGreaterThan(5);
}, 120000);


const memory = () => { const leaves = new Map<string, CargoLeafV1>(); const store: CargoReceiptStoreV1 = {read: (crate) => leaves.get(crate), write: (leaf) => { leaves.set(leaf.crate, JSON.parse(JSON.stringify(leaf))); }}; return {leaves, store}; };
const textHost = (files: Record<string, string>, log: string[] = []) => ({read: async (path: string) => { log.push(path); return path in files ? {value: tree(files[path]!), sha: createHash("sha256").update(files[path]!).digest("hex")} : null; }, digest: async (path: string) => (path in files ? createHash("sha256").update(files[path]!).digest("hex") : null)});
const textContext = (files: Record<string, string>, store?: CargoReceiptStoreV1, log?: string[]) => createCargoResolutionContextV1(textHost(files, log), policy, store);
const allIdentities = async (ctx: ReturnType<typeof textContext>) => { const ids: Record<string, string> = {}; for (const [crate, tests] of queries) ids[`${crate}:${tests ? "t" : "b"}`] = await crateIdentityV1(ctx, crate, tests); return ids; };

test("a warm run over receipts derives and parses nothing and returns the cold identities", async () => {
  const {leaves, store} = memory(), cold = textContext(fixture.files, store), ids = await allIdentities(cold);
  expect(ids).toEqual(fixture.expect.base);
  expect(cold.counts.derived).toBeGreaterThan(0);
  expect(await flushCargoReceiptsV1(cold)).toBe(leaves.size);
  const log: string[] = [], warm = textContext(fixture.files, store, log);
  expect(await allIdentities(warm)).toEqual(fixture.expect.base);
  expect({derived: warm.counts.derived, parsed: warm.counts.parsed, written: await flushCargoReceiptsV1(warm)}).toEqual({derived: 0, parsed: 0, written: 0});
  expect(log).toEqual([]);
  expect(warm.counts.digested).toBeLessThanOrEqual(Object.keys(fixture.files).length);
});

test("a comment-only edit re-derives one crate and moves no identity; a dependency edit moves only the dependents", async () => {
  const {leaves, store} = memory(), seed = textContext(fixture.files, store);
  await allIdentities(seed);
  await flushCargoReceiptsV1(seed);
  const comment = {...fixture.files, "c/Cargo.toml": "# only a comment\n" + fixture.files["c/Cargo.toml"]}, ctx = textContext(comment, store);
  expect(await allIdentities(ctx)).toEqual(fixture.expect.base);
  expect(ctx.counts.derived).toBe(2);
  expect([...ctx.dirty].sort()).toEqual(["c"]);
  const edited = {...fixture.files, "c/Cargo.toml": fixture.files["c/Cargo.toml"].replace('version = "0.3.0"', 'version = "0.3.1"').replace("[dependencies]", '[dependencies]\nextra = { path = "../devonly" }')}, moved = textContext(edited, store), after = await allIdentities(moved), scratch = await allIdentities(textContext(edited));
  expect(after).toEqual(scratch);
  expect(Object.keys(after).filter((key) => after[key] !== fixture.expect.base[key as keyof typeof fixture.expect.base]).sort()).toEqual(["a:b", "a:t", "b:b", "b:t", "c:b"]);
  expect(moved.counts.derived).toBe(2);
  expect(leaves.size).toBeGreaterThan(3);
});

test("a workspace manifest edit re-derives the crates of that workspace and none of the others", async () => {
  const {store} = memory(), seed = textContext(fixture.files, store);
  await allIdentities(seed);
  await flushCargoReceiptsV1(seed);
  const edited = {...fixture.files, "Cargo.toml": fixture.files["Cargo.toml"].replace('members = ["a", "b", "c", "devonly"]', 'members = ["a", "b", "c", "devonly", "e"]')}, ctx = textContext(edited, store);
  expect(await allIdentities(ctx)).toEqual(fixture.expect.base);
  expect(ctx.counts.derived).toBe(8);
  expect(new Set(await Promise.all(["nested/d"].map((crate) => crateIdentityV1(ctx, crate, false)))).size).toBe(1);
});

const locks = (extra = "") => ({"Cargo.lock": 'version = 4\n\n[[package]]\nname = "a"\nversion = "0.1.0"\ndependencies = ["b"]\n\n[[package]]\nname = "b"\nversion = "0.2.0"\ndependencies = ["c"]\n\n[[package]]\nname = "c"\nversion = "0.3.0"\n\n[[package]]\nname = "devonly"\nversion = "0.1.0"\n' + extra, "nested/Cargo.lock": 'version = 4\n\n[[package]]\nname = "d"\nversion = "0.1.0"\n'});
test("the locked slice of a crate is reused while the lock file and the crate's files keep their digests", async () => {
  const files = {...fixture.files, ...locks()}, {store} = memory(), seed = textContext(files, store), cold = await cargoResolutionPartsV1(seed, "a/Cargo.toml");
  await flushCargoReceiptsV1(seed);
  const log: string[] = [], warm = textContext(files, store, log);
  expect(await cargoResolutionPartsV1(warm, "a/Cargo.toml")).toEqual({...cold});
  expect(log.filter((path) => path.endsWith("Cargo.lock"))).toEqual([]);
  const unrelated = {...files, ...locks('\n[[package]]\nname = "unrelated"\nversion = "9.9.9"\n')}, appended = textContext(unrelated, store, log);
  expect(await cargoResolutionPartsV1(appended, "a/Cargo.toml")).toEqual({...cold});
  expect(appended.dirty.has("a")).toBe(true);
  const moved = {...files, ...locks().constructor === Object ? {"Cargo.lock": locks()["Cargo.lock"].replace('name = "c"\nversion = "0.3.0"', 'name = "c"\nversion = "0.3.0"\nchecksum = "changed"')} : {}};
  expect((await cargoResolutionPartsV1(textContext(moved, store), "a/Cargo.toml")).lock).not.toBe(cold.lock);
  const missing = textContext(fixture.files), refused = await cargoResolutionPartsV1(missing, "a/Cargo.toml");
  expect(refused.lock).toBe(createHash("sha256").update("missing\0Cargo.lock").digest("hex"));
  const stale = textContext({...files, "Cargo.lock": 'version = 4\n\n[[package]]\nname = "other"\nversion = "1.0.0"\n'});
  await expect(cargoResolutionPartsV1(stale, "a/Cargo.toml")).rejects.toThrow("Selected local Cargo lock package refused");
  expect((await cargoResolutionPartsV1(textContext({...files, "Cargo.lock": 'version = 4\n'}), "a/Cargo.toml", {tolerant: true})).lock).toStartWith("unresolved:");
});

test("a corrupt, foreign, unsealed or schema-violating receipt is no receipt", async () => {
  const {leaves, store} = memory(), seed = textContext(fixture.files, store);
  await allIdentities(seed);
  await flushCargoReceiptsV1(seed);
  const leaf = leaves.get("a")!, bytes = encodeCargoReceiptV1(leaf);
  expect(decodeCargoReceiptV1(bytes, "a")).toEqual(expect.objectContaining({crate: "a", version: 1}));
  expect(decodeCargoReceiptV1(bytes, "b")).toBeUndefined();
  expect(decodeCargoReceiptV1(bytes.subarray(0, bytes.length - 3), "a")).toBeUndefined();
  expect(decodeCargoReceiptV1(Buffer.from("not a receipt"), "a")).toBeUndefined();
  const tampered = encodeCargoReceiptV1({...leaf, t: {...leaf.t!, id: "f".repeat(64)}});
  const fraud = JSON.parse(require("node:zlib").brotliDecompressSync(tampered).toString()); fraud.t.id = "0".repeat(64);
  expect(decodeCargoReceiptV1(require("node:zlib").brotliCompressSync(Buffer.from(JSON.stringify(fraud))), "a")).toBeUndefined();
  expect(cargoReceiptNameV1("a")).toMatch(/^rr\/[a-f0-9]{24}\.json$/u);
  const emptyStore = {read: () => ({version: 1 as const, crate: "a", t: {...leaf.t!, id: "1".repeat(64)}} as never)};
  expect(await crateIdentityV1(textContext(fixture.files, emptyStore), "a", true)).toBe(fixture.expect.base["a:t" as keyof typeof fixture.expect.base]);
});
