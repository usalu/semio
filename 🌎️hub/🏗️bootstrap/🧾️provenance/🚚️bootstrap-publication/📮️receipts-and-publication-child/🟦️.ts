/** 📮️ Trusted bootstrap receipts and publication child: the publication receipt contract and receipt, the unconfirmed publication error and the publication child run with its receipt.
 * @see ../../../../📦️packages/🦀️rust/📜️script.ts
 */
import { terminateOwnedChildTree } from "@semio-tech/process/execution";
import { createHash } from "node:crypto";
import { closeSync, openSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { type FreshBuildControlV1 } from "@semio-tech/framework-plugin/describe/source-epoch";
import { hubSchemaExport } from "../../../../📦️packages/🟦️typescript/🟦️.ts";
import { documentOpenNeutralObject } from "../../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapPublicationRevision } from "../🔄️rotation-and-current-pointer/🟦️.ts";

/** 🧬️ Resolves the sole scope-owned publication receipt contract for the production decoder. */
function trustedCatalogPublicationReceiptContract(): (value: unknown) => boolean {
  return hubSchemaExport(resolve(dirname(fileURLToPath(import.meta.url)), "../../../.."), "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogPublicationReceiptV1");
}

/** 🧾️ Preserves both native outcomes while matching the exact request and next canonical revision. */
export function trustedBootstrapPublicationReceipt(bytes: Uint8Array, command: Readonly<Record<string, unknown>>, expectedRevision: string): Readonly<{ currentSha256: string; publicationRevision: string; outcome: "durable" | "replaced-unconfirmed" }> {
  if (bytes.byteLength === 0 || bytes.byteLength > 4096) throw new Error("trusted publication receipt exceeds its bound; reconcile current");
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  const row = documentOpenNeutralObject(JSON.parse(text), ["schema", "requestId", "profileId", "generationId", "bundleSha256", "publicationRevision", "currentSha256", "outcome"]);
  if (!trustedCatalogPublicationReceiptContract()(row)) throw new Error("trusted publication receipt does not satisfy its scope-owned contract; reconcile current");
  if (["requestId", "profileId", "generationId", "bundleSha256"].some((key) => row[key] !== command[key]) || row.publicationRevision !== expectedRevision || text !== JSON.stringify(row) + "\n")
    throw new Error("trusted publication receipt is not exact; reconcile current");
  const publicationRevision = trustedBootstrapPublicationRevision(row.publicationRevision);
  const pointer = { profileId: row.profileId, generationId: row.generationId, bundleSha256: row.bundleSha256, publicationRevision };
  const currentSha256 = createHash("sha256").update(JSON.stringify(pointer) + "\n").digest("hex");
  if (row.currentSha256 !== currentSha256) throw new Error("trusted publication receipt token differs from its exact pointer; reconcile current");
  return Object.freeze({ currentSha256, publicationRevision, outcome: row.outcome });
}

/** 🚦️ Keeps a validated visible replacement distinct from missing or malformed process evidence. */
export class TrustedBootstrapPublicationUnconfirmed extends Error {
  constructor(readonly receipt: ReturnType<typeof trustedBootstrapPublicationReceipt>, readonly evidenceRoot: string) {
    super(`trusted publication replacement is visible but durability is unconfirmed; do not activate or retry; evidence=${evidenceRoot}`);
  }
}

export type TrustedPublicationChildResultV1 = Readonly<{ bytes: Buffer; status: number | null; signal: string | null; failed: string }>;

/** 👶️ Retains one direct publication child until close with bounded evidence and cancellation. */
export async function runTrustedPublicationChild(binaryPath:string,args:readonly string[],input:Uint8Array,dataRoot:string,evidenceRoot:string,control:Pick<FreshBuildControlV1,"cancelled"|"remainingMs">&{progressed?:()=>void}):Promise<TrustedPublicationChildResultV1>{
 if(!isAbsolute(binaryPath)||!isAbsolute(dataRoot)||!isAbsolute(evidenceRoot)||input.byteLength===0||input.byteLength>4096)throw Error("publication child authority exceeds its boundary");
 return new Promise((accept,reject)=>{
  let stdout:number|undefined,stderr:number|undefined,child:ReturnType<typeof spawn>,timer:ReturnType<typeof setInterval>|undefined,closed=false,outputBytes=0,diagnosticBytes=0,failed="";const chunks:Buffer[]=[],failures:unknown[]=[];
  const record=(error:unknown)=>{if(!failures.some(value=>Object.is(value,error)))failures.push(error);};
  const closeFiles=()=>{for(const descriptor of[stdout,stderr])if(descriptor!==undefined)try{closeSync(descriptor);}catch(error){record(error);}stdout=stderr=undefined;};
  const settleFailure=()=>failures.length>1?new AggregateError(failures,"Publication operation and evidence close failed"):failures[0];
  try{stdout=openSync(join(evidenceRoot,"publication.stdout.json"),"wx",0o600);stderr=openSync(join(evidenceRoot,"publication.stderr.txt"),"wx",0o600);child=spawn(binaryPath,[...args],{env:{...process.env,OS_HUB_DATA:dataRoot},shell:false,windowsHide:true,stdio:["pipe","pipe","pipe"]});}catch(error){record(error);closeFiles();reject(settleFailure());return;}
  child.once("close",(status,signal)=>{closed=true;clearInterval(timer);closeFiles();if(failures.length){for(const chunk of chunks)chunk.fill(0);reject(settleFailure());return;}const bytes=Buffer.concat(chunks);for(const chunk of chunks)chunk.fill(0);accept({bytes,status,signal,failed});});
  const terminate=(reason:string)=>{failed||=reason;if(!closed)try{terminateOwnedChildTree(child);}catch(error){record(error);}};
  const guard=(body:()=>void)=>{if(closed)return;try{body();}catch(error){record(error);terminate("publication phase or evidence setup failed");}};
  guard(()=>{
   child.once("error",()=>terminate("publication child could not start"));
   child.stdin!.on("error",()=>terminate("publication command transport failed"));
   child.stdout!.on("data",(chunk:Buffer)=>guard(()=>{control.progressed?.();const retained=Buffer.from(chunk.subarray(0,Math.max(0,4096-outputBytes)));if(retained.length){writeSync(stdout!,retained);chunks.push(retained);}outputBytes+=chunk.length;if(outputBytes>4096)terminate("publication stdout exceeded its receipt bound");}));
   child.stderr!.on("data",(chunk:Buffer)=>guard(()=>{control.progressed?.();const retained=chunk.subarray(0,Math.max(0,65536-diagnosticBytes));if(retained.length)writeSync(stderr!,retained);diagnosticBytes+=chunk.length;if(diagnosticBytes>65536)terminate("publication diagnostics exceeded their bound");}));
   timer=setInterval(()=>guard(()=>{if(control.cancelled())terminate("publication cancelled");else if(control.remainingMs()<=0)terminate("publication timed out");}),100);
   if(control.cancelled()||control.remainingMs()<=0){terminate("publication phase already exhausted");return;}
   child.stdin!.end(input);
  });
 });
}

/** 🚥️ Classifies complete transport evidence before trusting any receipt prefix. */
export function trustedPublicationChildReceipt(result: TrustedPublicationChildResultV1, command: Readonly<Record<string, unknown>>, revision: string, evidenceRoot: string): ReturnType<typeof trustedBootstrapPublicationReceipt> {
  if (result.failed || result.signal || (result.status !== 0 && result.status !== 1)) throw new Error(`publication process outcome is indeterminate; reconcile current, do not retry or activate; evidence=${evidenceRoot}`);
  const receipt = trustedBootstrapPublicationReceipt(result.bytes, command, revision);
  if (receipt.outcome === "replaced-unconfirmed" && result.status === 1) throw new TrustedBootstrapPublicationUnconfirmed(receipt, evidenceRoot);
  if (receipt.outcome !== "durable" || result.status !== 0) throw new Error(`publication receipt and exit status disagree; reconcile current, do not retry or activate; evidence=${evidenceRoot}`);
  return receipt;
}
