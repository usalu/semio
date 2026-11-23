/** 📇️ Contract: a directory whose `readdir` row is verified in the same call answers the `lstat`, `exists`, `fileExists` and `directoryExists` rows of its entries from that one listing, so a verification pass costs one read per directory and not one probe per child; the verdicts equal an independent Python model on every scenario, a fact that no longer holds (changed type, appeared, vanished) is still refused, and a child the listing cannot prove (no listing in the call, a link, a name that differs only by letter case) is probed as before. */
import fs,{mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{afterAll,expect,test}from"bun:test";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{directoryFactOfV1,observationsHold,withObservedWorldV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type Row={kind:string;name:string;fact:unknown};
type Scenario={name?:string;entries:Record<string,string>;rows:Row[]};
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const model=(scenarios:readonly Scenario[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({scenarios}))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).answers as boolean[][];
};
const build=(entries:Record<string,string>)=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"listed-children-")));scratch.push(base);
 for(const[name,type]of Object.entries(entries)){if(type==="directory")mkdirSync(join(base,name));else writeFileSync(join(base,name),"x\n");}
 return base;
};
const probed=["lstatSync","statSync","existsSync"];
const hold=async(base:string,row:Row,listed:boolean)=>{
 const target=join(base,row.name),probes:string[]=[];
 const port=new Proxy(fs,{get:(source,key)=>probed.includes(key as string)?(path:unknown,...rest:unknown[])=>{if(String(path)===target)probes.push(String(key));return(source as never as Record<string,Function>)[key as string]!(path,...rest);}:(source as never)[key as never]});
 const rows=[...listed?[{kind:"readdir",path:base,fact:directoryFactOfV1("readdir",base)}]:[],{kind:row.kind,path:target,fact:row.fact}];
 const ok=await withObservedWorldV1(port as never,()=>observationsHold(rows as never,4));
 return{ok,probes};
};

test("with the listing in the call every verdict equals the model and a confirmed child costs no probe; without it every child is probed",async()=>{
 const answers=model(fixture.scenarios as Scenario[]);
 for(const[at,scenario]of(fixture.scenarios as Scenario[]).entries()){
  const base=build(scenario.entries);
  for(const[index,row]of scenario.rows.entries()){
   const expected=answers[at]![index]!,withListing=await hold(base,row,true),without=await hold(base,row,false);
   expect(withListing.ok,scenario.name+" / "+row.kind+" "+row.name+" (listed)").toBe(expected);
   expect(without.ok,scenario.name+" / "+row.kind+" "+row.name+" (probed)").toBe(expected);
   if(expected)expect(withListing.probes,scenario.name+" / "+row.kind+" "+row.name).toEqual([]);
   expect(without.probes.length,scenario.name+" / "+row.kind+" "+row.name).toBeGreaterThan(0);
  }
 }
});

test("a name that differs from a listed one only by letter case is left to the real probe, so the verdict is that of the file system",async()=>{
 const base=build({Foo:"file"}),row={kind:"lstat",name:"foo",fact:"!ENOENT"},{ok,probes}=await hold(base,row,true);
 expect(probes.length).toBeGreaterThan(0);
 expect(ok).toBe(!fs.existsSync(join(base,"foo")));
});

test("a link is never answered from the listing",async()=>{
 const base=build({real:"file"});
 try{symlinkSync(join(base,"real"),join(base,"link"));}catch{return;}
 const{ok,probes}=await hold(base,{kind:"lstat",name:"link",fact:"symlink"},true);
 expect(ok).toBe(true);
 expect(probes.length).toBeGreaterThan(0);
});

test("one pass over a wide directory reads its listing once and probes none of its children",async()=>{
 const entries:Record<string,string>={};for(let at=0;at<300;at++)entries["n"+String(at).padStart(3,"0")]="file";
 const base=build(entries),reads:string[]=[],probes:string[]=[];
 const port=new Proxy(fs,{get:(source,key)=>key==="readdirSync"?(path:unknown,...rest:unknown[])=>{reads.push(String(path));return(source.readdirSync as Function)(path,...rest);}:probed.includes(key as string)?(path:unknown,...rest:unknown[])=>{if(String(path).startsWith(base+"\\n")||String(path).startsWith(base+"/n"))probes.push(String(path));return(source as never as Record<string,Function>)[key as string]!(path,...rest);}:(source as never)[key as never]});
 const rows=[...Object.keys(entries).flatMap(name=>[{kind:"lstat",path:join(base,name),fact:"file"},{kind:"fileExists",path:join(base,name),fact:true}]),{kind:"readdir",path:base,fact:directoryFactOfV1("readdir",base)}];
 const ok=await withObservedWorldV1(port as never,()=>observationsHold(rows as never,8));
 expect(ok).toBe(true);
 expect(probes).toEqual([]);
 expect(reads.filter(path=>path===base).length).toBe(1);
});
