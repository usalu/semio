import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {storePathsV1} from "../../../📒️journal/🟦️.ts";
import {BootstrapRefusedV1,writeBootstrapArchiveV1} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`litter-${label}-`));scratch.push(path);return path;};
const hour=3_600_000;
const make=(file:string,ageMs:number):string=>{mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"x".repeat(4096));const when=(Date.now()-ageMs)/1000;utimesSync(file,when,when);return file;};

test("the hourly archive round sweeps the AppleDouble files of the whole store, also where the round ends without an archive, and keeps young ones and real names",async()=>{
 const store=temp("store"),worktree=temp("tree"),paths=storePathsV1(store);
 const old=[
  make(join(paths.blobs,"ab","._"+"c".repeat(62)),2*hour),
  make(join(paths.heads,"._g1-m01_head.json"),2*hour),
  make(join(paths.origins,"g1-m01","._attest.json"),2*hour),
  make(join(paths.origins,"g1-m01","segments","._000000000000.jsonl"),2*hour),
  make(join(paths.snapshots,"g1-m01","._snapshot.json"),2*hour),
  make(join(paths.tmp,"._g1-m01-1-x"),2*hour),
  make(join(paths.root,"fleet","install","._pack.tar"),2*hour),
  make(join(paths.root,"bootstrap","code","d","._sealed.json"),2*hour),
  make(join(paths.root,"._origins"),2*hour)
 ];
 const partners=[make(join(paths.root,"fleet","install","pack.tar"),0),make(join(paths.root,"bootstrap","code","d","sealed.json"),0)];
 const real=make(join(paths.origins,"g1-m01","attest.json"),0);
 mkdirSync(paths.tmp,{recursive:true});
 const round=(now:number)=>writeBootstrapArchiveV1({worktree,storeRoot:store,originId:"g1-server",now:()=>now}).then(()=>undefined,error=>error);
 expect(await round(Date.now())).toBeInstanceOf(BootstrapRefusedV1);
 for(const file of old)expect(existsSync(file),file).toBe(true);
 expect(await round(Date.now()+30*60_000)).toBeInstanceOf(BootstrapRefusedV1);
 for(const file of old)expect(existsSync(file),file).toBe(true);
 expect(await round(Date.now()+2*hour)).toBeInstanceOf(BootstrapRefusedV1);
 for(const file of old)expect(existsSync(file),file).toBe(false);
 expect(existsSync(real)).toBe(true);
 for(const file of partners)expect(existsSync(file),file).toBe(true);
});
