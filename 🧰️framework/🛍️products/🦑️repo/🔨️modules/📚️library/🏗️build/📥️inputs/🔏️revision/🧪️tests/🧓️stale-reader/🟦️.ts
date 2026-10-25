/** 🧓️ Contract: a revision reader whose own implementation or schema was edited while it ran refuses with the stale-reader error, whatever the edit did to the file (appended, shortened, replaced, removed), instead of a size-bound refusal that hides the cause; an invocation pinned to the implementation it loaded finishes instead, also for the modules it loads while it runs, because Bun evaluates the top level of a dynamically imported module outside the async context that imported it. */
import {expect,test} from "bun:test";
import {appendFileSync,copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";

const framework=fileURLToPath(new URL("../../../../../../../../../",import.meta.url)),own=(path:string)=>fileURLToPath(new URL(path,import.meta.url));
const mirrored=[own("../../🟨️.mjs"),own("../../🧬️schema/🔣️.json"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🟦️.ts"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🧬️schema","🔣️.json")];
const load=async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"stale-reader-")),target=join(root,"target.txt");
 for(const path of mirrored){const copy=join(root,relative(framework,path));mkdirSync(dirname(copy),{recursive:true});copyFileSync(path,copy);}
 writeFileSync(target,"data");
 const module=join(root,relative(framework,mirrored[0]!)),specifier=module+"?"+Math.random(),loaded=await import(specifier);
 return{root,module,specifier,target,loaded};
};
const stale="Stale loaded Source revision reader; restart required";

test("an unchanged reader captures",async()=>{
 const{root,target,loaded}=await load();
 try{expect(loaded.captureBoundedSourceRevisionV1(target,1024).sha256).toMatch(/^[a-f0-9]{64}$/u);}finally{rmSync(root,{recursive:true,force:true});}
});
for(const[name,edit]of Object.entries({
 "bytes appended to the implementation":(row:{module:string})=>appendFileSync(row.module,"\n// edited\n".repeat(50)),
 "implementation shortened":(row:{module:string})=>writeFileSync(row.module,"export {};\n"),
 "implementation replaced by same-size bytes":(row:{module:string})=>{const text=readFileSync(row.module,"utf8");writeFileSync(row.module,text.slice(0,-1)+(text.endsWith("x")?"y":"x"));},
 "implementation removed":(row:{module:string})=>rmSync(row.module),
}))test("stale reader error when "+name,async()=>{
 const row=await load();
 try{edit(row as never);expect(()=>row.loaded.captureBoundedSourceRevisionV1(row.target,1024)).toThrow(stale);}finally{rmSync(row.root,{recursive:true,force:true});}
});

test("the stale-reader error is the typed stale implementation the outermost command restarts on",async()=>{
 const row=await load();
 try{appendFileSync(row.module,"\n// edited\n".repeat(50));const error=(()=>{try{row.loaded.captureBoundedSourceRevisionV1(row.target,1024);}catch(failure){return failure as Error&{code?:string;name:string};}return undefined;})();
  expect(error?.code).toBe("SEMIO_STALE_IMPLEMENTATION");expect(error?.name).toBe("StaleImplementationErrorV1");
 }finally{rmSync(row.root,{recursive:true,force:true});}
});

test("a process that runs one invocation opts in, and then its pinned invocation finishes also for a module that reads its Sources at its top level while it loads; a service never opts in, so a pin never covers a sibling invocation",async()=>{
 const row=await load(),flag=globalThis as unknown as Record<symbol,{open:number;processWide:boolean}>,key=Symbol.for("semio.implementationPin.count.v1");
 try{
  const probe=join(row.root,"probe.mjs");
  writeFileSync(probe,`import{captureBoundedSourceRevisionV1}from${JSON.stringify(row.specifier)};export const sha=captureBoundedSourceRevisionV1(${JSON.stringify(row.target)},1024).sha256;`);
  appendFileSync(row.module,"\n// edited by a peer\n".repeat(20));
  expect(row.loaded.implementationPinnedV1()).toBe(false);
  await expect(import(probe+"?unpinned")).rejects.toThrow(stale);
  const sibling=await row.loaded.withImplementationPinV1(async()=>{const inside=row.loaded.implementationPinnedV1(),loadedInside=await import(probe+"?service").then(()=>true,()=>false);const outside=await new Promise<boolean>(accept=>setTimeout(()=>accept(row.loaded.implementationPinnedV1()),0));return{inside,loadedInside,outside};});
  expect(sibling.inside).toBe(true);expect(sibling.loadedInside).toBe(false);
  row.loaded.useProcessWideImplementationPinV1();
  const seen=await row.loaded.withImplementationPinV1(async()=>{const inside=row.loaded.implementationPinnedV1(),module=await import(probe+"?pinned"),again=row.loaded.captureBoundedSourceRevisionV1(row.target,1024).sha256;return{inside,sha:module.sha,again};});
  expect(seen.inside).toBe(true);expect(seen.sha).toBe(seen.again);
  expect(row.loaded.implementationPinnedV1()).toBe(false);
  await expect(import(probe+"?after")).rejects.toThrow(stale);
 }finally{if(flag[key])flag[key].processWide=false;rmSync(row.root,{recursive:true,force:true});}
});

test("the pin of one invocation does not outlive it, also when its body fails",async()=>{
 const row=await load();
 try{
  await expect(row.loaded.withImplementationPinV1(async()=>{throw new Error("body failed");})).rejects.toThrow("body failed");
  expect(row.loaded.implementationPinnedV1()).toBe(false);
  await Promise.all([row.loaded.withImplementationPinV1(async()=>{await Bun.sleep(10);}),row.loaded.withImplementationPinV1(async()=>{await Bun.sleep(30);})]);
  expect(row.loaded.implementationPinnedV1()).toBe(false);
 }finally{rmSync(row.root,{recursive:true,force:true});}
});
