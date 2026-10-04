/** 🪆️ Actual schema and arbitrary octets belong to the logical owner before raw PNG publication. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "../../../🧫️fixtures/🪶️sqlite/🪆️owner/🔣️.json";
import contract from "../../../🧫️fixtures/🪶️sqlite/🪆️owner/🧬️schema/🔣️.json";
import {parsePngSnapshot} from "../../../🟦️.ts";
import {pngSnapshotToSqliteDatabase,pngSnapshotFromSqliteDatabase} from "../../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {PNG as IndependentPng} from "pngjs";
test("PNG logical owner preserves arbitrary literal schema and octets through interpreted SQLite",async()=>{
 const validate=new Ajv({strict:true}).compile(contract);expect(validate(corpus)).toBe(true);expect(validate({...corpus,unexpected:true})).toBe(false);expect(validate({...corpus,cases:[{schema:"a",bytes:[256]},...corpus.cases.slice(1)]})).toBe(false);
 for(const item of corpus.cases){expect(()=>IndependentPng.sync.read(Buffer.from(item.bytes))).toThrow();expect(parsePngSnapshot(item)).toEqual(item);const source=await pngSnapshotToSqliteDatabase(item),database=Database.deserialize(await exportSqliteDatabase(source));try{expect(database.query("SELECT schema,role FROM png_document").get()).toEqual({schema:item.schema,role:"literal"});expect(database.query("SELECT value FROM png_literal_octet ORDER BY ordinal").all().map((row:any)=>row.value)).toEqual(item.bytes);expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(database.serialize())))).toEqual(item);if(item.bytes.length){database.run("UPDATE png_literal_octet SET value=? WHERE ordinal=0",[corpus.editedOctet]);expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(database.serialize())))).toEqual({schema:item.schema,bytes:[corpus.editedOctet,...item.bytes.slice(1)]});}}finally{database.close();}}
});
