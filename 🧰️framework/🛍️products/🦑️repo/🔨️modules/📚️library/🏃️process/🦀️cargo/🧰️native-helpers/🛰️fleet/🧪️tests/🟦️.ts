import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,cpSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetFetchContextV1} from "../../../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import {syncFleetIdentityV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {readSchedulerReceiptV1,schedulerReceiptFileV1,schedulerReceiptHoldsV1,schedulerIdentityV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🔖️scheduler/🟦️.ts";
import {SCHEDULER_SHARE_POLICY_V1,shareSchedulerBuildV1} from "../🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`sched-share-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const template=directory("identity");mkdirSync(join(template,".cache","f"),{recursive:true});writeFileSync(join(template,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:template,key:"k"}));
await syncFleetIdentityV1({root:template});
const crate=directory("crate");writeFileSync(join(crate,"Cargo.toml"),'[package]\nname="x"\n[[bin]]\nname="semio-build-scheduler"\npath="main.rs"\n');writeFileSync(join(crate,"Cargo.lock"),"# lock\n");writeFileSync(join(crate,"main.rs"),"fn main(){}\n");
const machine=(store:string)=>{
 const root=directory("machine");mkdirSync(join(root,".cache","f"),{recursive:true});cpSync(join(template,".cache","f","identity"),join(root,".cache","f","identity"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins","g1-m01"),{recursive:true});cpSync(join(template,".cache","f","identity","registration.json"),join(store,"v1","origins","g1-m01","fleet-auth.json"));
 const target=join(root,"target");mkdirSync(target,{recursive:true});
 return {root,target,context:fleetFetchContextV1({},root,{storeRoot:store,remote:undefined})};
};
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const run=(m:ReturnType<typeof machine>,counter:{builds:number},toolchain="tc1")=>shareSchedulerBuildV1({workspace:m.root,targetDirectory:m.target,toolchain,environment:{SEMIO_LANGUAGE:"en"},mode:"prototype",manifest:join(crate,"Cargo.toml"),context:m.context,say:()=>{},build:async()=>{
 counter.builds++;mkdirSync(join(m.target,"prototype"),{recursive:true});
 const out=new Map<string,string>();
 for(const name of SCHEDULER_SHARE_POLICY_V1.binaries){const path=join(m.target,"prototype",name+(process.platform==="win32"?".exe":""));writeFileSync(path,`#!/bin/sh\necho ${name} tc1\n`);chmodSync(path,0o755);out.set(name,path);}
 return out;
}});

test("a scheduler built on one machine is restored, not built, on a second machine of the same platform and keeps a valid local receipt",async()=>{
 const store=directory("store"),first=machine(store),second=machine(store),counter={builds:0};
 const built=await run(first,counter);expect(counter.builds).toBe(1);
 const restored=await run(second,counter);expect(counter.builds).toBe(1);
 for(const name of SCHEDULER_SHARE_POLICY_V1.binaries){expect(sha(restored.get(name)!)).toBe(sha(built.get(name)!));if(process.platform!=="win32")expect(statSync(restored.get(name)!).mode&0o111).not.toBe(0);}
 const receipt=readSchedulerReceiptV1(join(second.target,schedulerReceiptFileV1()));
 expect(receipt).toBeDefined();
 expect(schedulerReceiptHoldsV1(receipt,schedulerIdentityV1("tc1",{SEMIO_LANGUAGE:"en"}),join(crate,"Cargo.toml"))?.size).toBe(SCHEDULER_SHARE_POLICY_V1.binaries.length);
},120000);

test("a different compiler identity or changed source is never served a foreign scheduler",async()=>{
 const store=directory("store"),first=machine(store),second=machine(store),third=machine(store),counter={builds:0};
 await run(first,counter);await run(second,counter,"tc2");expect(counter.builds).toBe(2);
 writeFileSync(join(crate,"main.rs"),"fn main(){println!();}\n");await run(third,counter);expect(counter.builds).toBe(3);
 writeFileSync(join(crate,"main.rs"),"fn main(){}\n");
},120000);

test("without a fleet store the scheduler is simply built locally",async()=>{
 const m=machine(directory("store")),counter={builds:0};
 const alone=await shareSchedulerBuildV1({workspace:m.root,targetDirectory:m.target,toolchain:"tc1",environment:{SEMIO_LANGUAGE:"en"},mode:"prototype",manifest:join(crate,"Cargo.toml"),context:fleetFetchContextV1({},m.root,{storeRoot:undefined,remote:undefined}),say:()=>{},build:async()=>{counter.builds++;return new Map([["semio-build-scheduler","x"]]);}});
 expect(counter.builds).toBe(1);expect(alone.get("semio-build-scheduler")).toBe("x");
},60000);
