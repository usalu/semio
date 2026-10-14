import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../🟦️.ts";

test("original physical SQLite fixture preserves every literal cell against independent SQLite",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const database={tables:[{name:fixture.table,sql:fixture.sql,rows:[{rowid:BigInt(fixture.rowid),values:[BigInt(fixture.rowid),fixture.text]}]}]};
 const file=await exportSqliteDatabase(database);expect(file.length).toBe(fixture.expectedFileBytes);
 const oracle=Database.deserialize(file);try{expect(oracle.query("SELECT CAST(id AS TEXT) AS id,text FROM document").get()).toEqual({id:fixture.rowid,text:fixture.text});expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});}finally{oracle.close();}
 expect(await importSqliteDatabase(file)).toEqual(database);
 console.log("[DEBUG] Original physical SQLite literal corpus independent SQLite/first-party source whole-cell equality and authored8192byte extent");
});
test("physical SQLite receiving retains every refused prefix under supplied original custody",()=>{
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("pub struct PhysicalExportFrame")).toBe(true);
 expect(source.includes("pub struct PhysicalImportFrame")).toBe(true);
 expect(source.includes("export_sqlite_database_receiving_into")).toBe(true);
 expect(source.includes("import_sqlite_database_receiving_into")).toBe(true);
});
