import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync,existsSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
const snapshot=fileURLToPath(new URL("../../",import.meta.url));
test("declared compiled refusal owner closes all i32 fields and eight exact dialect coordinates",()=>{
 expect(existsSync(join(snapshot,"🔣️.json"))).toBe(true);
 const validate=new Ajv({strict:true}).addKeyword({keyword:"x-semio-state",schemaType:"string"}).compile(JSON.parse(readFileSync(join(snapshot,"🔣️.json"),"utf8")));
 for(const value of [-2147483648,-1,0,1,2147483647])expect(validate({value})).toBe(true);
 for(const value of [{},{value:null},{value:"0"},{value:0.5},{value:-2147483649},{value:2147483648},{value:0,other:1}])expect(validate(value)).toBe(false);
 const fixture=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🔣️.json"),"utf8"));
 const contract=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(snapshot,"🪶️sqlite/🧬️schema/🔣️.json"),"utf8")));
 expect(contract(fixture)).toBe(true);
 for(const wrong of [{...fixture,owner:"fixture.neutral-host-fixture.counter"},{...fixture,subsets:fixture.subsets.slice(1)},{...fixture,rows:[[1,0]]},{...fixture,identity:2},{...fixture,encodings:["binary"]},{...fixture,extra:1}])expect(contract(wrong)).toBe(false);
 expect(fixture.subsets).toEqual(["invalid-value","canceled","ownership-limit","allocation-failed","work-limit","depth-limit","unsupported-owner","invariant-violated"]);
 const sql=readFileSync(join(snapshot,"🪶️sqlite/🗄️.sql"),"utf8");
 for(const [index,value]of fixture.values.entries()){
  const db=new Database(":memory:");try{
   db.exec(sql);db.query("INSERT INTO fixture_refusal(id,value)VALUES(1,?)").run(value);
   expect(db.query("SELECT id,value,typeof(value) AS storage FROM fixture_refusal").get()).toEqual({id:1,value,storage:"integer"});
   expect(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual([{name:"fixture_refusal"}]);
   expect(db.query("PRAGMA table_info(fixture_refusal)").all().map(row=>(row as {name:string}).name)).toEqual(["id","value"]);
   const reopened=Database.deserialize(db.serialize());try{expect(reopened.query("SELECT id,value FROM fixture_refusal").all().map(row=>Object.values(row))).toEqual([fixture.rows[index]]);}finally{reopened.close();}
   const word=new DataView(new ArrayBuffer(4));word.setInt32(0,value,true);expect(word.getInt32(0,true)).toBe(value);
   db.query("UPDATE fixture_refusal SET value=? WHERE id=1").run(-value-1);
   expect(db.query("SELECT value FROM fixture_refusal").get()).toEqual({value:-value-1});
   expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
   expect(()=>db.query("INSERT INTO fixture_refusal VALUES(1,0)").run()).toThrow();
  }finally{db.close();}
 }
});
