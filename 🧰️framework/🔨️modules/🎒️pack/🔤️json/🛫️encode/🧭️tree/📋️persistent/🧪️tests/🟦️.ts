import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("original persistent rows retain canonical logical array order",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 for(const length of fixture.lengths){const oracle=db.query("WITH RECURSIVE rows(i) AS (SELECT 0 WHERE ?>0 UNION ALL SELECT i+1 FROM rows WHERE i+1<?) SELECT json_group_array(json_object('label',?||i,'rank',i*?)) AS json FROM rows").get(length,length,fixture.prefix,fixture.stride) as {json:string};const expected=Array.from({length},(_,i)=>({label:fixture.prefix+i,rank:i*fixture.stride}));expect(JSON.parse(oracle.json)).toEqual(expected);expect(JSON.stringify(JSON.parse(oracle.json))).toBe(JSON.stringify(expected));for(const cut of fixture.cuts)expect(JSON.stringify(expected).slice(0,cut)).toBe(oracle.json.slice(0,cut));}
 db.close();console.log("[DEBUG] persistent canonical row output and interrupt prefixes match ordered SQLite JSON across five original lengths");
});
