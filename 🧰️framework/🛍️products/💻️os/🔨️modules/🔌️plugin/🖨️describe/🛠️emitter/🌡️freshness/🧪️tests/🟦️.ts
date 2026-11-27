/** 🌡️ Contract: an emitter built from one checkout snapshot is judged against the files of the generation it is compared with, never against paths that no longer exist and never as falsely fresh. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {checkoutSnapshotRelativeV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🟦️.ts";
import {descriptorEmitterRecordedPathV1,staleDescriptorEmitterCargoInputsV1,staleDescriptorEmitterInputsV1} from "../🟦️.ts";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("a path recorded below any snapshot root is named by its workspace-relative part, every other path is not",()=>{
 expect(checkoutSnapshotRelativeV1("/m/.cache/s/0123456789ab/a/b.rs")).toBe("a/b.rs");
 expect(checkoutSnapshotRelativeV1("C:\\m\\.cache\\s\\0123456789ab\\a\\b.rs")).toBe("a/b.rs");
 expect(checkoutSnapshotRelativeV1("/m/a/s/0123456789/b.rs")).toBeUndefined();
 expect(checkoutSnapshotRelativeV1("/m/a/b.rs")).toBeUndefined();
 expect(descriptorEmitterRecordedPathV1("/live","/live/a.rs")).toBe("/live/a.rs");
 expect(descriptorEmitterRecordedPathV1("/live","/x/.cache/s/0123456789ab/a/b.rs")).toBe("/live/a/b.rs");
 expect(descriptorEmitterRecordedPathV1("/live","/elsewhere/c.rs")).toBe("/elsewhere/c.rs");
 expect(descriptorEmitterRecordedPathV1("/live","/live/.cache/s/0123456789ab/a/b.rs")).toBe("/live/a/b.rs");
 expect(descriptorEmitterRecordedPathV1("/live/.cache/s/fedcba987654","/live/.cache/s/0123456789ab/a/b.rs")).toBe("/live/.cache/s/fedcba987654/a/b.rs");
 expect(descriptorEmitterRecordedPathV1("/snap","/live/a/b.rs","/live")).toBe("/snap/a/b.rs");
 expect(descriptorEmitterRecordedPathV1("/snap","/live/.cache/b/c","/live")).toBe("/live/.cache/b/c");
 expect(descriptorEmitterRecordedPathV1("/snap","/other/x","/live")).toBe("/other/x");
});

test("an emitter recorded from an older snapshot is fresh for a generation whose files are equal and stale for the files that differ",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"fr-")));
 try{
  mkdirSync(join(root,"a"),{recursive:true});writeFileSync(join(root,"a","same.rs"),"same");writeFileSync(join(root,"a","moved.rs"),"new");
  const old="/state/.cache/s/0123456789ab",receipt=JSON.stringify({strings:[old+"/a/same.rs",old+"/a/moved.rs",old+"/gone.rs"],inputs:[[0,"file",sha("same")],[1,"file",sha("old")],[2,"file",sha("x")]]});
  expect(await staleDescriptorEmitterInputsV1(receipt,root)).toEqual({checked:3,stale:["a/moved.rs","gone.rs"]});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the Cargo lock and manifest of an emitter built in the live tree or in an earlier snapshot are found in the snapshot being compared",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"fc-")));
 try{
  writeFileSync(join(root,"Cargo.lock"),"lock");writeFileSync(join(root,"Cargo.toml"),"[package]\nname=\"e\"\nversion=\"0.1.0\"\n");
  for(const recorded of ["/live/m",  "/state/.cache/s/0123456789ab"]){
   const observation={manifest:recorded+"/Cargo.toml",args:["build","--manifest-path",recorded+"/Cargo.toml"],units:[],invocationInputs:[{path:recorded+"/Cargo.lock",kind:"file",sha256:sha("lock")},{path:recorded+"/Cargo.toml",kind:"file",sha256:sha("[package]\nname=\"e\"\nversion=\"0.1.0\"\n")}]} as never;
   const retained={origin:{cargoHome:"/home"},files:new Map()};
   expect(await staleDescriptorEmitterCargoInputsV1(observation,root,retained,undefined,"/live/m")).toEqual({stale:[]});
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
