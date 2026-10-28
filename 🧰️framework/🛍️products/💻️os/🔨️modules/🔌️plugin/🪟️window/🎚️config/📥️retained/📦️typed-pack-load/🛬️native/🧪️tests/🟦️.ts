import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import law from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
test("native saved Window portable history agrees with independent JSON and SQLite replay",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);expect(law.maximumHeaderCopyBytes).toBe(32768);
 const db=new Database(":memory:");
 try{db.exec("CREATE TABLE operations(position INTEGER PRIMARY KEY,window TEXT NOT NULL,wire TEXT NOT NULL)");for(const[position,zoom]of law.zooms.entries()){const wire=JSON.stringify({kind:"zoom",zoom});expect(JSON.parse(Buffer.from(wire).toString("utf8"))).toEqual({kind:"zoom",zoom});db.query("INSERT INTO operations VALUES(?,?,?)").run(position,law.window,wire);}const result=db.query("SELECT json_extract(wire,'$.zoom') AS zoom FROM operations WHERE window=? ORDER BY position DESC LIMIT 1").get(law.window)as{zoom:number};expect(result.zoom).toBe(law.zooms.at(-1)!);expect(result.zoom).not.toBe(law.initialZoom);for(const cut of law.cancelAt){expect(db.query("SELECT count(*) AS count FROM operations WHERE position<?").get(cut)).toEqual({count:Math.min(cut,law.zooms.length)});}}
 finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const original of["admit_native_pair","step_original(native","take_owned_into","admit_awaiting_envelope","receive_original_envelope","Box::write","close_sources","WindowConfigPackSource::Framed","next_push_allocation_bytes"]){expect(source.includes(original)).toBe(true);}for(const cold of["M::decode_op","RetainedConfigStoreHydration","apply_mutation(","StepContext::new"]){expect(source.includes(cold)).toBe(false);}
});
