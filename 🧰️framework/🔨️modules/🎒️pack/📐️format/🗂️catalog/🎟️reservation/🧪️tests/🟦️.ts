import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
test("catalog append reservations preserve immutable funded extents and match SQLite",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));expect(new Ajv2020({strict:true}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.cases){expect(row.appends<=row.maximum).toBe(true);let capacity=0;const targets:number[]=[],payloads:number[]=[],independent:number[]=[];const db=new Database(":memory:");db.run("CREATE TABLE owner(capacity INTEGER,page_items INTEGER,target INTEGER)");db.run("INSERT INTO owner VALUES(0,?,?)",[row.pageItems,row.countKnown?row.appends:row.maximum]);for(let count=1;count<=row.appends;count++){const target=row.countKnown?row.appends:row.maximum;targets.push(target);if(count>capacity){expect(capacity%row.pageItems).toBe(0);const extent=Math.min(row.pageItems,target-capacity);payloads.push(extent);capacity+=extent;}const bytes=(db.query("SELECT CASE WHEN capacity<? THEN min(page_items,target-capacity) ELSE 0 END AS extent FROM owner").get(count)as{extent:number}).extent;if(bytes){independent.push(bytes);db.run("UPDATE owner SET capacity=capacity+?",[bytes]);}}expect(targets).toEqual(row.expectedTargets);expect(payloads).toEqual(row.expectedPayloads);expect(independent).toEqual(payloads);db.close();}
});
