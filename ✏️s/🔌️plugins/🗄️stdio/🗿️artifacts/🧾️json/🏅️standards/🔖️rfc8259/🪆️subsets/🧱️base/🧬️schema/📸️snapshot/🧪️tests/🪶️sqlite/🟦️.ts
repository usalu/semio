/** 🧫️ Shared JSON semantic corpus with independent SQL editing and ownership validation. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type { JsonSnapshot, JsonValue } from "../../🟦️.ts";
import { parseJsonSnapshot } from "../../🟦️.ts";
import { jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase, JSON_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: JsonSnapshot = { schema: fixture.schema, value: { kind: "object", members: [
  { key: fixture.memberKeys[0]!, value: { kind: "number", lexeme: fixture.numberLexemes[0]! } },
  { key: fixture.memberKeys[1]!, value: { kind: "array", items: [{ kind: "bool", value: true }, { kind: "bool", value: false }, { kind: "null" }, { kind: "string", value: "Grüße 🌠\u0000" }, { kind: "object", members: [] }, { kind: "array", items: [] }] } },
  { key: fixture.memberKeys[2]!, value: { kind: "number", lexeme: fixture.numberLexemes[1]! } },
] } };

test("JSON native lexemes and ordered duplicate members expose independent relational SQL", async () => {
  expect(parseJsonSnapshot(input)).toEqual(input);
  expect(JSON_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await jsonSnapshotToSqliteDatabase(input);
  expect(await jsonSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).all()).toEqual([{ key: "z", kind: "number", number_lexeme: fixture.numberLexemes[0] }, { key: "a", kind: "array", number_lexeme: null }, { key: "z", kind: "number", number_lexeme: fixture.numberLexemes[1] }]);
    expect(db.query("SELECT count(*) AS count FROM json_value").get()).toEqual({ count: fixture.valueCount });
    expect(db.query("SELECT count(*) AS count FROM json_object_member").get()).toEqual({ count: fixture.memberCount });
    expect(db.query("SELECT count(*) AS count FROM json_array_element").get()).toEqual({ count: fixture.elementCount });
    db.query("UPDATE json_value SET string_value=? WHERE kind='string'").run(fixture.editedString);
    const edited = await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect((edited.value as Extract<JsonValue, { kind: "object" }>).members[1]!.value).toEqual({ kind: "array", items: [{ kind: "bool", value: true }, { kind: "bool", value: false }, { kind: "null" }, { kind: "string", value: fixture.editedString }, { kind: "object", members: [] }, { kind: "array", items: [] }] });
  } finally { db.close(); }
});

test("JSON typed intermediate number strings and duplicate members retain independent numeric query cells",async()=>{
 const cases=fixture.typedNumberCases.map(c=>({lexeme:c.lexeme.repeat("repeat"in c?c.repeat:1),numeric:c.numeric}));
 const snapshot:JsonSnapshot={schema:fixture.logicalNative.schema,value:{kind:"object",members:cases.map(c=>({key:fixture.logicalNative.duplicateKey,value:{kind:"number",lexeme:c.lexeme}}))}};
 expect(parseJsonSnapshot(snapshot)).toEqual(snapshot);
 const database=await jsonSnapshotToSqliteDatabase(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const rows=db.query("SELECT m.key,v.number_lexeme,v.number_value FROM json_object_member m JOIN json_value v ON v.id=m.value_id ORDER BY m.ordinal").all()as{key:string,number_lexeme:string,number_value:number|null}[];
  for(let i=0;i<cases.length;i++){
   const source=cases[i]!,row=rows[i]!;expect(row.key).toBe(fixture.logicalNative.duplicateKey);expect(row.number_lexeme).toBe(source.lexeme);expect(row.number_value).toBe(source.numeric);
   let oracle:number|null=null;try{const value:unknown=JSON.parse(source.lexeme);if(typeof value==="number"&&Number.isFinite(value))oracle=value===0?0:value;}catch{}
   expect(row.number_value).toBe(oracle);
  }
  expect(await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
  db.query("UPDATE json_value SET number_lexeme=?,number_value=NULL WHERE id=2").run("arbitrary intermediate lexeme");
  const edited=await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
  expect((edited.value as Extract<JsonValue,{kind:"object"}>).members[0]!.value).toEqual({kind:"number",lexeme:"arbitrary intermediate lexeme"});
 }finally{db.close();}
});

test("JSON independently edited dangling, multiple-owner, disconnected-cycle and ordinal relations reject", async () => {
  for (const edit of [
    "UPDATE json_document SET root_value_id=999",
    "UPDATE json_array_element SET value_id=1 WHERE ordinal=0",
    "UPDATE json_array_element SET ordinal=99 WHERE ordinal=0",
    "UPDATE json_object_member SET object_value_id=3 WHERE ordinal=0",
    "INSERT INTO json_value VALUES (99,'array',NULL,NULL,NULL,NULL); INSERT INTO json_array_element VALUES (99,99,0,99)",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("JSON intermediate lexeme edits remain lossless and contradictory numeric or primitive columns reject", async () => {
 for(const lexeme of ["01","+1","1.","1e"," 1","1\\n","NaN"]){
  const source:JsonSnapshot={schema:input.schema,value:{kind:"number",lexeme}};
  expect(await jsonSnapshotFromSqliteDatabase(await jsonSnapshotToSqliteDatabase(source))).toEqual(source);
  const db=Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));
  try{db.query("UPDATE json_value SET number_lexeme=?,number_value=NULL WHERE kind='number'").run(lexeme);const restored=await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect((restored.value as Extract<JsonValue,{kind:"object"}>).members[0]!.value).toEqual({kind:"number",lexeme});}finally{db.close();}
 }
 for(const edit of["UPDATE json_value SET number_lexeme='1',number_value=2 WHERE kind='number'","UPDATE json_value SET number_lexeme='01',number_value=1 WHERE kind='number'","UPDATE json_value SET number_lexeme='0.1',number_value=NULL WHERE kind='number'","UPDATE json_value SET boolean_value=2 WHERE kind='boolean'"]){
  const db=Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(input)));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}
 }
});

test("JSON iterative trees honor bounds and cancellation without native serialization", async () => {
  const database = await jsonSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(jsonSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(jsonSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  let value: JsonValue = { kind: "null" };
  for (let depth = 0; depth < 3000; depth++) value = { kind: "array", items: [value] };
  const nested = await jsonSnapshotFromSqliteDatabase(await jsonSnapshotToSqliteDatabase({ schema: "deep", value }));
  let rebuilt = nested.value;
  let count = 0;
  while (rebuilt.kind === "array") { count++; rebuilt = rebuilt.items[0]!; }
  expect(count).toBe(3000);
  const controller = new AbortController();
  let events = 0;
  await expect(jsonSnapshotToSqliteDatabase({ schema: "deep", value }, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const reconstruction = new AbortController();
  await expect(jsonSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: () => reconstruction.abort() })).rejects.toMatchObject({ name: "AbortError" });
});


test("JSON neutral logical-native fixture independently retains owned schema and deep lexemes", async () => {
  let value: JsonValue = { kind: "number", lexeme: fixture.logicalNative.numberLexemes[0]! };
  for (let depth = 0; depth < fixture.logicalNative.depth; depth++) value = { kind: "array", items: [value] };
  const source = { schema: fixture.logicalNative.schema, value };
  const db = Database.deserialize(await exportSqliteDatabase(await jsonSnapshotToSqliteDatabase(source)));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT schema FROM json_document").get()).toEqual({ schema: source.schema });
    expect(db.query("WITH RECURSIVE depth(id,level) AS (SELECT root_value_id,0 FROM json_document UNION ALL SELECT e.value_id,d.level+1 FROM json_array_element e JOIN depth d ON e.array_value_id=d.id) SELECT max(level) AS depth FROM depth").get()).toEqual({ depth: fixture.logicalNative.depth });
    db.query("UPDATE json_value SET number_lexeme=?,number_value=CAST(? AS REAL) WHERE kind='number'").run(fixture.logicalNative.numberLexemes[1]!,fixture.logicalNative.numberLexemes[1]!);
    const restored = await jsonSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(restored.schema).toBe(source.schema);
    let leaf = restored.value, depth = 0;
    while (leaf.kind === "array") { expect(leaf.items.length).toBe(1); leaf = leaf.items[0]!; depth++; }
    expect(depth).toBe(fixture.logicalNative.depth);
    expect(leaf).toEqual({ kind: "number", lexeme: fixture.logicalNative.numberLexemes[1] });
  } finally { db.close(); }
});
