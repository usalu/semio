import refusalCorpus from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalCorpus.cases.find(c=>c.id==="canceled-projection")!.expectedKind;
/** 🧫️ Shared XML semantic corpus with independent typed SQL queries and edits. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import ownership from "../../🧫️fixtures/🫳️ownership/🔣️.json";
import logical from "../../🧫️fixtures/🫳️ownership/📸️logical/🔣️.json";
import ownershipSchema from "../../🧫️fixtures/🫳️ownership/🧬️schema/🔣️.json";
import Ajv from "ajv/dist/2020";
import AjvDraft07 from "ajv";
import snapshotSchema from "../../🔣️.json";
import type { XmlSnapshot, XmlNode } from "../../../../../../../../🟦️.ts";
import { xmlSnapshotToSqliteDatabase, xmlSnapshotFromSqliteDatabase, XML_SQLITE_SCHEMA } from "../../../../../../../../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import graph from "../../🧫️fixtures/🕸️documents/🔣️.json";
import graphSchema from "../../🧫️fixtures/🕸️documents/🧬️schema/🔣️.json";
import {xmlDocumentsToSqliteDatabase,xmlDocumentsFromSqliteDatabase} from "../../../../../../../../🟦️.ts";
import component from "../../🧫️fixtures/🧩️native-component/🔣️.json";
import componentSchema from "../../🧫️fixtures/🧩️native-component/🧬️schema/🔣️.json";
import { Buffer } from "node:buffer";

test("XML component neutral native streams match independent UTF-8 octets and exact SQL ownership",async()=>{
 expect(new Ajv({strict:true}).validate(componentSchema,component)).toBe(true);
 expect(Buffer.byteLength(component.control.unit.repeat(component.control.repeats),"utf8")).toBe(component.control.utf8Bytes);
 for(const item of component.cases){
  const bytes=item.nodeText===null?Buffer.alloc(0):Buffer.from(item.nodeText,"utf8");
  const root=item.nodeText===null?undefined:{kind:"text" as const,text:item.nodeText};
  expect(item.text).toBe(item.nodeText===null?"[[0],[0],[0],[],[]]":`[[1,T[${bytes.toString("hex")}]],[0],[0],[],[]]`);
  const binary=item.nodeText===null?[0,0,0,0,0]:[1,1,bytes.length,...bytes,0,0,0,0];
  expect(item.binary).toEqual(binary);
  const database=await xmlDocumentsToSqliteDatabase([{schema:"",doc:{root,prolog:[],epilog:[]}}]);
  expect(database.tables.reduce((count,table)=>count+table.rows.length,0)).toBe(item.rows);
  const sql=Database.deserialize(await exportSqliteDatabase(database));try{
   expect(sql.query("SELECT text FROM xml_text ORDER BY node_id").all()).toEqual(item.nodeText===null?[]:[{text:item.nodeText}]);
   expect([...Buffer.concat([Buffer.from(component.prefix),Buffer.from(binary),Buffer.from(component.suffix)]).subarray(component.prefix.length,-component.suffix.length)]).toEqual(binary);
  }finally{sql.close();}
 }
});

const input: XmlSnapshot = { schema: fixture.schema, doc: {
  declaration: { version: "1.0", encoding: "UTF-8", standalone: true, quote: "single" },
  doctype: { prologPosition: 2n, name: "catalog", externalId: { kind: "public", publicId: "-//Semio//DTD Example 1.0//EN", systemId: "catalog.dtd" }, declarations: [{ kind: "entity", parameter: false, name: "greeting", value: "Grüße 🌠" }, { kind: "entity", parameter: true, name: "parameter", value: "literal" }] },
  root: { kind: "element", name: "catalog", attrs: [{ name: "z", value: "last" }, { name: "a", value: "first" }], children: [
    { kind: "element", name: "item", attrs: [{ name: "code", value: "A" }], children: [{ kind: "text", text: "text & more" }, { kind: "cData", text: "<literal>" }, { kind: "comment", text: "inside" }, { kind: "processingInstruction", target: "render", data: "ready" }] },
    { kind: "element", name: "empty", attrs: [], children: [] },
  ] },
  prolog: [{ kind: "comment", text: "before" }, { kind: "processingInstruction", target: "build", data: "begin" }],
  epilog: [{ kind: "processingInstruction", target: "build", data: "end" }, { kind: "comment", text: "after" }],
} };

test("XML shared typed graph preserves independent document identities and SQL ownership",async()=>{
 expect(new Ajv({strict:true}).validate(graphSchema,graph)).toBe(true);const snapshots=graph.keys.map(schema=>({schema,doc:structuredClone(input.doc)}));
 for(const snapshot of snapshots)expect(new AjvDraft07({strict:false}).validate(snapshotSchema,JSON.parse(JSON.stringify(snapshot,(_key,value)=>typeof value==="bigint"?value.toString():value)))).toBe(true);
 const database=await xmlDocumentsToSqliteDatabase(snapshots);expect(await xmlDocumentsFromSqliteDatabase(database)).toEqual(snapshots);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const docs=db.query("SELECT id,schema,root_node_id FROM xml_document ORDER BY id").all() as {id:number;schema:string;root_node_id:number}[];expect(docs.map(doc=>doc.schema)).toEqual(graph.keys);expect(new Set(docs.map(doc=>doc.root_node_id)).size).toBe(graph.keys.length);
  db.query("UPDATE xml_element SET name=? WHERE node_id=?").run(graph.editedName,docs[graph.editedOrdinal]!.root_node_id);const restored=await xmlDocumentsFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));const expected=structuredClone(snapshots);const root=expected[graph.editedOrdinal]!.doc.root;if(root?.kind!=="element")throw new Error("fixture root");root.name=graph.editedName;expect(restored).toEqual(expected);
  db.query("UPDATE xml_document SET root_node_id=? WHERE id=?").run(docs[0]!.root_node_id,docs[1]!.id);await expect(xmlDocumentsFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();
 }finally{db.close();}
});

test("XML multiple empty-document ownership has genuine known-workload cancellation",async()=>{
 const snapshots=Array.from({length:graph.documents},(_,ordinal)=>({schema:String(ordinal),doc:{prolog:[],epilog:[]}}));
 for(const phase of ["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.total===graph.documents&&event.completed===graph.cancelAt){reached=true;controller.abort();}}};const database=await xmlDocumentsToSqliteDatabase(snapshots);const operation=phase==="projectSnapshot"?xmlDocumentsToSqliteDatabase(snapshots,options):xmlDocumentsFromSqliteDatabase(database,options);await expect(operation).rejects.toHaveProperty("kind",canceledKind);expect(reached).toBe(true);}
});

test("XML neutral owned long Unicode fields agree with independent SQLite", async () => {
  expect(new Ajv({ strict: true }).validate(ownershipSchema, ownership)).toBe(true);
  const text = ownership.unit.repeat(ownership.repeats);
  expect(new TextEncoder().encode(text).length).toBe(ownership.utf8Bytes);
  const snapshot: XmlSnapshot = { schema: text, doc: { prolog: [], epilog: [], root: { kind: "element", name: "root", attrs: [{ name: "key", value: text }], children: [{ kind: "cData", text }] } } };
  const database = await xmlSnapshotToSqliteDatabase(snapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("SELECT schema,length(CAST(schema AS BLOB)) AS bytes FROM xml_document").get()).toEqual({ schema: text, bytes: ownership.utf8Bytes });
    expect(db.query("SELECT value FROM xml_attribute").get()).toEqual({ value: text });
    expect(db.query("SELECT text FROM xml_cdata").get()).toEqual({ text });
    expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
  } finally { db.close(); }
});

test("XML schema-admitted declaration metadata survives logical SQLite independently of UTF-8 wire", async () => {
  const snapshot: XmlSnapshot = { schema: "owned UTF-16 declaration", doc: { declaration: { version: "1.0", encoding: "UTF-16", standalone: false, quote: "single" }, prolog: [], epilog: [], root: { kind: "element", name: "root", attrs: [], children: [] } } };
  expect(new AjvDraft07({ strict: false }).validate(snapshotSchema, snapshot)).toBe(true);
  const database = await xmlSnapshotToSqliteDatabase(snapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("SELECT version,encoding,standalone,quote FROM xml_declaration").get()).toEqual({ version: "1.0", encoding: "UTF-16", standalone: 0, quote: "single" });
    expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
  } finally { db.close(); }
});

test("XML schema-admitted intermediate boundaries preserve every owned field", async () => {
  expect(new AjvDraft07({ strict: false }).validate(snapshotSchema, logical)).toBe(true);
  const snapshot = parseXmlSnapshot(logical);expect(()=>validateXmlDocumentWireBoundary(snapshot.doc)).toThrow();
  const db = Database.deserialize(await exportSqliteDatabase(await xmlSnapshotToSqliteDatabase(snapshot)));
  try {
    expect(db.query("SELECT prolog_position_decimal FROM xml_doctype").get()).toEqual({ prolog_position_decimal: "9" });
    expect(db.query("SELECT m.position,n.kind FROM xml_document_misc m JOIN xml_node n ON n.id=m.node_id ORDER BY m.id").all()).toEqual([{ position: "prolog", kind: "text" }, { position: "prolog", kind: "element" }, { position: "epilog", kind: "cdata" }]);
    expect(db.query("SELECT name,value FROM xml_attribute ORDER BY ordinal").all()).toEqual([{ name: "duplicate", value: "first" }, { name: "duplicate", value: "second" }]);
    expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
  } finally { db.close(); }
});

test("XML handwritten components, attributes and declarations expose shared independent SQL", async () => {
  expect(XML_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await xmlSnapshotToSqliteDatabase(input);
  expect(await xmlSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).all()).toEqual([{ name: "z", value: "last" }, { name: "a", value: "first" }]);
    expect(db.query("SELECT DISTINCT kind FROM xml_node ORDER BY kind").all()).toEqual([...fixture.nodeKinds].sort().map(kind => ({ kind })));
    expect(db.query("SELECT name,parameter FROM xml_entity ORDER BY ordinal").all()).toEqual([{ name: fixture.entityNames[0], parameter: 0 }, { name: fixture.entityNames[1], parameter: 1 }]);
    expect(db.query("SELECT external_kind,prolog_position_decimal FROM xml_doctype").get()).toEqual({ external_kind: "public", prolog_position_decimal: "2" });
    db.query("UPDATE xml_attribute SET value=? WHERE element_node_id=1 AND ordinal=0").run(fixture.editedAttribute);
    db.run("UPDATE xml_declaration SET standalone=0");
    const edited = await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect((edited.doc.root as Extract<XmlNode, { kind: "element" }>).attrs[0]!.value).toBe(fixture.editedAttribute);
    expect(edited.doc.declaration!.standalone).toBe(false);
  } finally { db.close(); }
});

test("XML SQL-edited missing components, ownership cycles, ordinals and metadata reject", async () => {
  for (const edit of [
    "DELETE FROM xml_element WHERE node_id=1",
    "UPDATE xml_child SET child_node_id=1 WHERE id=1",
    "UPDATE xml_attribute SET ordinal=12 WHERE id=1",
    "UPDATE xml_doctype SET prolog_position_decimal=-1",
    "UPDATE xml_node SET kind='text' WHERE id=1",
    "UPDATE xml_entity SET doctype_id=999 WHERE id=1",
    "INSERT INTO xml_node VALUES(99,'element'); INSERT INTO xml_element VALUES(99,'island'); INSERT INTO xml_child VALUES(99,99,0,99)",
    "UPDATE xml_declaration SET quote='invalid'",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await xmlSnapshotToSqliteDatabase(input)));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(edit); await expect(xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("XML optional metadata and empty document preserve typed semantics", async () => {
  for (const externalId of [undefined, { kind: "system" as const, systemId: "Grüße.dtd" }, { kind: "public" as const, publicId: "public identifier", systemId: "system.dtd" }]) {
    for (const standalone of [undefined, false, true]) {
      const value: XmlSnapshot = { ...input, doc: { ...input.doc, doctype: { ...input.doc.doctype!, externalId }, declaration: { version: "1.0", standalone, quote: "double" } } };
      expect(await xmlSnapshotFromSqliteDatabase(await xmlSnapshotToSqliteDatabase(value))).toEqual(value);
    }
  }
  const empty: XmlSnapshot = { schema: "empty.xml", doc: { prolog: [], epilog: [] } };
  const db = Database.deserialize(await exportSqliteDatabase(await xmlSnapshotToSqliteDatabase(empty)));
  try { expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(empty); }
  finally { db.close(); }
});

test("XML projection and reconstruction obey aggregate bounds and cancellation", async () => {
  const database = await xmlSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(xmlSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(xmlSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  const controller = new AbortController();
  const many: XmlSnapshot = { schema: "large.xml", doc: { root: { kind: "element", name: "root", attrs: Array.from({ length: 1000 }, (_, index) => ({ name: "attribute" + index, value: "text" })), children: [] }, prolog: [], epilog: [] } };
  let events = 0;
  await expect(xmlSnapshotToSqliteDatabase(many, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ kind: canceledKind });
  const reconstruct = new AbortController();
  await expect(xmlSnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ kind: canceledKind });
  let root: XmlNode = { kind: "text", text: "leaf" };
  for (let depth = 0; depth < 2000; depth++) root = { kind: "element", name: "n", attrs: [], children: [root] };
  const deep = await xmlSnapshotFromSqliteDatabase(await xmlSnapshotToSqliteDatabase({ schema: "deep.xml", doc: { root, prolog: [], epilog: [] } }));
  let node = deep.doc.root!;
  let depth = 0;
  while (node.kind === "element") { depth++; node = node.children[0]!; }
  expect(depth).toBe(2000);
});

import positions from "../../🧫️fixtures/🧭️position/🔣️.json";
import positionSchema from "../../🧫️fixtures/🧭️position/🧬️schema/🔣️.json";
test("XML full unsigned64 position is an interpreted exact decimal scalar",async()=>{
 expect(new Ajv({strict:true}).validate(positionSchema,positions)).toBe(true);for(const invalid of positions.invalid)expect(()=>parseXmlPosition(invalid)).toThrow();
 for(const position of positions.positions){const snapshot={schema:"literal",doc:{doctype:{prologPosition:BigInt(position),name:"root",declarations:[]},prolog:[],epilog:[]}} as unknown as XmlSnapshot;
 const database=await xmlSnapshotToSqliteDatabase(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("SELECT prolog_position_decimal AS position FROM xml_doctype").get()).toEqual({position});expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);}finally{db.close();}
 }
});

import{parseXmlSnapshot,parseXmlPosition,validateXmlDocumentWireBoundary}from"../../🟦️.ts";

import nodeNative from "../../🧫️fixtures/🌱️native-node/🔣️.json";
import nodeNativeSchema from "../../🧫️fixtures/🌱️native-node/🧬️schema/🔣️.json";
test("XML actual node native component has no synthetic document fields",async()=>{
 expect(new Ajv({strict:true}).validate(nodeNativeSchema,nodeNative)).toBe(true);
 for(const item of nodeNative.cases){const octets=Buffer.from(item.text,"utf8");expect(item.wireText).toBe(`T[${octets.toString("hex")}]`);expect(item.wireBinary).toEqual([1,octets.length,...octets]);const database=await xmlDocumentsToSqliteDatabase([{schema:"",doc:{root:{kind:"text",text:item.text},prolog:[],epilog:[]}}]);const sql=Database.deserialize(await exportSqliteDatabase(database));try{expect(sql.query("SELECT text FROM xml_text").get()).toEqual({text:item.text});expect(database.tables.reduce((count,table)=>count+table.rows.length,0)-database.tables.find(table=>table.name==="xml_document")!.rows.length).toBe(item.rows);}finally{sql.close();}}
});
