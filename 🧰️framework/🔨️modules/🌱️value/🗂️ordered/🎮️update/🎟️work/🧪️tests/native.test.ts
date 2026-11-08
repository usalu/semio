import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import law from "../📐️.json";
import schema from "../🔣️.json";
/** 🌳️ Authored AVL cases bound actual alias, metadata and source-byte work independently. */
test("native ordered update work is derived from real issued aliases and authored key widths",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  const row=db.query("select max(json_array_length(value)) nodes from json_each(?)").get(JSON.stringify(law.cases)) as {nodes:number};
  const text=db.query("select max(length(cast(json_extract(p.value,'$[0]') as blob))) bytes from json_each(?) c,json_each(c.value) p").get(JSON.stringify(law.cases)) as {bytes:number};
  const actual=db.query("select max(4,1+3*?,1+2*?) maximumTurnWork").get(row.nodes,text.bytes) as {maximumTurnWork:number};
  expect(law.maximumTurnWork).toBe(actual.maximumTurnWork);
  expect(law.maximumMetadataEvents).toBe(1);
  expect(law.constructor).toEqual(db.query("select 2 nonemptyAliases,1 metadataWork,3 aliasWork,1 emptyWork,1+2*3 nonemptyWork").get());
 }finally{db.close()}
});
