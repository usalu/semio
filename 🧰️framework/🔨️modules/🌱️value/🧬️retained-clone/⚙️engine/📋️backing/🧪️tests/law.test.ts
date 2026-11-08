import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./law.schema.json";
import fixture from "./law.json";
test("resident array backing preserves ordered values under a finite grant",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE item(position INTEGER PRIMARY KEY,value INTEGER)");for(const [position,value] of fixture.values.entries())db.run("INSERT INTO item VALUES(?,?)",[position,value]);expect(db.query<{value:number},[]>("SELECT value FROM item ORDER BY position").all().map(row=>row.value)).toEqual(fixture.values);expect(fixture.large).toBeGreaterThan(fixture.maximumCopy);expect(fixture.zst).toBe(8193);expect(fixture.text).toBe("original β");expect(fixture.chunk*3+1).toBeLessThanOrEqual(4096);expect(fixture.discardChunk+1).toBeLessThanOrEqual(4096);expect(fixture.stock.capacityBytes).toBe(fixture.large+fixture.values.length*8)}finally{db.close()}});
