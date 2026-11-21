import {test, expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./interaction-completion.json";
import schema from "./interaction-completion.schema.json";

test("native interaction completion inserts only missing canonical IDs, in declaration order", ()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  const missing=db.query<{id:string},[string,string,string,number]>(`
   SELECT verb.value AS id
   FROM json_each(?1) AS verb
   WHERE ?4 AND NOT EXISTS (
    SELECT value FROM json_each(?2) WHERE value COLLATE BINARY = verb.value
    UNION ALL
    SELECT value FROM json_each(?3) WHERE value COLLATE BINARY = verb.value
   )
   ORDER BY verb.key
  `);
  for(const row of fixture.cases){
   const ids=missing.all(JSON.stringify(fixture.canonical),JSON.stringify(row.app),JSON.stringify(row.window),Number(row.hasInteractions)).map(({id})=>id);
   expect(ids,row.id).toEqual(row.append);
  }
 }finally{db.close();}
});
