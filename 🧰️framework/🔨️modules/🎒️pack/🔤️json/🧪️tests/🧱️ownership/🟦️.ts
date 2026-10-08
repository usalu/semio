import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import TOML from "@iarna/toml";
import { parseTree, getNodeValue, type Node, type ParseError } from "jsonc-parser";
import { JsonMemberPolicy } from "../../🧩️members/🟦️.ts";

const owner = resolve(import.meta.dir, "../.."), read = (path: string): string => readFileSync(resolve(owner, path), "utf8");

test("controlled JSON retirement retains neutral independent grants and original syntax", () => {
  const fixture = JSON.parse(read("🧫️fixtures/🎮️retirement/🔣️.json"));
  const errors: ParseError[] = [], tree = parseTree(fixture.source, errors, {disallowComments: true, allowTrailingComma: false});
  expect(errors).toEqual([]);
  expect(getNodeValue(tree!)).toEqual(JSON.parse(fixture.source));
});

test("completed JSON frame conserves original admission with independently parsed retained output",()=>{
  const fixture=JSON.parse(read("🧫️fixtures/📦️completed-frame/🔣️.json"));
  const errors:ParseError[]=[],tree=parseTree(fixture.source,errors,{disallowComments:true,allowTrailingComma:false});expect(errors).toEqual([]);expect(getNodeValue(tree!)).toEqual(JSON.parse(fixture.source));
});

test("owned member authority has exactly two schema-admitted explicit selections", () => {
  const validate = new Ajv({strict: true}).compile(JSON.parse(read("🧬️schema/🧩️members/🔣️.json")));
  expect(Object.values(JsonMemberPolicy)).toEqual(["Reject", "Replace"]);
  for (const selection of Object.values(JsonMemberPolicy)) expect(validate(selection)).toBe(true);
  for (const refused of [undefined, null, "default", "", {policy: "Reject"}]) expect(validate(refused)).toBe(false);
  const native = read("🧩️members/🦀️.rs");
  expect(native).toContain("pub enum JsonMemberPolicy");
  expect(native).toContain("Reject,");
  expect(native).toContain("Replace,");
  expect(native).not.toContain("Default");
});

test("the actual JSON primitive package owns its definitions and depends directly on Value", () => {
  const path = "📦️packages/🦀️rust/Cargo.toml";
  expect(existsSync(resolve(owner, path)), "actual independent Cargo owner").toBe(true);
  const source = read(path), native = Bun.TOML.parse(source) as {package: {name: string}; lib: {name: string; path: string}; dependencies: Record<string, unknown>};
  expect(native).toEqual(TOML.parse(source) as typeof native);
  expect(native.package.name).toBe("semio-framework-pack-json");
  expect(native.lib).toEqual({name: "semio_framework_pack_json", path: "../../🦀️.rs"});
  expect(Object.keys(native.dependencies)).toEqual(["semio-framework-value"]);
  expect(read("🦀️.rs")).not.toContain("protocol::value");
  expect(read("🦀️.rs")).not.toContain("$crate::json::");
});

test("all public JSON source readers require an explicit decoded-member policy", () => {
  const source = read("🦀️.rs");
  expect(source.includes("pub fn parse(input: &str, policy: JsonMemberPolicy)"), "ordinary reader policy").toBe(true);
  expect(source.includes("pub fn parse_bytes(input: &[u8], policy: JsonMemberPolicy)"), "byte reader policy").toBe(true);
  expect(source.includes("pub fn from_json_str<T: FromValue>(text: &str, policy: JsonMemberPolicy)"), "typed reader policy").toBe(true);
  expect(source.includes("pub fn from_json_str_controlled<T: FromValue>(text: &str, policy: JsonMemberPolicy,"), "controlled reader policy").toBe(true);
  expect(source.includes("DuplicateMember"), "owned decoded-member refusal").toBe(true);
});

test("independent JSON syntax authority validates every decoded-name duplicate and UTF8 byte offset", () => {
  const fixture = JSON.parse(read("🧫️fixtures/🧩️members/🔣️.json")) as {cases: {id: string; text: string; duplicate: null | {name: string; offset: number}}[]};
  for (const row of fixture.cases) {
    const errors: ParseError[] = [], tree = parseTree(row.text, errors, {disallowComments: true, allowTrailingComma: false})!;
    expect(errors, row.id).toEqual([]);
    const visit = (node: Node): null | {name: string; offset: number} => {
      if (node.type === "object") {
        const names = new Set<string>();
        for (const property of node.children ?? []) {
          const [key, value] = property.children!;
          if (names.has(key!.value)) return {name: key!.value, offset: Buffer.byteLength(row.text.slice(0, key!.offset))};
          names.add(key!.value);
          const duplicate = visit(value!);
          if (duplicate) return duplicate;
        }
      } else for (const child of node.children ?? []) { const duplicate = visit(child); if (duplicate) return duplicate; }
      return null;
    };
    expect(visit(tree), row.id).toEqual(row.duplicate);
  }
});


test("independent JSON member ownership corpus retains first positions and final descendant values", () => {
  const fixture = JSON.parse(read("🧫️fixtures/🧩️members/🔣️.json")) as {ownership: {text: string; keys: string[]}};
  const errors: ParseError[] = [], tree = parseTree(fixture.ownership.text, errors, {disallowComments: true, allowTrailingComma: false})!;
  expect(errors).toEqual([]);
  const reference = getNodeValue(tree) as Record<string, unknown>;
  expect(Object.keys(reference)).toEqual(fixture.ownership.keys);
  expect(reference).toEqual(JSON.parse(fixture.ownership.text));
  const keys = new Map<string, Node>();
  for (const property of tree.children!) keys.set(property.children![0]!.value, property.children![1]!);
  expect([...keys.keys()]).toEqual(fixture.ownership.keys);
  expect(Object.fromEntries([...keys].map(([key, value]) => [key, getNodeValue(value)]))).toEqual(reference);
});


test("independent JSON parser validates retained admission large candidates", () => {
  const fixture=JSON.parse(read("🧫️fixtures/🚦️owned-controls.json")) as {retainedAdmission:{textUnit:string;textRepeats:number;collectionItems:number;phases:string[]}};
  const law=fixture.retainedAdmission,label=law.textUnit.repeat(law.textRepeats);
  for(const source of [JSON.stringify(label),JSON.stringify(Array.from({length:law.collectionItems},(_,index)=>({index,label:index%128===0?label:""})))]){
    const errors:ParseError[]=[],tree=parseTree(source,errors,{disallowComments:true,allowTrailingComma:false})!;
    expect(errors).toEqual([]);expect(getNodeValue(tree)).toEqual(JSON.parse(source));
  }
  expect(law.phases).toEqual(["measure-string","materialize-string","collect-array","collect-object","materialize-array","materialize-object"]);
});


test("neutral borrowed-read fixture retains original Source syntax and independent jsonc-parser lexemes", async () => {
  const fixture = JSON.parse(read("🧫️fixtures/🫳️read-source.json")) as {source: string; duplicate: string; invalidUtf8: number[]; largeStringBytes: number; decodeAllocationBytes: number; stepUnits: number};
  const {decodeJsonSyntax} = await import("../../📥️decode/🟦️.ts");
  const operation = () => ({maximumBytes: fixture.decodeAllocationBytes, maximumNodes: 4096, maximumDepth: 128, chunk: 256, cancelled: () => false, progress: (_: unknown) => {}, yield: async () => {}});
  const syntax = (node: Node, text: string): Awaited<ReturnType<typeof decodeJsonSyntax>> => {
    if (node.type === "number") return {kind: "number", text: text.slice(node.offset, node.offset + node.length)};
    if (node.type === "null") return {kind: "null"};
    if (node.type === "string" || node.type === "boolean") return {kind: node.type, value: node.value};
    if (node.type === "array") return {kind: "array", items: (node.children ?? []).map(child => syntax(child, text))};
    return {kind: "object", members: (node.children ?? []).map(child => ({name: child.children![0]!.value, value: syntax(child.children![1]!, text)}))};
  };
  const errors: ParseError[] = [], tree = parseTree(fixture.source, errors, {disallowComments: true, allowTrailingComma: false});
  expect(errors).toEqual([]);
  expect(await decodeJsonSyntax(fixture.source, JsonMemberPolicy.Reject, operation())).toEqual(syntax(tree!, fixture.source));
  await expect(decodeJsonSyntax(fixture.duplicate, JsonMemberPolicy.Reject, operation())).rejects.toMatchObject({code: "duplicate-member"});
  expect(() => new TextDecoder("utf-8", {fatal: true}).decode(Uint8Array.from(fixture.invalidUtf8))).toThrow();
  const large = JSON.stringify("x".repeat(fixture.largeStringBytes));
  expect(await decodeJsonSyntax(large, JsonMemberPolicy.Reject, operation())).toEqual({kind: "string", value: JSON.parse(large)});
  console.log("[DEBUG] neutral original8194 source and exact numeric lexemes match first-party Source syntax, jsonc-parser and host JSON; Native paged retention remains a separate owning receipt");
});
