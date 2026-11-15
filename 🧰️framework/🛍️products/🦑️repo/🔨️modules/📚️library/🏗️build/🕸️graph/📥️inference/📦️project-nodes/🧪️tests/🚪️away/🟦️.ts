/** 🚪️ Contract: the project loop gives its governor slot back while a worker infers the project and passes the handle to the project it runs. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

test("the farm wait runs away from the governor slot",()=>{
 const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
 expect(source).toMatch(/governor\.run\(async\(\{away[,}]/u);
 expect(source).toContain("prepare:farm?()=>{missed.add(name);return away(()=>farm.prepare(");
 expect(source).toContain("inferProject(job,away)");
});
