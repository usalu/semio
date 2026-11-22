/** 🪶️ Contract: the product host-capacity module asks for admission without loading the graph. Importing it loads no module of the graph group or the inputs reader (a status or fleet command that only needs the capacity control, and a process that only asks the gate, pay for none of them) and neither does reading the sealed-read flag of a `show`. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const scratch=mkdtempSync(join(tmpdir(),"lazy-graph-")),bare=join(scratch,"bunfig.toml"),out=join(scratch,"modules.json");
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
writeFileSync(bare,"");

test("importing the product host-capacity module loads no graph module, and neither does the sealed-read flag of a show",()=>{
 const run=Bun.spawnSync([process.execPath,"--config="+bare,join(import.meta.dir,"🧲️probe","🟦️.ts"),out],{env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString().slice(0,600)).toBe(0);
 const seen=JSON.parse(readFileSync(out,"utf8")) as {before:string[];after:string[];sealed:boolean};
 const graph=(path:string)=>path.includes("/🏗️build/🕸️graph/")||path.includes("/🏗️build/📥️inputs/");
 expect(seen.before.filter(graph)).toEqual([]);
 expect(seen.before.length,"the whole closure of the module stays small").toBeLessThan(120);
 expect(seen.sealed).toBe(true);
 expect(seen.after.filter(graph),"reading the sealed flag loads no graph module either").toEqual([]);
});
