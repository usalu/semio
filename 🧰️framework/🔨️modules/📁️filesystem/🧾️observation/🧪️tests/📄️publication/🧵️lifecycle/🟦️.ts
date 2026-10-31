import {lstatSync} from "node:fs";
/** 🧵️ Prepared publication data precedes effects and survives only exact qualified resumption. */
import {expect,test} from "bun:test";
import {mkdir,mkdtemp,readFile,writeFile,rm,lstat,readdir,open} from "node:fs/promises";
import {join,resolve} from "node:path";
import {sha256} from "@noble/hashes/sha2.js";
import Ajv from "ajv";
import * as descriptor from "../../../📁️descriptor/🟦️.ts";
import schema from "../../../📁️descriptor/🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
const hash=(bytes:Uint8Array)=>Buffer.from(sha256(bytes)).toString("hex");
const portable=(value:unknown)=>JSON.parse(JSON.stringify(value,(_key,item)=>typeof item==="bigint"?item.toString():item));
const absent=async(path:string)=>await lstat(path).then(()=>false,error=>{if(error.code!=="ENOENT")throw error;return true;});

test("prepared hook observes immutable actual bytes before every publication namespace effect",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit publication test output required");await mkdir(output,{recursive:true});
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);const validate=ajv.getSchema(schema.$id+"#/definitions/RelativePreparedFilePublication")!,corpus=ajv.getSchema(schema.$id+"#/definitions/RelativeFilePublicationLifecycleFixture")!;expect(corpus(fixture)).toBe(true);expect(new Set(fixture.cases.map(row=>row.id)).size).toBe(fixture.cases.length);
 for(const row of fixture.cases){const workspace=await mkdtemp(join(resolve(output),"lifecycle-")),root=join(workspace,"target"),name="dependencies.json",marker=Object.freeze(Error("actual prepared owner refusal"));await mkdir(root);await writeFile(join(root,"unrelated"),"untouched");if(row.prior!==null)await writeFile(join(root,name),row.prior);let stopped=false,hookCalls=0,getters=0,preparedName:string|undefined,failed=false,failure:unknown;
  try{await descriptor.withArtifactRelativeDirectoryV1(root,()=>{if(stopped)throw marker;},async directory=>{const expected=await directory.captureFilePublication(name,65536),prepared=async(event:any)=>{
   hookCalls++;preparedName=event.stage.name;expect(validate(portable(event))).toBe(true);expect(event.version).toBe(1);expect(event.platform).toBe(process.platform==="win32"?"windows":"posix");expect(event.authority).toBe(false);expect(event.name).toBe(name);expect(event.names).toEqual(row.before==="file"?[name,"unrelated"]:["unrelated"]);for(const value of [event,event.parent,event.stage,event.stage.identity,event.names,...(event.prior?[event.prior,event.prior.identity]:[])])expect(Object.isFrozen(value)).toBe(true);
   expect(await absent(join(root,name))).toBe(row.before==="missing");if(row.prior!==null)expect(await readFile(join(root,name),"utf8")).toBe(row.prior);expect(hash(await readFile(join(root,event.stage.name)))).toBe(hash(Buffer.from(row.bytes)));expect(event.stage.sha256).toBe(hash(Buffer.from(row.bytes)));expect(event.stage.identity.ino).toBe((await lstat(join(root,event.stage.name),{bigint:true})).ino);expect(event.parent.ino).toBe(directory.stat().ino);if(event.prior)expect(event.prior.sha256).toBe(hash(Buffer.from(row.prior!)));if(event.platform==="windows"&&event.retainedName)expect(await absent(join(root,event.retainedName))).toBe(true);
   const journal=await open(join(workspace,"intent.json"),"wx");try{await journal.writeFile(JSON.stringify(portable(event)));await journal.sync();}finally{await journal.close();}await new Promise<void>(done=>setImmediate(done));
   if(row.effect==="hook-refusal")throw marker;if(row.effect==="stage-changed")await writeFile(join(root,event.stage.name),"foreign stage");if(row.effect==="prior-changed")await writeFile(join(root,name),"foreign prior");if(row.effect==="roster-changed")await writeFile(join(root,"foreign"),"foreign");if(row.effect==="cancelled")stopped=true;
  };let hooks:any={prepared};if(row.effect==="options-proxy")hooks=new Proxy(hooks,{});if(row.effect==="options-getter")hooks=Object.defineProperty({},"prepared",{get(){getters++;return prepared;}});if(row.effect==="options-extra")hooks={prepared,foreign:true};if(row.effect==="options-inherited")hooks=Object.create(hooks);if(row.effect==="callback-proxy")hooks={prepared:new Proxy(prepared,{})};if(row.effect==="thenable-result")hooks={prepared:(event:any)=>{hookCalls++;preparedName=event.stage.name;return Object.defineProperty({},"then",{get(){getters++;return()=>{};}});}};
  if(row.effect==="extra-arity")await (directory.publishFile as any).call(directory,expected,Buffer.from(row.bytes),hooks,undefined);else if(row.effect==="undefined-hook")await (directory.publishFile as any).call(directory,expected,Buffer.from(row.bytes),undefined);else await (directory.publishFile as any).call(directory,expected,Buffer.from(row.bytes),hooks);
 });}catch(error){failed=true;failure=error;}
  try{expect(!failed,row.id).toBe(row.accepted);expect(hookCalls,row.id).toBe(row.hookCalls);expect(getters,row.id).toBe(0);if(row.effect==="hook-refusal"||row.effect==="cancelled")expect(failure,row.id).toBe(marker);if(row.accepted)expect(await readFile(join(root,name),"utf8")).toBe(row.bytes);else if(row.effect==="prior-changed")expect(await readFile(join(root,name),"utf8")).toBe("foreign prior");else if(row.prior!==null)expect(await readFile(join(root,name),"utf8")).toBe(row.prior);else expect(await absent(join(root,name))).toBe(true);expect(await readFile(join(root,"unrelated"),"utf8")).toBe("untouched");if(!row.hookCalls)expect((await readdir(root)).sort()).toEqual(row.before==="file"?[name,"unrelated"]:["unrelated"]);else if(!row.accepted)expect(preparedName&&await absent(join(root,preparedName))).toBe(false);
  }finally{await rm(workspace,{recursive:true,force:true});}
 }
},120_000);

test("fresh descriptor recovery retains genuine prepared and idempotent completed owners",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit recovery output required");await mkdir(output,{recursive:true});
 for(const prior of [null,"old 🌍"]){const workspace=await mkdtemp(join(resolve(output),"recovery-")),root=join(workspace,"target"),target="dependencies.json",bytes=Buffer.from("new 👩‍💻");await mkdir(root);await writeFile(join(root,"unrelated"),"untouched");if(prior!==null)await writeFile(join(root,target),prior);let prepared:any,data:any;
  try{
   await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.captureFilePublication(target,65536),plan=directory.filePublicationPlan(expected);expect(Object.isFrozen(plan)).toBe(true);expect(descriptor.artifactRelativeFilePublicationStageNameV1(target,plan.stageName)).toBe(true);expect(()=>directory.filePublicationPlan({...expected})).toThrow();prepared=await directory.prepareFilePublication(expected,bytes,{prepared:async event=>{expect(event.stage.name).toBe(plan.stageName);expect(event.retainedName).toBe(plan.retainedName);data=descriptor.artifactRelativeFilePublicationPreparedDataV1(event);const file=await open(join(workspace,"intent.json"),"wx");try{await file.writeFile(JSON.stringify(data));await file.sync();}finally{await file.close();}}});expect(prepared.stage.name).toBe(plan.stageName);});
   expect(await readFile(join(root,prepared.stage.name))).toEqual(bytes);if(prior===null)expect(await absent(join(root,target))).toBe(true);else expect(await readFile(join(root,target),"utf8")).toBe(prior);
   data=JSON.parse(await readFile(join(workspace,"intent.json"),"utf8"));await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{await directory.withNewFile("transition",async file=>{file.write(Buffer.from("ok"),0,2,0);file.sync();});const entry=await directory.entry("transition");await directory.remove("transition",entry!);await directory.sync();});let published:any;
   await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const token=await (directory as any).captureFileRecovery(data);expect(token.state).toBe("prepared");expect(token.authority).toBe(false);await expect(Promise.resolve().then(()=>(directory as any).recoverFile({...token}))).rejects.toThrow();published=await (directory as any).recoverFile(token);expect(published.sha256).toBe(hash(bytes));});
   expect(await readFile(join(root,target))).toEqual(bytes);const before=await lstat(join(root,target),{bigint:true});
   await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const token=await (directory as any).captureFileRecovery(data);expect(token.state).toBe(prior===null?"completed":"published-with-retained");const result=await (directory as any).recoverFile(token);expect(result.identity.ino).toBe(before.ino);expect(result.identity.ctimeNs).toBe(before.ctimeNs);});
   if(published.retained){await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const entry=await directory.entry(published.retained.name);expect(entry?.ino).toBe(published.retained.identity.ino);expect(hash(await readFile(join(root,published.retained.name)))).toBe(hash(Buffer.from(prior!)));await directory.remove(published.retained.name,entry!);await directory.sync();});}
   await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const token=await (directory as any).captureFileRecovery(data);expect(token.state).toBe("completed");const result=await (directory as any).recoverFile(token);expect(result.retained).toBeNull();});
   expect((await readdir(root)).sort()).toEqual([target,"unrelated"]);expect(await readFile(join(root,"unrelated"),"utf8")).toBe("untouched");
  }finally{await rm(workspace,{recursive:true,force:true});}
 }
},120_000);

test("portable recovery cut layouts match independent XState and JSON Patch",async()=>{
 const {default:jsonpatch}=await import("fast-json-patch"),{createMachine,createActor}=await import("xstate"),{default:fixture}=await import("./🪃️recovery.json"),{captureArtifactRelativeFilePublicationPreparedDataV1,classifyArtifactRelativeFileRecoveryV1}=await import("../../../📁️descriptor/📄️publication/🟦️.ts");
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.getSchema(schema.$id+"#/definitions/RelativeFilePublicationRecoveryFixture")!(fixture)).toBe(true);
 const windows=(ino:number)=>({volumeSerial:"1",fileId:ino.toString(16).padStart(32,"0"),attributes:ino===1?16:32,security:"1".repeat(64),streams:"2".repeat(64)}),identity=(ino:number,clock:number,root=false)=>({dev:"1",ino:String(ino),mode:root?"16877":"33152",size:root?"4096":"3",nlink:root?"2":"1",mtimeNs:String(clock),ctimeNs:String(clock)}),old={identity:identity(10,100),sha256:hash(Buffer.from("old"))},fresh={identity:identity(20,200),sha256:hash(Buffer.from("new"))},native=(facts:any)=>Object.fromEntries(Object.entries(facts).map(([key,value])=>[key,BigInt(value as string)])),leaf=(value:any)=>value===null?null:{identity:native(value.identity),sha256:value.sha256};
 for(const row of fixture.cases){const data:any={version:1,platform:row.platform,parent:identity(1,50,true),name:"meta.json",stage:{name:".abcdef12",...fresh},prior:row.prior?old:null,retainedName:row.prior?(row.platform==="posix"?".abcdef12":".123456ab"):null,names:row.prior?["meta.json","unrelated"]:["unrelated"],authority:false,windows:row.platform==="windows"?{parent:windows(1),stage:windows(20),prior:row.prior?windows(10):null}:null},prepared=captureArtifactRelativeFilePublicationPreparedDataV1(data),initial={parent:data.parent,names:row.prior?[".abcdef12","meta.json","unrelated"]:[".abcdef12","unrelated"],target:row.prior?old:null,stage:fresh,retained:row.platform==="posix"&&row.prior?fresh:null,windows:row.platform==="windows"?{parent:windows(1),target:row.prior?windows(10):null,stage:windows(20),retained:null}:null},current=jsonpatch.applyPatch(initial,row.patches as any,true,false).newDocument;
  const fault={target:"refused"},machine=createMachine({initial:"prepared",states:{prepared:{on:{DISPLACE:row.platform==="windows"&&row.prior?"old-displaced":"refused",PUBLISH:row.prior?"published-with-retained":"completed",FAULT:fault}}, "old-displaced":{on:{PUBLISH:"published-with-retained",FAULT:fault}},"published-with-retained":{on:{RETIRE:"completed",FAULT:fault}},completed:{on:{FAULT:fault}},refused:{}}}),actor=createActor(machine).start();for(const type of row.events)actor.send({type});const reference=actor.getSnapshot().value==="refused"?null:actor.getSnapshot().value;actor.stop();expect(reference,row.id).toBe(row.expected);
  if(reference===null)expect(()=>classifyArtifactRelativeFileRecoveryV1(prepared,{parent:native(current.parent),names:current.names,target:leaf(current.target),stage:leaf(current.stage),retained:leaf(current.retained),windows:current.windows} as any)).toThrow();else expect(reference,row.id).toBe(classifyArtifactRelativeFileRecoveryV1(prepared,{parent:native(current.parent),names:current.names,target:leaf(current.target),stage:leaf(current.stage),retained:leaf(current.retained),windows:current.windows} as any));
 }
});

test("interrupted namespace moves resume under a genuinely fresh owner",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit cut output required");await mkdir(output,{recursive:true});
 for(const cut of process.platform==="win32"?["old-displaced","published"]:["published"]){const workspace=await mkdtemp(join(resolve(output),"cut-")),root=join(workspace,"target"),name="dependencies.json",marker=Object.freeze(Error("actual namespace cut"));await mkdir(root);await writeFile(join(root,name),"old");let data:any,armed=false,stopped=false;
  try{await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.captureFilePublication(name,64);await directory.prepareFilePublication(expected,Buffer.from("new"),{prepared:event=>{data=descriptor.artifactRelativeFilePublicationPreparedDataV1(event);}});});
   const observed=()=>{if(!armed)return;const exists=(name:string)=>{try{return lstatSync(join(root,name),{bigint:true});}catch(error){if((error as any).code!=="ENOENT")throw error;return null;}};const target=exists(name),retained=exists(data.retainedName);if(cut==="old-displaced"?!target&&retained!==null:target?.ino===BigInt(data.stage.identity.ino)){stopped=true;throw marker;}};
   await expect(descriptor.withArtifactRelativeDirectoryV1(root,observed,async directory=>{const token=await directory.captureFileRecovery(data);armed=true;await directory.recoverFile(token);})).rejects.toBe(marker);expect(stopped).toBe(true);armed=false;
   await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const token=await directory.captureFileRecovery(data);expect(token.state).toBe(cut==="old-displaced"?"old-displaced":"published-with-retained");const result=await directory.recoverFile(token);expect(result.sha256).toBe(hash(Buffer.from("new")));expect(await readFile(join(root,result.retained!.name),"utf8")).toBe("old");});
   expect(await readFile(join(root,name),"utf8")).toBe("new");
  }finally{await rm(workspace,{recursive:true,force:true});}
 }
},120_000);

test("captured recovery token refuses drift and belongs only to its live owner",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit drift output required");await mkdir(output,{recursive:true});
 for(const mode of ["stage","prior","roster","own-parent","foreign-frame"]){const workspace=await mkdtemp(join(resolve(output),"drift-")),root=join(workspace,"target"),name="dependencies.json";await mkdir(root);await writeFile(join(root,name),"old");let data:any,foreign:any;
  try{await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.captureFilePublication(name,64);await directory.prepareFilePublication(expected,Buffer.from("new"),{prepared:event=>{data=descriptor.artifactRelativeFilePublicationPreparedDataV1(event);}});});
   if(mode==="foreign-frame"){await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{foreign=await directory.captureFileRecovery(data);});await expect(descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{await directory.recoverFile(foreign);})).rejects.toThrow();}
   else await expect(descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const token=await directory.captureFileRecovery(data);if(mode==="stage")await writeFile(join(root,data.stage.name),"foreign stage");if(mode==="prior")await writeFile(join(root,name),"foreign prior");if(mode==="roster")await writeFile(join(root,"foreign"),"foreign");if(mode==="own-parent"){await directory.withNewFile("drift",async file=>{file.sync();});const entry=await directory.entry("drift");await directory.remove("drift",entry!);}await directory.recoverFile(token);})).rejects.toThrow();
   expect(await readFile(join(root,name),"utf8")).toBe(mode==="prior"?"foreign prior":"old");expect(await absent(join(root,data.stage.name))).toBe(false);
  }finally{await rm(workspace,{recursive:true,force:true});}
 }
},120_000);

test("fresh metadata attestation binds the identical private entry through removal",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit attestation output required");await mkdir(output,{recursive:true});const workspace=await mkdtemp(join(resolve(output),"attest-")),name="metadata.json",bytes=Buffer.from("actual metadata 🌍");
 try{await writeFile(join(workspace,name),bytes);await descriptor.withArtifactRelativeDirectoryV1(workspace,()=>{},async directory=>{const entry=await directory.entry(name);await expect(Promise.resolve().then(()=>directory.attestFilePublication(name,{...entry!},bytes.length))).rejects.toThrow();await expect(Promise.resolve().then(()=>directory.attestFilePublication(name,entry!,bytes.length-1))).rejects.toThrow();const attestation=await directory.attestFilePublication(name,entry!,bytes.length);expect(attestation.authority).toBe(false);expect(Object.isFrozen(attestation)).toBe(true);expect(Object.isFrozen(attestation.identity)).toBe(true);expect(attestation.identity.ino).toBe(entry!.ino);expect(attestation.sha256).toBe(hash(bytes));expect(attestation.windows===null).toBe(process.platform!=="win32");await directory.remove(name,entry!);await directory.sync();});expect(await absent(join(workspace,name))).toBe(true);
  await writeFile(join(workspace,name),bytes);await expect(descriptor.withArtifactRelativeDirectoryV1(workspace,()=>{},async directory=>{const entry=await directory.entry(name);await directory.attestFilePublication(name,entry!,bytes.length);await writeFile(join(workspace,name),"foreign");await directory.remove(name,entry!);})).rejects.toThrow();expect(await readFile(join(workspace,name),"utf8")).toBe("foreign");
 }finally{await rm(workspace,{recursive:true,force:true});}
},120_000);

test("portable current-entry metadata cases match independent SHA256 and the closed schema",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit corpus attestation output required");await mkdir(output,{recursive:true});const ajv=new Ajv({strict:true});ajv.addSchema(schema);const validate=ajv.getSchema(schema.$id+"#/definitions/RelativeFilePublicationAttestation")!;
 for(const row of fixture.attestations){const root=await mkdtemp(join(resolve(output),"attest-case-")),name="metadata.json",bytes=Buffer.from(row.bytes);expect(hash(bytes),row.id).toBe(row.sha256);
  try{if(row.effect==="directory")await mkdir(join(root,name));else{await writeFile(join(root,name),bytes);if(row.effect==="alias")await (await import("node:fs/promises")).link(join(root,name),join(root,"alias"));}
   let failed=false;try{await descriptor.withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const entry=await directory.entry(name),value=row.effect==="copy"?{...entry!}:row.effect==="proxy"?new Proxy(entry!,{}):entry!,actualName=row.effect==="wrong-name"?"other.json":name;
    const result=row.effect==="extra-arity"?await (directory.attestFilePublication as any).call(directory,actualName,value,row.maximumBytes,undefined):await directory.attestFilePublication(actualName,value,row.maximumBytes);expect(validate(portable(result))).toBe(true);expect(result.sha256).toBe(row.sha256);expect(result.identity.ino).toBe(entry!.ino);expect(result.authority).toBe(false);
   });}catch(error){failed=true;if(row.accepted)throw error;}expect(!failed,row.id).toBe(row.accepted);expect(await absent(join(root,name))).toBe(false);
  }finally{await rm(root,{recursive:true,force:true});}
 }
},120_000);

test("portable stage plans and hostile durable DATA never mint authority or call getters",async()=>{
 const {captureArtifactRelativeFilePublicationPreparedDataV1}=await import("../../../📁️descriptor/📄️publication/🟦️.ts");
 for(const row of fixture.stageNames)expect(descriptor.artifactRelativeFilePublicationStageNameV1(row.target,row.candidate),row.target).toBe(row.accepted);
 const facts=(ino:string,directory=false)=>({dev:"1",ino,mode:directory?"16877":"33152",size:directory?"4096":"3",nlink:directory?"2":"1",mtimeNs:"1",ctimeNs:"1"}),base={version:1,platform:"posix",parent:facts("1",true),name:"meta.json",stage:{name:".abcdef12",identity:facts("2"),sha256:hash(Buffer.from("new"))},prior:null,retainedName:null,names:["unrelated"],authority:false,windows:null};
 expect(captureArtifactRelativeFilePublicationPreparedDataV1(base).stage.identity.ino).toBe(2n);
 for(const effect of fixture.dataRefusals){let value:any=JSON.parse(JSON.stringify(base)),getters=0;
  if(effect==="unknown-key")value.unknown=true;if(effect==="getter")Object.defineProperty(value,"stage",{get(){getters++;return base.stage;}});if(effect==="proxy")value=new Proxy(value,{});if(effect==="identity-getter")Object.defineProperty(value.stage.identity,"ino",{get(){getters++;return"2";}});if(effect==="identity-number")value.stage.identity.ino=2;if(effect==="noncanonical-decimal")value.stage.identity.ino="02";if(effect==="out-of-range")value.stage.identity.ino="9".repeat(40);if(effect==="sparse-names")delete value.names[0];if(effect==="names-proxy")value.names=new Proxy(value.names,{});if(effect==="unknown-symbol")value[Symbol("unknown")]=true;if(effect==="foreign-platform")value.platform="freebsd";if(effect==="unpaired-name"){value.name="\ud800";value.stage.name="a";}if(effect==="negative-zero")value.parent.mtimeNs="-0";
  expect(()=>captureArtifactRelativeFilePublicationPreparedDataV1(value),effect).toThrow();expect(getters,effect).toBe(0);
 }
});

test("same-entry journal append is bounded, durable, and preserves partial cancellation",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit append output required");await mkdir(output,{recursive:true});
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);const validate=ajv.getSchema(schema.$id+"#/definitions/RelativeFilePublicationAttestation")!;
 for(const row of fixture.appends){const root=await mkdtemp(join(resolve(output),"append-")),name="funding.json",before=Buffer.from(row.before),bytes=Buffer.from(row.bytes.repeat(row.repeat)),expected=Buffer.concat([before,bytes]);expect(hash(expected),row.id).toBe(row.sha256);await writeFile(join(root,name),before);let getters=0,armed=false,cut=false,failed=false;
  try{
   if(row.effect==="alias")await(await import("node:fs/promises")).link(join(root,name),join(root,"alias"));
   const check=()=>{if(armed&&row.effect==="cancel-after-write"&&lstatSync(join(root,name)).size>before.length){cut=true;throw Error("actual partial append cut");}};
   try{await descriptor.withArtifactRelativeDirectoryV1(root,check,async directory=>{
    const entry=await directory.entry(name),actual=row.effect==="copy"?{...entry!}:row.effect==="proxy-entry"?new Proxy(entry!,{}):entry!;
    if(row.effect==="entry-drift")await writeFile(join(root,name),"foreign");
    if(row.effect==="roster-drift")await writeFile(join(root,"foreign"),"foreign");
    let input:Uint8Array=row.effect==="proxy-bytes"?new Proxy(bytes,{}):bytes;
    if(row.effect==="bytes-getter")Object.defineProperty(input,"byteLength",{get(){getters++;return 0;}});
    armed=true;const pending=row.effect==="extra-arity"?(directory as any).appendFile(name,actual,input,row.maximumBytes,undefined):(directory as any).appendFile(name,actual,input,row.maximumBytes);
    if(row.effect==="mutate-bytes")bytes.fill(0);const result=await pending;armed=false;expect(validate(portable(result))).toBe(true);expect(result.sha256).toBe(row.sha256);expect(result.identity.ino).toBe(entry!.ino);expect(result.authority).toBe(false);expect(Object.isFrozen(result)).toBe(true);
   });}catch(error){failed=true;if(row.accepted)throw error;}
   armed=false;expect(!failed,row.id).toBe(row.accepted);expect(getters,row.id).toBe(0);const actual=await readFile(join(root,name));
   if(row.accepted)expect(actual,row.id).toEqual(expected);
   else if(row.effect==="cancel-after-write"){expect(cut).toBe(true);expect(actual.length).toBeGreaterThan(before.length);expect(actual.length).toBeLessThan(expected.length);expect(actual).toEqual(expected.subarray(0,actual.length));}
   else expect(actual).toEqual(row.effect==="entry-drift"?Buffer.from("foreign"):before);
  }finally{await rm(root,{recursive:true,force:true});}
 }
},120_000);
