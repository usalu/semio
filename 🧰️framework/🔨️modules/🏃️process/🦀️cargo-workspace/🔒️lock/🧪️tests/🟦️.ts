import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{spawn}from"node:child_process";
import Ajv from"ajv";
import{ensureCargoLockV1,cargoLockStaleOutputV1,cargoLockMissingCrateV1,CargoLockRefusedV1,type CargoLockOutcomeV1,type CargoLockPortV1,type CargoLockRunV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🧫️fixtures/🔣️.json";

type Expectation={outcomes?:CargoLockOutcomeV1[];cargoCalls?:number;oracle?:"locked-ok"|"locked-stale";refusedCrate?:string};
type LockCase={name:string;files:Readonly<Record<string,string>>;manifest?:string;online?:boolean;steps:readonly{do:string;files?:Readonly<Record<string,string>>;expect:Expectation}[]};
const cases=corpus.cases as unknown as readonly LockCase[];

const cargo=(args:readonly string[],cwd:string,env:Record<string,string>):Promise<CargoLockRunV1>=>new Promise(done=>{
 const child=spawn("cargo",[...args],{cwd,env:{...process.env,...env},stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";
 child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});child.on("close",status=>done({status,stdout,stderr}));
});
const lease=new Set<string>();
const portFor=(root:string,env:Record<string,string>,calls:{count:number},lines:string[]):CargoLockPortV1=>({
 async run(args){calls.count++;return cargo(args,root,env);},
 async tryHold(resource){if(lease.has(resource))return null;lease.add(resource);return{release:()=>{lease.delete(resource);}};},
 pause:milliseconds=>new Promise(done=>setTimeout(done,milliseconds)),
 report:line=>{lines.push(line);},
});
const oracle=async(root:string,manifest:string,env:Record<string,string>,expected:"locked-ok"|"locked-stale")=>{
 const run=await cargo(["metadata","--format-version=1","--locked","--offline","--no-deps","--manifest-path",manifest],root,env),probe=await cargo(["update","--workspace","--locked","--offline","--manifest-path",manifest],root,env);
 expect(run.status===0||expected==="locked-stale").toBe(true);expect(probe.status===0).toBe(expected==="locked-ok");
 if(expected==="locked-stale")expect(cargoLockStaleOutputV1(probe.stderr)).toBe(true);
};

test("the lock corpus is closed under its schema",()=>{for(const row of corpus.cases)expect(new Ajv({strict:true}).validate(schema.definitions.Case,row)).toBe(true);});

test("stale and missing-crate markers read the real Cargo diagnostics",()=>{
 expect(cargoLockStaleOutputV1("error: cannot update the lock file /x/Cargo.lock because --locked was passed to prevent this")).toBe(true);
 expect(cargoLockStaleOutputV1("error: cannot create the lock file /x/Cargo.lock because --locked was passed")).toBe(true);
 expect(cargoLockStaleOutputV1("error: no matching package named `axum` found")).toBe(false);
 expect(cargoLockMissingCrateV1("error: no matching package named `axum` found\nlocation searched: crates.io index")).toBe("axum");
 expect(cargoLockMissingCrateV1("error: failed to select a version for the requirement `tokio = \"^9\"`")).toBe("tokio");
 expect(cargoLockMissingCrateV1("error: something else")).toBeNull();
});

for(const row of cases)test("Cargo lock currency: "+row.name,async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-lock-")),home=join(root,".home"),receipts=join(root,".receipts"),env={CARGO_HOME:home,CARGO_TARGET_DIR:join(root,".target")},calls={count:0},lines:string[]=[];
 const write=(files:Readonly<Record<string,string>>)=>{for(const[name,body]of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),body);}};
 const manifest=join(root,row.manifest??"Cargo.toml"),online=row.online===true;
 const request=(force=false)=>({workspaceRoot:root,manifest,receiptDirectory:receipts,language:"en" as const,offline:!online,force});
 try{
  mkdirSync(home,{recursive:true});write(row.files);
  for(const step of row.steps){
   const expect_=step.expect;
   if(step.do==="write")write(step.files!);
   else{
    calls.count=0;
    if(expect_.refusedCrate){
     const failure=await ensureCargoLockV1(request(),portFor(root,env,calls,lines)).then(()=>null,error=>error);
     expect(failure).toBeInstanceOf(CargoLockRefusedV1);expect(failure.crate).toBe(expect_.refusedCrate);
     expect(failure.texts.en).toContain(expect_.refusedCrate);expect(failure.texts.de).toContain(expect_.refusedCrate);expect(failure.texts.en).not.toBe(failure.texts.de);
    }else{
     const outcomes=step.do==="ensure-concurrent"?await Promise.all([0,1,2,3].map(()=>ensureCargoLockV1(request(),portFor(root,env,calls,lines)))):[await ensureCargoLockV1(request(step.do==="ensure-force"),portFor(root,env,calls,lines))];
     expect([...outcomes].sort()).toEqual([...expect_.outcomes!].sort());
    }
    if(expect_.cargoCalls!==undefined)expect(calls.count).toBe(expect_.cargoCalls);
   }
   if(expect_.oracle)await oracle(root,manifest,env,expect_.oracle);
  }
  if(!row.steps.some(step=>step.expect.refusedCrate))expect(lines.some(line=>line.includes("Cargo.lock"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
