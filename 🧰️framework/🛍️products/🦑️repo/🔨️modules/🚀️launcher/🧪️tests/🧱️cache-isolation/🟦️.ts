import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";

const module=new URL("../../",import.meta.url),read=(relative:string):any=>JSON.parse(readFileSync(new URL(relative,module),"utf8"));
const registry=read("🗂️registry/🔣️.json"),fixture=read("🧫️fixtures/🔣️.json"),runner=fileURLToPath(new URL("./🧮️runner/🟦️.ts",import.meta.url));

/** 🧱️ Writes one root whose three modules are byte-identical in every root, the middle one above the runtime transpiler cache minimum size. */
function writeRoot(root:string):void{
  mkdirSync(root,{recursive:true});
  writeFileSync(join(root,"dep.ts"),"export const dep = 'dep';\n");
  writeFileSync(join(root,"big.ts"),"import {dep} from './dep.ts';\nexport const big = dep + 'big';\n"+Array.from({length:fixture.cacheIsolation.paddingLines},(_,index)=>`export const v${index} = ${index}; // ${"padding ".repeat(12)}`).join("\n")+"\n");
  writeFileSync(join(root,"main.ts"),"import {big} from './big.ts';\nexport const out = big;\n");
}

function run(root:string,cache:string|undefined):{status:number|null;stdout:string;stderr:string}{
  const environment:Record<string,string|undefined>={...process.env,...registry.runtimes.bun.launch.environment};
  if(cache!==undefined)environment.BUN_RUNTIME_TRANSPILER_CACHE_PATH=cache;
  const result=Bun.spawnSync([process.execPath,runner,root],{env:environment,stdout:"pipe",stderr:"pipe"});
  return{status:result.exitCode,stdout:result.stdout.toString(),stderr:result.stderr.toString()};
}

test("the launcher declares the disabled runtime transpiler cache for the pinned runtime",()=>{
  expect(registry.runtimes.bun.launch.environment).toEqual(fixture.cacheIsolation.launchEnvironment);
});

test("two roots with byte-identical modules never replay each other's resolved import paths under the declared launch environment",()=>{
  const scratch=mkdtempSync(join(tmpdir(),"semio-cache-isolation-"));
  try{
    const first=join(scratch,"first"),second=join(scratch,"second");writeRoot(first);writeRoot(second);
    for(const root of[first,second]){const result=run(root,undefined);expect(result.stderr).toBe("");expect(JSON.parse(result.stdout)).toEqual({out:"depbig"});}
  }finally{rmSync(scratch,{recursive:true,force:true});}
});

test("the pinned runtime replays the first root's resolved import path from a shared transpiler cache, which is why the launcher disables it",()=>{
  if(Bun.version!==registry.runtimes.bun.version)return;
  const scratch=mkdtempSync(join(tmpdir(),"semio-cache-isolation-")),cache=join(scratch,"cache");
  try{
    const first=join(scratch,"first"),second=join(scratch,"second");writeRoot(first);writeRoot(second);mkdirSync(cache);
    expect(run(first,cache).status).toBe(0);
    const replay=run(second,cache);
    expect(replay.status).not.toBe(0);
    expect(replay.stderr).toContain("Uncaptured implementation module refused: "+join(first,"dep.ts"));
  }finally{rmSync(scratch,{recursive:true,force:true});}
});
