import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import {resolve} from "node:path";
import glob from "fast-glob";
import examples from "../🧫️fixtures/🧭️trial-authority/🔣️.json";

const owner=resolve(import.meta.dir,"../..");
for(const row of examples.cases)test("Value canonical authority "+row.id,()=>{
 const independent=glob.sync(row.path,{cwd:owner,onlyFiles:true,followSymbolicLinks:false}).length===1;
 expect(independent,row.path).toBe(row.present);
 expect(existsSync(resolve(owner,row.path))).toBe(independent);
});
