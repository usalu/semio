import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { Database } from "bun:sqlite";
import { WORKSPACE_ROOT } from "../../../../../../../../../../📜️script.ts";

/** 🧪️ Validates neutral generation values, ordinal keys and independent five-currency conservation. */
export function proceduralGenerationRootSelfTests(): number {
  const base=join(WORKSPACE_ROOT,"🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation"),read=(path:string)=>JSON.parse(readFileSync(join(base,path),"utf8")),schema=read("🧬️schema/🔣️.json"),fixture=read("🧫️fixtures/🔣️.json"),Ajv=createRequire(import.meta.url)("ajv"),validate=new Ajv({strict:true,allErrors:true}).compile({...schema,$ref:"#/$defs/GenerationValueV1"});
  for(const generation of fixture.generation.generations)for(const value of Object.values(generation.values))if(!validate(value))throw Error("generation value violates its semantic contract");
  const wire=JSON.stringify(fixture.generation);if(Buffer.byteLength(wire)<=16384||JSON.stringify(JSON.parse(wire))!==wire)throw Error("independent JSON lost large nested generation content");
  const law=read("🧫️fixtures/♻️retirement/🔣️.json");
  const db=new Database(":memory:");let checks=3;
  try{
    db.run("CREATE TABLE entry(key TEXT PRIMARY KEY,value TEXT NOT NULL)");
    for(const[key,value]of fixture.rankedValues.entries)db.run("INSERT INTO entry VALUES(?,?)",key,JSON.stringify(value));
    const keys=db.query<{key:string},[]>("SELECT key FROM entry ORDER BY key COLLATE BINARY").all().map(row=>row.key);
    if(JSON.stringify(keys)!==JSON.stringify(fixture.rankedValues.expectedKeys))throw Error("ranked generation keys disagree with independent SQLite");
    const actual=Object.fromEntries(db.query<{key:string;value:string},[]>("SELECT key,value FROM entry ORDER BY key COLLATE BINARY").all().map(row=>[row.key,JSON.parse(row.value)])),expected=Object.fromEntries(fixture.rankedValues.entries);
    for(const key of keys)if(JSON.stringify(actual[key])!==JSON.stringify(expected[key]))throw Error("ranked generation value changed");checks++;
    db.run("CREATE TABLE receipt(items INTEGER,copy INTEGER,capacity INTEGER,released INTEGER,depth INTEGER,max_items INTEGER,max_copy INTEGER,max_capacity INTEGER,max_release INTEGER,max_depth INTEGER,CHECK(items BETWEEN 0 AND max_items),CHECK(copy BETWEEN 0 AND max_copy),CHECK(capacity BETWEEN 0 AND max_capacity),CHECK(released BETWEEN 0 AND max_release),CHECK(depth BETWEEN 0 AND max_depth))");
    for(const row of law.cases){let accepted=true;try{db.run("INSERT INTO receipt VALUES(?,?,?,?,?,?,?,?,?,?)",[...row.receipt,...row.grant]);}catch{accepted=false;}if(accepted!==row.accepted)throw Error("independent grant predicate "+row.id);checks++;}
    db.run("CREATE TABLE conservation(held INTEGER,born INTEGER,released INTEGER,CHECK(held>=0 AND born>=0 AND released=held+born))");
    for(const row of law.conservation){db.run("INSERT INTO conservation VALUES(?,?,?)",row);checks++;}
    for(const row of law.rejectedConservation){let refused=false;try{db.run("INSERT INTO conservation VALUES(?,?,?)",row);}catch{refused=true;}if(!refused)throw Error("independent physical conservation admitted a contradictory policy");checks++;}
  }finally{db.close();}
  return checks;
}
