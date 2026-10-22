import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("in-dashboard keymap editing saves only valid deliberate changes against independent SQLite",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const keymap=JSON.parse(readFileSync(new URL("../../../../⚙️preferences/⌨️keymap/🔣️.json",import.meta.url),"utf8")),db=new Database(":memory:");
 try{for(const row of fixture.cases){const conflict=row.binding?db.query("select count(*) n from json_each(?1,'$.bindings') b,json_each(b.value,'$.keys') k,json_each(?2,'$.keys') proposed where json_extract(b.value,'$.scope')||'.'||json_extract(b.value,'$.action')<>json_extract(?2,'$.id') and k.value=proposed.value and json_extract(b.value,'$.scope')=substr(json_extract(?2,'$.id'),1,instr(json_extract(?2,'$.id'),'.')-1)").get(JSON.stringify(keymap),JSON.stringify(row.binding))as{n:number}:{n:0};const oracle=db.query("select case when ?1 then 'cancelled' when ?2 or (?3 is not null and instr(?3,'+')=0) then 'refused' when (?3 is not null and ?3<>?4) or ?5 then 'saved' else 'unchanged' end outcome").get(Number(row.cancel),conflict.n,row.prefix,keymap.prefix,Number(row.binding!==null))as{outcome:string};expect(oracle.outcome,row.name).toBe(row.expected);}expect(new Ajv({strict:true}).validate(schema,{...fixture,extra:true})).toBe(false);}finally{db.close();}
});
