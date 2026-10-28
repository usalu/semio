/** 🎭️ Serial native snapshot queries preserve the target actor's real command and publication lanes. */
import type { ReservationHub } from "../../🧩️hub/🟦️.ts";
import type { AppCommandValue, AppFrameValue, MediaExportHandle } from "../../../../../../../../🔨️modules/🪢️wire-contracts/🟦️.ts";
import { encodeAppCommand } from "../../../../../../../../🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts";
import { decodeAppFrame } from "../../../../../../../../🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { createShardCommandIngressPages } from "@semio-tech/framework-actor/shard-client";
import { browserActorCommandIngressStatus } from "../../../🧩️actor-wire-helpers/🟦️.ts";
import { readSnapshotPack, type SnapshotPackControl, type SnapshotPackPort } from "../🟦️.ts";

declare module "../../🧩️hub/🟦️.ts" {
 interface ReservationCollaborators { snapshots: ReservationSnapshotReads }
}

export class ReservationSnapshotReads {
 private sequence = BigInt(Number.MAX_SAFE_INTEGER);
 constructor(private readonly hub: ReservationHub) {}
 private command(make: (seq: bigint) => AppCommandValue): Promise<AppFrameValue> {
  return this.hub.lanes.enqueueTurn(async () => {
   const session = this.hub.session;
   this.hub.backbone.assertDocumentOwnerCurrent();
   const child = session.child;
   if (!child || this.sequence < 1n) throw Error("snapshot-read.unavailable");
   const sequence = this.sequence--;
   const command = encodeAppCommand(make(sequence));
   let reply: AppFrameValue | undefined;
   const receive = (payload: Uint8Array): boolean => {
    let frame: AppFrameValue;
    try { frame = decodeAppFrame(payload); } catch { return false; }
    const body = "DocumentIdentity" in frame ? frame.DocumentIdentity : "MediaExportSubmitted" in frame ? frame.MediaExportSubmitted : "MediaExportStatus" in frame ? frame.MediaExportStatus : "MediaExportChunk" in frame ? frame.MediaExportChunk : "Done" in frame ? frame.Done : "Error" in frame ? frame.Error : undefined;
    if (!body || body.in_reply_to === null || BigInt(body.in_reply_to) !== sequence) return false;
    if (reply) throw Error("snapshot-read.ambiguous-reply");
    reply = frame;
    return true;
   };
   const assertCurrent = () => { this.hub.backbone.assertDocumentOwnerCurrent(); if (session.child !== child) throw Error("snapshot-read.stale-target"); };
   const consume = async (result: BrowserActorChildValue): Promise<string> => {
    const status = browserActorCommandIngressStatus(result);
    await this.hub.patches.driveTurnResult(result, child, assertCurrent, null, receive);
    if (status === "backpressure" || status === "fault") throw Error("snapshot-read.command-refused");
    return status;
   };
   let status = "idle";
   for (const page of createShardCommandIngressPages({owner:0n,generation:session.generation,commandIndex:0,commandCount:1,instance:0,seq:sequence,command})) status = await consume(await this.hub.lanes.invokeCommandPage(child,page,assertCurrent));
   for (let turn=0;status!=="command-complete"&&turn<1024;turn++) status = await consume(await this.hub.lanes.invokeCommandPage(child,null,assertCurrent));
   if (status !== "command-complete" || !reply) throw Error("snapshot-read.missing-reply");
   if ("Error" in reply) throw Error("snapshot-read.target-refused");
   return reply;
  });
 }

 /** 🌤️ Captures the live target at an actual revision and exports that frozen native source. */
 async readHead(control:SnapshotPackControl):Promise<Uint8Array>{return this.readSelection(control,null);}

 async readSelection(control: SnapshotPackControl,checkpointId:string|null): Promise<Uint8Array> {
  const session = this.hub.session;
  control.assertCurrent();
  const identity = await this.command(seq=>({ReadDocumentIdentity:{seq}}));
  if (!("DocumentIdentity" in identity) || identity.DocumentIdentity.identity.app_instance_id !== 0 || identity.DocumentIdentity.identity.parent_document_id !== session.scope.documentId) throw Error("snapshot-read.foreign-document");
  const port: SnapshotPackPort = {
   submit: async (port,document,revision) => { const reply=await this.command(seq=>({SubmitMediaExport:{seq,port,expected_parent_document_id:document,expected_base_revision:revision,checkpoint_id:checkpointId}}));if(!("MediaExportSubmitted"in reply))throw Error("snapshot-read.missing-admission");return reply.MediaExportSubmitted.handle; },
   poll: async handle => {const reply=await this.command(seq=>({PollMediaExport:{seq,handle}}));if(!("MediaExportStatus"in reply))throw Error("snapshot-read.missing-status");return reply.MediaExportStatus;},
   take: async handle => {const reply=await this.command(seq=>({TakeMediaExportChunk:{seq,handle}}));if(!("MediaExportChunk"in reply))throw Error("snapshot-read.missing-chunk");return{handle:reply.MediaExportChunk.handle,data:Uint8Array.from(reply.MediaExportChunk.data),terminal:reply.MediaExportChunk.terminal};},
   cancel: async (handle: MediaExportHandle) => {const reply=await this.command(seq=>({CancelMediaExport:{seq,handle}}));if(!("Done"in reply))throw Error("snapshot-read.cancel-unconfirmed");},
  };
  return readSnapshotPack(port,{instance:0,documentId:session.scope.documentId,revision:identity.DocumentIdentity.identity.base_revision},control);
 }
}
