import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {existsSync,readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {importSqliteDatabase} from "../../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
test("full owned SQL reconstruction has a closed exact allocation contract and independent literal bytes",async()=>{
 const base=fileURLToPath(new URL("../../../",import.meta.url));
 const path=join(base,"🧫️fixtures/🪶️sqlite/💰️reconstruction/🔣️.json");
 expect(existsSync(path)).toBe(true);
 const f=JSON.parse(readFileSync(path,"utf8"));
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(base,"🪶️sqlite/💰️reconstruction/🧬️schema/🔣️.json"),"utf8")));
 expect(validate(f)).toBe(true);
 for(const wrong of[{...f,budgetCases:["exact"]},{...f,authority:"estimatedSlots"},{...f,retainAdmission:false},{...f,extra:true}])expect(validate(wrong)).toBe(false);
 const sql=readFileSync(join(base,"🪶️sqlite/🗄️.sql"),"utf8"),corpus=JSON.parse(readFileSync(join(base,"🧫️fixtures/🪶️sqlite/🔣️.json"),"utf8"));
 const text=f.textUnit.repeat(f.repeat),db=new Database(":memory:");
 try{
  db.exec("PRAGMA foreign_keys=ON");db.exec(sql);db.exec("PRAGMA application_id=1397576526; PRAGMA user_version=1");
  for(const [table,rows]of Object.entries(corpus.rows) as [string,(string|number|null)[][]][])for(const row of rows)db.query('INSERT INTO "'+table+'" VALUES ('+row.map(()=>"?").join(",")+")").run(...row);
  db.query('UPDATE "'+f.table+'" SET "'+f.column+'"=?').run(text);
  expect(Buffer.byteLength(text,"utf8")).toBe(f.utf8Bytes);expect(new TextEncoder().encode(text).byteLength).toBe(f.utf8Bytes);
  expect(db.query('SELECT length(CAST("'+f.column+'" AS BLOB)) AS bytes FROM "'+f.table+'"').get()).toEqual({bytes:f.utf8Bytes});
  expect(f.cancelAt).toBeLessThan(f.utf8Bytes);
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const decoded=await importSqliteDatabase(db.serialize());
  const table=decoded.tables.find(table=>table.name===f.table)!;
  const columns=db.query('PRAGMA table_info("'+f.table+'")').all() as {name:string}[];
  expect(table.rows[0]!.values[columns.findIndex(column=>column.name===f.column)]).toBe(text);
  const reopened=Database.deserialize(db.serialize());try{expect(reopened.query('SELECT "'+f.column+'" AS literal FROM "'+f.table+'"').get()).toEqual({literal:text});}finally{reopened.close();}
 }finally{db.close();}
});
