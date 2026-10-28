import {strict as assert} from "node:assert";
import {Buffer} from "node:buffer";
import {ReservationLinkedReads,readLinkedArtifactJson,readLinkedSnapshotBlob} from "../🟦️.ts";
import {artifacts} from "../../../🧾️document-state/🟦️.ts";
import {documentRuntimeKeyV1,decodePackValue} from "../../../../../../🟦️.ts";
import fixture from "./🔣️.json";

/** 📬️ Exercises real asynchronous request custody, exact faults and stale completion suppression. */
export async function verifyLinkedReadLedger(): Promise<void> {
 const abort=new AbortController();
 const snapshot=await readLinkedArtifactJson(new TextEncoder().encode('{"target":{"artifactId":"weather","dialect":{"artifactKind":"s.stdio.epw","standard":"energyplus","subset":"*"}},"pin":{"kind":"snapshot","blob":{"hash":"hash","size":18446744073709551615,"mediaType":"application/vnd.semio.pack"}},"role":"weather"}'),abort.signal);
 assert.equal(snapshot.pin.kind,"snapshot");
 if(snapshot.pin.kind==="snapshot")assert.equal(snapshot.pin.blob.size,18446744073709551615n);
 await assert.rejects(readLinkedArtifactJson(new TextEncoder().encode('{"role":"weather","role":"other"}'),abort.signal));
 const key=documentRuntimeKeyV1({kind:"hub",dataClass:"persistedShared",spaceId:fixture.spaceId,documentId:fixture.documentId});
 const previous=artifacts.get(key);
 try {
  for(const mode of fixture.cases){
   const sourceAbort=new AbortController();
   const targetAbort=new AbortController();
   const events:unknown[]=[];
   const original=Uint8Array.from(fixture.bytes);
   let release:((bytes:Uint8Array)=>void)|undefined;
   let selectedCheckpoint:string|undefined;
   let finished:()=>void=()=>{};
   const settled=new Promise<void>(resolve=>{finished=resolve;});
   const child={};
   const session={abort:sourceAbort,child,generation:3n,closed:false,scope:{spaceId:fixture.spaceId}};
   const hub={session,backbone:{assertDocumentOwnerCurrent:()=>{}},lanes:{enqueueTurn:async(work:()=>Promise<void>)=>work(),invokePoll:async(_child:unknown,next:unknown[])=>{events.push(...structuredClone(next));finished();return{};}},patches:{driveTurnResult:async()=>{}},retirement:{close:()=>{throw Error("unexpected actor retirement");}}};
   if(mode!=="missing")artifacts.set(key,{docAbort:targetAbort,executionTargetLease:{live:true,fields:()=>({parentDialect:mode==="dialect"?{...fixture.dialect,standard:"other"}:fixture.dialect})},browserActorReservation:{readSnapshotHead:async()=>new Promise<Uint8Array>(resolve=>{release=resolve;}),readSnapshotCheckpoint:async(id:string)=>{selectedCheckpoint=id;return new Promise<Uint8Array>(resolve=>{release=resolve;});}}} as never);
   else artifacts.delete(key);
   const ledger=new ReservationLinkedReads(hub as never);
   const link={target:{artifactId:fixture.documentId,dialect:fixture.dialect},pin:mode==="checkpoint"?{kind:"checkpoint",id:"pinned"}:{kind:"head"},role:"weather"};
   ledger.schedule({req:17n,link:new TextEncoder().encode(JSON.stringify(link))});
   if(mode==="head"||mode==="checkpoint"||mode==="retired"){
    assert.equal(events.length,0,"requesting actor remains free while target is pending");
    for(let turn=0;!release&&turn<32;turn++)await new Promise(resolve=>setTimeout(resolve,0));
    if(mode==="retired"){session.closed=true;sourceAbort.abort();}
    assert.ok(release);if(mode==="checkpoint")assert.equal(selectedCheckpoint,"pinned");else assert.equal(selectedCheckpoint,undefined);release!(original);
   }
   if(mode==="retired"){await new Promise(resolve=>setTimeout(resolve,0));assert.equal(events.length,0);assert.ok(original.every(byte=>byte===0));}
   else {
    await settled;
    const event=events[0] as {tag:string;val:{req:bigint;outcome:{tag:string;val:Uint8Array}}};
    assert.equal(events.length,1);assert.equal(event.tag,"completed");assert.equal(event.val.req,17n);
    if(mode==="head"||mode==="checkpoint"){assert.equal(event.val.outcome.tag,"ok");assert.equal(Buffer.from(event.val.outcome.val).toString("hex"),Buffer.from(fixture.bytes).toString("hex"));}
    else {assert.equal(event.val.outcome.tag,"fault");assert.equal((decodePackValue(event.val.outcome.val) as {code:string}).code,"artifact.link.resolve");}
   }
  }
 } finally {if(previous)artifacts.set(key,previous);else artifacts.delete(key);}
 console.log("[DEBUG] Linked read ledger: native completed request identity, responsive source, exact faults and stale retirement PASS");
}

/** 🧱️ Exact BLAKE3 pin, byte count, original body retirement and interruption transcripts. */
export async function verifyLinkedSnapshotBlob(): Promise<void> {
 for(const mode of ["valid",...fixture.snapshot.refusals]){
  const abort=new AbortController();
  const original=Uint8Array.from(fixture.snapshot.bytes);
  const response=new Response(new ReadableStream({start(controller){controller.enqueue(original);controller.close();}}),{headers:{"content-length":String(original.length)}});
  const pin={hash:mode==="hash"?"0".repeat(64):fixture.snapshot.hash,size:BigInt(original.length+(mode==="length"?1:0)),mediaType:mode==="mediaType"?"text/plain":"application/vnd.semio.pack"};
  if(mode==="cancelled")abort.abort();
  const read=readLinkedSnapshotBlob(response,pin,{signal:abort.signal,deadlineMs:Date.now()+10000,assertCurrent:()=>{if(mode==="stale")throw Error("stale");},progress:()=>{},yieldTurn:async()=>{}});
  if(mode==="valid"){const bytes=await read;assert.deepEqual(Array.from(bytes),fixture.snapshot.bytes);assert.ok(original.every(byte=>byte===0));}
  else await assert.rejects(read);
 }
 console.log("[DEBUG] Linked snapshot: exact BLAKE3/size and five refusal transcripts PASS");
}
