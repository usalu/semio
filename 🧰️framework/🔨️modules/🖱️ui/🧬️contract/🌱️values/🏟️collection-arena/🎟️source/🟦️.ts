import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("UI arena publication requires its admitted native Source and complete owned opening",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 for(const row of corpus.cases){const result=db.query("select (? and ? and ? and not ?) ready").get(row.admitted,row.opened,row.registered,row.closing)as{ready:number};expect(Boolean(result.ready)).toBe(row.ready);}
 db.close();
});
