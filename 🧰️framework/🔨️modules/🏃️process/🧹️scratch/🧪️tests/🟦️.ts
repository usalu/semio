/** 🧹️ Contract of repo-owned scratch: one declared home below `.cache` (disk, budgeted and swept by `repo cache gc`), a directory named by its owner process, removal on completion, error and cancellation, and a private temporary directory per process that is removed when the process ends. An independent Python check says whether a path still exists.
 * @see ./🥒️.feature
 * @see ../🟦️.ts
 * @see ../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🧹️scratch/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import protocol from "../🔣️.json" with {type:"json"};
import {createScratchDirectoryV1,isolateProcessScratchV1,removeScratchDirectoryV1,scratchHomeV1,withScratchDirectoryV1} from "../🟦️.ts";
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};

const checkouts:string[]=[];
const checkout=():string=>{const root=mkdtempSync(join(tmpdir(),"scr-"));checkouts.push(root);writeFileSync(join(root,"📋️project.json"),"{}");writeFileSync(join(root,"package.json"),"{}");return root;};
afterAll(()=>{for(const root of checkouts)rmSync(root,{recursive:true,force:true});});
const pythonExists=(path:string):boolean=>spawnSync("python3",["-I","-c","import os,sys;print(os.path.exists(sys.argv[1]))",path],{encoding:"utf8"}).stdout.trim()==="True";
const units=new RegExp(protocol.units,"u");

test("the home is the declared directory of the checkout, and the operating system directory outside every checkout",()=>{
 const root=checkout();
 expect(scratchHomeV1({root})).toBe(join(root,protocol.root,protocol.directory));
 expect(scratchHomeV1({root:join(root,"deeper","still")})).toBe(join(root,protocol.root,protocol.directory));
 expect(scratchHomeV1({root:dirname(tmpdir())==="/"?"/":dirname(tmpdir())}),"outside a checkout").not.toContain(protocol.root+"/"+protocol.directory);
});

for(const item of cases.lifecycle)test(`scratch ${item.name}: the directory is gone afterwards`,async()=>{
 const root=checkout();let seen="";
 const work=withScratchDirectoryV1(item.prefix,async directory=>{
  seen=directory;expect(units.test(basename(directory)),"named by its owner").toBe(true);expect(basename(directory).startsWith(process.pid+"-")).toBe(true);
  for(let index=0;index<item.fills;index++){mkdirSync(join(directory,"d"+index,"deep"),{recursive:true});writeFileSync(join(directory,"d"+index,"deep","file"),"x");}
  expect(dirname(directory)).toBe(scratchHomeV1({root}));
  if(item.ending==="throw")throw Error("died");
  if(item.ending==="cancel"){const controller=new AbortController();setTimeout(()=>controller.abort(Error("cancelled")),5);await new Promise((_,reject)=>controller.signal.addEventListener("abort",()=>reject(controller.signal.reason)));}
 },{root});
 if(item.ending==="complete")await work;else await expect(work).rejects.toThrow(item.ending==="throw"?"died":"cancelled");
 expect(seen).not.toBe("");expect(existsSync(seen)).toBe(false);expect(pythonExists(seen),"independent check").toBe(false);
});

test("removing a directory that is already gone is fine",()=>{
 const root=checkout(),directory=createScratchDirectoryV1("gone",{root});
 removeScratchDirectoryV1(directory);removeScratchDirectoryV1(directory);expect(existsSync(directory)).toBe(false);
});

for(const item of cases.process)test(`a process with scratch isolation: ${item.name}`,()=>{
 const root=checkout(),home=scratchHomeV1({root}),exits:(()=>void)[]=[],target={env:{...("inherit" in item?{TMPDIR:join(home,"1-parent-abc")}:{}),...("environment" in item?item.environment:{})} as Record<string,string|undefined>,once:(event:string,listener:()=>void)=>{if(event==="exit")exits.push(listener);},kill:()=>true,pid:process.pid} as unknown as NodeJS.Process;
 const directory=isolateProcessScratchV1({root},target);
 if(!item.isolates){expect(directory).toBeUndefined();return;}
 expect(directory).toBeDefined();expect(dirname(directory!)).toBe(home);expect(existsSync(directory!)).toBe(true);
 for(const name of protocol.environment)expect(target.env[name],name).toBe(directory);
 for(const exit of exits)exit();
 expect(existsSync(directory!)).toBe(false);expect(pythonExists(directory!)).toBe(false);
});

test("the test preload gives a bun process a private temporary directory on disk below the home and removes it when the process ends",()=>{
 const script=join(import.meta.dir,"..","..","🧪️testing","🟨️bun","🧹️scratch","📜️script.ts"),environment:NodeJS.ProcessEnv={...process.env};
 for(const name of protocol.environment)delete environment[name];
 const run=spawnSync(process.execPath,["--preload",script,"-e","console.log(require('node:os').tmpdir());console.log(process.env.TMPDIR===require('node:os').tmpdir())"],{encoding:"utf8",env:environment,cwd:import.meta.dir});
 expect(run.status,run.stderr).toBe(0);
 const [seen,same]=run.stdout.trim().split("\n") as [string,string];
 expect(same).toBe("true");expect(units.test(basename(seen)),seen).toBe(true);expect(basename(dirname(seen))).toBe(protocol.directory);
 expect(existsSync(seen),"removed at exit").toBe(false);expect(pythonExists(seen),"independent check").toBe(false);
});
