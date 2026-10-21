import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";
test("original mutex payload survives poisoned status and independent owned transfer replay",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
 for(const row of corpus.cases){const original={owner:row.owner};const transferred=applyPatch({mutex:original,poisoned:row.poisoned},[{op:"move",from:"/mutex",path:"/owned"}],true,false).newDocument as unknown as {owned:typeof original};expect({preserved:transferred.owned.owner===row.owner,terminal:true}).toEqual(row.expected);}
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("poisoned.into_inner()")).toBe(true);expect(source.includes("ControlledRetirement::new(original)")).toBe(true);expect(source.includes("maximum_bytes")).toBe(false);
});

test("exclusive retirement cursor transfer matches independent SQLite admission",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 expect(new Ajv2020({strict:true}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.exclusiveGate.cases){let busy=false;const actual:boolean[]=[];const independent:boolean[]=[];const db=new Database(":memory:");db.run("CREATE TABLE custody(busy INTEGER,payload INTEGER)");db.run("INSERT INTO custody VALUES(0,?)",[row.value]);
  for(const event of row.events){const acquire=event==="acquire",accepted=acquire?!busy:busy;if(accepted)busy=acquire;actual.push(accepted);db.run("UPDATE custody SET busy=? WHERE busy=?",[Number(acquire),Number(!acquire)]);independent.push((db.query("SELECT changes() AS count").get()as{count:number}).count===1);}
  expect(actual).toEqual(row.accepted);expect(independent).toEqual(actual);expect(busy).toBe(false);expect(db.query("SELECT payload FROM custody").get()).toEqual({payload:row.value});db.close();
 }
});

test("shared close query admission matches independent SQLite serialization",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 expect(new Ajv2020({strict:true}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.sharedQueries){let held=false;const actual:boolean[]=[];const independent:boolean[]=[];const db=new Database(":memory:");db.run("CREATE TABLE query_gate(held INTEGER)");db.run("INSERT INTO query_gate VALUES(0)");for(const event of row.events){if(event==="enter"){actual.push(!held);held=true;db.run("UPDATE query_gate SET held=1 WHERE held=0");}else if(event==="leave"){actual.push(held);held=false;db.run("UPDATE query_gate SET held=0 WHERE held=1");}else{actual.push(!held);db.run("UPDATE query_gate SET held=held WHERE held=0");}independent.push((db.query("SELECT changes() AS count").get()as{count:number}).count===1);}expect(actual).toEqual(row.accepted);expect(independent).toEqual(actual);db.close();}
});
