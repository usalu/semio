import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("bounded native collections agree with independent JSON schemas and SQLite key identity",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(corpus)).toBe(true);
 const scalar={type:"integer",minimum:0,maximum:9007199254740991};const map=ajv.compile({type:"object",maxProperties:32,additionalProperties:scalar});const bytes=ajv.compile({type:"array",maxItems:32768,items:{type:"integer",minimum:0,maximum:255}});const list=ajv.compile({type:"array",maxItems:32,items:scalar});
 const db=new Database(":memory:");
 for(const row of corpus.cases){const value=JSON.parse(row.wire);const unique=(db.query("select count(*)=count(distinct key) ok from json_each(?)").get(row.wire)as{ok:number}).ok===1;const accepted=Boolean(({map,bytes,list}[row.kind](value)))&&(row.kind!=="map"||unique);expect(accepted).toBe(row.accepted);if(accepted)expect(value).toEqual(row.expected);}
 db.close();
});
