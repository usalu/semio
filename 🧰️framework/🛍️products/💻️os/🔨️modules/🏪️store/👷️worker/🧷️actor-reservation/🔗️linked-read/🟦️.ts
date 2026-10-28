/** 🔗️ Original linked reads resolve asynchronously while the requesting actor remains responsive. */
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import { artifactState } from "../../🧾️document-state/🟦️.ts";
import { browserActorBytes, browserActorRecord } from "../../🧩️actor-wire-helpers/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { parseArtifactLink } from "../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🟦️.ts";
import { decodeJsonSyntax, type JsonSyntaxNode } from "../../../../../../../🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts";
import { hubSessionFetch } from "../../🔐️hub-session/🟦️.ts";
import type { SnapshotPackControl } from "../📖️snapshot-read/🟦️.ts";
import policy from "../📖️snapshot-read/🔣️.json";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { readBoundedExecutionTargetBody } from "../../📥️execution-target-read/🟦️.ts";
import { Blake3Hasher, hexLower } from "../../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import { encodePackValue } from "../../../../../🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
 interface ReservationCollaborators { links: ReservationLinkedReads }
}

/** 🔢️ Retains native u64 link metadata and refuses duplicate JSON members. */
export async function readLinkedArtifactJson(bytes: Uint8Array, signal: AbortSignal) {
 const syntax = await decodeJsonSyntax(new TextDecoder("utf-8",{fatal:true}).decode(bytes),"Reject",{maximumBytes:16384,maximumNodes:64,maximumDepth:8,chunk:4096,cancelled:()=>signal.aborted,progress:()=>{},yield:()=>new Promise(resolve=>globalThis.setTimeout(resolve,0))});
 function value(node: JsonSyntaxNode): unknown {
  switch(node.kind){
   case "null": return null;
   case "boolean": case "string": return node.value;
   case "number": if(!/^-?(0|[1-9][0-9]*)$/.test(node.text))throw Error("link-read.integer-required");return BigInt(node.text);
   case "array": return node.items.map(value);
   case "object": return Object.fromEntries(node.members.map(member=>[member.name,value(member.value)]));
  }
 }
 return parseArtifactLink(value(syntax));
}

/** 🔏️ Verifies an exact content pin before handing its original bytes to the native receiver. */
export async function readLinkedSnapshotBlob(response: FetchTimeoutResponse, blob: Readonly<{hash:string;size:bigint;mediaType:string}>, control: SnapshotPackControl): Promise<Uint8Array> {
 if(!/^[a-f0-9]{64}$/.test(blob.hash)||blob.size<1n||blob.size>BigInt(policy.maximumBytes)||blob.mediaType!==policy.mediaType)throw Error("link-read.invalid-blob");
 const bytes=await readBoundedExecutionTargetBody(response,Number(blob.size),policy.maximumBytes,{signal:control.signal,deadlineAtMs:control.deadlineMs,assertCurrent:control.assertCurrent},received=>control.progress(BigInt(received)));
 try{
  const hash=new Blake3Hasher();
  for(let offset=0;offset<bytes.length;offset+=65536){
   if(control.signal.aborted||Date.now()>=control.deadlineMs)throw Error("link-read.cancelled");
   control.assertCurrent();hash.update(bytes.subarray(offset,offset+65536));await control.yieldTurn();
  }
  control.assertCurrent();if(control.signal.aborted||Date.now()>=control.deadlineMs)throw Error("link-read.cancelled");
  if(hexLower(hash.digest())!==blob.hash)throw Error("link-read.blob-integrity");
  return bytes;
 }catch(error){bytes.fill(0);throw error;}
}

export class ReservationLinkedReads {
 private readonly pending = new Map<bigint, AbortController>();
 constructor(private readonly hub: ReservationHub) {}

 /** 📬️ Retains the actual request id until one completion or its original actor closes. */
 schedule(raw: BrowserActorChildValue): void {
  const body = browserActorRecord(raw,"link-read.invalid-request");
  const req = typeof body.req === "bigint" ? body.req : typeof body.req === "number" && Number.isSafeInteger(body.req) ? BigInt(body.req) : -1n;
  if (req < 0n || req > 0xffffffffffffffffn || this.pending.has(req)) throw Error("link-read.invalid-request");
  const controller = new AbortController();
  const session = this.hub.session;
  const child = session.child;
  const generation = session.generation;
  const abort = () => controller.abort();
  session.abort.signal.addEventListener("abort",abort,{once:true});
  this.pending.set(req,controller);
  const assertSource = () => { if(session.child!==child || session.generation!==generation || session.closed)throw Error("link-read.retired");this.hub.backbone.assertDocumentOwnerCurrent(); };
  const current = () => { if(controller.signal.aborted)throw Error("link-read.cancelled");assertSource(); };
  const work = async (): Promise<Uint8Array> => {
   current();
   if (this.pending.size > 4) throw Error("link-read.capacity");
   const bytes = browserActorBytes(body.link,"link-read.invalid-link");
   if(bytes.byteLength>16384)throw Error("link-read.link-capacity");
   const link = await readLinkedArtifactJson(bytes,controller.signal);
   if(link.pin.kind==="snapshot"){
    const blob=link.pin.blob;
    if(!/^[a-f0-9]{64}$/.test(blob.hash)||blob.size<1n||blob.size>BigInt(policy.maximumBytes)||blob.mediaType!==policy.mediaType)throw Error("link-read.invalid-blob");
    const deadlineAtMs=Date.now()+120000;
    const response=await hubSessionFetch(`/blobs/${blob.hash}`,{method:"GET"},{timeoutMs:120000,signal:controller.signal,admit:()=>{current();return true;},retain:()=>{current();return true;}});
    return readLinkedSnapshotBlob(response,blob,{signal:controller.signal,deadlineMs:deadlineAtMs,assertCurrent:current,progress:()=>{},yieldTurn:()=>new Promise(resolve=>globalThis.setTimeout(resolve,0))});
   }
   const target = artifactState(link.target.artifactId,session.scope.spaceId);
   const lease = target?.executionTargetLease;
   const reservation = target?.browserActorReservation;
   if(!target || !lease?.live || !reservation)throw Error("link-read.target-unavailable");
   const dialect = lease.fields().parentDialect;
   if(dialect.artifactKind!==link.target.dialect.artifactKind || dialect.standard!==link.target.dialect.standard || dialect.subset!==link.target.dialect.subset)throw Error("link-read.target-dialect");
   const targetAbort = () => controller.abort();
   target.docAbort.signal.addEventListener("abort",targetAbort,{once:true});
   const timer = globalThis.setTimeout(abort,120000);
   try {
    const control={signal:controller.signal,deadlineMs:Date.now()+120000,assertCurrent:()=>{current();if(target.browserActorReservation!==reservation || !lease.live)throw Error("link-read.stale-target");},progress:()=>{},yieldTurn:()=>new Promise<void>(resolve=>globalThis.setTimeout(resolve,0))};
    return link.pin.kind==="checkpoint"?await reservation.readSnapshotCheckpoint(link.pin.id,control):await reservation.readSnapshotHead(control);
   } finally {globalThis.clearTimeout(timer);target.docAbort.signal.removeEventListener("abort",targetAbort);}
  };
  const complete = async (bytes: Uint8Array, fault: boolean): Promise<void> => {
   try {
    if(session.closed || session.child!==child || !child)return;
    await this.hub.lanes.enqueueTurn(async()=>{
     assertSource();
     const result = await this.hub.lanes.invokePoll(child,[{tag:"completed",val:{req,outcome:{tag:fault?"fault":"ok",val:bytes}}}],null,assertSource);
     await this.hub.patches.driveTurnResult(result,child,assertSource);
    });
   } finally {bytes.fill(0);}
  };
  void work().then(bytes=>complete(bytes,false),error=>complete(encodePackValue({origin:"framework",code:"artifact.link.resolve",severity:"error",message:error instanceof Error?error.message:"link-read.failed",scope:{},retryable:false}),true)).catch(()=>{if(!session.closed)this.hub.retirement.close();}).finally(()=>{session.abort.signal.removeEventListener("abort",abort);if(this.pending.get(req)===controller)this.pending.delete(req);});
 }
}
