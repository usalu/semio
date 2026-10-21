/** 🕰️ Contract: the repository glue of the graph generation keeps its protocol. Shared readers never take a writer, the builder queues on one generation scope, the facts resource is only the bounded lease handed to the store, documents stay inside the original store, and the partial inference keeps its order. The model is the fixture's ordered text laws, checked here against the graph source and in Python by `🐍️oracle.py`. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {readFileSync,readdirSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),graphPath=fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),source=readFileSync(graphPath,"utf8");
const body=(name:string):string=>{
 const row=fixture.functions[name as keyof typeof fixture.functions],start=source.indexOf(row.start),end=source.indexOf(row.end,start+row.start.length);
 expect(start,name+" start").toBeGreaterThan(-1);expect(source.indexOf(row.start,start+1),name+" start unique").toBe(-1);expect(end,name+" end").toBeGreaterThan(start);
 return source.slice(start,end);
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.order)test("order: "+row.name,()=>{const text=body(row.function),before=text.indexOf(row.before),after=text.indexOf(row.after);expect(before,"before text").toBeGreaterThan(-1);expect(after,"after text").toBeGreaterThan(-1);expect(before).toBeLessThan(after);});
for(const row of fixture.present)test("present: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(true);});
for(const row of fixture.absent)test("absent: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(false);});
test("the independent text model reaches the same verdict",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json"),graphPath]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("every self-check of a loaded implementation lets a pinned invocation finish",()=>{
 const library=fileURLToPath(new URL("../../../../",import.meta.url)),offenders:string[]=[],bare=(name:string)=>name.replace(/️/gu,"");
 const walk=(directory:string)=>{
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   if(entry.name.startsWith(".")||entry.name==="node_modules"||entry.name==="target"||bare(entry.name).endsWith("tests"))continue;
   const path=join(directory,entry.name);
   if(entry.isDirectory())walk(path);
   else if(/\.(?:ts|mjs)$/u.test(entry.name)&&!entry.name.endsWith(".d.mts"))for(const[index,line]of readFileSync(path,"utf8").split("\n").entries())if((line.includes("Stale loaded")||line.includes("Stale inference inventory reuse"))&&!line.includes("implementationPinnedV1()"))offenders.push(path.slice(library.length)+":"+(index+1));
  }
 };
 for(const entry of readdirSync(library,{withFileTypes:true}))if(entry.isDirectory()&&[bare("🏗️build"),bare("⚡️caching")].includes(bare(entry.name)))walk(join(library,entry.name));
 expect(offenders).toEqual([]);
 expect(source.includes("const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;")).toBe(true);
});
