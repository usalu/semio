import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import Ajv from "ajv";
import ts from "typescript";
import { Database } from "bun:sqlite";
import Parser from "web-tree-sitter";
import fixture from "./🔣️.json" with { type: "json" };
import schema from "./🧬️.json" with { type: "json" };
import receipt from "../../../⚛️reactor/🎟️context/🌉️call/📤️receipt/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { readPluginRuntimeSource } from "../../../⚛️reactor/🏃️runtime/📖️source-text/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", import.meta.dir)), "out/tree-sitter-rust.wasm")));

/** 🔌️ Every authored job crossing is a real selected core export, including output handback and complete closure. */
test("owned interpreter selects all six original WIT job exports", () => {
  expect(new Ajv2020({ strict: true }).validate(schema, fixture)).toBe(true);
  const source = readFileSync(resolve(owner, "🦀️.rs"), "utf8"), tree = parser.parse(source);
  const enumNode = tree.rootNode.namedChildren.find(node => node.type === "enum_item" && node.childForFieldName("name")?.text === "OwnedSemioExport");
  expect(enumNode).toBeDefined();
  const variants = enumNode!.childForFieldName("body")!.namedChildren.filter(node => node.type === "enum_variant").map(node => node.childForFieldName("name")!.text);
  const constants = new Map(tree.rootNode.namedChildren.filter(node => node.type === "const_item").map(node => [node.childForFieldName("name")!.text, node.childForFieldName("value")!.text]));
  const implementation = tree.rootNode.namedChildren.find(node => node.type === "impl_item" && node.childForFieldName("type")?.text === "OwnedSemioExport")!;
  const members = implementation.childForFieldName("body")!.namedChildren;
  const all = members.find(node => node.type === "const_item" && node.childForFieldName("name")?.text === "ALL")!.childForFieldName("value")!.text;
  const names = members.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "core_name")!.childForFieldName("body")!.text;
  const typeBody = members.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "core_type")!.childForFieldName("body")!;
  const typeArms = typeBody.descendantsOfType("match_arm");
  const wit = readFileSync(resolve(owner, "../🧬️schema/📜️.wit"), "utf8");
  const runtimeTree = parser.parse(readFileSync(resolve(owner, "🏃️runtime/🦀️.rs"), "utf8"));
  const operation = runtimeTree.rootNode.namedChildren.find(node => node.type === "enum_item" && node.childForFieldName("name")?.text === "OwnedOperation")!;
  const operations = operation.childForFieldName("body")!.namedChildren.filter(node => node.type === "enum_variant").map(node => node.childForFieldName("name")!.text);
  const operationImpl = runtimeTree.rootNode.namedChildren.find(node => node.type === "impl_item" && node.childForFieldName("type")?.text === "OwnedOperation")!;
  const selected = operationImpl.childForFieldName("body")!.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "export")!.childForFieldName("body")!;
  const db = new Database(":memory:");
  try {
    db.run("create table original_export (variant text primary key, core text unique, wit text unique)");
    for (const row of fixture.jobs) {
      db.query("insert into original_export values (?,?,?)").run(row.variant, row.core, row.wit);
      expect(variants).toContain(row.variant);
      expect(constants.get(row.constant)).toBe(JSON.stringify(row.core));
      expect(all.match(new RegExp("\\bSelf::" + row.variant + "\\b", "g"))?.length).toBe(1);
      expect(names).toContain("Self::" + row.variant + " => " + row.constant);
      expect(wit).toContain(row.wit + ": async func(");
      expect(operations).toContain(row.variant);
      expect(selected.descendantsOfType("match_arm").filter(node => node.text.includes("Self::" + row.variant + " => OwnedSemioExport::" + row.variant)).length).toBe(1);
    }
    expect((db.query("select count(*) as count from original_export").get() as { count: number }).count).toBe(6);
    for (const row of [...fixture.jobs, ...fixture.helpers]) {
      expect(variants).toContain(row.variant);
      expect(constants.get(row.constant)).toBe(JSON.stringify(row.core));
      expect(all.match(new RegExp("\\bSelf::" + row.variant + "\\b", "g"))?.length).toBe(1);
      const arm = typeArms.find(node => new RegExp("\\bSelf::" + row.variant + "\\b").test(node.text))!.text.replace(/\s+/gu, "");
      const parameters = row.parameters[0] === "i32" ? "ValueType::I32,ValueType::I32" : "ValueType::I64;" + row.parameters.length;
      expect(arm).toContain("parameters:vec![" + parameters + "],results:vec![ValueType::I64]");
    }
  } finally { db.close(); runtimeTree.delete(); tree.delete(); }
});

/** 🚀️ GUI commands admit the actual outer Nx caller and declare source versus native execution explicitly. */
test("job contract GUI launches match the supported original Nx caller", () => {
  const root = resolve(owner, "../../../../../.."), target = JSON.parse(readFileSync(resolve(owner, "📦️packages/🦀️rust/📋️project.json"), "utf8"));
  const library = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
  const admit = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(resolve(library, "🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json"), "utf8")));
  expect(target.targets[fixture.commands.source].metadata.semio.execution).toEqual({ version: 1, kind: "source" });
  expect(target.targets[fixture.commands.native].metadata.semio.execution).toEqual({ version: 1, kind: "native", scope: "test" });
  for (const file of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const parsed = ts.parseConfigFileTextToJson(file, readFileSync(resolve(root, file), "utf8"));
    expect(parsed.error).toBeUndefined();
    for (const command of Object.values(fixture.commands)) {
      const selected = "@semio-tech/framework-plugin-interpreter:" + command;
      const rows = parsed.config.configurations.filter((row: { command?: string }) => row.command?.split(" ").includes(selected));
      expect(rows.length).toBe(1);
      const row = rows[0], policy = JSON.parse(row.env.SEMIO_SCRIPT_POLICY);
      expect(policy.maximumElapsedMilliseconds).toBeGreaterThan(0);
      expect(row.env.SEMIO_SCRIPT_PROCESS_INVOCATION).toBeUndefined();
      if (command === fixture.commands.native) {
        const replace = (value: string) => value.replaceAll("${workspaceFolder}", root);
        const caller = JSON.parse(replace(row.env.SEMIO_SCRIPT_CAPABILITIES));
        expect(admit(caller)).toBe(true);
        expect(caller.selection).toEqual(["repo", "run", selected]);
        expect(caller.arguments).toEqual(caller.selection);
        expect(replace(row.env.CARGO_TARGET_DIR)).toBe(caller.native.artifactDirectory);
        expect(row.env.CARGO_NET_OFFLINE).toBe(String(caller.native.network.offline));
        expect(row.env.SEMIO_BUILD_MODE).toBe(caller.buildMode);
      }
    }
  }
  const bootstrap = readFileSync(resolve(library, "⚡️caching/🚀️bootstrap/📜️script.ts"), "utf8");
  expect(bootstrap).toContain("createScriptProcessEnvelope(policy, capabilities, Date.now())");
  expect(bootstrap).toContain("const encoded = process.env.SEMIO_SCRIPT_POLICY");
});

/** 🕸️ The independent system engine validates and executes the authored shared job core type. */
test("authored job selector core types match actual export signatures and independent WebAssembly", () => {
  const runtime = readPluginRuntimeSource();
  for (const row of [...fixture.jobs, ...fixture.helpers]) {
    const signature = new RegExp("fn " + row.core + "\\(([^)]*)\\)\\s*->\\s*u64", "u").exec(runtime);
    expect(signature, row.core).not.toBeNull();
    const tree = parser.parse("pub extern \"C\" fn " + row.core + "(" + signature![1] + ")->u64{0}");
    try {
      const parameters = tree.rootNode.namedChildren.find(node => node.type === "function_item")!.childForFieldName("parameters")!.namedChildren;
      expect(parameters.map<string>(node => node.childForFieldName("type")!.text === "u32" ? "i32" : node.childForFieldName("type")!.text === "u64" ? "i64" : "unsupported"), row.core).toEqual(row.parameters);
    } finally { tree.delete(); }
    const type = [1, 96, row.parameters.length, ...row.parameters.map(type => type === "i32" ? 127 : 126), 1, 126], name = Buffer.from(row.core), exported = [1, name.length, ...name, 0, 0];
    const bytes = Buffer.from([0,97,115,109,1,0,0,0,1,type.length,...type,3,2,1,0,7,exported.length,...exported,10,6,1,4,0,66,0,11]);
    expect(WebAssembly.validate(bytes), row.core).toBe(true);
    const compiled = new WebAssembly.Module(bytes);
    expect(WebAssembly.Module.exports(compiled).map(({ name, kind }) => ({ name, kind }))).toEqual([{ name: row.core, kind: "function" }]);
    const instance = new WebAssembly.Instance(compiled), invoke = instance.exports[row.core] as (...args: (number | bigint)[]) => bigint;
    expect(invoke(...row.parameters.map(type => type === "i32" ? 0 : 0n))).toBe(0n);
  }
  expect(fixture.results).toEqual(["i64"]);
});

/** 🧾️ The owned decoder recognizes the actual current fixed receipt before the dynamic JSON result path. */
test("owned result decoder preserves the original current fixed actor receipt", () => {
  const bytes = Buffer.from(receipt.failureHex, "hex");
  expect(bytes.length).toBe(receipt.extent);
  expect(bytes.subarray(0, 4).toString()).toBe("SAC2");
  expect(receipt.fields.map((_, index) => bytes.readBigUInt64LE(8 + index * 8).toString())).toEqual(receipt.fields.map(String));
  const db = new Database(":memory:"), tree = parser.parse(readFileSync(resolve(owner, "🏃️runtime/🦀️.rs"), "utf8"));
  try {
    expect(db.query("select length(?) as extent,hex(substr(?,1,4)) as magic").get(bytes, bytes)).toEqual({ extent: receipt.extent, magic: "53414332" });
    const decoder = tree.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "decode_owned_result")!;
    const body = decoder.childForFieldName("body")!.text;
    expect(body.indexOf('bytes.starts_with(b"SAC2")')).toBeGreaterThanOrEqual(0);
    expect(body.indexOf("OriginalActorFixedReceipt::read(bytes)")).toBeLessThan(body.indexOf("serde_json::from_slice(bytes)"));
    expect(body).toContain("TurnFault::ActorCall");
  } finally { db.close(); tree.delete(); }
});
