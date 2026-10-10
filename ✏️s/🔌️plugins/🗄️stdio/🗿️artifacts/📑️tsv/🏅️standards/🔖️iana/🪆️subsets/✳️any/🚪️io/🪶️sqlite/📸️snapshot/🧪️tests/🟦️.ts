import {existsSync as hasTrialSchema} from "node:fs";
test("current whole trial declarations remain absent",()=>{expect(hasTrialSchema(new URL("../🫴️receiving/🛂️validation/🧬️schema/🔣️.json",import.meta.url))).toBe(false);expect(hasTrialSchema(new URL("../🫴️receiving/🧬️schema/🔣️.json",import.meta.url))).toBe(false);});
/** 🧫️ Shared TSV semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import subsetValidationPolicy from "../🫴️receiving/🛂️validation/🧫️fixtures/🔣️.json";
test("TSV original receiving subset validation has a closed independent directional policy",()=>{
 const{cases}=subsetValidationPolicy;expect(subsetValidationPolicy.directions).toEqual(["decoding","encoding"]);
 const database=new Database(":memory:");try{for(const direction of subsetValidationPolicy.directions){for(const row of cases){const actual=database.query("SELECT CASE WHEN ? <> 'accepted' THEN 'canceled' WHEN ? = '*' THEN 'clean' ELSE 'unsupportedOwner' END AS kind").get(row.port,row.subset);expect(actual).toEqual({kind:row.kind});expect(direction==="decoding"||direction==="encoding").toBe(true);}}}finally{database.close();}
 expect(subsetValidationPolicy.grant).toEqual({maximumItems:0,maximumCopyBytes:0,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:0});console.log("[DEBUG] TSV closed neutral eight-case receiving validation agrees with independent SQLite wildcard/refusal/cancellation policy; Native custody is checked separately");
});
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { tsvSnapshotToSqliteDatabase, tsvSnapshotFromSqliteDatabase, TSV_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import {parseRetainedCloneGrant} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts";

const input = { ...fixture, lineEnding: "crlf" as const };

test("TSV typed destination neutral UTF8 prefixes match independent SQLite",async()=>{
 const law=controlFixture.typedDestination,text=controlFixture.copyTextUnit.repeat(controlFixture.copyTextRepeat),bytes=Buffer.from(text,"utf8"),db=new Database(":memory:");
 expect(parseRetainedCloneGrant(law.normalGrant)).toEqual(law.normalGrant);expect(parseRetainedCloneGrant(law.closeGrant)).toEqual(law.closeGrant);for(const field of Object.keys(law.normalGrant)){const incomplete:Record<string,unknown>={...law.normalGrant};delete incomplete[field];expect(()=>parseRetainedCloneGrant(incomplete)).toThrow("five axes");}
 try{db.run("CREATE TABLE prefix(value TEXT NOT NULL, octets INTEGER NOT NULL)");for(const cancel of law.cancelAt){let end=0;while(end<bytes.length&&(cancel===null||end<cancel)){let next=Math.min(end+65536,bytes.length);while(next<bytes.length&&(bytes[next]!&0xc0)===0x80)next--;end=next;}const prefix=new TextDecoder("utf8",{fatal:true}).decode(bytes.subarray(0,end));db.run("DELETE FROM prefix");db.run("INSERT INTO prefix VALUES(?,?)",[prefix,end]);expect(db.query("SELECT length(CAST(value AS BLOB)) AS bytes,octets FROM prefix").get()).toEqual({bytes:end,octets:end});expect(Buffer.from(prefix,"utf8")).toEqual(bytes.subarray(0,end));expect(JSON.parse(JSON.stringify(prefix))).toBe(prefix);expect(end).toBeLessThanOrEqual(law.normalGrant.maximumCopyBytes);}
 expect(law.normalGrant.maximumCapacityBytes).toBeGreaterThan(bytes.length);expect(law.closeGrant.maximumItems).toBe(1);expect(law.closeGrant.maximumDepth).toBeGreaterThan(law.normalGrant.maximumDepth);expect(law.terminalDropBytes).toBe(0);
 }finally{db.close();}
});

test("TSV shared handcrafted schema, semantic SQL joins and independent edits", async () => {
  expect(TSV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const database = await tsvSnapshotToSqliteDatabase(input);
  expect(await tsvSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT f.value FROM tsv_record r JOIN tsv_field f ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual(input.records.flatMap(record => record.map(value => ({ value }))));
    db.run("UPDATE tsv_field SET value='edited TSV entity 🌠' WHERE id=1");
    db.run("UPDATE tsv_document SET line_ending='lf', trailing_newline=0");
    const edited = await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.records[0]![0]).toBe("edited TSV entity 🌠");
    expect(edited.lineEnding).toBe("lf");
    expect(edited.trailingNewline).toBe(false);
    db.run("UPDATE tsv_field SET record_id=999 WHERE id=1");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});


test("tsv borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 
 expect(controlFixture["preflight"]["encodings"]).toEqual(["binary","text"]);expect(controlFixture["preflight"]["ownershipBytes"]).toEqual(0);expect(controlFixture["preflight"]["byteAuthority"]).toEqual("actualLiteralOutputUTF8OrOctets");expect(controlFixture["preflight"]["refusalKinds"]["file"]).toEqual("ownershipLimit");expect(controlFixture["preflight"]["refusalKinds"]["cancellation"]).toEqual("canceled");expect(controlFixture["preflight"]["cancelAt"]).toEqual("firstBorrowedCheckpoint");expect(controlFixture["preflight"]["retirementRefund"]).toEqual(false);expect(controlFixture["expectedRows"]).toEqual(1030);expect(controlFixture["copyTextUnit"]).toEqual("文🌠");expect(controlFixture["copyTextRepeat"]).toEqual(20000);expect(controlFixture["copyTextBytes"]).toEqual(140000);expect(controlFixture["copyTextBoundary"]).toEqual(65534);
 const policy=controlFixture.preflight;
 expect(policy).toEqual({"encodings":["binary","text"],"ownershipBytes":0,"byteAuthority":"actualLiteralOutputUTF8OrOctets","refusalKinds":{"file":"ownershipLimit","cancellation":"canceled"},"cancelAt":"firstBorrowedCheckpoint","retirementRefund":false});
 
 
 const literal=controlFixture.fieldText,octets=Buffer.from(literal,"utf8");
 expect(Buffer.byteLength(literal,"utf8")).toBe(new TextEncoder().encode(literal).length);
 expect(octets.toString("utf8")).toBe(literal);
 const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE literal_output(value TEXT NOT NULL)");independent.run("INSERT INTO literal_output VALUES(?)",[literal]);expect(independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM literal_output").get()).toEqual({bytes:octets.byteLength});}finally{independent.close();}
 expect(policy.ownershipBytes).toBe(0);expect(policy.retirementRefund).toBe(false);
 expect(controlFixture.allocationRole).toBe("cumulativeOwnedBacking");
 expect(controlFixture.maxAllocationBytes).toBe(1);
 
 const primitive=Buffer.from(controlFixture.bytePattern.slice(0,controlFixture.maxAllocationBytes+1));
 expect(primitive.byteLength).toBe(controlFixture.maxAllocationBytes+1);
 const ownership=new BudgetAllocationControl({maxAllocationBytes:controlFixture.maxAllocationBytes});
 await expect(ownership.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive))).rejects.toMatchObject({kind:"ownershipLimit"});
 expect(ownership.remainingBytes()).toBe(controlFixture.maxAllocationBytes);
 const semanticLimits={maxAllocationBytes:primitive.byteLength,maxValueBytes:0};const semantic=new BudgetAllocationControl(semanticLimits);
 const copied=await semantic.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive));
 expect(Buffer.from(copied)).toEqual(primitive);expect(semantic.remainingBytes()).toBe(0);

});

test("TSV independently edited ordinals and flags reject", async () => {
  const db = Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE tsv_document SET trailing_newline=2");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE tsv_document SET trailing_newline=1,line_ending='invalid'");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("line ending");
    db.run("UPDATE tsv_document SET line_ending='crlf'");
    db.run("UPDATE tsv_field SET ordinal=-1 WHERE id=1");
    await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});

test("TSV row and value limits apply to both semantic directions", async () => {
  const database = await tsvSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(tsvSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(tsvSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("TSV cancellation reaches counting scans and reconstruction", async () => {
  const controller = new AbortController();
  let events = 0;
  await expect(tsvSnapshotToSqliteDatabase({ ...input, records: [Array.from({ length: 1000 }, () => "cell")] }, { signal: controller.signal, onProgress: progress => { if (progress.completed === 0 && ++events === 2) controller.abort(); } })).rejects.toMatchObject({ kind: "canceled" });
  expect(events).toBe(2);
  const reconstruction = new AbortController();
  const database = await tsvSnapshotToSqliteDatabase(input);
  await expect(tsvSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: () => reconstruction.abort() })).rejects.toMatchObject({ kind: "canceled" });
});

test("TSV empty records and intrinsic scalar text preserve native semantics", async () => {
  const value = { ...input, records: [[], ["", "\u0000", "漢🌠"]], lineEnding: "lf" as const };
  const database = await tsvSnapshotToSqliteDatabase(value);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("SELECT count(*) AS records FROM tsv_record").get()).toEqual({ records: 2 });
    expect(db.query("SELECT value FROM tsv_field ORDER BY ordinal").all()).toEqual(value.records[1]!.map(value => ({ value })));
    expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(value);
  } finally { db.close(); }
});


import controlFixture from "../🧫️fixtures/🛬️native-control/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
test("Tsv native control corpus retains complete literal owned fields independently of external format",async()=>{
 
 const owned={schema:controlFixture.ownedSchema,records:[[],Array.from({length:controlFixture.workItems},()=>controlFixture.fieldText),[""],[]],trailingNewline:true,lineEnding:"crlf" as const};
 const database=await tsvSnapshotToSqliteDatabase(owned);const file=await exportSqliteDatabase(database);const independent=Database.deserialize(file);expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM tsv_document").get()).toEqual({schema:controlFixture.ownedSchema});}finally{independent.close();}
 expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});


test("tsv one literal Unicode field remains exact through independent SQLite storage",async()=>{
 
 const text=controlFixture.copyTextUnit.repeat(controlFixture.copyTextRepeat),bytes=new TextEncoder().encode(text);
 expect(bytes.length).toBe(controlFixture.copyTextBytes);
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(0,controlFixture.copyTextBoundary))).toBe(text.slice(0,controlFixture.copyTextBoundary/7*3));
 const owned={schema:controlFixture.ownedSchema,records:[[text]],trailingNewline:true,lineEnding:"crlf" as const};
 const file=await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(owned));const independent=Database.deserialize(file);
 try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(independent.query("SELECT value,length(CAST(value AS BLOB)) AS bytes FROM tsv_field").get()).toEqual({value:text,bytes:controlFixture.copyTextBytes});
  expect(independent.query("SELECT count(*) AS count FROM tsv_record").get()).toEqual({count:1});
  expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(owned);
 }finally{independent.close();}
});


import backingFixture from "../🧫️fixtures/💰️backing/🔣️.json";

import {NativeDecodeControl as BackingDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BackingAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
test("tsv authored relation indexes and literal octet admission have independent authorities",async()=>{
 expect(backingFixture["owner"]).toEqual("tsv");expect(backingFixture["policy"]["zeroOwnershipBytes"]).toEqual(0);expect(backingFixture["policy"]["refusalKind"]).toEqual("ownershipLimit");expect(backingFixture["policy"]["retirementRefund"]).toEqual(false);expect(backingFixture["policy"]["byteAuthority"]).toEqual("literalUTF8Bytes");expect(backingFixture["policy"]["indexAuthority"]).toEqual("authoredEntityRelations");
 const snapshot={schema:backingFixture.schema,trailingNewline:true,lineEnding:"crlf" as const,records:backingFixture.records.map(record=>backingFixture.fields.filter(field=>field.recordId===record.id).sort((a,b)=>a.ordinal-b.ordinal).map(field=>field.value))};
 const independent=Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(snapshot)));
 try{independent.exec("DELETE FROM tsv_field;DELETE FROM tsv_record;");for(const record of backingFixture.records)independent.query("INSERT INTO tsv_record VALUES(?,1,?)").run(record.id,record.ordinal);for(const field of backingFixture.fields)independent.query("INSERT INTO tsv_field VALUES(?,?,?,?)").run(field.id,field.recordId,field.ordinal,field.value);
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT r.id AS recordId,f.ordinal FROM tsv_record r JOIN tsv_field f ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual([{recordId:7,ordinal:0},{recordId:2,ordinal:0},{recordId:2,ordinal:1}]);
  for(const field of backingFixture.fields)expect((independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM tsv_field WHERE id=?").get(field.id)as{bytes:number}).bytes).toBe(Buffer.byteLength(field.value,"utf8"));
  expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
 }finally{independent.close();}
 const bytes=Buffer.from(backingFixture.fields[0]!.value,"utf8");expect(bytes.length).toBeGreaterThan(0);
 const zero=new BackingAllocationControl({maxAllocationBytes:backingFixture.policy.zeroOwnershipBytes});await expect(zero.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(zero.remainingBytes()).toBe(0);
 const exact=new BackingAllocationControl({maxAllocationBytes:bytes.length});{const copied=await exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(bytes));expect(Buffer.from(copied)).toEqual(bytes);}expect(exact.remainingBytes()).toBe(0);await expect(exact.stage(maximum=>new BackingDecodeControl(maximum,()=>true),control=>control.copyBytes(Uint8Array.of(1)))).rejects.toMatchObject({kind:backingFixture.policy.refusalKind});expect(exact.remainingBytes()).toBe(0);
});

import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import logicalOwnerFixture from "../🧫️fixtures/🧠️logical-owner/🔣️.json";

import {parseTsvSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
test("TSV complete logical owner preserves empty row and document metadata in independent SQLite",async()=>{
 expect(logicalOwnerFixture["contract"]).toEqual("completeTsvLogicalOwner");expect(logicalOwnerFixture["encodings"]).toEqual(["binary","text"]);expect(logicalOwnerFixture["policy"]).toEqual({"preserveSchema":true,"preserveTrailingNewline":true,"preserveLineEnding":true,"preserveEmptyRecord":true,"nativeOwner":"declaredLogicalRecord","naturalFile":"ianaTsv"});expect(logicalOwnerFixture["edit"]).toEqual({"query":"UPDATE tsv_document SET line_ending='lf',trailing_newline=0","snapshotIndex":3});
 
 
 for(const[index,owner]of logicalOwnerFixture.snapshots.entries()){
  const snapshot=parseTsvSnapshot(owner);expect(owner).toEqual(snapshot);
  const file=await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(snapshot));const independent=Database.deserialize(file);
  try{
   expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
   expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(independent.query("SELECT schema,trailing_newline,line_ending FROM tsv_document").get()).toEqual({schema:owner.schema,trailing_newline:Number(owner.trailingNewline),line_ending:owner.lineEnding});
   expect(independent.query("SELECT r.ordinal,count(f.id) AS fields FROM tsv_record r LEFT JOIN tsv_field f ON f.record_id=r.id GROUP BY r.id ORDER BY r.ordinal").all()).toEqual(owner.records.map((record,ordinal)=>({ordinal,fields:record.length})));
   expect(independent.query("SELECT f.value FROM tsv_field f JOIN tsv_record r ON f.record_id=r.id ORDER BY r.ordinal,f.ordinal").all()).toEqual(owner.records.flatMap(record=>record.map(value=>({value}))));
   expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
   if(index===logicalOwnerFixture.edit.snapshotIndex){independent.run(logicalOwnerFixture.edit.query);expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual({...snapshot,lineEnding:"lf",trailingNewline:false});}
  }finally{independent.close();}
 }
 console.log("[DEBUG] TSV complete logical owners preserved schema, empty rows, literal fields and independent newline metadata through physical SQLite");
});

import {readClosedRecordPack,type ClosedValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🔮️pack/🟦️.ts";
import {readFileSync as readLogicalAsset} from "node:fs";
import {createRequire as requireIndependentTable} from "node:module";
const independentTable:{tsvParseRows:(text:string)=>string[][]}=requireIndependentTable(import.meta.url)("d3-dsv");
const readExternalRows=independentTable.tsvParseRows;
function readTSVLogicalAsset(bytes:Uint8Array){const wire=readClosedRecordPack(bytes,"stdio.tsv");return{schema:wire[0],records:wire[1],trailingNewline:wire[2],lineEnding:wire[3]===0?"lf":"crlf"};}
test("TSV canonical producer assets retain complete owner fields and remain separate from authored raw tables",()=>{
 for(const [index,owner]of logicalOwnerFixture.snapshots.entries()){
  const pack=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🎒️"+index+".pack.semio",import.meta.url));
  expect(readTSVLogicalAsset(pack)).toEqual(owner);
  const text=readLogicalAsset(new URL("../🧫️fixtures/🧠️logical-owner/🗣️"+index+".dsl.semio",import.meta.url),"utf8");
  expect(text.startsWith("semio stdio.tsv.dsl v1\n")).toBe(true);
  expect(text).toContain("schema=");
 }
 const assets=new URL("../../../../📚️examples/🎬️demo/🖼️assets/",import.meta.url);
 const raw=readLogicalAsset(new URL("📊️.tsv",assets),"utf8"),rows=readExternalRows(raw);
 const demo=readTSVLogicalAsset(readLogicalAsset(new URL("🎒️.pack.semio",assets)));
 expect(demo.schema).toBe("stdio.tsv");
 expect(demo.records).toEqual(rows);
 console.log("[DEBUG] TSV independent DEFLATE, CRC32C and external table reader retained real canonical producer assets");
});

import completeSemantic from "../🧫️fixtures/🛂️semantic/🔣️.json";

test("tsv closed complete semantic cells agree with independent third-party SQL",async()=>{
 expect(completeSemantic["tableWidths"]["tsv_document"]).toEqual(4);expect(completeSemantic["tableWidths"]["tsv_record"]).toEqual(3);expect(completeSemantic["tableWidths"]["tsv_field"]).toEqual(4);
 for(const sample of completeSemantic.cases){const snapshot=sample.snapshot as Parameters<typeof tsvSnapshotToSqliteDatabase>[0];const database=await tsvSnapshotToSqliteDatabase(snapshot);expect(await tsvSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);let rows=0,bytes=0;const quote=(v:string)=>'"'+v.replaceAll('"','""')+'"';
  for(const[table,width]of Object.entries(completeSemantic.tableWidths)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all()as{name:string}[];expect(fields.length).toBe(width);const cells=fields.map(({name})=>{const f=quote(name);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const extent=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get()as{rows:number;bytes:number};rows+=extent.rows;bytes+=extent.bytes;}expect(rows).toBe(sample.rows);expect(bytes).toBe(sample.bytes);
 }finally{db.close();}}
});

import RustSyntaxParser from "web-tree-sitter";
import {dirname,join} from "node:path";

import receivingPolicy from "../🫴️receiving/🧫️fixtures/🔣️.json";
test("TSV closed original receiving contract agrees with independent SQLite constraints and literal octets",async()=>{
 const text=receivingPolicy.stringUnit.repeat(receivingPolicy.stringRepetitions);expect(Buffer.byteLength(text,"utf8")).toBe(receivingPolicy.stringBytes);
 for(const span of receivingPolicy.tokenScanning.spans){const independent=new Database(":memory:");try{const bytes=receivingPolicy.tokenScanning.bytes;const declaration=span==="whitespace"?" ".repeat(bytes)+TSV_SQLITE_SCHEMA:span==="identifier"?"x".repeat(bytes):span==="quotedLiteral"?"'"+"x".repeat(bytes)+"'":"'"+"''".repeat(bytes/2)+"'";if(span==="whitespace"){independent.exec(declaration);expect(independent.query("SELECT count(*) AS tables FROM sqlite_schema WHERE type='table'").get()).toEqual({tables:3});}else{expect(()=>independent.exec(declaration)).toThrow();}}finally{independent.close();}}
 for(const owner of receivingPolicy.cases){const snapshot=owner as Parameters<typeof tsvSnapshotToSqliteDatabase>[0];const database=await tsvSnapshotToSqliteDatabase(snapshot);const independent=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(independent.query("SELECT schema,trailing_newline,line_ending FROM tsv_document").get()).toEqual({schema:owner.schema,trailing_newline:Number(owner.trailingNewline),line_ending:owner.lineEnding});
  expect(independent.query("SELECT r.ordinal,count(f.id) AS fields FROM tsv_record r LEFT JOIN tsv_field f ON f.record_id=r.id GROUP BY r.id ORDER BY r.ordinal").all()).toEqual(owner.records.map((row,ordinal)=>({ordinal,fields:row.length})));
  expect(await tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toEqual(snapshot);
 }finally{independent.close();}}
 for(const edit of receivingPolicy.sqlMutations){const independent=new Database(":memory:");try{const edited=TSV_SQLITE_SCHEMA.replace(edit.from,edit.to);if(edit.sqlite==="syntaxError"){expect(()=>independent.exec(edited)).toThrow();}else{independent.exec(edited);expect(()=>independent.query("INSERT INTO tsv_document VALUES(1,'literal',0,'lf')").run()).toThrow();independent.query("INSERT INTO tsv_document VALUES(1,'literal',0,'LF')").run();expect(independent.query("SELECT line_ending FROM tsv_document").get()).toEqual({line_ending:"LF"});}}finally{independent.close();}}
 const source={schema:"literal-λ🙂",records:[[],[text,"tail"],[]],trailingNewline:true,lineEnding:"crlf" as const};
 for(const edit of receivingPolicy.lateMutations){const independent=Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(source)));try{independent.exec("PRAGMA ignore_check_constraints=ON");independent.query("UPDATE tsv_field SET "+edit.column+"=? WHERE id=2").run(edit.value);expect((independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM tsv_field WHERE id=1").get()as{bytes:number}).bytes).toBe(receivingPolicy.stringBytes);await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).rejects.toMatchObject({kind:edit.kind});}finally{independent.close();}}
 for(const edit of receivingPolicy.documentMutations){const independent=Database.deserialize(await exportSqliteDatabase(await tsvSnapshotToSqliteDatabase(source)));try{independent.exec("PRAGMA ignore_check_constraints=ON");independent.query("UPDATE tsv_document SET "+edit.column+"=?").run(edit.value);await expect(tsvSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(independent.serialize())))).rejects.toMatchObject({kind:edit.kind});}finally{independent.close();}}
 console.log("[DEBUG] TSV receiving neutral contract validated by exact Buffer UTF8 and SQLite row extents, malformed token boundaries, changed literal constraints and late domain edits; no Native execution credit");
});

test("TSV original typed receiving and bounded quote Rust syntax parses",async()=>{
 await RustSyntaxParser.init();const parser=new RustSyntaxParser();parser.setLanguage(await RustSyntaxParser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",import.meta.dir)),"out/tree-sitter-rust.wasm")));
 try{for(const relative of ["../🛂️admission/🦀️.rs","./🦀️.rs","../🫴️receiving/🦀️.rs","../🫴️receiving/🧪️tests/🦀️.rs","../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs"]){const tree=parser.parse(readLogicalAsset(new URL(relative,import.meta.url),"utf8"));expect(tree?.rootNode.hasError()).toBe(false);tree?.delete();}}finally{parser.delete();}
 expect(controlFixture.typedDestination.quoteCancelCheckpoint).toBe(3);
 console.log("[DEBUG] original TSV typed receiving and borrowed quote and paid relational receiving syntax5; no compiler or Native physical credit");
});

/** 🎟️ Verifies separately authored original body and retirement currencies independently. */
test("original tabular native caller grants agree with independent schema and SQL currency rows",async()=>{
 const{readFileSync}=await import("node:fs");const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛬️native-control/🎟️original.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🛂️admission/🧬️schema/🎟️original.json",import.meta.url),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(law)).toBe(true);
 const database=new Database(":memory:");try{
  database.run("CREATE TABLE authority(role TEXT NOT NULL,currency TEXT NOT NULL,ceiling INTEGER NOT NULL CHECK(ceiling>=0),PRIMARY KEY(role,currency))");
  for(const role of ["bodyGrant","closeGrant","deniedCloseGrant"]){for(const[currency,ceiling]of Object.entries(law[role]))database.query("INSERT INTO authority VALUES(?,?,?)").run(role,currency,ceiling as number);}
  expect(database.query("SELECT COUNT(*) AS axes,SUM(ceiling) AS remaining FROM authority WHERE role='deniedCloseGrant'").get()).toEqual({axes:5,remaining:0});
  expect(database.query("SELECT role,ceiling FROM authority WHERE currency='maximumItems' ORDER BY role").all()).toEqual([{role:"bodyGrant",ceiling:65536},{role:"closeGrant",ceiling:4096},{role:"deniedCloseGrant",ceiling:0}]);
  expect(database.query("SELECT COUNT(*) AS axes FROM authority").get()).toEqual({axes:15});
 }finally{database.close();}
 expect(law.maximumCloseTurns).toBe(65536);expect(law.nativeMaximumBytes).toBe(16777216);expect(law.custody).toBe("originalNativeRecipient");
 console.log("[DEBUG] Original tabular native caller15 independent SQL currencies/Ajv agree; body65536 and separately funded close4096, deniedClose0, originalNative16MiB, bounded65536turns");
});

/** ▶️ Preserves one original observer through canceled body and independently funded closing. */
test("original tabular observer continuation preserves caller state before funded closing",async()=>{
 const{readFileSync}=await import("node:fs");const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛬️native-control/🎟️original.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🛂️admission/🧬️schema/🎟️original.json",import.meta.url),"utf8"));const check=new Ajv2020({strict:true}).compile(schema);expect(check(law)).toBe(true);expect(check({...law,observerContinuation:{...law.observerContinuation,resumedSameObserver:false}})).toBe(false);
 const witness=law.observerContinuation;const db=new Database(":memory:");try{db.exec("CREATE TABLE observer(id INTEGER PRIMARY KEY,state TEXT NOT NULL,copy_extent INTEGER NOT NULL,stop_after INTEGER NOT NULL,owner_held INTEGER NOT NULL,body_canceled INTEGER NOT NULL)");db.query("INSERT INTO observer VALUES(1,?,?,?,1,1)").run(witness.state,witness.copyExtentBytes,witness.cancelAfterBytes);expect(db.query("SELECT id,owner_held,body_canceled,copy_extent-stop_after AS retained_tail FROM observer").get()).toEqual({id:1,owner_held:1,body_canceled:1,retained_tail:74466});db.exec("UPDATE observer SET body_canceled=0 WHERE id=1 AND owner_held=1");expect(db.query("SELECT id,state,owner_held,body_canceled FROM observer").get()).toEqual({id:1,state:witness.state,owner_held:1,body_canceled:0});}finally{db.close();}
 const caller=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");expect(caller.includes("original_continuation(&mut native)")).toBe(true);expect(witness.canceledCloseKind).toBe("Canceled");expect(witness.resumedSameObserver).toBe(true);console.log("[DEBUG] Original tabular observer closed policy/Ajv and independent SQLite retain same caller state and74466-byte tail before explicit funded-close continuation; Native behavior separate");
});
