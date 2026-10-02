import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import fixture from "../../🧫️fixtures/🏭️producer/🔣️.json";
import schema from "../../🧬️schema/🏭️producer/🔣️.json";

test("scalar snapshot metadata agrees with the independent SQLite field roster",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE field(id INTEGER PRIMARY KEY,key TEXT UNIQUE NOT NULL,shape TEXT NOT NULL,optional INTEGER NOT NULL CHECK(optional IN(0,1)))");
  for(const field of fixture.fields)db.run("INSERT INTO field VALUES(?,?,?,?)",field.id,field.key,field.shape,field.optional?1:0);
  expect(db.query("SELECT id,key,shape,optional FROM field ORDER BY id").all().map(row=>{const value=row as {id:number;key:string;shape:string;optional:number};return {...value,optional:!!value.optional};})).toEqual(fixture.fields);
 }finally{db.close();}
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 expect(source).toContain("fn spec_producer()");
 const rows=[...source.matchAll(/\((\d+),"([^"]+)",dsl::Shape::(\w+),(true|false)\)/g)].map(([,id,key,shape,optional])=>({id:Number(id),key,shape,optional:optional==="true"}));
 expect(rows).toEqual(fixture.fields);
 console.log(`scalar-schema-producer fields=${rows.length} independent-sqlite=${fixture.fields.length}`);
});
