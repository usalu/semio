import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {acquireQueuedResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import * as code from "../../🟦️.ts";

const leasesModule=fileURLToPath(new URL("../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts",import.meta.url));
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const world=()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-code-")),names=["SEMIO_LEASE_ORDER","SEMIO_LEASE_ORDER_POLICY","SEMIO_LEASE_REGISTRY_DIRECTORY"] as const,saved=names.map(name=>process.env[name]);
 process.env.SEMIO_LEASE_ORDER="enforce";delete process.env.SEMIO_LEASE_ORDER_POLICY;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"registry");
 const leases=join(root,"leases"),namespace=join(root,"rc"),directory=join(namespace,"a".repeat(64));mkdirSync(directory,{recursive:true});
 return{root,leases,namespace,directory,restore:()=>{names.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});rmSync(root,{recursive:true,force:true});}};
};
const peerReader=async(leases:string,resource:string)=>{
 const child=spawn(process.execPath,["-e",`const m=await import(${JSON.stringify(leasesModule)});const l=await m.acquireQueuedResourceLease({directory:process.argv[1],resource:process.argv[2],mode:"shared",owner:"peer-build",signal:new AbortController().signal});process.stdout.write("held\\n");process.stdin.resume();process.stdin.on("end",()=>{l.release();process.exit(0);});`,leases,resource],{stdio:["pipe","pipe","inherit"],env:process.env});
 await new Promise<void>((done,fail)=>{child.stdout!.on("data",chunk=>{if(String(chunk).includes("held"))done();});child.once("exit",code=>fail(Error("peer reader exited "+code)));});
 return{stop:()=>new Promise<void>(done=>{child.once("exit",()=>done());child.stdin!.end();})};
};

test("A publisher whose content-addressed code directory a peer already published and now reads for its whole build claims no writer and does not wait for that build",async()=>{
 const w=world(),signal=new AbortController().signal;
 try{
  writeFileSync(join(w.directory,"entry.mjs"),"export{}\n");writeFileSync(join(w.directory,"receipt.json"),"{}\n");
  const peer=await peerReader(w.leases,"rust-code:"+w.directory);
  try{
   const publication=await acquireQueuedResourceLease({directory:w.leases,resource:"rust-code-publication:"+w.namespace,mode:"exclusive",owner:"law",signal});
   try{
    expect(typeof (code as Record<string,unknown>).claimRustUnitCodeWriterV1,"the publisher claims its writer through one function").toBe("function");
    const complete=async()=>sha("export{}\n")===sha("export{}\n");
    const started=Date.now(),writer=await code.claimRustUnitCodeWriterV1(w.leases,w.directory,complete,{signal});
    expect(writer,"a complete directory held by a peer's reader needs no writer").toBeUndefined();
    expect(Date.now()-started,"and the claim does not wait for the peer's build").toBeLessThan(5000);
   }finally{publication.release();}
  }finally{await peer.stop();}
 }finally{w.restore();}
},60000);

test("The writer of an incomplete directory still waits for the reader in the queue and is granted when it ends",async()=>{
 const w=world(),signal=new AbortController().signal;
 try{
  const peer=await peerReader(w.leases,"rust-code:"+w.directory);
  let granted=false;
  const claim=code.claimRustUnitCodeWriterV1(w.leases,w.directory,async()=>false,{signal}).then(writer=>{granted=true;return writer;});
  await new Promise(done=>setTimeout(done,1500));
  expect(granted,"the writer waits while the reader holds an incomplete directory").toBe(false);
  await peer.stop();
  const writer=await claim;expect(writer).toBeDefined();writer!.release();
 }finally{w.restore();}
},60000);

test("Characterization of the retired protocol: a publication holder that queues for the writer of a directory a peer reads is refused after the hold-and-wait grace",async()=>{
 const w=world(),signal=new AbortController().signal;
 try{
  const peer=await peerReader(w.leases,"rust-code:"+w.directory);
  try{
   const publication=await acquireQueuedResourceLease({directory:w.leases,resource:"rust-code-publication:"+w.namespace,mode:"exclusive",owner:"law",signal});
   try{await expect(acquireQueuedResourceLease({directory:w.leases,resource:"rust-code:"+w.directory,mode:"exclusive",owner:"law",signal})).rejects.toMatchObject({name:"LeaseOrderErrorV1",violation:{kind:"hold-and-wait",heldClass:"namespace",requestedClass:"pin"}});}
   finally{publication.release();}
  }finally{await peer.stop();}
 }finally{w.restore();}
},90000);
