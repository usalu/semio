import{createIssuedResourceControlV1 as artifactEntryControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {resourceLeasePathV1} from "@semio-tech/process/leases";
import {captureOwnedCommandV1} from "@semio-tech/process/execution";
import {test,expect,spyOn} from "bun:test";
import * as leases from "@semio-tech/process/leases";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,lstatSync,statfsSync,existsSync,rmSync,utimesSync} from "node:fs";
import {join} from "node:path";
import {scriptInvocationEnvironment} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "@semio-tech/process/artifacts/native-build/storage/image";
import {repositoryLeaseDirectoryV1} from "../../../📥️opening/🟦️.ts";
import {withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1} from "@semio-tech/repo-test/storage";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
const sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");

test("portable graph headroom request is bounded before physical work",async()=>{
 const api=await import("../🟦️.ts"),oracle=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);
 try{for(const row of fixture.requests){let accepted=true;try{api.readBuildGraphHeadroomRequestV1(row.value);}catch{accepted=false;}expect(accepted).toBe(row.accepted);expect(validate(row.value)).toBe(row.accepted);const independent=oracle.query("select json_type(?1,'$.wait') in ('true','false') and (select count(*) from json_each(?1))=2 and json_type(?1,'$.minimumAvailableBytes')='integer' and json_extract(?1,'$.minimumAvailableBytes') between 0 and 2684354560 admitted").get(JSON.stringify(row.value))as{admitted:number|null};expect(Boolean(independent.admitted)).toBe(row.accepted);}}finally{oracle.close();}
});

test.skipIf(process.platform!=="darwin")("actual graph headroom pressure, reader, integrity and deadline custody",async()=>withBuildTestInvocationV1(async original=>{for(const scenario of["pressure","reader","queued","queued-cancel","corrupt","invalid-request"]as const)await withRepositoryTestArtifactsV1(process.cwd(),original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{
 const api=await import("../🟦️.ts"),row=fixture.physical,parent=mkdtempSync(join(repositoryTestArtifactRootV1(process.cwd()),"gh-")),directory=join(parent,"i"),leaseDirectory=repositoryLeaseDirectoryV1(process.cwd()),control=scriptCompilerAdmissionControlV1(original,"graph:headroom:"+scenario,60000),options={directory,leaseDirectory,filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,control};let image=await openNativeImageStorageV1(options),reader:typeof image|undefined;
 const root=image.root,snapshots=join(root,"snapshots"),unknown=join(root,"unknown.input"),canonical=join(root,"canonical.json"),authority=join(root,"authority.json"),paths:string[]=[];
 try{
  mkdirSync(snapshots);for(let i=0;i<2;i++){const bytes=Buffer.alloc(row.snapshotBytes,i+1),path=join(snapshots,sha(bytes)+".json");writeFileSync(path,bytes);utimesSync(path,new Date(1000+i),new Date(1000+i));paths.push(path);expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(sha(bytes));}
  const source=Buffer.alloc(row.canonicalBytes,32);writeFileSync(canonical,source);writeFileSync(authority,JSON.stringify({version:3,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},membership:"c".repeat(64),graph:sha(source),observations:[]}));writeFileSync(unknown,Buffer.alloc(row.unknownBytes,73));const old=lstatSync(unknown),before=statfsSync(root),minimumAvailableBytes=before.bavail*before.bsize+row.requiredRecoveryBytes;
  if(scenario==="corrupt")writeFileSync(paths[0]!,Buffer.alloc(row.snapshotBytes,17));
  if(scenario==="reader"||scenario==="queued"||scenario==="queued-cancel"){reader=await openNativeImageStorageV1(options);const probe="const{DatabaseSync}=require('node:sqlite'),db=new DatabaseSync(process.argv[1]);let blocked=false;try{db.exec('pragma busy_timeout=0;begin exclusive;rollback');}catch(e){if(e.code!=='ERR_SQLITE_ERROR'||!String(e).includes('locked'))throw e;blocked=true;}finally{db.close();}console.log(JSON.stringify({blocked}));",output=await captureOwnedCommandV1("node",["--disable-warning=ExperimentalWarning","-e",probe,resourceLeasePathV1(leaseDirectory,"native-fixed-image:"+directory)],process.cwd(),{control,env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:8192,maximumLines:64}});expect(JSON.parse(output.trim())).toEqual({blocked:true});}
  await image.close();
  if(scenario==="invalid-request"){await expect(api.recoverBuildGraphImageHeadroomV1(options,{minimumAvailableBytes:Number.NaN,wait:false})).rejects.toThrow("Closed graph headroom request required");expect(paths.every(existsSync)).toBe(true);const expired=scriptCompilerAdmissionControlV1(original,"graph:expired-headroom",1);await Bun.sleep(5);await expect(api.recoverBuildGraphImageHeadroomV1({...options,control:expired},{minimumAvailableBytes,wait:false})).rejects.toThrow();expect(paths.every(existsSync)).toBe(true);}
  else if(scenario==="corrupt"){await expect(api.recoverBuildGraphImageHeadroomV1(options,{minimumAvailableBytes,wait:false})).rejects.toThrow("Graph snapshot content identity changed");expect(paths.every(existsSync)).toBe(true);expect(existsSync(canonical)).toBe(true);writeFileSync(paths[0]!,Buffer.alloc(row.snapshotBytes,1));const receipt=readFileSync(authority);writeFileSync(authority,JSON.stringify({...JSON.parse(receipt.toString()),graph:"d".repeat(64)}));await expect(api.recoverBuildGraphImageHeadroomV1(options,{minimumAvailableBytes,wait:false})).rejects.toThrow("Current graph publication identity changed");expect(paths.every(existsSync)).toBe(true);writeFileSync(authority,receipt);for(let i=0;i<4097;i++)writeFileSync(join(snapshots,"unknown-"+i),"");await expect(api.recoverBuildGraphImageHeadroomV1(options,{minimumAvailableBytes,wait:false})).rejects.toThrow("Bounded graph snapshot inventory required");expect(paths.every(existsSync)).toBe(true);}
  else if(scenario==="queued"||scenario==="queued-cancel"){
   const abort=new AbortController(),waiting=artifactEntryControlV1(original,"graph:headroom:"+scenario,60000,abort.signal),actual=leases.acquireQueuedResourceLease,waits:string[]=[],observer=spyOn(leases,"acquireQueuedResourceLease").mockImplementation(request=>actual(request.mode==="exclusive"?{...request,onWait:async event=>{await request.onWait?.(event);waits.push(event.resource);}}:request));
   try{let settled=false;const pending=api.recoverBuildGraphImageHeadroomV1({...options,control:waiting},{minimumAvailableBytes,wait:true}).finally(()=>{settled=true;});void pending.catch(()=>{});await Bun.sleep(1500);expect(settled).toBe(false);expect(paths.every(existsSync)).toBe(true);expect(existsSync(canonical)).toBe(true);expect(waits.length).toBeGreaterThan(0);
    if(scenario==="queued"){await reader!.close();reader=undefined;const result=await pending;expect(result.acquired).toBe(true);expect(result.availableBytes).toBeGreaterThanOrEqual(minimumAvailableBytes);expect(paths.some(existsSync)).toBe(false);expect(existsSync(canonical)).toBe(false);expect(result.retiredFiles).toBe(4);}
    else{abort.abort(Object.freeze(Error("Owned graph headroom cancellation")));await expect(pending).rejects.toThrow("Owned graph headroom cancellation");expect(paths.every(existsSync)).toBe(true);expect(existsSync(canonical)).toBe(true);expect(existsSync(authority)).toBe(true);}
   }finally{observer.mockRestore();}
  }
  else{const result=await api.recoverBuildGraphImageHeadroomV1(options,{minimumAvailableBytes,wait:false});expect(result.acquired).toBe(scenario!=="reader");if(scenario==="reader"){expect(paths.every(existsSync)).toBe(true);expect(existsSync(canonical)).toBe(true);}else{expect(result.availableBytes).toBeGreaterThanOrEqual(minimumAvailableBytes);expect(paths.some(existsSync)).toBe(false);expect(existsSync(canonical)).toBe(false);expect(existsSync(authority)).toBe(false);expect(result.retiredFiles).toBe(4);}}
  expect(lstatSync(unknown).ino).toBe(old.ino);expect(readFileSync(unknown)).toEqual(Buffer.alloc(row.unknownBytes,73));
 }finally{await reader?.close();await image.close();await retireNativeImageStorageV1(options);rmSync(parent,{recursive:true});}
});}),120000);
