import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️.json";
test("recursive native UI values agree with independent JSON and SQLite descendant identity",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 for(const row of corpus.cases){const sameKeys=db.query("select count(*)=count(distinct parent||char(0)||key) uniqueKeys from json_tree(?) where parent is not null and typeof(key)='text'").get(row.wire)as{uniqueKeys:number};expect(Boolean(sameKeys.uniqueKeys)).toBe(row.accepted);if(row.accepted)expect(JSON.parse(row.wire)).toEqual(row.expected);}
 db.close();
});
