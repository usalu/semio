import { EventEmitter } from "node:events";
import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { createHash } from "node:crypto";
import { joinProtectedActorResponsesV1, ProtectedActorResponseCaptureV1, executeProtectedActorChildV1, type ProtectedActorBrowserPortV1, protectedActorChildUrlV1, observeNormalDevHubPublicationOwnerV1 } from "../🟦️.ts";
import { blake3Hex } from "../../../../../../../🔨️modules/🔏️hash/🟦️.ts";

/** 🧪️ Exercises semantic byte joins using plain cases and independent JSON Schema/WebCrypto oracles. */
export async function testProtectedActorPublicationV1(root: string): Promise<void> {
  const cases=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
  for(const row of cases.childUrls){const actual=new URL(protectedActorChildUrlV1(row.root,row.baseUrl));assert.equal(actual.origin,new URL(row.baseUrl).origin);assert(actual.pathname.startsWith(row.prefix),row.root);}
  const source=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json"),"utf8"));
  const fields=structuredClone(source.manifest), body=Object.fromEntries(["component","descriptor","actor"].map(name=>[name,Buffer.from(cases[name])]));
  const claim=(bytes:Uint8Array)=>({sha256:createHash("sha256").update(bytes).digest("hex"),byteLength:bytes.byteLength});
  Object.assign(fields.component,claim(body.component),{blake3:blake3Hex(body.component)});Object.assign(fields.descriptor,claim(body.descriptor));
  Object.assign(fields.package,{componentSha256:fields.component.sha256,componentBlake3:fields.component.blake3,descriptorByteSha256:fields.descriptor.sha256});
  Object.assign(fields.browserActor,claim(body.actor),{sourceComponentSha256:fields.component.sha256,sourceDescriptorByteSha256:fields.descriptor.sha256});
  const expected={generationId:fields.catalog.generationId,pluginId:fields.package.pluginId,packageId:fields.package.packageId,component:fields.component,descriptor:fields.descriptor,actor:fields.browserActor};
  const make=()=>["manifest","component","descriptor","browser-actor"].map(asset=>({asset,url:source.hubOrigin+"/spaces/"+encodeURIComponent(source.intent.scope.spaceId)+"/documents/"+encodeURIComponent(source.intent.scope.documentId)+"/execution-target/"+asset,method:"POST",status:200,authenticated:true,intent:structuredClone(source.intent),body:asset==="manifest"?Buffer.from(JSON.stringify(fields)):new Uint8Array(body[asset==="browser-actor"?"actor":asset])}));
  const result=joinProtectedActorResponsesV1(expected,make(),source.hubOrigin);assert.equal(result.fields.package.pluginId,expected.pluginId);
  for(const mutation of cases.mutations){const rows=make(), selected=structuredClone(expected);if(mutation==="unauthenticated")rows[0].authenticated=false;else if(mutation==="get")rows[0].method="GET";else if(mutation==="wrong-origin")rows[0].url=rows[0].url.replace(source.hubOrigin,"http://foreign.test");else if(mutation==="wrong-scope")rows[1].intent.scope.documentId="other";else if(mutation==="caller-package")rows[1].intent.packageId=selected.packageId;else if(mutation==="corrupt-body")rows[2].body=Buffer.from("corrupt");else if(mutation==="wrong-generation")selected.generationId="0".repeat(64);else if(mutation==="wrong-actor-source")selected.actor.sourceComponentSha256="0".repeat(64);else if(mutation==="duplicate-asset")rows[3]=rows[2];else throw Error("unknown neutral mutation");assert.throws(()=>joinProtectedActorResponsesV1(selected,rows,source.hubOrigin),mutation);}
  assert(readFileSync(join(root,"bun.lock"),"utf8").includes("\"playwright@"),"the independent browser oracle must be locked for a clean install");
  for(const row of cases.unfinishedResponses)for(const phase of cases.responsePhases){
    const signal=new AbortController(),network=new EventEmitter(),capture=new ProtectedActorResponseCaptureV1(network,source.hubOrigin);let finish!:()=>void,allocated=0,sizesCalls=0;const originalBody=new Uint8Array([9]);
    const unfinished=new Promise<null>(resolve=>{finish=()=>resolve(null);});
    network.emit("response",{url:()=>make()[3].url,status:()=>200,headers:()=>({"content-length":"1"}),finished:()=>phase==="finished"?unfinished:Promise.resolve(null),request:()=>({method:()=>"POST",headers:()=>({}),allHeaders:async()=>{if(phase==="allHeaders")await unfinished;return{authorization:"Bearer neutral-test-only"};},postDataJSON:()=>source.intent,sizes:async()=>{sizesCalls++;if(phase==="sizes")await unfinished;return{responseBodySize:1};}}),body:async()=>{allocated++;if(phase==="body")await unfinished;return originalBody;}});
    const timers=row.cancelAfterMs===null?[]:[setTimeout(()=>signal.abort(),row.cancelAfterMs)];
    let guard!:ReturnType<typeof setTimeout>;
    const observed=new Promise<string>(resolve=>{guard=setTimeout(()=>resolve("observation-expired"),row.observationBoundMs);capture.take(expected,fields.artifact.kind,fields.scope.spaceId,{signal:signal.signal,deadlineAtMs:Date.now()+row.deadlineMs}).then(()=>resolve("resolved"),()=>resolve("refused"));});
    const outcome=await observed;clearTimeout(guard);for(const timer of timers)clearTimeout(timer);capture.close();finish();await new Promise(resolve=>setTimeout(resolve,0));
    assert.equal(outcome,row.expected,"unfinished original response "+phase+"/"+row.name+" must stop inside its selected wait boundary");assert.equal(sizesCalls,phase==="finished"?0:1,"late completed response must not advance beyond its suspended read phase");assert.equal(allocated,phase==="finished"||phase==="sizes"?0:1);if(allocated)assert(originalBody.every(byte=>byte===0),"late or already copied original response bytes must be wiped");await assert.rejects(()=>capture.take(expected,fields.artifact.kind,fields.scope.spaceId),/closed/u);assert.equal(network.listenerCount("response"),0);
  }
  for(const row of cases.unfinishedResponses){
    const network=new EventEmitter(),capture=new ProtectedActorResponseCaptureV1(network,source.hubOrigin),abort=new AbortController();
    const timer=row.cancelAfterMs===null?undefined:setTimeout(()=>abort.abort(),row.cancelAfterMs);
    await assert.rejects(()=>capture.take(expected,fields.artifact.kind,fields.scope.spaceId,{signal:abort.signal,deadlineAtMs:Date.now()+row.deadlineMs}));if(timer!==undefined)clearTimeout(timer);
    assert.equal(network.listenerCount("response"),0,"an incomplete idle chain must close at "+row.name);await assert.rejects(()=>capture.take(expected,fields.artifact.kind,fields.scope.spaceId),/closed/u);
    let copied=0;network.emit("response",{url:()=>make()[3].url,status:()=>200,headers:()=>({"content-length":"1"}),finished:async()=>null,request:()=>({method:()=>"POST",headers:()=>({}),allHeaders:async()=>({authorization:"Bearer neutral-only"}),postDataJSON:()=>source.intent,sizes:async()=>({responseBodySize:1})}),body:async()=>{copied++;return new Uint8Array(1);}});await new Promise(resolve=>setTimeout(resolve,0));assert.equal(copied,0);
  }
  const expiredNetwork=new EventEmitter(),expiredCapture=new ProtectedActorResponseCaptureV1(expiredNetwork,source.hubOrigin),expiredRows=make();
  for(const row of expiredRows)expiredNetwork.emit("response",{url:()=>row.url,status:()=>200,headers:()=>({"content-length":String(row.body.byteLength)}),finished:async()=>null,request:()=>({method:()=>"POST",headers:()=>({}),allHeaders:async()=>({authorization:"Bearer neutral-only"}),postDataJSON:()=>row.intent,sizes:async()=>({responseBodySize:row.body.byteLength})}),body:async()=>row.body});
  await new Promise(resolve=>setTimeout(resolve,0));await assert.rejects(()=>expiredCapture.take(expected,fields.artifact.kind,fields.scope.spaceId,{signal:new AbortController().signal,deadlineAtMs:Date.now()-1}));assert.equal(expiredNetwork.listenerCount("response"),0);assert(expiredRows.every(row=>row.body.every(byte=>byte===0)));
  console.log("protected-pending-response-neutral: waits=8; locked browser oracle in existing containment owner; idle waits=2; expired complete=1; late originals wiped; closed observation has no response listener");
  const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));const {schemaScopeIdFromDocumentId,schemaCollectionContractPath,inventorySchemaScopes}=await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");assert.equal(schemaCollectionContractPath(relative(root,join(import.meta.dir,"../🧬️schema/🔣️.json")).replaceAll("\\","/")),false,"a genuine publication observation contract must have a semantic owner outside test collections");assert.equal(schemaScopeIdFromDocumentId(schema.$id),"os.dev.verification.publication");assert.equal(new URL(schema.$id).pathname,"/os/dev/verification/publication/component.json");const inventory=inventorySchemaScopes(root),scope=inventory.catalog.scopes["os.dev.verification.publication"];assert.equal(scope?.path,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧬️schema");assert(scope.exports.ProtectedActorExecutionV1);assert(!inventory.diagnostics.some(row=>row.path.startsWith(scope.path)&&row.code.startsWith("schema-fixture")));const {default:Ajv}=await import("ajv");const ajv=new Ajv({strict:false});ajv.addSchema(schema);for(const response of result.responses)assert(ajv.getSchema(schema.$id+"#/$defs/ProtectedResponseV1")!(response));
  for(const asset of ["component","descriptor","actor"])assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256",body[asset])).toString("hex"),claim(body[asset]).sha256);
  const network=new EventEmitter(),capture=new ProtectedActorResponseCaptureV1(network,source.hubOrigin);let allocated=0;
  network.emit("response",{url:()=>make()[3].url,status:()=>200,headers:()=>({"content-length":"1"}),finished:async()=>null,request:()=>({method:()=>"POST",headers:()=>({}),allHeaders:async()=>({authorization:"Bearer neutral-test-only"}),postDataJSON:()=>source.intent,sizes:async()=>({responseBodySize:2})}),body:async()=>{allocated++;return new Uint8Array(2);}});
  await assert.rejects(()=>capture.take(expected,fields.artifact.kind,fields.scope.spaceId));capture.close();assert.equal(allocated,0,"encoded body over declared/actual capacity must refuse before body allocation");
  const latestNetwork=new EventEmitter(),latestCapture=new ProtectedActorResponseCaptureV1(latestNetwork,source.hubOrigin,{selection:expected,kindId:fields.artifact.kind});
  const first=make();
  const observe=(row:typeof first[number])=>latestNetwork.emit("response",{url:()=>row.url,status:()=>200,headers:()=>({"content-length":String(row.body.byteLength)}),finished:async()=>null,request:()=>({method:()=>"POST",headers:()=>({}),allHeaders:async()=>({authorization:"Bearer neutral-test-only"}),postDataJSON:()=>row.intent,sizes:async()=>({responseBodySize:row.body.byteLength})}),body:async()=>row.body});
  for(const row of first)observe(row);await latestCapture.take(expected,fields.artifact.kind,fields.scope.spaceId);
  const next=make();next[0].intent.clientInstanceId+="-reopened";observe(next[0]);await assert.rejects(()=>latestCapture.take(expected,fields.artifact.kind,fields.scope.spaceId));
  assert(first.every(row=>row.body.every(byte=>byte===0)),"reopened document must retire previous complete captured group before retaining the next");latestCapture.close();
  const localOwner=await import("../../../🚀️local-hub/🏃️execution/🟦️.ts");const owned=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"normal-dev-hub-owner-neutral-")),dataRoot=join(owned,"data"),hubUrl="http://127.0.0.1:8877",leasePaths=localOwner.devHubLeasePathsV1(localOwner.devHubLeaseRootV1(owned),8877,dataRoot);
  const started=Date.now(),control={maxBytes:1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>false,remainingMs:()=>60_000-(Date.now()-started),onProgress:()=>{}};
  await assert.rejects(()=>observeNormalDevHubPublicationOwnerV1(owned,dataRoot,hubUrl,control));
  const lease={pid:process.pid,port:8877,dataDir:dataRoot,hubUrl,acquiredAt:Date.now()};for(const path of leasePaths){mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,JSON.stringify(lease));}
  const observed=await observeNormalDevHubPublicationOwnerV1(owned,dataRoot,hubUrl,control);assert.equal(observed.pid,process.pid);assert.equal(observed.leases.length,2);assert(ajv.getSchema(schema.$id+"#/$defs/NormalDevHubOwnerV1")!(observed));writeFileSync(leasePaths[1],JSON.stringify({...lease,hubUrl:"http://127.0.0.1:8878"}));await assert.rejects(()=>observeNormalDevHubPublicationOwnerV1(owned,dataRoot,hubUrl,control));
  const {runPhysicalLeaseChecksV1}=await import("../../../🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🔐lease/🧪️tests/🟦️.ts");await runPhysicalLeaseChecksV1(root);
  console.log("protected-actor-publication-neutral: 1 join; 13 semantic refusals; actual-size preallocation; latest-group retirement; URLs=2; normal-owner=3; Ajv=5 WebCrypto=3; normal wrapper consumer imported; actual catalog scope admitted; unfinished waits=8; locked browser oracle in existing containment owner; idle waits=2 expired complete=1");
}


/** 🧵️ Runs the new evidence executor in the existing containment owner's real neutral Chromium page. */
export interface ProtectedActorBrowserTestPortV1 extends ProtectedActorBrowserPortV1 {
  waitForFunction(execute:()=>boolean,input:undefined,options:Readonly<{timeout:number}>):Promise<unknown>;
}

export async function testProtectedActorChildExecutionV1(root:string,page:ProtectedActorBrowserTestPortV1,baseUrl:string):Promise<void>{
  const cases=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));let oracleCount=0;
  for(const row of cases.unfinishedResponses)for(const phase of cases.responsePhases){
    let output:string;
    if(row.cancelAfterMs===null){
      output="resolved";try{await page.waitForFunction(()=>false,undefined,{timeout:row.deadlineMs});}catch(error){assert.equal((error as Error).name,"TimeoutError");output="refused";}
    }else output=await page.evaluate(async (delay:number)=>{
      const abort=new AbortController(),pending=new Promise<never>((_,reject)=>abort.signal.addEventListener("abort",()=>reject(abort.signal.reason),{once:true})),timer=setTimeout(()=>abort.abort(),delay);
      try{await pending;return"resolved";}catch(error){if(!(error instanceof DOMException)||error.name!=="AbortError")throw error;return"refused";}finally{clearTimeout(timer);}
    },row.cancelAfterMs);
    assert.equal(output,row.expected,"independent locked browser oracle "+phase+"/"+row.name);oracleCount++;
  }
  assert.equal(oracleCount,8);

  const {encodePackValue}=await import("../../../../../🟦️.ts");
  const descriptor=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🧫️fixtures/🔣️.json"),"utf8")).descriptor,staged=encodePackValue(descriptor);
  for(const key of Object.keys(descriptor.hashes))descriptor.hashes[key]="";
  const guest=encodePackValue(descriptor),actor=Buffer.from(`export async function activate(){return{async invoke(path,args){if(path.join('.')!=='describe.describe'||args.length)throw Error('neutral describe path');return new Uint8Array(${JSON.stringify(Array.from(guest))});},async close(){}};}`);
  const joined={actor,descriptor:staged} as Parameters<typeof executeProtectedActorChildV1>[2];
  const childUrl=protectedActorChildUrlV1(root,baseUrl);
  const owner={root,baseUrl};
  const result=await executeProtectedActorChildV1(page,childUrl,joined,owner);assert.equal(result.describe.sha256,createHash("sha256").update(guest).digest("hex"));assert(result.cancelled&&result.capacityRestored&&result.sourceDetached);assert(result.stages.includes("active"));
  const forged=`export function browserActorChildCapacity(){return{actors:0,bytes:0};}export async function reserveBrowserActorChild(admission,signal){let phase='ready';signal.addEventListener('abort',()=>{phase='closed';});return{async load(bytes,onProgress){structuredClone(bytes,{transfer:[bytes]});for(const stage of ['verified','imported','active'])onProgress({stage});phase='active';},progress(){return{phase,sourceDetached:true,resultTransfersDetached:1};},async invoke(){if(phase==='closed')throw Error('closed');return new Uint8Array(${JSON.stringify(Array.from(guest))});},close(){phase='closed';}};}`;
  await assert.rejects(()=>executeProtectedActorChildV1(page,"data:text/javascript,"+encodeURIComponent(forged),joined,owner),"a foreign module must not forge successful child execution evidence");
  const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));const {default:Ajv}=await import("ajv");const ajv=new Ajv({strict:false});ajv.addSchema(schema);assert(ajv.getSchema(schema.$id+"#/$defs/ChildExecutionV1")!(result));
  guest.fill(0);staged.fill(0);actor.fill(0);
  console.log("protected-actor-child-neutral: actual Chromium load/describe/detach/abort/capacity/postcancel; foreign-success forgery refused; Ajv=1 locked-Playwright/Chromium-wait-oracles=8");
}
