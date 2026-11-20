/** 🧩️ Tests for the component run collector. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {COMPONENT_RUN_POLICY_V1 as policy,collectComponentRunsV1,componentRunComponentsV1,componentRunRecencyV1,componentRunTextV1,planComponentRunsV1,type ComponentRunV1} from "../🟦️.ts";

const hour=3600000,ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(componentSchema,"runs.json");
const python=(states:readonly unknown[]):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify(states))});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString()) as unknown[];};
const run=(store:string,name:string,ageMs:number,components:readonly string[],bytes=65536):string=>{
 const path=join(store,policy.directory,name);for(const directory of["l","r","j"])mkdirSync(join(path,directory),{recursive:true});
 writeFileSync(join(path,"l","cargo.log"),"x".repeat(bytes));
 for(const component of components){mkdirSync(join(path,policy.materializedDirectory,component),{recursive:true});}
 const when=new Date(Date.now()-ageMs);for(const directory of["l","r","j",""])utimesSync(join(path,directory),when,when);utimesSync(join(path,"l","cargo.log"),when,when);utimesSync(path,when,when);
 return path;
};

test("the fixture is closed and the planner equals the independent Python oracle on every vector and on 400 seeded random states",()=>{
 const validate=ajv.compile({$ref:"runs.json#/definitions/Vector"});for(const vector of fixture.vectors)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.vectors)expect(planComponentRunsV1(vector.input as never) as unknown,vector.name).toEqual(vector.remove);
 expect(python(fixture.vectors.map(vector=>vector.input))).toEqual(fixture.vectors.map(vector=>vector.remove));
 let seed=20261119;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},states:{needBytes:number;runs:ComponentRunV1[]}[]=[];
 for(let index=0;index<400;index++){
  const runs:ComponentRunV1[]=[];
  for(let at=0;at<next(12);at++)runs.push({path:`/c/runs/w${at.toString(16).padStart(2,"0")}`,bytes:next(50)*1048576,lastUsedMs:next(6)*hour,components:["a","b","c"].filter(()=>next(3)===0),held:next(4)===0});
  states.push({needBytes:next(3)?next(400)*1048576:0,runs});
 }
 const expected=python(states);states.forEach((state,at)=>expect(planComponentRunsV1(state) as unknown,JSON.stringify(state).slice(0,200)).toEqual(expected[at]));
});

test("laws: a held run and the newest run of every component never go, and only what the need asks for is planned",()=>{
 let seed=7;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;};
 for(let index=0;index<300;index++){
  const runs:ComponentRunV1[]=[];for(let at=0;at<next(10);at++)runs.push({path:`/r/w${at}`,bytes:1+next(100),lastUsedMs:next(8),components:["x","y"].filter(()=>next(2)===0),held:next(3)===0});
  const need=next(300),plan=planComponentRunsV1({needBytes:need,runs}),gone=new Set(plan.map(row=>row.path));
  for(const row of runs){if(row.held)expect(gone.has(row.path)).toBe(false);}
  for(const component of["x","y"]){const owners=[...runs].filter(row=>!row.held&&row.components.includes(component)).sort((left,right)=>left.lastUsedMs-right.lastUsedMs||(left.path<right.path?-1:1));if(owners.length)expect(gone.has(owners.at(-1)!.path)).toBe(false);}
  const freed=plan.reduce((sum,row)=>sum+row.bytes,0);if(plan.length)expect(freed-plan.at(-1)!.bytes).toBeLessThan(need);
 }
});

test("a collection measures real run folders, keeps runs a build still writes to and the newest run of each component, and speaks English and German",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"runs-")),store=join(root,"m");
 try{
  const old=run(store,"wold",30*hour,["wfc","wfc.linked"]),mid=run(store,"wmid",20*hour,["wfc"]),empty=run(store,"wnone",25*hour,[]),latest=run(store,"wlatest",10*hour,["wfc.linked"]),live=run(store,"wlive",60000,["wfc"]),fresh=run(store,"wfresh",0,[]);
  expect(componentRunComponentsV1(old)).toEqual(["wfc"]);expect(Math.abs(Date.now()-componentRunRecencyV1(mid)-20*hour)).toBeLessThan(120000);
  const lines:string[]=[],dry=await collectComponentRunsV1(store,{dryRun:true,needBytes:()=>10**12,report:{language:"en",write:line=>lines.push(line)}});
  expect(dry.removed.map(row=>row.path)).toEqual([old,empty,mid]);expect([old,empty,mid,latest,live,fresh].every(existsSync)).toBe(true);expect(dry.held).toBe(2);expect(lines.some(line=>line.includes("would remove component run"))).toBe(true);
  const german:string[]=[],result=await collectComponentRunsV1(store,{dryRun:false,needBytes:()=>65536*2,report:{language:"de",write:line=>german.push(line)}});
  expect(result.removed.map(row=>row.path)).toEqual([old,empty]);expect([old,empty].some(existsSync)).toBe(false);expect([mid,latest,live,fresh].every(existsSync)).toBe(true);expect(german.some(line=>line.includes("entfernt"))).toBe(true);
  const none=await collectComponentRunsV1(store,{dryRun:false,needBytes:()=>0});expect(none.removed).toEqual([]);
  const satisfied=await collectComponentRunsV1(store,{dryRun:false,needBytes:()=>10**12,satisfied:()=>true});expect(satisfied.removed).toEqual([]);
  const rest=await collectComponentRunsV1(store,{dryRun:false,needBytes:()=>10**12});expect(rest.removed.map(row=>row.path)).toEqual([mid]);expect(existsSync(latest)&&existsSync(live)&&existsSync(fresh)).toBe(true);
  for(const language of["en","de"]as const)for(const key of Object.keys(componentSchema.definitions.Messages.const)as Array<keyof typeof componentSchema.definitions.Messages.const>)expect(componentRunTextV1(key,language,{path:"p",bytes:"1 MiB",root:"r",count:1,total:2,held:0,kept:1})).not.toMatch(/\{[a-zA-Z]+\}/u);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("targeted apply measures only eligible runs needed by the live target and reports completed removal",async()=>{
 const law=fixture.collection,validate=ajv.compile({$ref:"runs.json#/definitions/Collection"});
 expect(validate(law)).toBe(true);
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??realpathSync(tmpdir()),"run-progress-"));
 try{
  const rows=law.candidates.map(row=>({...row,path:run(root,row.name,row.ageMilliseconds,row.components)}));
  const now=Date.now(),oracle=python([{needBytes:law.needBytes,runs:rows.map(row=>({
   path:row.path,lastUsedMs:componentRunRecencyV1(row.path),components:row.components,
   held:row.ageMilliseconds<policy.heldMilliseconds,bytes:65536,
  }))}]) as {path:string}[][];
  expect(oracle[0]!.map(row=>basename(row.path))).toEqual(law.removed);
  const dry=await collectComponentRunsV1(root,{dryRun:true,needBytes:()=>law.needBytes,nowMs:now});
  const log=lstatSync(join(rows[0]!.path,"l","cargo.log"));
  expect(dry.freedBytes).toBe((log.blocks??Math.ceil(log.size/512))*512);
  expect(rows.every(row=>existsSync(row.path))).toBe(true);
  const measured=new Set<string>(),completed:string[]=[];
  let satisfied=false;
  const result=await collectComponentRunsV1(root,{
   dryRun:false,needBytes:()=>law.needBytes,satisfied:()=>satisfied,nowMs:now,
   progress:async(_done,_total,phase,path)=>{
    if(phase==="measure"&&path)measured.add(basename(path));
    if(phase==="remove"&&path){expect(existsSync(path)).toBe(false);completed.push(basename(path));satisfied=true;}
   },
  });
  expect([...measured]).toEqual(law.measured);
  expect(completed).toEqual(law.removed);
  expect(result.removed.map(row=>basename(row.path))).toEqual(law.removed);
  expect(rows.filter(row=>existsSync(row.path)).map(row=>row.name)).toEqual(rows.slice(1).map(row=>row.name));
  const changed=rows[1]!.path;
  const skipped=await collectComponentRunsV1(root,{
   dryRun:false,needBytes:()=>law.needBytes,nowMs:now,
   progress:async(_done,_total,phase,path)=>{
    if(phase==="measure"&&path===changed)utimesSync(join(changed,"l","cargo.log"),new Date(now),new Date(now));
   },
  });
  expect(skipped.removed).toEqual([]);
  expect(existsSync(changed)).toBe(true);
  const fallbackRows=law.candidates.slice(0,2).map(row=>run(root,row.name+"-fallback",row.ageMilliseconds,row.components));
  const fallback=await collectComponentRunsV1(root,{
   dryRun:false,needBytes:()=>law.needBytes,nowMs:now,satisfied:()=>undefined,
  });
  expect(fallback.removed.map(row=>row.path)).toEqual(fallbackRows.slice(0,1));
  expect(existsSync(fallbackRows[1]!)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
