import { strict as assert } from "node:assert";
import { Buffer } from "node:buffer";
import Ajv from "ajv";
import { readSnapshotPack, type SnapshotPackPort } from "../🟦️.ts";
import fixture from "./🔣️.json";
import policy from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 🧪️ Independent Buffer reconstruction and hostile original export transcripts. */
export async function verifySnapshotPackReader(): Promise<void> {
 assert.equal(new Ajv({strict:true}).compile(schema)(policy), true);
 const handle = {app_instance_id:fixture.instance,parent_document_id:fixture.documentId,base_revision:BigInt(fixture.revision),operation_id:BigInt(fixture.operation),generation:BigInt(fixture.generation)};
 const expected = Buffer.concat(fixture.chunks.map(chunk => Buffer.from(chunk)));
 assert.equal(expected.toString("hex"), fixture.expectedHex);
 for (const mode of ["valid", ...fixture.refusals,"cancelFailure"]) {
  const abort = new AbortController();
  let cancelled = 0, index = 0;
  const originals: Uint8Array[] = [];
  const port: SnapshotPackPort = {
   submit: async (port,document,revision) => { assert.equal(port,policy.port);assert.equal(document,fixture.documentId);assert.equal(revision,handle.base_revision);return mode === "foreignHandle" ? {...handle,base_revision:10n} : handle; },
   poll: async () => {
    if(mode === "cancelled")abort.abort();
    return {handle,state:"complete",applied_progress:5n,checkpoint_available:false,mime_type:(mode === "wrongMediaType" || mode === "cancelFailure") ? "text/plain" : policy.mediaType,total_bytes:mode === "oversized" ? BigInt(policy.maximumBytes)+1n : BigInt(expected.length),detail:""};
   },
   take: async () => {
    const data = mode === "stalled" ? new Uint8Array() : Uint8Array.from(fixture.chunks[index++] ?? []);
    originals.push(data);
    return {handle,data,terminal:mode === "truncated" || (mode !== "stalled" && index === fixture.chunks.length)};
   },
   cancel: async original => {assert.deepEqual(original,handle);cancelled++;if(mode === "cancelFailure")throw Error("original cancellation refused");},
  };
  const run = readSnapshotPack(port,{instance:handle.app_instance_id,documentId:handle.parent_document_id,revision:handle.base_revision},{signal:abort.signal,deadlineMs:Date.now()+10000,assertCurrent:()=>{},progress:()=>{},yieldTurn:async()=>{}});
  if(mode === "valid") {const actual=await run;assert.equal(Buffer.from(actual).toString("hex"),expected.toString("hex"));assert.equal(cancelled,0);}
  else {await assert.rejects(run,error=>mode !== "cancelFailure" || error instanceof AggregateError && error.errors.length === 2 && error.errors[0].message === "snapshot-read.media-type" && error.errors[1].message === "original cancellation refused");assert.equal(cancelled,mode === "foreignHandle" ? 0 : 1);}
  assert.ok(originals.every(original => original.every(byte=>byte===0)),"original chunks retire before success/refusal");
 }
 console.log("[DEBUG] Snapshot Pack reader: Buffer + Ajv valid original transfer and seven refusal transcripts PASS");
}
