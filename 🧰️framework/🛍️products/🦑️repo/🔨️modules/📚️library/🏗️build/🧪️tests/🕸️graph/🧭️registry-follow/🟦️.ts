/** 🧭️ Contract: a peer that creates, removes or moves an owner while the graph is read is a later change; the read follows the registry (bounded) and only a tree that is inconsistent at the final check is refused, in English and German. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {Database} from "bun:sqlite";
import {withImplementationPinV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {readBuildOwnerRegistrySourceV1} from "../../../🕸️graph/📥️inference/🟨️.mjs";
import {settleOwnerRegistryV1,withOwnerRegistryCustodyV1} from "../../../🕸️graph/📥️load/🧭️registry-follow/🟦️.ts";

import {createScriptProcessEnvelope,withIssuedResourceControlV1,withIssuedResourceOperationV1,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";

const row=(name:string,root:string)=>({name,root,manifest:root+"/Cargo.toml",packages:[]});
const refusal=(root:string)=>`Native dependency has no build project owner: ${root} (required by dependent:build; candidates=4785)`;
const ownerFault=(root:string)=>Object.assign(Error(refusal(root)),{code:"BUILD_SOURCE_AUTHORITY"});
const completedCustody=<T>(body:()=>Promise<T>)=>()=>withOwnerRegistryCustodyV1<T>(new AbortController().signal,contain=>contain(body));
const lab=(owners:object[],files:Record<string,string>={})=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"registry-follow-")),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 write("📋️build-owners.json",JSON.stringify({version:1,owners,catalogProviders:[]}));
 for(const[path,text]of Object.entries(files))write(path,text);
 return{root,write,registry:()=>JSON.parse(readFileSync(join(root,"📋️build-owners.json"),"utf8")).owners.map((owner:{root:string})=>owner.root) as string[],put:(owners:object[])=>write("📋️build-owners.json",JSON.stringify({version:1,owners,catalogProviders:[]}))};
};

for(const pinned of [false,true])test(`follows an owner registered after enumeration (inherited pin: ${pinned})`,async()=>{
 const l=lab([row("@t/a","a/rust")]);
 try{
  let calls=0;const seen:string[][]=[];
  const updatedRegistry=[row("@t/a","a/rust"),row("@t/new","m/rust")];
  const read=()=>settleOwnerRegistryV1(l.root,completedCustody(async()=>{
   calls++;
   seen.push(JSON.parse(readBuildOwnerRegistrySourceV1(l.root)).owners.map((owner:{root:string})=>owner.root));
   if(calls===1){l.put(updatedRegistry);throw ownerFault("m/rust");}
   return "sealed";
  }));
  const value=await (pinned?withImplementationPinV1(read):read());
  const reference=new Database(":memory:");
  try{
   const expected=reference.query("select json_group_array(json_extract(value, '$.root')) as roots from json_each(?1)").get(JSON.stringify(updatedRegistry)) as {roots:string};
   expect(JSON.stringify(seen[1])).toBe(expected.roots);
  }finally{reference.close();}
  expect(value).toBe("sealed");expect(calls).toBe(2);expect(seen[1]).toEqual(["a/rust","m/rust"]);
 }finally{rmSync(l.root,{recursive:true,force:true});}
});

test("a crate with a manifest and no row is adopted by the reconciler and the read goes on",async()=>{
 const l=lab([row("@t/a","a/rust")],{"m/rust/Cargo.toml":"[package]\nname = \"m-crate\"\n"});
 try{
  let calls=0;
  expect(await settleOwnerRegistryV1(l.root,completedCustody(async()=>{if(++calls===1)throw ownerFault("m/rust");return calls;}))).toBe(2);
  expect(l.registry()).toContain("m/rust");
 }finally{rmSync(l.root,{recursive:true,force:true});}
});

test("an owner a peer removed or renamed meanwhile is followed; the vanished-manifest refusal too",async()=>{
 const l=lab([row("@t/a","a/rust"),row("@t/gone","g/rust")]);
 try{
  let calls=0;
  expect(await settleOwnerRegistryV1(l.root,completedCustody(async()=>{if(++calls===1){l.put([row("@t/a","a/rust")]);throw Object.assign(Error("Native execution contract requires a native project owner: dependent:build; Build owner manifest absent: g/rust/Cargo.toml (required by dependent:build)"),{code:"BUILD_OWNER_MANIFEST_ABSENT"});}return "ok";}))).toBe("ok");
  calls=0;
  expect(await settleOwnerRegistryV1(l.root,completedCustody(async()=>{if(++calls===1){l.put([row("@t/a","a/rust"),row("@t/moved","h/rust")]);throw ownerFault("g/rust");}return "ok";}))).toBe("ok");
 }finally{rmSync(l.root,{recursive:true,force:true});}
});

test("only a tree that stays inconsistent is refused, typed, English and German, naming the roots",async()=>{
 const l=lab([row("@t/a","a/rust")]);
 try{
  let calls=0;
  const error=await settleOwnerRegistryV1(l.root,completedCustody(async()=>{calls++;throw ownerFault("m/rust");})).catch(value=>value);
  expect(calls).toBe(1);expect(error.code).toBe("BUILD_OWNER_REGISTRY_INCONSISTENT");expect(error.roots).toEqual(["m/rust"]);
  expect(error.message).toContain("inconsistent");expect(error.message).toContain("inkonsistent");expect(error.message).toContain("m/rust");expect(error.cause.message).toBe(refusal("m/rust"));
  calls=0;
  const churn=await settleOwnerRegistryV1(l.root,completedCustody(async()=>{calls++;l.put([row("@t/a","a/rust"),row("@t/c"+calls,"c"+calls+"/rust")]);throw ownerFault("m/rust");})).catch(value=>value);
  expect(calls).toBe(5);expect(churn.code).toBe("BUILD_OWNER_REGISTRY_INCONSISTENT");
 }finally{rmSync(l.root,{recursive:true,force:true});}
});

test("any other failure, and a cancelled read, passes through untouched",async()=>{
 const l=lab([row("@t/a","a/rust")]);
 try{
  const other=Error("disk full");
  expect(await settleOwnerRegistryV1(l.root,async()=>{throw other;}).catch(value=>value)).toBe(other);
  const cancelled=ownerFault("m/rust");
  expect(await settleOwnerRegistryV1(l.root,completedCustody(async()=>{throw cancelled;}),{aborted:()=>true}).catch(value=>value)).toBe(cancelled);
 }finally{rmSync(l.root,{recursive:true,force:true});}
});


const issued=()=>createScriptProcessEnvelope({version:1,owner:"registry-follow-fixture",maximumElapsedMilliseconds:5000},{},Date.now());
const manifest="[package]\nname = \"m-crate\"\n";

test("a registry fault settles its issued physical attempt before reconciliation and admits a fresh attempt",async()=>{
 const l=lab([row("@t/a","a/rust")],{"m/rust/Cargo.toml":manifest}),oracle=new Database(":memory:");
 oracle.run("create table evidence (ordinal integer primary key, event text)");
 let calls=0,seen:string[]=[];
 const append=(event:string)=>{seen.push(event);return event;};
 try{
  const result=await withScriptProcessEnvelope(issued(),invocation=>withIssuedResourceControlV1(invocation,"fixture:graph",5000,control=>{
   return settleOwnerRegistryV1(l.root,()=>withOwnerRegistryCustodyV1<number>(control.signal,contain=>withIssuedResourceOperationV1(control,async()=>{
    const ordinal=++calls;
    try{return await contain(async()=>{append("attempt "+ordinal);if(ordinal===1)throw ownerFault("m/rust");expect(l.registry()).toContain("m/rust");return ordinal;});}
    finally{await Promise.resolve();append("release "+ordinal);}
   })),{say:()=>append("registry followed")});
  }));
  expect(result).toBe(2);
  oracle.run("insert into evidence values(1,'attempt 1'),(2,'release 1'),(3,'registry followed'),(4,'attempt 2'),(5,'release 2')");
  const expected=oracle.query("select event from evidence order by ordinal").all() as {event:string}[];
  expect(seen).toEqual(expected.map(row=>row.event));
 }finally{oracle.close();rmSync(l.root,{recursive:true,force:true});}
});

test("unexpected, unsettled and cancelled custody are never promoted into registry retries",async()=>{
 for(const fault of ["raw","cleanup","cancel","untyped"]){
  const l=lab([row("@t/a","a/rust")],{"m/rust/Cargo.toml":manifest}),source=ownerFault("m/rust"),cleanup=new Error("Fixture store close failed"),cancellation=new Error("Fixture caller cancelled"),untyped=new Error(source.message),localStop=new AbortController();
  let calls=0;
  try{
   const run=withScriptProcessEnvelope(issued(),invocation=>withIssuedResourceControlV1(invocation,"fixture:graph",5000,control=>settleOwnerRegistryV1(l.root,async()=>{
    calls++;
    if(fault==="raw")return withIssuedResourceOperationV1(control,async()=>{throw source;});
    return withOwnerRegistryCustodyV1(AbortSignal.any([control.signal,localStop.signal]),contain=>withIssuedResourceOperationV1(control,async()=>{
     try{return await contain(async()=>{throw fault==="untyped"?untyped:source;});}
     finally{if(fault==="cleanup")throw cleanup;if(fault==="cancel")localStop.abort(cancellation);}
    }));
   })));
   await expect(run).rejects.toBe(fault==="cleanup"?cleanup:fault==="cancel"?cancellation:fault==="untyped"?untyped:source);
   expect(calls,fault).toBe(1);expect(l.registry(),fault).toEqual(["a/rust"]);
  }finally{rmSync(l.root,{recursive:true,force:true});}
 }
});
