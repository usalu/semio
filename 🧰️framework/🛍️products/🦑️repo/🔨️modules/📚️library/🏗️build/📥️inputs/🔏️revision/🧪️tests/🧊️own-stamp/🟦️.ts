import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {mirrorRevisionReaderDirectoryV1} from "../🪞️mirror/🟦️.ts";

const module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),schema=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const counting=`import*as fs from"node:fs";export*from"node:fs";export const lstatSync=(...args)=>{globalThis.__lstat++;return fs.lstatSync(...args);};`;
/** 🔗️ Named ESM imports of a built-in are bound at link time, so patching `fs.lstatSync` never reaches the module; the preload instead rewrites the sandbox copy's `node:fs` import to a counting twin that re-exports every other binding unchanged. */
const preload=(twin:string)=>`globalThis.__lstat=0;Bun.plugin({name:"lstat-count",setup(build){build.onLoad({filter:/own-stamp-[^\\/]*[\\/].*[\\/]🟨️\.mjs$/},({path})=>({contents:require("node:fs").readFileSync(path,"utf8").replaceAll('from"node:fs"','from'+JSON.stringify(${JSON.stringify(twin)})),loader:"js"}));}});`;
type Row={sha?:string;error?:string;lstat:number};
type Helpers=Readonly<{file:(index:number)=>string;own:string}>;
type Sandbox=Readonly<{directory:string;own:string;files:readonly string[]}>;

const sandboxes:Sandbox[]=[];
/** 🧪️ Copies the module and its schema into a fresh directory, so a child process can load a module whose own files the test may change. */
function sandbox():Sandbox{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"own-stamp-")),mirror=mirrorRevisionReaderDirectoryV1(directory),own=join(mirror,"🟨️.mjs");
 mkdirSync(join(mirror,"🧬️schema"));copyFileSync(module,own);copyFileSync(schema,join(mirror,"🧬️schema","🔣️.json"));
 const old=new Date(Date.now()-60000);for(const path of [own,join(mirror,"🧬️schema","🔣️.json")])utimesSync(path,old,old);
 const files=[0,1,2,3].map(index=>{const path=join(directory,`data-${index}.txt`);writeFileSync(path,`source ${index}\n`);return path;});
 const twin=join(directory,"counting.mjs");writeFileSync(twin,counting);writeFileSync(join(directory,"preload.cjs"),preload(twin));
 const made={directory,own,files};sandboxes.push(made);return made;
}
const prepared=[sandbox(),sandbox(),sandbox(),sandbox(),sandbox()];
Bun.sleepSync(2300);
afterAll(()=>{for(const made of sandboxes)rmSync(made.directory,{recursive:true,force:true});});

/** 🪜️ Runs the steps in a child process that imports the sandbox copy (its ctime and mtime are older than the settle interval, so the stamp path is eligible), counting the lstat calls of every capture. */
function scenario(made:Sandbox,steps:(helpers:Helpers)=>readonly string[]):Row[]{
 const body=steps({file:index=>JSON.stringify(made.files[index]),own:JSON.stringify(made.own)}).join("\n");
 const program=`const fs=require("node:fs");const m=await import(${JSON.stringify(made.own)});const out=[];\nconst run=file=>{const before=globalThis.__lstat;try{out.push({sha:m.captureBoundedSourceRevisionV1(file,4096).sha256,lstat:globalThis.__lstat-before});}catch(error){out.push({error:error.message,lstat:globalThis.__lstat-before});}};\n${body}\nconsole.log(JSON.stringify(out));`;
 const run=spawnSync(process.execPath,["--preload",join(made.directory,"preload.cjs"),"-e",program],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return JSON.parse(run.stdout.trim().split("\n").at(-1)!);
}
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("An unchanged settled module is trusted by its stamp: exact digests, and later captures cost fewer lstat calls than after a touch",()=>{
 const unchanged=scenario(prepared[0]!,({file})=>[`run(${file(0)});run(${file(1)});run(${file(2)});`]);
 expect(unchanged.map(row=>row.sha)).toEqual([0,1,2].map(index=>sha(`source ${index}\n`)));
 if(python)for(const index of [0,1,2]){const digest=spawnSync(python,["-I","-c","import hashlib,sys;print(hashlib.sha256(sys.argv[1].encode()).hexdigest())",`source ${index}\n`],{encoding:"utf8"}).stdout.trim();expect(digest).toBe(unchanged[index]!.sha!);}
 const touched=scenario(prepared[1]!,({file,own})=>[`run(${file(0)});const now=new Date();fs.utimesSync(${own},now,now);run(${file(1)});run(${file(2)});`]);
 expect(touched.map(row=>row.sha)).toEqual([0,1,2].map(index=>sha(`source ${index}\n`)));
 expect(unchanged[1]!.lstat).toBeLessThan(touched[1]!.lstat);
});

test("A change of the module bytes after load is still refused, whether it grows the file or keeps its size, and a pure touch is not",()=>{
 const grown=scenario(prepared[2]!,({file,own})=>[`run(${file(0)});fs.appendFileSync(${own},"\\n//changed\\n");run(${file(1)});`]);
 expect(grown[0]!.sha).toBe(sha("source 0\n"));
 expect(grown[1]!.error).toMatch(/^(Stale loaded Source revision reader; restart required|Bounded physical Source revision required)$/u);
 const flipped=scenario(prepared[3]!,({file,own})=>[`run(${file(0)});fs.writeFileSync(${own},fs.readFileSync(${own},"utf8").replace("Stale loaded","Stale LOADED"));run(${file(1)});`]);
 expect(flipped[1]!.error).toBe("Stale loaded Source revision reader; restart required");
 const touched=scenario(prepared[4]!,({file,own})=>[`run(${file(0)});const now=new Date();fs.utimesSync(${own},now,now);run(${file(1)});`]);
 expect(touched[1]!.sha).toBe(sha("source 1\n"));
});
