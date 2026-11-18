/** 🩹️ Contract: the line of a parked owner never reads the snapshot of a pool whose farm has none yet, so a graph read does not die with `undefined is not an object (evaluating 's.busy')` while a pool is still being born.
 * @see ../../🟨️.mjs `describe` */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";

const source=readFileSync(join(dirname(fileURLToPath(import.meta.url)),"..","..","🟨️.mjs"),"utf8");

test("describe skips pools whose farm has no snapshot",()=>{
 const line=source.split("\n").find(row=>row.includes("describe() {"))!;
 expect(line).toContain("pool.farm?.snapshot !== undefined");
});
