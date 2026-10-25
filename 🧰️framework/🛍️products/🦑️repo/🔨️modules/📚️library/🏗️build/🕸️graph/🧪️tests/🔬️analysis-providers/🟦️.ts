/** 🔬️ Contract: a recorded `analysis` fact names its parser provider and fails closed while that provider is unregistered, so every process that verifies recorded facts registers the structural parsers first. Verified with two processes (record, then verify with and without registration) against an independent Python oracle of the on-disk digest. */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";

const library=fileURLToPath(new URL("../../../../",import.meta.url)),child=fileURLToPath(new URL("./🟨️.mjs",import.meta.url)),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const run=(args:string[],input?:string)=>{const result=Bun.spawnSync([process.execPath,child,...args],{stdin:input===undefined?"ignore":new TextEncoder().encode(input),stdout:"pipe",stderr:"pipe"});expect(result.exitCode,result.stderr.toString()).toBe(0);return result.stdout.toString();};

test("a fresh process holds a recorded source analysis only after it registered the parsers",()=>{
 const directory=mkdtempSync(join(tmpdir(),"analysis-providers-")),file=join(directory,"module.ts");
 try{
  writeFileSync(file,"import {a} from \"./a.ts\";\nexport const b=a;\n");
  const rows=JSON.parse(run(["record",library,file]));
  expect(rows.length).toBeGreaterThan(0);
  const verdict=JSON.parse(run(["verify",library,file],JSON.stringify(rows)));
  expect(verdict).toEqual({rows:rows.length,before:false,after:true});
  const expected=JSON.parse(Bun.spawnSync(["python3",oracle,file],{stdin:new TextEncoder().encode(JSON.stringify(rows)),stdout:"pipe"}).stdout.toString());
  expect(expected).toEqual({rows:rows.length,holds:true});
 }finally{rmSync(directory,{recursive:true,force:true});}
});
