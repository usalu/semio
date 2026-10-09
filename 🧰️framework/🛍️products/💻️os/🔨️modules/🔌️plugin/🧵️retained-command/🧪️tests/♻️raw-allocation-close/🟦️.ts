import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Buffer} from "node:buffer";
test("raw retirement examples separate copied content from original backing release",()=>{
 const root=join(import.meta.dir,"../..");
 const fixture=JSON.parse(readFileSync(join(root,"🧫️fixtures/🚪️raw-allocation-close.json"),"utf8"));
 for(const row of fixture.cases){const original=Buffer.alloc(row.capacity);expect(row.initializedBytes).toBeLessThanOrEqual(original.byteLength);expect(row.expectedCopiedBytes).toBe(original.subarray(0,row.initializedBytes).byteLength);expect(row.expectedReleasedBytes).toBe(original.byteLength);}
 expect(fixture.cases.some((row:{capacity:number;initializedBytes:number})=>row.capacity>fixture.maximumCopyBytes&&row.initializedBytes===0)).toBe(true);
});
