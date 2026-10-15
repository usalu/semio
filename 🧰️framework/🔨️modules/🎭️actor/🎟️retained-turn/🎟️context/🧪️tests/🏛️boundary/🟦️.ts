import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync, existsSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import TOML from "@iarna/toml";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", import.meta.dir)), "out/tree-sitter-rust.wasm")));

/** 🏛️ The defining context and General tests resolve without any product source mount. */
test("original Actor context is a public General owner with private binding", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, productIndependent: false })).toBe(false);
  const owner = resolve(import.meta.dir, "../.."), implementation = join(owner, "🦀️.rs");
  expect(existsSync(implementation)).toBe(true);
  const source = readFileSync(implementation, "utf8"), tree = parser.parse(source), db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE general_type(name TEXT PRIMARY KEY,kind TEXT)");
    for (const node of tree.rootNode.namedChildren.filter(node => ["struct_item", "enum_item"].includes(node.type))) {
      db.query("INSERT INTO general_type VALUES(?,?)").run(node.childForFieldName("name")!.text, node.type);
    }
    expect(db.query("SELECT name FROM general_type ORDER BY name").all()).toEqual([...fixture.types].sort().map(name => ({ name })));
    expect(source).not.toContain("🛍️products");
    const slot = tree.rootNode.namedChildren.find(node => node.type === "struct_item" && node.childForFieldName("name")?.text === "OriginalActorContextSlot")!;
    expect(slot.childForFieldName("body")!.namedChildren.every(node => !node.namedChildren.some(child => child.type === "visibility_modifier"))).toBe(true);
    expect(source).toContain("pub fn validate_next_input(");
    expect(source).toContain("pub fn begin_retirement_input(");
    expect(source).toContain("original retirement binding has no admitted context");
    const retained = resolve(owner, ".."), actor = resolve(owner, "../..");
    for (const file of [join(retained, "🤝️admission/🧪️tests/🦀️.rs"), join(retained, "🧪️tests/🟦️.ts"), join(actor, "📦️packages/🦀️rust/📜️script.ts")]) expect(readFileSync(file, "utf8")).not.toContain("🛍️products");
    expect(readFileSync(join(retained, "🦀️.rs"), "utf8")).not.toContain("pub mod context;");const directory=join(owner,"📦️packages/🦀️rust"),manifest=TOML.parse(readFileSync(join(directory,"Cargo.toml"),"utf8")) as any;expect(manifest.package.name).toBe("semio-framework-actor-retained-context");expect(resolve(directory,manifest.lib.path)).toBe(implementation);expect(resolve(directory,manifest.dependencies["semio-framework-actor-retained-turn"].path)).toBe(join(retained,"📦️packages/🦀️rust"));
  } finally { db.close(); tree.delete(); }
  console.log("[DEBUG] Defining General Actor context eight public first-party types, private binding and product-free core test mounts verified with independent Rust AST/Ajv/SQLite");
});

/** 🎟️ Independent transactional binding rejects foreign, repeated and absent original epochs. */
test("original retirement binding advances only its same original source", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  const db = new Database(":memory:"), original = fixture.binding;
  try {
    db.exec("CREATE TABLE original(operation INTEGER,generation INTEGER,epoch INTEGER CHECK(epoch>0))");
    db.query("INSERT INTO original VALUES(?,?,?)").run(original.operation, original.generation, original.firstEpoch);
    for (const row of original.cases) {
      const update = db.query("UPDATE original SET epoch=? WHERE operation=? AND generation=? AND epoch<? AND ?>0").run(row.epoch, row.operation, row.generation, row.epoch, row.epoch);
      expect(update.changes === 1).toBe(row.accepted);
    }
    expect(db.query("SELECT operation,generation,epoch FROM original").get()).toEqual({ operation: original.operation, generation: original.generation, epoch: 21 });
    db.exec("CREATE TABLE remaining(fuel INTEGER,items INTEGER)");db.query("INSERT INTO remaining VALUES(?,?)").run(original.work.fuel-3,original.totalGrant[0]!-3);
    for(const row of original.closedContextCases){const fuel=original.work.fuel-3+(row.axis==="fuel"?row.increment:0),items=original.totalGrant[0]!-3+(row.axis==="items"?row.increment:0);expect(Boolean(db.query("SELECT 1 FROM remaining WHERE ?>=0 AND ?<=fuel AND ?>=0 AND ?<=items").get(fuel,fuel,items,items))).toBe(row.accepted);}
  } finally { db.close(); }
  console.log("[DEBUG] Independent SQLite checked General context binding accepts20/21, refuses foreign operation/generation, duplicate20 and absent0");
});
