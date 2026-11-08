import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import law from "../📐️.json";
import schema from "../🔣️.json";
/** 🎟️ Empty-map cursor close has native election and physical handback costs, with one metadata event per turn. */
test("cursor retirement work bounds the defining original storage primitives",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  expect(law.maximumTurnWork).toBe((db.query("select max(1,3,4) work").get() as {work:number}).work);
  expect(law.maximumMetadataEvents).toBe(1);
  expect(law.cursors).toEqual(["update","lookup"]);
 }finally{db.close()}
});
