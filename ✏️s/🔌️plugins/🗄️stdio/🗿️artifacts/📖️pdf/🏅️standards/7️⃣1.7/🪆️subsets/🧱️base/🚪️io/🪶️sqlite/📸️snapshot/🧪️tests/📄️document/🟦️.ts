/** 📖️ Independent SQLite verifies every top-level owned document relationship. */

test("PDF closed native semantic extent independently measures literal SQL cells",async()=>{
  const plan=JSON.parse(await Bun.file(new URL("../../🧫️fixtures/🛂️semantic/🔣️.json",import.meta.url)).text());expect(plan["encodings"]).toEqual(["binary","text"]);expect(plan["metadata"]).toEqual("excluded from direct domain");
  for(const item of plan.cases){
    const value=pdfSnapshotFromNativeJson({schema:item.schema.repeat(item.repeat??1),declaredVersion:item.declaredVersion,pages:[],fonts:[],images:[],forms:[],extGStates:[],shadings:[],patterns:[],colorSpaces:[],properties:[],outlines:[],namedDestinations:[],pageLabels:[],embeddedFiles:[],outputIntents:[],info:{},catalogExtra:[],trailer:[],objects:[]});
    const database=await pdf17SnapshotToSqliteDatabase(value),sql=new Database(":memory:");
    for(const table of database.tables){sql.run(table.sql);const fields=sql.query(`PRAGMA table_info("${table.name}")`).all() as {name:string}[];const insert=sql.query(`INSERT INTO "${table.name}"(${fields.map(field=>'"'+field.name+'"').join(",")}) VALUES(${fields.map(()=>"?").join(",")})`);for(const row of table.rows)insert.run(...row.values);}
    let rows=0,bytes=0;const counts:Record<string,number>={};
    for(const table of sql.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all() as {name:string}[]){
      const fields=sql.query(`PRAGMA table_info("${table.name}")`).all() as {name:string}[],expression=fields.map(field=>`CASE typeof("${field.name}") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("${field.name}" AS BLOB)) WHEN 'blob' THEN length("${field.name}") ELSE 0 END`).join("+");
      const n=sql.query(`SELECT COUNT(*) AS rows,COALESCE(SUM(${expression}),0) AS bytes FROM "${table.name}"`).get() as {rows:number,bytes:number};rows+=n.rows;bytes+=n.bytes;if(n.rows)counts[table.name]=n.rows;
    }
    sql.close();expect(counts).toEqual(item.counts);expect(rows).toBe(item.rows);expect(bytes).toBe(item.valueBytes);
    await expect(pdf17SnapshotToSqliteDatabase(value,{maxRows:item.rows,maxValueBytes:item.valueBytes})).resolves.toBeDefined();
    await expect(pdf17SnapshotToSqliteDatabase(value,{maxValueBytes:item.valueBytes-1})).rejects.toThrow();
  }
},{timeout:30_000});

import { expect,test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { parsePdfSnapshot } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { pdfSnapshotFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/📄️document/🟦️.ts";
import { pdf17SnapshotToSqliteDatabase,pdf17SnapshotFromSqliteDatabase } from "../../🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF complete page fields preserve every exceptional IEEE word and optional presence",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📄️document/🧫️fixtures/🔣️.json",import.meta.url)).text());const cases=JSON.parse(await Bun.file(new URL("../../🧫️fixtures/🔢ieee.json",import.meta.url)).text());for(const item of cases.binary64){const value=pdfSnapshotFromNativeJson(fixture);const word={bits:BigInt(`0x${item.bits}`)};const page=value.pages[0]!;page.mediaBox=[word,word,word,word];page.cropBox=[word,word,word,word];page.bleedBox=[word,word,word,word];page.trimBox=[word,word,word,word];page.artBox=[word,word,word,word];page.userUnit=word;page.duration=word;const bytes=await exportSqliteDatabase(await pdf17SnapshotToSqliteDatabase(value));const sql=Database.deserialize(bytes);expect(sql.query("SELECT CAST(user_unit_bits AS TEXT) AS bits,user_unit_class AS class,user_unit IS NULL AS nullQuery FROM pdf_page").get()).toEqual({bits:BigInt.asIntN(64,word.bits).toString(),class:item.class,nullQuery:item.class==="nan"?1:0});expect(await pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(value);sql.close();}
},{timeout:30_000});
test("PDF complete reconstruction rejects partial geometry, document IDs, bad ordering and weakened SQL",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📄️document/🧫️fixtures/🔣️.json",import.meta.url)).text());const value=pdfSnapshotFromNativeJson(fixture);const bytes=await exportSqliteDatabase(await pdf17SnapshotToSqliteDatabase(value));for(const edit of["UPDATE pdf_page SET crop_left=NULL,crop_left_bits=NULL,crop_left_class=NULL","UPDATE pdf_document SET document_id_first=NULL","UPDATE pdf_document_page SET ordinal=1"]){const sql=Database.deserialize(bytes);sql.query(edit).run();await expect(pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).rejects.toThrow();sql.close();}const database=await importSqliteDatabase(bytes);const weakened={...database,tables:database.tables.map(table=>table.name==="pdf_document"?{...table,sql:table.sql.replace("NOT NULL",'"NOT NULL"')}:table)};await expect(pdf17SnapshotFromSqliteDatabase(weakened)).rejects.toThrow();await expect(pdf17SnapshotToSqliteDatabase(value,{maxRows:4})).rejects.toThrow();await expect(pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes),{maxValueBytes:32})).rejects.toThrow();await expect(pdf17SnapshotToSqliteDatabase(value,{signal:AbortSignal.abort()})).rejects.toThrow();await expect(pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes),{signal:AbortSignal.abort()})).rejects.toThrow();
},{timeout:30_000});

test("PDF complete native document admission constructs the same Binary64 and bigint model",async()=>{const fixture=JSON.parse(await Bun.file(new URL("../../📄️document/🧫️fixtures/🔣️.json",import.meta.url)).text());const value=parsePdfSnapshot(pdfSnapshotFromNativeJson(fixture));expect(value.pages[0]!.mediaBox[2]).toEqual({bits:0x4059000000000000n});expect(value.objects[0]!.value).toEqual({kind:"int",value:-123n});expect(value).toEqual(pdfSnapshotFromNativeJson(fixture));});
test("PDF full document ownership remains queryable and editable without native codecs",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📄️document/🧫️fixtures/🔣️.json",import.meta.url)).text());const value=pdfSnapshotFromNativeJson(fixture);const database=await pdf17SnapshotToSqliteDatabase(value);const bytes=await exportSqliteDatabase(database);const sql=Database.deserialize(bytes);expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT d.schema,p.media_right,p.metadata,a.contents FROM pdf_document d JOIN pdf_document_page r ON r.document_id=d.id JOIN pdf_page p ON p.id=r.page_id JOIN pdf_page_annotation pa ON pa.page_id=p.id JOIN pdf_annotation a ON a.id=pa.annotation_id").get()).toEqual({schema:"owned-extension-schema",media_right:100,metadata:"page metadata",contents:"annotation"});expect(await pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(value);sql.query("UPDATE pdf_annotation SET contents='Independent edit'").run();const edited=await pdf17SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()));expect(edited.pages[0]!.annotations![0]!.contents).toBe("Independent edit");sql.close();
},{timeout:30_000});

test("PDF direct domain row allowance excludes I/O metadata",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📄️document/🧫️fixtures/🔣️.json",import.meta.url)).text());const value=pdfSnapshotFromNativeJson(fixture);const database=await pdf17SnapshotToSqliteDatabase(value);const count=database.tables.reduce((total,table)=>total+table.rows.length,0);const sql=Database.deserialize(await exportSqliteDatabase(database));let independent=0;for(const table of database.tables)independent+=Number((sql.query(`SELECT COUNT(*) AS count FROM "${table.name}"`).get() as {count:number}).count);expect(independent).toBe(count);expect(sql.query("SELECT name FROM sqlite_schema WHERE name='semio_snapshot'").all()).toEqual([]);sql.close();expect(await pdf17SnapshotToSqliteDatabase(value,{maxRows:count})).toEqual(database);expect(await pdf17SnapshotFromSqliteDatabase(database,{maxRows:count})).toEqual(value);await expect(pdf17SnapshotToSqliteDatabase(value,{maxRows:count-1})).rejects.toThrow();await expect(pdf17SnapshotFromSqliteDatabase(database,{maxRows:count-1})).rejects.toThrow();
},{timeout:30_000});

test("PDF native row fixture independently counts every COS value and relationship",async()=>{
  const plan=JSON.parse(await Bun.file(new URL("../../🧫️fixtures/🛫️row-admission.json",import.meta.url)).text());
  
  expect(plan["schema"]).toEqual("semio.pdf.native-row-admission/v1");
  const value=pdfSnapshotFromNativeJson({schema:"row-admission",declaredVersion:"1.7",pages:[],fonts:[],images:[],forms:[],extGStates:[],shadings:[],patterns:[],colorSpaces:[],properties:[],outlines:[],namedDestinations:[],pageLabels:[],embeddedFiles:[],outputIntents:[],info:{},catalogExtra:[],trailer:[],objects:[{id:{num:1,gen:0},value:{kind:"array",value:Array.from({length:plan.arrayLength},()=>({kind:"null"}))}}]});
  const database=await pdf17SnapshotToSqliteDatabase(value,{maxRows:plan.totalRows});
  const sql=Database.deserialize(await exportSqliteDatabase(database));
  const counts:Record<string,number>={};let total=0;
  for(const row of sql.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[]){const count=(sql.query(`SELECT COUNT(*) AS n FROM "${row.name}"`).get() as {n:number}).n;if(count)counts[row.name]=count;total+=count;}
  expect(counts).toEqual(plan.counts);expect(total).toBe(plan.totalRows);sql.close();
  await expect(pdf17SnapshotToSqliteDatabase(value,{maxRows:plan.refusedRows})).rejects.toThrow();
},{timeout:30_000});
