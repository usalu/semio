import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import fixture from "../../🧫️fixtures/🛫️cos-output.json";
import schema from "../../🧬️schema/🛫️cos-output.json";
test("all ten COS wire tags retain independent JSON and intrinsic octet identities",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE wire(ordinal INTEGER PRIMARY KEY,body TEXT NOT NULL CHECK(json_valid(body)))");for(const[index,entry]of fixture.cases.entries())db.run("INSERT INTO wire VALUES(?,?)",index,JSON.stringify(entry.wire));
 expect(db.query("SELECT json_extract(body,'$.kind') AS kind FROM wire ORDER BY ordinal").all()).toEqual(fixture.cases.map(entry=>({kind:entry.wire.kind})));
 expect(db.query("SELECT body FROM wire ORDER BY ordinal").all().map(row=>JSON.parse((row as {body:string}).body))).toEqual(fixture.cases.map(entry=>entry.wire));
 for(const entry of fixture.cases){const wire=entry.wire as {kind:string;value?:unknown;data?:number[]};if(wire.kind==="str"||wire.kind==="stream"){const bytes=Uint8Array.from((wire.kind==="str"?wire.value:wire.data)as number[]);expect(db.query("SELECT hex(?) AS octets").get(bytes)).toEqual({octets:"00FF25"});}}
 }finally{db.close();}console.log(`pdf-cos-output tags=${fixture.cases.length} intrinsic-octet-witnesses=2`);
});
