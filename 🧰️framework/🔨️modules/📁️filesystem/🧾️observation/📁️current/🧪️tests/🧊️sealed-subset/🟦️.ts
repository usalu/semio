/** 🧊️ Contract: after a session sealed its inputs, a concurrent rewrite of a shared-lane input never fails the post-build recheck, while a rewrite of a claim still held live does; an independent Python sha256 validates the claimed digests. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,sep} from "node:path";
import {CurrentPhysicalFilesV1,CurrentPhysicalOwnerV1} from "../../🟦️.ts";
import type {FileObservationControlV1} from "../../../🟦️.ts";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const control=():FileObservationControlV1=>({maxBytes:64*1024*1024,maxWork:1_000_000,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>60_000,onProgress:()=>undefined});
const python=testPythonV1();
let root:string;
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"sealed-subset-")));});
afterAll(()=>rmSync(root,{recursive:true,force:true}));

test("a rewritten sealed input is outside the subset recheck, a rewritten live claim is refused",async()=>{
 const input=join(root,"input.rlib"),output=join(root,"output.rlib");writeFileSync(input,"input-v1");writeFileSync(output,"output-v1");
 const files=new CurrentPhysicalFilesV1(),owner=new CurrentPhysicalOwnerV1(root,control(),files);
 await owner.read(input);await owner.read(output);
 const oracle=Bun.spawnSync([python!,"-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",input]).stdout.toString().trim();
 expect(files.claims.get(input)!.sha256).toBe(oracle);
 writeFileSync(input,"input-v2-by-a-peer-lane");writeFileSync(output,"output-v2-by-a-peer-lane");
 await expect(files.recheck(control)).rejects.toThrow("before transfer");
 await files.subset(new Set()).recheck(control);
 await expect(files.subset(new Set([output])).recheck(control)).rejects.toThrow("before transfer");
 expect([...files.subset(new Set([input])).claims.keys()]).toEqual([input]);
});

test("the provenance session rechecks sealed inputs only at close, never again for a completed view",async()=>{
 const source=await Bun.file(join(import.meta.dir,"../../../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts")).text(),body=source.slice(source.indexOf("private async recheckCompletedFiles"));
 expect(body.slice(0,400)).toContain('const sealed=this.state==="closed"');
 expect(body.slice(0,2000)).toContain("shared.files.subset(");
 expect(body.slice(0,2000)).toContain("this.originalClaims.has(path)");
});

test("a peer adding a file to a sealed input directory passes, to a live output directory fails",async()=>{
 const sealedDirectory=join(root,"sealed","out"),liveDirectory=join(root,"live","out");mkdirSync(sealedDirectory,{recursive:true});mkdirSync(liveDirectory,{recursive:true});
 const input=join(sealedDirectory,"dep.rlib"),output=join(liveDirectory,"deliverable.wasm");writeFileSync(input,"dep");writeFileSync(output,"wasm");
 const owner=new CurrentPhysicalOwnerV1(root,control(),new CurrentPhysicalFilesV1());
 await owner.read(input);await owner.read(output);await owner.digest(sealedDirectory);await owner.digest(liveDirectory);
 const live=(path:string)=>output.startsWith(path+sep);
 await owner.recheckStructure();
 writeFileSync(join(sealedDirectory,"peer.rlib"),"added by a peer lane");
 await expect(owner.recheckStructure()).rejects.toThrow("roster changed");
 await owner.recheckStructure(live);
 writeFileSync(join(liveDirectory,"peer.wasm"),"added by a peer lane");
 await expect(owner.recheckStructure(live)).rejects.toThrow("roster changed");
});

test("the sealed recheck keeps rosters only of directories holding an original output claim",async()=>{
 const source=await Bun.file(join(import.meta.dir,"../../../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts")).text(),body=source.slice(source.indexOf("private async recheckCompletedFiles"),source.indexOf("private async recheckCompletedFiles")+2600);
 expect(body).toContain("shared.structures(sealed?");
 expect(body).toContain("this.originalClaims");
});
