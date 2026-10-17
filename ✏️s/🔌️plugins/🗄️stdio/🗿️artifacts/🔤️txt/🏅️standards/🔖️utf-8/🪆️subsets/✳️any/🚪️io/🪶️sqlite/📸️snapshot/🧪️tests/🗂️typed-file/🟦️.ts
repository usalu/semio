/** 🗂️ Complete typed TXT snapshots stay independently readable as SQLite files and keep their registered full-file laws. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {SqliteOperation,exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "@semio-tech/framework";
import {txtSnapshotToSqliteDatabase,txtSnapshotFromSqliteDatabase} from "../../🟦️.ts";
import type {TxtSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import fixture from "../🧫️fixtures/🗂️typed-file.json";
import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";

const read=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");
const metadataSql=readFileSync(frameworkSourcePath("🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql"),"utf8").trim().replace(/;$/,"");
const snapshots=fixture.cases.map(row=>row.snapshot as TxtSnapshot);

test("complete typed TXT snapshots preserve all legal fields in independently readable SQLite files",async()=>{
 const admitSnapshot=new Ajv({strict:true}).addKeyword("x-semio-state").compile(JSON.parse(read("../../../../../🧬️schema/📸️snapshot/🔣️.json")));
 for(const [index,snapshot]of snapshots.entries()){expect(admitSnapshot(snapshot)).toBe(true);const operation=new SqliteOperation({maxAllocationBytes:67108864});const projected=await txtSnapshotToSqliteDatabase(snapshot,operation);const database:SqliteDatabase={tables:[...projected.tables,{name:"semio_snapshot",sql:metadataSql,rows:[{rowid:1n,values:[1n,fixture.dialect.artifactKind,fixture.dialect.standard,fixture.dialect.subset,1n,fixture.encoding]}]}]};const bytes=await exportSqliteDatabase(database,operation);expect(Buffer.from(bytes.subarray(0,16)).toString()).toBe("SQLite format 3\0");const independent=Database.deserialize(bytes);try{
 expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all()).toEqual([{name:"semio_snapshot"},{name:"text_document"},{name:"text_line"}]);expect(independent.query("SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot").get()).toEqual({artifact_kind:fixture.dialect.artifactKind,standard:fixture.dialect.standard,subset:fixture.dialect.subset,schema_version:1,native_encoding:fixture.encoding});
 const document=independent.query("SELECT schema,trailing_newline,line_ending FROM text_document").get()as{schema:string;trailing_newline:number;line_ending:string};const lines=independent.query("SELECT content FROM text_line ORDER BY ordinal").all()as{content:string}[];expect({schema:document.schema,lines:lines.map(row=>row.content),trailingNewline:document.trailing_newline===1,lineEnding:document.line_ending==="crlf"?"crLf":"lf"}).toEqual(snapshot);expect(lines.length).toBe(snapshot.lines.length);
 const own=await importSqliteDatabase(bytes,operation);expect(await txtSnapshotFromSqliteDatabase({tables:own.tables.filter(table=>table.name!=="semio_snapshot")},operation)).toEqual(snapshot);
 const independentFile=independent.serialize();const imported=await importSqliteDatabase(independentFile,operation);expect(await txtSnapshotFromSqliteDatabase({tables:imported.tables.filter(table=>table.name!=="semio_snapshot")},operation)).toEqual(snapshot);expect(operation.ownedBytes).toBeGreaterThan(0);
 }finally{independent.close();}
 const ending=snapshot.lineEnding==="lf"?"\n":"\r\n";expect(snapshot.lines.join(ending)+(snapshot.trailingNewline?ending:"")).toBe(fixture.cases[index]!.raw);
 }
 for(const [leftId,rightId]of fixture.collisions){const left=fixture.cases.find(row=>row.id===leftId)!,right=fixture.cases.find(row=>row.id===rightId)!;expect(left.raw).toBe(right.raw);expect(left.snapshot).not.toEqual(right.snapshot);}
});

test("registered full TXT file laws borrow genuine Store reads and retire accepted complete output",()=>{
 const laws=read("../🦀️.rs");
 const start=laws.indexOf("async fn txt_original_receiving_registered_complete_store_and_destination");expect(start).toBeGreaterThan(0);const registered=laws.slice(start);
 for(const name of["plain_test_store","snapshot_read().unwrap().into_erased()","io_export_sqlite_snapshot_read","io_import_sqlite_snapshot_into","try_return_to_registry_witness","close_plain_test_store","txt_full_file_retire","lines.capacity()","schema.capacity()","registered_full_fixture"])expect(registered.includes(name)).toBe(true);
});
