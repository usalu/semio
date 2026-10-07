import Ajv from "ajv";
import {parseSvgNode} from "../../../../🧬️schema/📸️snapshot/🧩️document/🟦️.ts";
import {bindSvgAttribute,printSvgAttribute,nativeSvgDocument} from "../../../📝️text/📸️snapshot/🧮️attributes/🟦️.ts";
import { SaxesParser } from "saxes";
import boundaryContract from "../🧫️fixtures/🧭️boundaries/🔣️.json";

import { validateXmlDocumentWireBoundary } from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import ownership from "../🧫️fixtures/💰️backing/🔣️.json";

/** 🧫️ SVG's shared typed XML corpus retains independent SVG table names. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { parseSvgSnapshot, type SvgSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { SVG_SQLITE_SCHEMA, svgSnapshotToSqliteDatabase, svgSnapshotFromSqliteDatabase } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
const input: SvgSnapshot = { schema: fixture.schema, doc: {
  declaration: { version: "1.0", encoding: "UTF-8", quote: "single" },
  prolog: [{ kind: "comment", text: "before" }], epilog: [{ kind: "comment", text: "after" }],
  doctype: { prologPosition: 1n, name: "svg", externalId: { kind: "system", systemId: "svg.dtd" }, declarations: [{ kind: "entity", parameter: false, name: "title", value: "Grüße 🌠" }] },
  root: { kind: "element", name: "svg", attrs: [{ name: "xmlns", value: bindSvgAttribute("xmlns","http://www.w3.org/2000/svg") }, { name: "viewBox", value: bindSvgAttribute("viewBox","0 0 100 100") }], children: [
    { kind: "element", name: "g", attrs: [{ name: "transform", value: bindSvgAttribute("transform","translate(10 20) rotate(30)") }], children: [
      { kind: "element", name: "path", attrs: [{ name: "id", value: bindSvgAttribute("id","shape") }, { name: "d", value: bindSvgAttribute("d","M0 0L10 20Z") }], children: [] },
      { kind: "element", name: "text", attrs: [], children: [{ kind: "text", text: "Grüße 🌠" }] },
      { kind: "element", name: "style", attrs: [], children: [{ kind: "cData", text: "path { fill: red; }" }] },
      { kind: "comment", text: "kept" }, { kind: "processingInstruction", target: "render", data: "ready" },
    ] },
  ] },
} };

test("SVG handcrafted thirteen tables preserve typed metadata, nodes and independently edited path attributes", async () => {
  expect(SVG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  expect(parseSvgSnapshot(input)).toEqual(input);
  expect(()=>parseSvgSnapshot({ ...input, doc: { ...input.doc, doctype: { ...input.doc.doctype!, prologPosition: input.doc.doctype!.prologPosition!.toString() } } })).toThrow();
  const database = await svgSnapshotToSqliteDatabase(input);
  expect(await svgSnapshotFromSqliteDatabase(database)).toEqual(input);
  expect(database.tables.every(table => table.name.startsWith("svg_"))).toBe(true);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).get()).toEqual({ value: "M 0 0 L 10 20 Z" });
    expect(db.query("SELECT encoding,quote FROM svg_declaration").get()).toEqual({ encoding: "UTF-8", quote: "single" });
    expect(db.query("SELECT name,value FROM svg_entity").get()).toEqual({ name: "title", value: "Grüße 🌠" });
    db.query("UPDATE svg_attribute SET value=? WHERE name='d'").run(fixture.editedPath);
    const edited = await svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    const root = edited.doc.root!;
    if (root.kind !== "element" || root.children[0]?.kind !== "element" || root.children[0].children[0]?.kind !== "element") throw new Error("SVG path fixture tree");
    expect(root.children[0].children[0].attrs[1]!.value).toEqual(bindSvgAttribute("d",fixture.editedPath));
    expect(edited.doc.declaration).toEqual(input.doc.declaration);
    expect(edited.doc.doctype).toEqual(input.doc.doctype);
  } finally { db.close(); }
});

test("SVG independent root, component, ownership, boundary and metadata edits reject", async () => {
  for (const edit of [
    "UPDATE svg_element SET name='html' WHERE node_id=1",
    "DELETE FROM svg_element WHERE node_id=1",
    "UPDATE svg_child SET child_node_id=1 WHERE id=1",
    "UPDATE svg_attribute SET ordinal=99 WHERE id=1",
    "UPDATE svg_doctype SET prolog_position_decimal='18446744073709551616'",
    "UPDATE svg_entity SET doctype_id=999",
    "UPDATE svg_document_misc SET ordinal=99 WHERE position='prolog'",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await svgSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("SVG namespaced roots, missing roots and bounds obey typed SVG semantics", async () => {
  const value: SvgSnapshot = { schema: "namespaced.svg", doc: { prolog: [], epilog: [], root: { kind: "element", name: "s:svg", attrs: [], children: [] } } };
  expect(await svgSnapshotFromSqliteDatabase(await svgSnapshotToSqliteDatabase(value))).toEqual(value);
  for (const root of [undefined,{ kind: "element" as const, name: "html", attrs: [], children: [] },{ kind: "text" as const, text: "svg" }]) await expect(svgSnapshotToSqliteDatabase({ ...value, doc: { ...value.doc, root } })).rejects.toThrow("svg root");
  const database = await svgSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 },{ maxValueBytes: 0 }]) {
    await expect(svgSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(svgSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("SVG projection and reconstruction carry cancellation through typed XML helpers", async () => {
  for (const direction of ["project","reconstruct"]) {
    const database = await svgSnapshotToSqliteDatabase(input);
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? svgSnapshotToSqliteDatabase(input, options) : svgSnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ kind: "canceled" });
  }
});

test("SVG SQLite retains declaration encoding metadata independently of an external UTF-8 document export", async () => {
  const database = await svgSnapshotToSqliteDatabase(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const control = await Bun.file(new URL("../🧫️fixtures/🛂️native.json", import.meta.url)).json();
    db.query("UPDATE svg_declaration SET version=?,encoding=?").run(control.metadata.version, control.metadata.encoding);
    db.query("UPDATE svg_doctype SET prolog_position_decimal=?").run(control.metadata.prologPosition);
    const restored = await svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(restored.doc.declaration).toEqual({ ...input.doc.declaration!, version: control.metadata.version, encoding: control.metadata.encoding });
    expect(restored.doc.doctype!.prologPosition).toBe(18446744073709551615n);
    expect(await svgSnapshotFromSqliteDatabase(await svgSnapshotToSqliteDatabase(restored))).toEqual(restored);
  } finally { db.close(); }
});


test("SVG neutral native controls retain the full Unicode field and independently counted entity rows", async () => {
  const control = await Bun.file(new URL("../🧫️fixtures/🛂️native.json", import.meta.url)).json();
  const text = control.text.repeat(control.repeat);
  const value: SvgSnapshot = { schema: control.literalSchema, doc: { prolog: [], epilog: [], root: { kind: "element", name: "svg", attrs: [], children: [{ kind: "text", text }] } } };
  expect(parseSvgSnapshot(value)).toEqual(value);
  const database = await svgSnapshotToSqliteDatabase(value);
  expect(await svgSnapshotFromSqliteDatabase(database)).toEqual(value);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT text,length(CAST(text AS BLOB)) AS bytes FROM svg_text").get()).toEqual({ text, bytes: Buffer.byteLength(text) });
    expect(db.query("SELECT schema FROM svg_document").get()).toEqual({ schema: control.literalSchema });
  } finally { db.close(); }
  const wide: SvgSnapshot = { schema: control.literalSchema, doc: { prolog: [], epilog: [], root: { kind: "element", name: "svg", attrs: [], children: Array.from({ length: control.wideChildren }, () => ({ kind: "element" as const, name: "g", attrs: [], children: [] })) } } };
  const wideDatabase = await svgSnapshotToSqliteDatabase(wide);
  expect(wideDatabase.tables.reduce((rows, table) => rows + table.rows.length, 0)).toBe(3 + 3 * control.wideChildren);
  const wideDb = Database.deserialize(await exportSqliteDatabase(wideDatabase));
  try { expect(wideDb.query("SELECT COUNT(*) AS count FROM svg_child").get()).toEqual({ count: control.wideChildren }); }
  finally { wideDb.close(); }
});

test("SVG actual paid owner keeps literal native fields distinct from external markup",async()=>{
 
 const contract=ownership as unknown as {literal:string,literalUtf8Hex:string,tableCount:number,snapshotFields:string[]};
 expect(Buffer.from(contract.literal,"utf8").toString("hex")).toBe(contract.literalUtf8Hex);
 const value:SvgSnapshot={schema:"SVG owned 世界",doc:{prolog:[],epilog:[],root:{kind:"element",name:"svg",attrs:[],children:[{kind:"text",text:contract.literal}]}}};
 const db=Database.deserialize(await exportSqliteDatabase(await svgSnapshotToSqliteDatabase(value)));
 try{
  expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:contract.tableCount});
  expect(db.query("SELECT hex(CAST(text AS BLOB)) AS octets FROM svg_text").get()).toEqual({octets:contract.literalUtf8Hex.toUpperCase()});
  expect(db.query("SELECT (SELECT count(*) FROM svg_document)+(SELECT count(*) FROM svg_node)+(SELECT count(*) FROM svg_element)+(SELECT count(*) FROM svg_text)+(SELECT count(*) FROM svg_child) AS rows").get()).toEqual({rows:6});
  expect(contract.snapshotFields).toEqual(["schema","root","doctype","declaration","prolog","epilog"]);
 }finally{db.close();}
});

test("SVG owned metadata and boundary nodes remain literal while external XML validates publication",async()=>{
 
 const authored=await Bun.file(new URL("../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧫️fixtures/🧭️document-boundaries/🔣️.json",import.meta.url)).json();
 expect(authored.invalidAuthored.map((item:{id:string})=>item.id)).toEqual(boundaryContract.cases);
 const mutationSchema=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔣️.json",import.meta.url)).json();
 expect(mutationSchema.oneOf.map((item:{properties:{mutation:{const:string}}})=>item.properties.mutation.const.replace(/[A-Z]/g,letter=>"-"+letter.toLowerCase()))).toEqual(boundaryContract.mutationKinds);
 for(const [index,case_]of authored.invalidAuthored.entries()){
  const value:SvgSnapshot={schema:"stdio.svg",doc:{root:{kind:"element",name:"svg",attrs:[],children:[]},prolog:[],epilog:[]}};
  if(index===0){value.doc.prolog=[{kind:"comment",text:"before"}];value.doc.doctype={prologPosition:BigInt(case_.doctypePosition),name:"svg",declarations:[]};}
  else if(index===1)value.doc.epilog=[{kind:"text",text:"outside"}];
  else{value.doc.declaration={version:"1.0",encoding:case_.encoding,quote:case_.quote};value.doc.root={kind:"element",name:"svg",attrs:[],children:[{kind:"text",text:case_.rootText}]};}
  expect(()=>validateXmlDocumentWireBoundary(nativeSvgDocument(value.doc))).toThrow();
  const db=Database.deserialize(await exportSqliteDatabase(await svgSnapshotToSqliteDatabase(value)));
  try{
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   if(index===0)expect(db.query("SELECT prolog_position_decimal AS position FROM svg_doctype").get()).toEqual({position:String(case_.doctypePosition)});
   if(index===1)expect(db.query("SELECT n.kind FROM svg_document_misc m JOIN svg_node n ON n.id=m.node_id WHERE m.position='epilog'").get()).toEqual({kind:"text"});
   if(index>=2)expect(db.query("SELECT hex(CAST(encoding AS BLOB)) AS octets FROM svg_declaration").get()).toEqual({octets:Buffer.from(case_.encoding).toString("hex").toUpperCase()});
   expect(await svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(value);
  }finally{db.close();}
 }
 const wellFormed=(text:string)=>{let accepted=true;const parser=new SaxesParser();parser.on("error",()=>{accepted=false;});parser.write(text).close();return accepted;};
 expect(wellFormed('<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg/>')).toBe(true);
 expect(wellFormed('<?xml version="1.0" encoding="UTF-8" standalone="no?><svg/>')).toBe(false);
 expect(wellFormed('<svg/>outside')).toBe(false);
 expect(wellFormed("<?xml version='1.0' encoding='ISO-8859-1'?><svg/>")).toBe(true);
});


test("SVG typed attribute corpus agrees with independent Saxes native attribute admission",async()=>{
 const corpus=await Bun.file(new URL("../../../../🧫️fixtures/🧩️typed-attributes/🔣️.json",import.meta.url)).json();
 for(const row of corpus.cases){
  let observed:string|undefined;const parser=new SaxesParser();parser.on("opentag",tag=>{observed=String(tag.attributes[row.name]);});parser.write(`<svg ${row.name}="${row.native}"/>`).close();
  expect(observed).toBe(row.native);const owned=bindSvgAttribute(row.name,observed!);expect(owned).toEqual(row.owned);expect(bindSvgAttribute(row.name,printSvgAttribute(owned))).toEqual(row.owned);
 }
 for(const row of corpus.invalid)expect(()=>bindSvgAttribute(row.name,row.native)).toThrow();
 for(const row of corpus.invalidOwned)expect(()=>parseSvgNode({kind:"element",name:"svg",attrs:[row],children:[]})).toThrow();
});
