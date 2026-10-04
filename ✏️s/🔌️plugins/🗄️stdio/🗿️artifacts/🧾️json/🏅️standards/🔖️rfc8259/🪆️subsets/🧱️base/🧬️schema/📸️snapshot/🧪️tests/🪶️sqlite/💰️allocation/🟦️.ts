/** 💰️ Independent SQL values and closed native backing allowances stay distinct. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../../../🧫️fixtures/🪶️sqlite/💰️allocation/🔣️.json";
import schema from "../../../🧫️fixtures/🪶️sqlite/💰️allocation/🧬️schema/🔣️.json";
import nativeSchema from "../../../🧫️fixtures/🪶️sqlite/💰️allocation/🧬️native-allowance/🔣️.json";
import neutral from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";

test("JSON controlled Native budget is closed owned backing with unchanged witnesses", () => {
 const validate=new Ajv({strict:true,allErrors:true}).compile(nativeSchema);
 expect(validate(neutral.controlledNative)).toBe(true);
 expect(validate({...neutral.controlledNative,maxValueBytes:4096})).toBe(false);
 expect(validate({...neutral.controlledNative,maxAllocationBytes:undefined})).toBe(false);
 expect(validate({...neutral.controlledNative,cancelCompleted:0})).toBe(false);
});

test("JSON owned input allocation has a closed independent allowance contract", () => {
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,semanticBytes:fixture.maximumAllocationBytes})).toBe(false);
 expect(validate({...fixture,maximumAllocationBytes:undefined})).toBe(false);
 expect(validate({...fixture,carrier:"serialized JSON"})).toBe(false);
});

test("independent SQLite retains Unicode and NUL beyond a native semantic allowance", () => {
 const text=fixture.schema.repeat(fixture.schemaRepeat);
 const bytes=new TextEncoder().encode(text);
 expect(bytes.length).toBeGreaterThan(fixture.semanticBytes);
 expect(bytes.length).toBeLessThan(fixture.maximumAllocationBytes);
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toBe(text);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE document(id INTEGER PRIMARY KEY, schema TEXT NOT NULL);CREATE TABLE member(id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES document(id),ordinal INTEGER NOT NULL,key TEXT NOT NULL,value TEXT NOT NULL)");
  db.query("INSERT INTO document VALUES(1,?)").run(text);
  db.query("INSERT INTO member VALUES(1,1,0,?,?)").run(fixture.literal,fixture.literal);
  db.query("INSERT INTO member VALUES(2,1,1,?,?)").run(fixture.literal,"");
  expect(db.query("SELECT length(CAST(schema AS BLOB)) AS bytes FROM document").get()).toEqual({bytes:bytes.length});
  expect(db.query("SELECT schema FROM document").get()).toEqual({schema:text});
  expect(db.query("SELECT key,value FROM member ORDER BY ordinal").all()).toEqual([{key:fixture.literal,value:fixture.literal},{key:fixture.literal,value:""}]);
  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
 }finally{db.close();}
});
