import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("last original sealed alias hands one native payload cursor to its caller frontier",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 db.run("CREATE TABLE aliases (position INTEGER PRIMARY KEY, original TEXT NOT NULL)");
 for(const row of fixture.cases){db.run("DELETE FROM aliases");for(let i=0;i<row.aliases;i++)db.run("INSERT INTO aliases VALUES (?,?)",[i,row.text]);let handoffs=0;for(let i=0;i<row.aliases;i++){const original=db.query("SELECT original FROM aliases WHERE position=?").get(i) as {original:string};expect(original.original).toBe(row.text);db.run("DELETE FROM aliases WHERE position=?",[i]);if((db.query("SELECT count(*) AS n FROM aliases").get() as {n:number}).n===0)handoffs++;}expect(handoffs).toBe(row.expectedChildHandoffs);expect(row.expectedNativePayloadFrames).toBe(row.aliases===0?0:1);expect(row.expectedNestedQueues).toBe(0);}
 for(const row of fixture.admission){const accepted=row.supported&&row.birthKnown&&row.items>0&&row.capacity==="exact"&&row.depth>0;const oracle=db.query("SELECT ? AND ? AND ?>0 AND ?='exact' AND ?>0 AS accepted").get(Number(row.supported),Number(row.birthKnown),row.items,row.capacity,row.depth)as{accepted:number};expect(accepted).toBe(row.accepted);expect(Boolean(oracle.accepted)).toBe(row.accepted);}
 db.close();
});
