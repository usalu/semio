import {createHash} from "node:crypto";
import {readFile,stat} from "node:fs/promises";
import {dirname,join} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {listOriginsV1,readEventSpansV1,readHeadV1,readThroughReplacementV1,replaceOriginFileV1,storePathsV1,type FleetCursorV1} from "../📒️journal/🟦️.ts";
import {importClosureV1,moduleSourcesV1,restartJitterMillisecondsV1,StoreStalledV1,type FailureSummaryV1} from "../🛡️resilience/🟦️.ts";
import gitSchema from "../🌿️git/🧬️schema/🔣️.json" with {type:"json"};
import type {GitStageObservationV1,GitStateV1} from "../🌿️git/🟦️.ts";
import {platformV1} from "../🗺️topology/🟦️.ts";
import {nextDelayV1,silentLatencyV1,type LatencyReportV1,type TierNameV1} from "../🎣️tiers/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪟️ The digest and counts of the working-tree view of one worker (`git status` as a git GUI shows it): entries with a staged change, worktree-modified, worktree-deleted and untracked entries; `stale` is true while the counts and the digest are not those of the working tree: they are those of the previous run of the daemon (kept until the first fresh `git status` arrives), or the daemon knows of changes (published, applied, a moved stage) that the view does not yet reflect; an origin with a stale view is not settled, so the fleet verdict waits (`lagging`) instead of reporting `diverged`; `digest` is the sha256 over the sorted `XY path` lines of everything not purely staged, joined by `\n`, which is also the sha256 of `origins/<origin>/view.txt`. */
export type FleetViewV1=Readonly<{staged:number;modified:number;deleted:number;untracked:number;digest:string;stale?:boolean}>;

/** 🛡️ What a worker measures about the inputs of its own view (see `GitGuardsV1` of the git module): the git `version`, the hash of the `pins`, the `volume` (case and normalisation behaviour, link capability, the Windows long path policy or null), and the counts `infoExclude`, `caseIgnore`, `caseMismatch`, `trackedIgnored`, `trackedRefused`, `links`, `overBudget` and `modeMismatch` (null where the executable bit cannot be read). */
export type FleetGuardsV1=Readonly<{version:string|null;pins:string;volume:Readonly<{caseInsensitive:boolean;normalisationInsensitive:boolean;symlinks:boolean;longPaths:boolean|null}>;infoExclude:number;caseIgnore:number;caseMismatch:number;trackedIgnored:number;trackedRefused:number;links:number;overBudget:number;modeMismatch:number|null}>;

/** 🩺️ What one worker states about itself in `origins/<origin>/attest.json`: identity, clocks, replication cursors, the order-independent digest of its shared files, the git state, the working-tree view, the guards of that view and the digest of its own code. */
export type FleetAttestationV1=Readonly<{version:1;originId:string;groupId:string;platform:string;wallMs:number;hlc:string;sync:boolean;code:string;cursors:Readonly<Record<string,FleetCursorV1>>;files:number;manifest:string;head:Readonly<{ref:string;commit:string}>|null;indexTree:string|null;view:FleetViewV1|null;guards:FleetGuardsV1|null;latency:LatencyReportV1;queue:Readonly<{incoming:number}>;checkpoints?:FleetCheckpointsV1;pending?:Readonly<{code:string;ok:false;reason:string}>;failures?:readonly FailureSummaryV1[];fallback?:FleetFallbackV1;legacy?:true}>;

/** 📍️ A cut through the event history: for every origin the position up to which the worker has read and folded its journal. The fold of the events below a cut is deterministic, so two workers that hold the same cut hold the same shared files. */
export type FleetCutV1=Readonly<Record<string,FleetCursorV1>>;

/** 📌️ The shared-file state of a worker at a cut at which it had applied everything it had read (empty incoming queue, no apply in flight): `manifest` is the digest over all shared files, `buckets` the same digest split into 256 buckets by path (2 bytes each, 1024 hex characters), `wallMs` the time of the cut. */
export type FleetManifestCheckpointV1=Readonly<{cursors:FleetCutV1;wallMs:number;files:number;manifest:string;buckets:string}>;

/** 🌿️ The git state of a worker (`HEAD` commit and stage tree) as read from its disk. Reading takes time while events keep arriving, so the reading belongs to no single cut: everything below `cursors` (a cut at which the worker had applied everything) had been applied, and nothing beyond `until` (the cut after the reading, `cursors` when equal) had been read. */
export type FleetGitCheckpointV1=Readonly<{cursors:FleetCutV1;until?:FleetCutV1;wallMs:number;head:string|null;indexTree:string|null}>;

/** 🪟️ The working-tree view of a worker, with the same span as the git state (`cursors` to `until`): `digest` over all lines, `buckets` the lines split by path into 256 buckets (2 bytes each), `pending` the 256-bit mask (64 hex characters, bit i is bucket i) of the buckets in which the daemon knew of a change that the view did not yet show. */
export type FleetViewCheckpointV1=Readonly<{cursors:FleetCutV1;until?:FleetCutV1;wallMs:number;digest:string;buckets:string;pending:string}>;

/** 🧭️ The newest checkpoints of one worker, one per component, each with its own cut: the shared files, the git state and the view are read at different moments, so each carries the cut at which it was read. A component is absent until the worker could take it once. */
export type FleetCheckpointsV1=Readonly<{manifest?:FleetManifestCheckpointV1;git?:FleetGitCheckpointV1;view?:FleetViewCheckpointV1}>;

/** 🪂️ The guard of a worker that runs a fallback code instead of the code of its worktree: `source` is the sealed `snapshot` of this machine or a snapshot that a healthy peer published to the `store`, `reason` why the worktree code was not used. */
export type FleetFallbackV1=Readonly<{guard:"code-fallback";source:"snapshot"|"store";reason:string}>;

/** 🔎️ The fields over which settled workers must agree: the manifest, the head commit, the stage tree, the view digest and the guards that produce a divergence (the pins hash, private exclude rules, case-dependent ignore rules, case mismatches, tracked files that an ignore rule matches, refused tracked paths and, among machines that can read it, the executable-bit mismatches). */
export type FleetFieldV1="code"|"manifest"|"head.commit"|"indexTree"|"view.digest"|"guards.pins"|"guards.infoExclude"|"guards.caseIgnore"|"guards.caseMismatch"|"guards.trackedIgnored"|"guards.trackedRefused"|"guards.modeMismatch";

/** ⚠️ A difference of capability that does not make a verdict fail: `git-version` (this origin runs another git than most), `volume` (another case or normalisation behaviour than most), `over-budget` (tracked paths longer than this operating system holds) and `symlinks` (tracked links on a volume that cannot create them). */
export type FleetWarningV1=Readonly<{originId:string;kind:"git-version"|"volume"|"over-budget"|"symlinks"|"code-fallback"|"guards-unknown"|"blobs"|"no-checkpoints"|"git-broken"|"stage"|"never-attested";detail:string}>;



/** ⚖️ The verdict of one worker: `ok`, `off` (sync off, excluded), `retired` (left the fleet for good, excluded), `missing` (no attestation), `stale`, `lagging` (not settled) or `diverged`. */
export type FleetVerdictV1="ok"|"off"|"retired"|"missing"|"stale"|"lagging"|"diverged";

/** 🪦️ The retirement record `origins/<origin>/retired.json`: `by` is the origin of the machine that wrote it, `retired` is true on retirement and false on the undo that re-admits the origin; the newer `hlc` wins when stores are relayed. */
export type RetirementV1=Readonly<{version:1;originId:string;by:string;hlc:string;wallMs:number;reason:string;retired:boolean}>;

/** 📋️ One row of the fleet table. */
export type FleetRowV1=Readonly<{originId:string;platform:string|null;sync:boolean|null;ageSeconds:number|null;behindOrigins:number|null;incoming:number|null;code:string|null;fresh:boolean;settled:boolean;view:FleetViewV1|null;latency:LatencyReportV1|null;verdict:FleetVerdictV1;differs:readonly FleetFieldV1[];basis?:"checkpoint";checkpointSeconds?:number|null}>;

/** 🧮️ One value that deviates from the value most settled workers hold. */
export type FleetDifferenceV1=Readonly<{originId:string;field:FleetFieldV1;value:string|null;expected:string|null}>;

/** 🗒️ The fleet verdict: `ok` exactly when every origin with sync on is fresh, settled and agrees. */
export type FleetReportV1=Readonly<{version:1;ok:boolean;freshSeconds:number;rows:readonly FleetRowV1[];differing:readonly FleetDifferenceV1[];warnings:readonly FleetWarningV1[]}>;

/** 🧱️ One term of the manifest digest: a shared path, its operation and the content hash (put) or target (link). */
export type FleetTermV1=Readonly<{path:string;operation:"put"|"link"|"delete";value:string}>;

/** ⏳️ How old an attestation may be to count as fresh. */
export const freshSecondsV1=60;

/** 🕰️ How far an attestation may lie in the future of the reader (clock skew) before it no longer counts as fresh: a dead writer whose clock runs ahead must not look alive. */
export const skewSecondsV1=30;

const realmBase=".cache/f/index-base",realmEntries=".cache/f/index/";
const validators=new Map<string,CompiledJsonSchemaSubset>();
const missing=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";

/** 🛡️ Validates a value against one definition of the attestation schema. */
export function requireAttestShapeV1(definition:"Attestation"|"Report"|"Retirement"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🚫️ Tells whether a path belongs to the virtual index realm; the realm is covered by `indexTree`, not by the manifest. */
export function realmPathV1(path:string):boolean{return path===realmBase||path.startsWith(realmEntries);}

/** 🧱️ The 32-byte term of one shared path, or `undefined` for a delete or a realm path, which contribute nothing: the sha256 of `path\0operation\0value`. */
export function manifestTermV1(term:FleetTermV1):Uint8Array|undefined{
 if(term.operation==="delete"||realmPathV1(term.path))return undefined;
 return createHash("sha256").update(`${term.path}\0${term.operation}\0${term.value}`).digest();
}

/** ✖️ Folds a term into a running digest: XOR is its own inverse, so adding and removing a term use the same call. */
export function foldTermV1(digest:Uint8Array,term:Uint8Array):void{for(let index=0;index<32;index++)digest[index]=(digest[index]??0)^(term[index]??0);}

/** 🔤️ Spells a running digest as lower-case hex. */
export function digestHexV1(digest:Uint8Array):string{return Buffer.from(digest.buffer,digest.byteOffset,32).toString("hex");}

/** 🧮️ Computes the manifest digest from scratch; the replica maintains the same value incrementally. */
export function recomputeManifestV1(terms:Iterable<FleetTermV1>):Readonly<{manifest:string;files:number}>{
 const digest=new Uint8Array(32);
 let files=0;
 for(const term of terms){const folded=manifestTermV1(term);if(folded!==undefined){foldTermV1(digest,folded);files++;}}
 return {manifest:digestHexV1(digest),files};
}

/** 🪣️ The number of buckets a digest is split into by path. */
export const bucketCountV1=256;

const bucketWidth=2;

/** 🪣️ The bucket of a path: the low eight bits of the 32-bit FNV-1a hash over the UTF-8 bytes of the NFC spelling, the same on every platform and language. */
export function bucketOfV1(path:string):number{
 let hash=0x811c9dc5;
 for(const byte of Buffer.from(path.normalize("NFC"),"utf8")){hash^=byte;hash=Math.imul(hash,0x01000193);}
 return (hash>>>0)&(bucketCountV1-1);
}

/** 🆕 Creates empty buckets (256 times 2 zero bytes). */
export function emptyBucketsV1():Uint8Array{return new Uint8Array(bucketCountV1*bucketWidth);}

/** ✖️ Folds the first two bytes of a term into the bucket of its path; like the digest it is its own inverse. */
export function foldBucketV1(buckets:Uint8Array,bucket:number,term:Uint8Array):void{for(let index=0;index<bucketWidth;index++){const at=bucket*bucketWidth+index;buckets[at]=(buckets[at]??0)^(term[index]??0);}}

/** 🔤️ Spells buckets as 1024 lower-case hex characters. */
export function bucketsHexV1(buckets:Uint8Array):string{return Buffer.from(buckets.buffer,buckets.byteOffset,buckets.length).toString("hex");}

/** 🧮️ The buckets of the manifest terms, from scratch; the replica maintains the same value incrementally. */
export function recomputeBucketsV1(terms:Iterable<FleetTermV1>):string{
 const buckets=emptyBucketsV1();
 for(const term of terms){const folded=manifestTermV1(term);if(folded!==undefined)foldBucketV1(buckets,bucketOfV1(term.path),folded);}
 return bucketsHexV1(buckets);
}

/** 🪟️ The buckets of the lines of a working-tree view (`XY path`, bucket by the path after the third character, term = sha256 of the line). */
export function viewBucketsV1(lines:readonly string[]):string{
 const buckets=emptyBucketsV1();
 for(const line of lines)foldBucketV1(buckets,bucketOfV1(line.slice(3)),createHash("sha256").update(line).digest());
 return bucketsHexV1(buckets);
}

/** 🎭️ Spells a set of buckets as a 256-bit mask in 64 hex characters (bit i is bucket i). */
export function bucketMaskV1(buckets:Iterable<number>):string{
 let mask=0n;
 for(const bucket of buckets)mask|=1n<<BigInt(bucket);
 return mask.toString(16).padStart(bucketCountV1/4,"0");
}

/** 🎭️ The buckets named by a mask. */
export function maskBucketsV1(mask:string):ReadonlySet<number>{
 const value=BigInt(`0x${mask}`),found=new Set<number>();
 for(let bucket=0;bucket<bucketCountV1;bucket++)if((value>>BigInt(bucket))&1n)found.add(bucket);
 return found;
}

/** 🎭️ The mask of every bucket: a view in which nothing can be compared. */
export const everyBucketMaskV1="f".repeat(bucketCountV1/4);

/** 🌿️ Tells whether a path decides the `HEAD` commit, the stage or what a view calls tracked, ignored or attributed: the virtual index realm, `.git/HEAD`, `.git/refs/heads/**`, `.git/packed-refs`, `.git/shallow` and every `.gitignore`, `.gitattributes` and `.gitmodules`. Git objects, remote-tracking refs and tags (an auto-fetch writes them all the time) change none of them. */
export function gitPathV1(path:string):boolean{return realmPathV1(path)||path===".git/HEAD"||path===".git/packed-refs"||path===".git/shallow"||path.startsWith(".git/refs/heads/")||/(?:^|\/)\.git(?:ignore|attributes|modules)$/u.test(path);}

/** 🧾️ One event of a journal, as far as the comparison needs it: the cursor behind it, its path and the wall-clock milliseconds in its hybrid logical clock. */
export type GapEventV1=Readonly<{end:FleetCursorV1;path:string;ms:number}>;

/** 🪟️ The events of one origin between two cursors (`from` exclusive, `to` inclusive), in journal order. */
export type GapWindowV1=Readonly<{from:FleetCursorV1;to:FleetCursorV1;events:readonly GapEventV1[]}>;

/** 📏️ The part of the history a state may stem from: everything below `from` is in it, nothing beyond `to` is. A checkpoint taken at one cut has `from` equal to `to`. */
export type FleetSpanV1=Readonly<{from:FleetCutV1;to:FleetCutV1}>;

/** 📏️ The span of a checkpoint. */
export const spanOfV1=(checkpoint:Readonly<{cursors:FleetCutV1;until?:FleetCutV1}>):FleetSpanV1=>({from:checkpoint.cursors,to:checkpoint.until??checkpoint.cursors});

/** 🕳️ The events that lie between two spans: how many, the buckets their paths fall into, and whether one of them touches the git state. */
export type FleetGapV1=Readonly<{events:number;buckets:ReadonlySet<number>;git:boolean}>;

const startOfJournal:FleetCursorV1={segment:0,bytes:0};
const behind=(left:FleetCursorV1,right:FleetCursorV1):boolean=>left.segment<right.segment||left.segment===right.segment&&left.bytes<right.bytes;

/** 🕳️ The events that two spans do not certainly share: per origin those after the lowest `from` up to the highest `to`. Everything the two workers agree on must be equal outside the buckets those events touch, because the fold of a set of events is deterministic. `undefined` when a window does not cover a difference (the journal was compacted, is unreadable, or the difference is too large). */
export function gapBetweenV1(left:FleetSpanV1,right:FleetSpanV1,windows:Readonly<Record<string,GapWindowV1|undefined>>):FleetGapV1|undefined{
 const buckets=new Set<number>();
 let events=0,git=false;
 for(const origin of new Set([...Object.keys(left.from),...Object.keys(left.to),...Object.keys(right.from),...Object.keys(right.to)])){
  const lows=[left.from[origin]??startOfJournal,right.from[origin]??startOfJournal],highs=[left.to[origin]??startOfJournal,right.to[origin]??startOfJournal],low=behind(lows[0]!,lows[1]!)?lows[0]!:lows[1]!,high=behind(highs[0]!,highs[1]!)?highs[1]!:highs[0]!;
  if(!behind(low,high))continue;
  const window=windows[origin];
  if(window===undefined||behind(low,window.from)||behind(window.to,high))return undefined;
  for(const event of window.events)if(behind(low,event.end)&&!behind(high,event.end)){events++;buckets.add(bucketOfV1(event.path));if(gitPathV1(event.path))git=true;}
 }
 return {events,buckets,git};
}

/** ⏱️ The wall-clock time of the oldest event that lies behind the heads of the store but not below `cut`: `null` when the cut has seen everything, `undefined` when the windows do not cover the difference. */
export function oldestUnseenV1(cut:FleetCutV1,heads:FleetCutV1,windows:Readonly<Record<string,GapWindowV1|undefined>>):number|null|undefined{
 let oldest:number|null=null;
 for(const [origin,head] of Object.entries(heads)){
  const at=cut[origin]??startOfJournal;
  if(!behind(at,head))continue;
  const window=windows[origin];
  if(window===undefined||behind(at,window.from)||behind(window.to,head))return undefined;
  for(const event of window.events)if(behind(at,event.end)&&!behind(head,event.end))oldest=oldest===null?event.ms:Math.min(oldest,event.ms);
 }
 return oldest;
}

/** 📍️ A copy of the cursors of a replica as a cut, in the order of the origin ids. */
export function cutOfV1(cursors:Readonly<Record<string,FleetCursorV1>>):FleetCutV1{
 return Object.fromEntries(Object.entries(cursors).sort(([one],[two])=>one<two?-1:1).map(([origin,cursor])=>[origin,{segment:cursor.segment,bytes:cursor.bytes}]));
}

/** 🟰️ Tells whether two cuts name the same position for every origin. */
export function sameCutV1(left:FleetCutV1,right:FleetCutV1):boolean{
 for(const origin of new Set([...Object.keys(left),...Object.keys(right)])){
  const one=left[origin]??startOfJournal,two=right[origin]??startOfJournal;
  if(one.segment!==two.segment||one.bytes!==two.bytes)return false;
 }
 return true;
}

/** ✋️ How long a named but unpublished change of the git state keeps the git and view readings from being taken: past it the name is taken for noise (a git GUI that refreshes `.git/index` all the time) and the readings go on; a change that stays unpublished for so long is a divergence worth naming. */
const holdGitMilliseconds=30_000;

/** 📖️ The most events and the most segments of one origin that the comparison reads. */
export const gapLimitsV1=Object.freeze({events:50_000,segments:8});

/** 📖️ Reads, for every origin whose position differs between the cuts, the events between its lowest and its highest position. An origin whose journal cannot be read back to the lowest position, or not up to the highest, or that exceeds the limits, gets no window; a comparison that needs it is not decided. */
export async function readGapWindowsV1(storeRoot:string,cuts:readonly FleetCutV1[],signal?:AbortSignal):Promise<Readonly<Record<string,GapWindowV1>>>{
 const windows:Record<string,GapWindowV1>={};
 for(const origin of new Set(cuts.flatMap(cut=>Object.keys(cut)))){
  signal?.throwIfAborted();
  let low=cuts[0]![origin]??startOfJournal,high=low;
  for(const cut of cuts){const at=cut[origin]??startOfJournal;if(behind(at,low))low=at;if(behind(high,at))high=at;}
  if(!behind(low,high)||high.segment-low.segment>=gapLimitsV1.segments)continue;
  try{
   const read=await readEventSpansV1(storeRoot,origin,low,high.segment-low.segment+1,undefined,gapLimitsV1.events);
   if(behind(read.cursor,high))continue;
   windows[origin]={from:low,to:high,events:read.events.flatMap((event,index)=>behind(high,read.ends[index]!)?[]:[{end:read.ends[index]!,path:event.path,ms:Number(event.hlc.slice(0,13))}])};
  }catch(error){if(signal?.aborted===true)throw error;}
 }
 return windows;
}

/** 🧬️ The sha256 over the module sources and the files they import: first every `🟦️.ts` and `📜️script.ts` below the module directory (tests excluded, sorted by relative path), then every other file those sources import, directly or through each other (shared libraries, JSON schemas and text catalogues, sorted by the path relative to the module directory); each file contributes `path\0length\0bytes`. */
export async function codeDigestV1(root:string):Promise<string>{
 const own=await moduleSourcesV1(root),hash=createHash("sha256");
 for(const path of [...own,...await importClosureV1(root,own)]){
  const data=await readFile(join(root,...path.split("/")));
  hash.update(`${path}\0${data.length}\0`);hash.update(data);
 }
 return hash.digest("hex");
}

/** 📌️ The module directory of the running code. */
export const codeRootV1=dirname(import.meta.dirname);

/** 🔭️ A cached reader of the code digest: it rereads the sources only when a size or modification time of a source or of a file they import changed. */
export function createCodeProbeV1(root:string=codeRootV1):()=>Promise<string>{
 let signature="",digest="",closure:string[]=[];
 const stamp=async(paths:readonly string[]):Promise<string>=>{
  const parts:string[]=[];
  for(const path of paths){const info=await stat(join(root,...path.split("/"))).catch(()=>undefined);parts.push(`${path}:${info?.size}:${info?.mtimeMs}`);}
  return parts.join("\n");
 };
 return async()=>{
  const own=await moduleSourcesV1(root),next=`${await stamp(own)}\n--\n${await stamp(closure)}`;
  if(next!==signature||digest===""){
   closure=await importClosureV1(root,own);
   digest=await codeDigestV1(root);
   signature=`${await stamp(own)}\n--\n${await stamp(closure)}`;
  }
  return digest;
 };
}

/** 📍️ The path of the attestation of an origin. */
export function attestPathV1(storeRoot:string,originId:string):string{return join(storePathsV1(storeRoot).origins,originId,"attest.json");}

/** ✍️ Replaces the attestation of the own origin through tmp + fsync + rename in the store, like `head.json`; nothing else in the store is touched. */
export async function writeAttestationV1(storeRoot:string,attestation:FleetAttestationV1):Promise<void>{
 requireAttestShapeV1("Attestation",attestation,"Closed fleet attestation required");
 await replaceOriginFileV1(storeRoot,attestation.originId,"attest.json",JSON.stringify(attestation));
}

/** 🪙 Reads the attestation of code that predates the guards (and, older still, the latency report) as present with unknown guards: `guards` is `null`, a missing latency is silent, and the result is marked `legacy` so that `fleet` compares it and warns instead of calling it missing. */
const olderV1=(value:unknown):{attestation:unknown;legacy:boolean}=>{
 if(typeof value!=="object"||value===null||Array.isArray(value)||(value as {version?:unknown}).version!==1)return {attestation:value,legacy:false};
 const record=value as Record<string,unknown>;
 if("guards" in record)return {attestation:value,legacy:false};
 return {attestation:{...record,guards:null,..."latency" in record?{}:{latency:silentLatencyV1()}},legacy:true};
};

/** 👀️ Reads the attestation of one origin; `undefined` when there is none, it is torn or invalid, or it was replaced under every attempt to read it. */
export async function readAttestationV1(storeRoot:string,originId:string):Promise<FleetAttestationV1|undefined>{
 const text=await readThroughReplacementV1(()=>readFile(attestPathV1(storeRoot,originId),"utf8"));
 if(text===undefined)return undefined;
 try{
  const {attestation,legacy}=olderV1(JSON.parse(text) as unknown);
  requireAttestShapeV1("Attestation",attestation,"Closed fleet attestation required");
  const found=attestation as FleetAttestationV1;
  return found.originId===originId?legacy?{...found,legacy:true}:found:undefined;
 }catch{return undefined;}
}

/** 📚️ Reads the attestations of all origins of a store. */
export async function readAttestationsV1(storeRoot:string,signal?:AbortSignal):Promise<Readonly<Record<string,FleetAttestationV1>>>{
 const found:Record<string,FleetAttestationV1>={};
 for(const origin of await listOriginsV1(storeRoot)){
  signal?.throwIfAborted();
  const attestation=await readAttestationV1(storeRoot,origin);
  if(attestation!==undefined)found[origin]=attestation;
 }
 return found;
}

/** 📍️ The path of the working-tree view of an origin: the sorted `XY path` lines of everything that is not purely staged, joined by `\n` without a trailing newline, so that its sha256 is `view.digest`. */
export function viewPathV1(storeRoot:string,originId:string):string{return join(storePathsV1(storeRoot).origins,originId,"view.txt");}

/** ✍️ Replaces the view file of the own origin through tmp + fsync + rename in the store. */
export async function writeViewLinesV1(storeRoot:string,originId:string,lines:readonly string[]):Promise<void>{
 await replaceOriginFileV1(storeRoot,originId,"view.txt",lines.join("\n"));
}

/** 👀️ Reads the view lines of one origin; `undefined` when there is no file, or when its digest is not the one `expected` (a torn or outdated copy). */
export async function readViewLinesV1(storeRoot:string,originId:string,expected?:string):Promise<readonly string[]|undefined>{
 const text=await readThroughReplacementV1(()=>readFile(viewPathV1(storeRoot,originId),"utf8"));
 if(text===undefined)return undefined;
 if(expected!==undefined&&createHash("sha256").update(text).digest("hex")!==expected)return undefined;
 return text===""?[]:text.split("\n");
}

/** 🧾️ The lines only another origin shows and the lines only this machine shows, each in the order of the input. */
export type ViewDifferenceV1=Readonly<{originId:string;onlyThere:readonly string[];onlyHere:readonly string[]}>;

/** 🔬️ The result of the view comparison: `here` is the digest of this machine (`null` when it attests no view or its file is missing), `differing` lists every origin whose lines differ, `unavailable` every origin that attests a view but whose file is missing or does not match its digest. */
export type ViewDiffReportV1=Readonly<{here:string|null;differing:readonly ViewDifferenceV1[];unavailable:readonly string[]}>;

/** 🔬️ Compares the view lines of this machine with those of every other origin that is not retired and attests a view (an origin without git is exempt). Each side is the multiset difference of the sorted lines. */
export async function viewDiffV1(storeRoot:string,originId:string,signal?:AbortSignal):Promise<ViewDiffReportV1>{
 const attestations=await readAttestationsV1(storeRoot,signal),retirements=await readRetirementsV1(storeRoot,signal),own=attestations[originId]?.view??null,here=own===null?undefined:await readViewLinesV1(storeRoot,originId,own.digest);
 const differing:ViewDifferenceV1[]=[],unavailable:string[]=[];
 if(here===undefined)return {here:null,differing,unavailable};
 const count=(lines:readonly string[]):Map<string,number>=>{const found=new Map<string,number>();for(const line of lines)found.set(line,(found.get(line)??0)+1);return found;};
 const left=(lines:readonly string[],other:Map<string,number>):string[]=>{const rest=new Map(other),kept:string[]=[];for(const line of lines){const have=rest.get(line)??0;if(have>0)rest.set(line,have-1);else kept.push(line);}return kept;};
 for(const [origin,attestation] of Object.entries(attestations).sort(([one],[two])=>one<two?-1:1)){
  signal?.throwIfAborted();
  if(origin===originId||attestation.view===null||retirements[origin]?.retired===true)continue;
  const there=await readViewLinesV1(storeRoot,origin,attestation.view.digest);
  if(there===undefined){unavailable.push(origin);continue;}
  if(attestation.view.digest===own!.digest)continue;
  differing.push({originId:origin,onlyThere:left(there,count(here)),onlyHere:left(here,count(there))});
 }
 return {here:own!.digest,differing,unavailable};
}

/** 📍️ The path of the retirement record of an origin. */
export function retiredPathV1(storeRoot:string,originId:string):string{return join(storePathsV1(storeRoot).origins,originId,"retired.json");}

/** ✍️ Replaces the retirement record of an origin through tmp + fsync + rename in the store. This is the one deliberate exception to "own origin directory only": a machine retires or re-admits another origin, because that origin is silent and cannot write itself. */
export async function writeRetirementV1(storeRoot:string,retirement:RetirementV1):Promise<void>{
 requireAttestShapeV1("Retirement",retirement,"Closed fleet retirement required");
 await replaceOriginFileV1(storeRoot,retirement.originId,"retired.json",JSON.stringify(retirement));
}

/** 👀️ Reads the retirement record of one origin; `undefined` when there is none or it is invalid. */
export async function readRetirementV1(storeRoot:string,originId:string):Promise<RetirementV1|undefined>{
 const text=await readThroughReplacementV1(()=>readFile(retiredPathV1(storeRoot,originId),"utf8"));
 if(text===undefined)return undefined;
 try{
  const value=JSON.parse(text) as unknown;
  requireAttestShapeV1("Retirement",value,"Closed fleet retirement required");
  return (value as RetirementV1).originId===originId?value as RetirementV1:undefined;
 }catch{return undefined;}
}

/** 📚️ Reads the retirement records of all origins of a store, undone ones included. */
export async function readRetirementsV1(storeRoot:string,signal?:AbortSignal):Promise<Readonly<Record<string,RetirementV1>>>{
 const found:Record<string,RetirementV1>={};
 for(const origin of await listOriginsV1(storeRoot)){
  signal?.throwIfAborted();
  const retirement=await readRetirementV1(storeRoot,origin);
  if(retirement!==undefined)found[origin]=retirement;
 }
 return found;
}

const valueOf=(attestation:FleetAttestationV1,field:FleetFieldV1):string|null=>{
 if(field==="code")return attestation.code;
 if(field==="manifest")return attestation.manifest;
 if(field==="head.commit")return attestation.head?.commit??null;
 if(field==="indexTree")return attestation.indexTree;
 if(field==="view.digest")return attestation.view?.digest??null;
 const guards=attestation.guards;
 if(guards===null)return null;
 const value=guards[field.slice(7) as "pins"|"infoExclude"|"caseIgnore"|"caseMismatch"|"trackedIgnored"|"trackedRefused"|"modeMismatch"];
 return value===null?null:String(value);
};
const fields:readonly FleetFieldV1[]=["code","manifest","head.commit","indexTree","view.digest","guards.pins","guards.infoExclude","guards.caseIgnore","guards.caseMismatch","guards.trackedIgnored","guards.trackedRefused","guards.modeMismatch"];
const majority=(values:readonly string[]):string|undefined=>{
 const counts=new Map<string,number>();
 for(const value of values)counts.set(value,(counts.get(value)??0)+1);
 let best:string|undefined,most=0;
 for(const [value,count] of [...counts].sort((left,right)=>left[0]<right[0]?-1:1))if(count>most){best=value;most=count;}
 return best;
};
const blobsWarnMilliseconds=120_000,brokenWindowMilliseconds=120_000;

/** 🩹 The reason an origin states for a git repository it cannot read (`git-broken-<reason>` among its failures, noted while the daemon finds it broken), `undefined` when it states none within the last two minutes. */
const brokenReasonV1=(attestation:FleetAttestationV1):string|undefined=>attestation.failures?.find(failure=>failure.kind.startsWith("git-broken-")&&attestation.wallMs-failure.lastMs<brokenWindowMilliseconds)?.kind.slice(11);
/** 🪞 The recent reason a stage waits or cannot be certified. A later exact tree supersedes an observation failure. */
const stageReasonV1=(attestation:FleetAttestationV1):string|undefined=>attestation.failures?.find(failure=>failure.kind.startsWith("stage-")&&(attestation.indexTree===null||!failure.kind.startsWith("stage-observe-"))&&attestation.wallMs-failure.lastMs<brokenWindowMilliseconds)?.kind.slice(6);
const capabilities=(origins:readonly string[],attestations:Readonly<Record<string,FleetAttestationV1>>):FleetWarningV1[]=>{
 const warnings:FleetWarningV1[]=[],measured=origins.filter(origin=>attestations[origin]!.guards!==null);
 const usual=majority(measured.map(origin=>attestations[origin]!.guards!.version??"-")),volume=(origin:string):string=>{const found=attestations[origin]!.guards!.volume;return `case=${found.caseInsensitive?"insensitive":"sensitive"} normalisation=${found.normalisationInsensitive?"insensitive":"sensitive"}`;},common=majority(measured.map(volume));
 for(const origin of measured){
  const guards=attestations[origin]!.guards!;
  if((guards.version??"-")!==usual)warnings.push({originId:origin,kind:"git-version",detail:guards.version??"-"});
  if(volume(origin)!==common)warnings.push({originId:origin,kind:"volume",detail:volume(origin)});
  if(guards.overBudget>0)warnings.push({originId:origin,kind:"over-budget",detail:String(guards.overBudget)});
  if(!guards.volume.symlinks&&guards.links>0)warnings.push({originId:origin,kind:"symlinks",detail:String(guards.links)});
 }
 for(const origin of origins){const item=attestations[origin]!;if(item.legacy===true)warnings.push({originId:origin,kind:"guards-unknown",detail:item.code.slice(0,12)});}
 for(const origin of origins){const item=attestations[origin]!,wanted=item.failures?.find(failure=>failure.kind==="blob-wanted"&&item.wallMs-failure.lastMs<blobsWarnMilliseconds);if(wanted!==undefined)warnings.push({originId:origin,kind:"blobs",detail:String(wanted.count)});}
 for(const origin of origins){const fallback=attestations[origin]!.fallback;if(fallback!==undefined)warnings.push({originId:origin,kind:"code-fallback",detail:`${fallback.source}: ${fallback.reason}`.slice(0,200)});}
 return warnings;
};

type TrackV1="manifest"|"git"|"view";
type ItemV1=Readonly<{field:FleetFieldV1;bucket:number;left:string;right:string;same:boolean}>;
type ComparisonV1=Readonly<{items:readonly ItemV1[];known:ReadonlySet<TrackV1>}>;
const recentAt=(now:number,wallMs:number):boolean=>{const raw=Math.floor((now-wallMs)/1000);return raw<=freshSecondsV1&&raw>=-skewSecondsV1;};
const sliceOf=(buckets:string,bucket:number):string=>buckets.slice(bucket*4,bucket*4+4);
const everyBucket=Array.from({length:bucketCountV1},(_,bucket)=>bucket);
const comparedFields:readonly FleetFieldV1[]=["manifest","head.commit","indexTree","view.digest"];
const spell=(bucket:number,value:string):string=>bucket<0?value:`#${bucket}=${value}`;

/** 🔬️ Compares two workers at the cuts of their checkpoints, component by component, and returns every value compared as an item (`same` or not) together with the components that could be decided. A component is *known* when both checkpoints are at most 60 s old and the events between the two spans are known and leave something to compare; then everything outside the buckets those events touch must be equal. Shared files: the buckets no event between the spans touched (all of them when the spans are the same point). Git state: compare `HEAD` when no intervening event touches Git. A null stage tree beside a known `HEAD` leaves the stage undecided, so Git is known only when both stages can be compared. View: the buckets that no event between the spans and no pending change on either side touches, undecided when an event touches the git state; a worker without view is exempt. */
export function compareCheckpointsV1(left:FleetCheckpointsV1,right:FleetCheckpointsV1,now:number,windows:Readonly<Record<string,GapWindowV1|undefined>>):ComparisonV1{
 const items:ItemV1[]=[],known=new Set<TrackV1>();
 const spread=(field:FleetFieldV1,skip:ReadonlySet<number>,one:string,two:string):void=>{for(const bucket of everyBucket)if(!skip.has(bucket))items.push({field,bucket,left:sliceOf(one,bucket),right:sliceOf(two,bucket),same:sliceOf(one,bucket)===sliceOf(two,bucket)});};
 const one=left.manifest,two=right.manifest;
 if(one!==undefined&&two!==undefined&&recentAt(now,one.wallMs)&&recentAt(now,two.wallMs)){
  const gap=gapBetweenV1(spanOfV1(one),spanOfV1(two),windows);
  if(gap!==undefined&&(gap.events===0||gap.buckets.size<bucketCountV1)){
   known.add("manifest");
   spread("manifest",gap.buckets,one.buckets,two.buckets);
  }
 }
 const first=left.git,second=right.git;
 if(first!==undefined&&second!==undefined&&recentAt(now,first.wallMs)&&recentAt(now,second.wallMs)){
  const gap=gapBetweenV1(spanOfV1(first),spanOfV1(second),windows);
  if(gap!==undefined&&!gap.git){
   items.push({field:"head.commit",bucket:-1,left:first.head??"-",right:second.head??"-",same:first.head===second.head});
   if((first.indexTree!==null||first.head===null)&&(second.indexTree!==null||second.head===null)){
    known.add("git");
    items.push({field:"indexTree",bucket:-1,left:first.indexTree??"-",right:second.indexTree??"-",same:first.indexTree===second.indexTree});
   }
  }
 }
 const above=left.view,below=right.view;
 if(above===undefined||below===undefined)known.add("view");
 else if(recentAt(now,above.wallMs)&&recentAt(now,below.wallMs)){
  const gap=gapBetweenV1(spanOfV1(above),spanOfV1(below),windows);
  if(gap!==undefined&&!gap.git){
   const skip=new Set([...gap.buckets,...maskBucketsV1(above.pending),...maskBucketsV1(below.pending)]);
   if(skip.size<bucketCountV1){
    known.add("view");
    spread("view.digest",skip,above.buckets,below.buckets);
   }
  }
 }
 return {items,known};
}

/** 🧮️ A pair of workers whose values differ in one compared component (`left` is the value of `one`, `right` that of `two`). */
export type ApartPairV1=Readonly<{one:string;two:string;left:string;right:string}>;

/** 🪢️ Picks, for one compared component, the workers that disagree with the rest: the worker with the most differing partners (less its agreeing partners that are still counted) is a victim, its pairs leave, and the next one is picked from what is left. Workers that tie for the most while they differ from each other are named `ambiguous` and their mutual pairs leave without a victim (a 2-against-2 split cannot say who is wrong). Every pass removes at least one pair, so the loop ends after at most `apart.length` passes: tied workers that differ from everybody else but not from each other (two outliers against a majority) used to repeat the same pass forever and held the event loop of the daemon for good. */
export function separateOutliersV1(apart:readonly ApartPairV1[],alike:readonly Readonly<{one:string;two:string}>[],pointed:number):Readonly<{victims:readonly Readonly<{origin:string;first:ApartPairV1}>[];ambiguous:readonly string[]}>{
 const victims:{origin:string;first:ApartPairV1}[]=[],ambiguous=new Set<string>(),removed=new Set<string>();
 for(let rest=apart;rest.length>0;){
  const score=new Map<string,number>();
  for(const item of rest){score.set(item.one,(score.get(item.one)??0)+1);score.set(item.two,(score.get(item.two)??0)+1);}
  for(const item of alike){
   if(removed.has(item.one)||removed.has(item.two))continue;
   if(score.has(item.one))score.set(item.one,score.get(item.one)!-1);
   if(score.has(item.two))score.set(item.two,score.get(item.two)!-1);
  }
  let most=-Infinity,tied:string[]=[];
  for(const [origin,value] of [...score].sort((left,right)=>left[0]<right[0]?-1:1)){if(value>most){most=value;tied=[origin];}else if(value===most)tied.push(origin);}
  if(tied.length>1&&pointed>=3){
   const kept=rest.filter(item=>!(tied.includes(item.one)&&tied.includes(item.two)));
   if(kept.length<rest.length){
    for(const origin of tied)ambiguous.add(origin);
    rest=kept;
    continue;
   }
  }
  const victim=tied[tied.length-1]!;
  victims.push({origin:victim,first:rest.find(item=>item.one===victim||item.two===victim)!});
  removed.add(victim);
  rest=rest.filter(item=>item.one!==victim&&item.two!==victim);
 }
 return {victims,ambiguous:[...ambiguous]};
}

/** ⚖️ The pure fleet verdict. An origin whose newest retirement record says `retired` has the verdict `retired`: it is not compared, does not need to be fresh or settled, and the other origins need not have followed its head. An origin is *fresh* when its attestation is at most 60 s old (an attestation more than 30 s in the reader's future is not). Two kinds of origin are compared. A worker that attests checkpoints (`basis: "checkpoint"`) is compared at the cuts of its checkpoints, so that a fleet in which edits never stop can agree: it is *settled* when its shared-file checkpoint is at most 60 s old, it need not be at the head of the store; every pair of such workers is compared by `compareCheckpointsV1` (the events between their cuts come from `windows`; without a window a pair that differs in position is undecided); a worker that differs from most of the others at a decided component is `diverged` in it (the one with the most disagreements goes first, the larger origin id on a tie, and the rest are judged again), one for which some component could not be decided against any other worker is `lagging`. A worker without checkpoints (older code) is compared the old way: it is *settled* when its cursors equal every head of the store, its incoming queue is empty and its view is not `stale`, and the settled ones must agree on `manifest`, `head.commit`, `indexTree` and `view.digest`; such a worker and a checkpointed one are not compared with each other (a `no-checkpoints` warning says so). All live workers must agree on `code` and the divergence-producing `guards.*` fields; differences of capability (git version, volume behaviour, paths over the budget, links without link support) are `warnings` and never change a verdict (an origin whose `view` is `null` has no git and is exempt from the view, one whose `guards` are `null` from every `guards.*` field, as is a `null` `modeMismatch`); the value held by most of them (the smaller on a tie) is the expected one. `ok` needs every origin with sync on that is not retired to be `ok`; an origin with a head but no attestation is `missing`. */
export function judgeFleetV1(input:Readonly<{now:number;heads:Readonly<Record<string,Readonly<{segment:number;bytes:number}>>>;attestations:Readonly<Record<string,FleetAttestationV1>>;retirements:Readonly<Record<string,RetirementV1>>;windows?:Readonly<Record<string,GapWindowV1|undefined>>;registered?:readonly string[]}>):FleetReportV1{
 const retired=new Set(Object.entries(input.retirements).filter(([,item])=>item.retired).map(([origin])=>origin)),origins=[...new Set([...Object.keys(input.heads),...Object.keys(input.attestations),...retired])].sort();
 const rows=new Map<string,FleetRowV1>(),state=new Map<string,{fresh:boolean;settled:boolean;live:boolean;checkpoint:boolean}>();
 for(const origin of origins){
  const attestation=input.attestations[origin];
  if(attestation===undefined){rows.set(origin,{originId:origin,platform:null,sync:null,ageSeconds:null,behindOrigins:null,incoming:null,code:null,fresh:false,settled:false,view:null,latency:null,verdict:retired.has(origin)?"retired":"missing",differs:[]});state.set(origin,{fresh:false,settled:false,live:false,checkpoint:false});continue;}
  const raw=Math.floor((input.now-attestation.wallMs)/1000),age=Math.max(0,raw);
  let behind=0;
  for(const [other,head] of Object.entries(input.heads)){if(retired.has(other))continue;const cursor=attestation.cursors[other];if(cursor===undefined||cursor.segment!==head.segment||cursor.bytes!==head.bytes)behind++;}
  const checkpoint=attestation.checkpoints!==undefined,taken=attestation.checkpoints?.manifest,fresh=raw<=freshSecondsV1&&raw>=-skewSecondsV1,unseen=taken===undefined?undefined:oldestUnseenV1(taken.cursors,Object.fromEntries(Object.entries(input.heads).filter(([other])=>!retired.has(other))),input.windows??{}),settled=checkpoint?taken!==undefined&&recentAt(input.now,taken.wallMs)&&unseen!==undefined&&(unseen===null||input.now-unseen<=freshSecondsV1*1000):behind===0&&attestation.queue.incoming===0&&attestation.view?.stale!==true;
  state.set(origin,{fresh,settled,live:attestation.sync&&fresh&&settled&&!retired.has(origin),checkpoint});
  rows.set(origin,{originId:origin,platform:attestation.platform,sync:attestation.sync,ageSeconds:age,behindOrigins:behind,incoming:attestation.queue.incoming,code:attestation.code.slice(0,12),fresh,settled,view:attestation.view,latency:attestation.latency,verdict:retired.has(origin)?"retired":!attestation.sync?"off":!fresh?"stale":!settled?"lagging":"ok",differs:[],...checkpoint?{basis:"checkpoint" as const,checkpointSeconds:taken===undefined?null:Math.max(0,Math.floor((input.now-taken.wallMs)/1000))}:{}});
 }
 const differing:FleetDifferenceV1[]=[],diverged=new Map<string,FleetFieldV1[]>();
 const live=origins.filter(origin=>state.get(origin)!.live),counted=live.filter(origin=>!state.get(origin)!.checkpoint),pointed=live.filter(origin=>state.get(origin)!.checkpoint);
 for(const field of fields){
  const pool=field==="code"||field.startsWith("guards.")?live:counted,voters=field==="manifest"||field==="head.commit"||field==="indexTree"?pool:pool.filter(origin=>valueOf(input.attestations[origin]!,field)!==null),counts=new Map<string|null,number>();
  for(const origin of voters){const value=valueOf(input.attestations[origin]!,field);counts.set(value,(counts.get(value)??0)+1);}
  let expected:string|null=null,best=-1;
  for(const [value,count] of [...counts].sort((left,right)=>String(left[0]??"")<String(right[0]??"")?-1:1))if(count>best){best=count;expected=value;}
  for(const origin of voters){
   const value=valueOf(input.attestations[origin]!,field);
   if(value!==expected){differing.push({originId:origin,field,value,expected});diverged.set(origin,[...diverged.get(origin)??[],field]);}
  }
 }
 const known=new Map<string,Set<TrackV1>>(pointed.map(origin=>[origin,new Set<TrackV1>()])),keys=new Map<number,{field:FleetFieldV1;bucket:number;apart:{one:string;two:string;left:string;right:string}[];alike:{one:string;two:string}[]}>();
 for(const [index,one] of pointed.entries())for(const two of pointed.slice(index+1)){
  const found=compareCheckpointsV1(input.attestations[one]!.checkpoints!,input.attestations[two]!.checkpoints!,input.now,input.windows??{});
  for(const track of found.known){known.get(one)!.add(track);known.get(two)!.add(track);}
  for(const item of found.items){
   const rank=comparedFields.indexOf(item.field)*1000+item.bucket+1;
   let entry=keys.get(rank);
   if(entry===undefined){entry={field:item.field,bucket:item.bucket,apart:[],alike:[]};keys.set(rank,entry);}
   if(item.same)entry.alike.push({one,two});else entry.apart.push({one,two,left:spell(item.bucket,item.left),right:spell(item.bucket,item.right)});
  }
 }
 const flagged=new Map<string,Map<FleetFieldV1,{value:string[];expected:string[]}>>(),ambiguous=new Set<string>();
 for(const rank of [...keys.keys()].sort((left,right)=>left-right)){
  const entry=keys.get(rank)!,separated=separateOutliersV1(entry.apart,entry.alike,pointed.length);
  for(const origin of separated.ambiguous)ambiguous.add(origin);
  for(const {origin:victim,first} of separated.victims){
   const fields=flagged.get(victim)??new Map<FleetFieldV1,{value:string[];expected:string[]}>();
   flagged.set(victim,fields);
   const sides=fields.get(entry.field)??{value:[],expected:[]};
   fields.set(entry.field,sides);
   sides.value.push(first.one===victim?first.left:first.right);sides.expected.push(first.one===victim?first.right:first.left);
  }
 }
 for(const [origin,fields] of [...flagged].sort((left,right)=>left[0]<right[0]?-1:1))for(const [field,sides] of fields){
  differing.push({originId:origin,field,value:sides.value.join(","),expected:sides.expected.join(",")});
  diverged.set(origin,[...diverged.get(origin)??[],field]);
 }
 const undecided=(origin:string):boolean=>{
  const others=pointed.filter(other=>other!==origin);
  if(others.length===0)return false;
  const mine=input.attestations[origin]!.checkpoints!.view!==undefined&&others.some(other=>input.attestations[other]!.checkpoints!.view!==undefined);
  return !(known.get(origin)!.has("manifest")&&known.get(origin)!.has("git")&&(!mine||known.get(origin)!.has("view")));
 };
 const headed=origins.some(origin=>{const item=input.attestations[origin];return item!==undefined&&!retired.has(origin)&&state.get(origin)!.fresh&&item.sync&&item.head!==null;}),brokenOrigins=new Map<string,string>();
 for(const origin of origins){
  const item=input.attestations[origin];
  if(item===undefined||retired.has(origin)||!state.get(origin)!.fresh||!item.sync)continue;
  const reason=brokenReasonV1(item)??(headed&&item.head===null&&item.indexTree!==null?"head-null":undefined);
  if(reason!==undefined)brokenOrigins.set(origin,reason);
 }
 for(const origin of brokenOrigins.keys()){
  const fields=diverged.get(origin)??[];
  if(!fields.includes("head.commit")){
   const commits=origins.map(other=>input.attestations[other]?.head?.commit).filter(commit=>commit!==undefined);
   differing.push({originId:origin,field:"head.commit",value:null,expected:commits[0]??null});
   diverged.set(origin,[...fields,"head.commit"]);
  }
 }
 const finished=origins.map(origin=>{
  const row=rows.get(origin)!,names=diverged.get(origin);
  if(names!==undefined)return {...row,verdict:"diverged" as const,differs:names};
  return row.verdict==="ok"&&state.get(origin)!.checkpoint&&(ambiguous.has(origin)||undecided(origin))?{...row,verdict:"lagging" as const}:row;
 });
 const unproven:FleetWarningV1[]=pointed.length===0?[]:origins.filter(origin=>input.attestations[origin]!==undefined&&!state.get(origin)!.checkpoint&&input.attestations[origin]!.sync&&!retired.has(origin)).map(origin=>({originId:origin,kind:"no-checkpoints" as const,detail:input.attestations[origin]!.code.slice(0,12)}));
 const silent:FleetWarningV1[]=[...new Set(input.registered??[])].filter(origin=>input.heads[origin]===undefined&&input.attestations[origin]===undefined&&!retired.has(origin)).sort().map(origin=>({originId:origin,kind:"never-attested" as const,detail:"fleet-auth.json"}));
 const brokenWarnings:FleetWarningV1[]=[...brokenOrigins].map(([origin,reason])=>({originId:origin,kind:"git-broken" as const,detail:reason}));
 const stageWarnings:FleetWarningV1[]=origins.flatMap(origin=>{const item=input.attestations[origin],reason=item===undefined||retired.has(origin)||!state.get(origin)!.fresh||!item.sync?undefined:stageReasonV1(item);return reason===undefined?[]:[{originId:origin,kind:"stage" as const,detail:reason}];});
 return {version:1,ok:finished.every(row=>row.verdict==="ok"||row.verdict==="off"||row.verdict==="retired")&&finished.some(row=>row.verdict==="ok"),freshSeconds:freshSecondsV1,rows:finished,differing,warnings:[...capabilities(live,input.attestations),...unproven,...brokenWarnings,...stageWarnings,...silent]};
}

/** 🪢 The fields in which `originId` disagrees with the rest of the settled fleet and which a repair of its own state can mend: the manifest. Empty while the origin agrees, is not settled, or differs only in code, git state or view. */
export function repairableDriftV1(report:FleetReportV1,originId:string):readonly FleetFieldV1[]{
 const row=report.rows.find(item=>item.originId===originId);
 return row?.verdict==="diverged"?row.differs.filter(field=>field==="manifest"):[];
}

/** 🌐️ Reads the attestations, retirement records and heads of a store, reads the journal events that lie between the cuts of the checkpoints, and judges the fleet at `now`. */
export async function fleetReportV1(storeRoot:string,now:number=Date.now(),signal?:AbortSignal):Promise<FleetReportV1>{
 const attestations=await readAttestationsV1(storeRoot,signal),retirements=await readRetirementsV1(storeRoot,signal),heads:Record<string,{segment:number;bytes:number}>={};
 for(const origin of await listOriginsV1(storeRoot)){
  signal?.throwIfAborted();
  const head=await readHeadV1(storeRoot,origin);
  if(head!==undefined)heads[origin]={segment:head.segment,bytes:head.bytes};
 }
 const cuts=Object.values(attestations).filter(item=>item.sync&&item.checkpoints!==undefined).flatMap(item=>[item.checkpoints!.manifest,item.checkpoints!.git,item.checkpoints!.view]).filter(item=>item!==undefined&&recentAt(now,item.wallMs)).flatMap(item=>[item!.cursors,..."until" in item!&&item.until!==undefined?[item.until]:[]]);
 const windows=cuts.length>0?await readGapWindowsV1(storeRoot,[...cuts,Object.fromEntries(Object.entries(heads).filter(([origin])=>retirements[origin]?.retired!==true))],signal):{};
 const registered:string[]=[];
 for(const origin of await listOriginsV1(storeRoot)){signal?.throwIfAborted();if(await stat(join(storePathsV1(storeRoot).origins,origin,"fleet-auth.json")).then(()=>true,()=>false))registered.push(origin);}
 return judgeFleetV1({now,heads,attestations,retirements,windows,registered});
}

/** 🧰️ What the attester reads from the replica every round. */
export type AttesterSnapshotV1=Readonly<{sync:boolean;files:number;manifest:string;cursors:Readonly<Record<string,FleetCursorV1>>;incoming:number;hlc:string;activity?:number;buckets?:string;busy?:boolean}>;

/** 🪟️ The working-tree view the attester asks for: the counts, the digest and the sorted lines that `view.txt` holds. */
export type AttesterViewV1=FleetViewV1&Readonly<{lines:readonly string[];stage?:readonly string[]}>;

/** 🔬️ What the attester asks of an incremental view: the `previous` view (read by a complete pass), the worktree `paths` that changed since it and whether the stage or `HEAD` moved. */
export type AttesterViewRequestV1=Readonly<{previous:AttesterViewV1;paths:readonly string[];restage:Readonly<{before:string|null;after:string|null}>|null}>;

/** 📣️ The paths the replica changed since the last call (published or applied); `overflow` says that too many changed to name them. Each call forgets what it returned. */
export type AttesterChangesV1=Readonly<{paths:readonly string[];overflow:boolean}>;

/** 🌿️ The git state the attestation carries. */
export type AttesterGitV1=GitStateV1;
const gitObservationKinds=new Set<string>(gitSchema.definitions.StageObservation.enum);
const gitUnavailable=(state:AttesterGitV1|undefined):boolean=>state?.observation!==undefined||state?.present===true&&state.indexTree===null;

/** 🧪️ The answer of the self-check of new code: it starts (`ok`), it does not (`reason`), or the code changed meanwhile and the check must be repeated (`again`). */
export type RestartCheckV1=Readonly<{ok:true}>|Readonly<{ok:false;reason:string;again?:boolean}>;

/** ⚙️ How an attester is wired: identity, clock, code location, intervals and the injected side effects. */
export type AttesterOptionsV1=Readonly<{retryMilliseconds?:number;storeRoot:string;originId:string;groupId:string;platform?:string;now:()=>number;heartbeatMilliseconds?:number;stableMilliseconds?:number;codeRoot?:string;watchRoot?:string;check?:(digest:string,signal?:AbortSignal)=>Promise<RestartCheckV1>;busy?:()=>boolean;jitterSpanMilliseconds?:number;minimumUptimeMilliseconds?:number;busyCapMilliseconds?:number;refused?:(digest:string,reason:string)=>void;admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;fallback?:FleetFallbackV1;returnMilliseconds?:number;failures?:()=>readonly FailureSummaryV1[];suspect?:(digest:string,running:string)=>string|undefined;suspicious?:(digest:string,reason:string)=>void;ran?:(digest:string)=>void;store?:<T>(work:()=>Promise<T>)=>Promise<T>;git:(signal?:AbortSignal)=>Promise<AttesterGitV1>;gitObservationFailed?:(reason:GitStageObservationV1)=>void;reachable:()=>Promise<boolean>;prepare?:()=>Promise<void>;view?:(signal?:AbortSignal,request?:AttesterViewRequestV1)=>Promise<AttesterViewV1|null>;changes?:()=>AttesterChangesV1;unsettled?:()=>readonly string[];viewMilliseconds?:number;viewDebounceMilliseconds?:number;deltaDebounceMilliseconds?:number;deltaLimit?:number;latency?:()=>LatencyReportV1;timed?:(name:TierNameV1,costMs:number,delayMs:number)=>void;viewSeed?:()=>FleetViewV1|undefined;onView?:(view:FleetViewV1)=>void;guards?:(signal?:AbortSignal)=>Promise<FleetGuardsV1|null>;guardsMilliseconds?:number;guardsMinimumMilliseconds?:number}>;

/** 🩺️ The attester: `start` records the digest of the code that runs (`code` of every attestation); `tick` watches the code on disk (`watchRoot`, by default the running code) and answers `restart` only when the digest there differs from the running one, has stayed the same for the stable time (5 s), the admission (`admit`: another origin attests the digest, a code snapshot of it lies in the store, or all of it was written on this machine; otherwise the daemon waits, shows `pending` with `waiting: …` and asks again every heartbeat, and a digest that changes starts again), the self-check (`check`, a fresh process that loads the new code) passed, the deterministic per-origin jitter (0 to 60 s from origin id and new digest) has elapsed, the process has run for at least 30 s and the replica is not busy (`busy`: incoming queue not empty or an apply batch in flight; given up after 5 minutes); a failed check keeps the old code running, calls `refused`, shows as `pending:{code,ok:false,reason}` in the attestation and is repeated only when the digest on disk changes again, except a check that only timed out or could not start (`retryableCheckV1`), which is repeated after `retryMilliseconds` (five minutes). It writes the attestation when a value changed (at most once per second) or the heartbeat is due (every 5 s), never while the store is unreachable. The working-tree view (`git status`, 15 to 20 s of CPU on 270 000 files) is asked once at start and afterwards only when something relevant changed (the manifest, a cursor, the activity counter of the replica that counts publishes, applies, hints and sweep findings, or the git state) and then at most once every `viewMilliseconds` (120 s) or, when a view took long, nineteen times its duration (at most 5 % of the time busy), when the change has been quiet for `viewDebounceMilliseconds` (5 s) or has waited a whole spacing; the duration is reported through `timed`; an idle replica never asks again; never concurrently with itself and cancellable by the signal; `view.txt` is written before the attestation that carries its digest and only when the digest changed. The guards (see `FleetGuardsV1`) are measured in the background after the git state is known, every `guardsMilliseconds` (5 minutes) and, at most every `guardsMinimumMilliseconds` (10 s), after the view digest changed; the attestation states `null` until the first measurement arrives and never waits for it. Git is asked at most once per heartbeat, in the background. The first certified Git state is reported while the working-tree view is still being measured. A pending first Git read gets a liveness receipt after one heartbeat. */
export type AttesterV1={start(signal?:AbortSignal):Promise<void>;tick(snapshot:AttesterSnapshotV1,signal?:AbortSignal):Promise<"idle"|"written"|"restart">;pending():Readonly<{code:string;ok:false;reason:string}>|null;running():string};

/** ⏳️ Whether a refused self-check says only that the machine was too slow or could not start the check (a timeout, a process that did not start): the code is not blamed, so the check is repeated after a pause (the daemon doubles its deadline each time) instead of the machine staying on its old code until the digest changes again. */
export const retryableCheckV1=(reason:string):boolean=>/^self-check (timed out|could not start)/u.test(reason);

/** 🏭️ Creates an attester. */
export function createAttesterV1(options:AttesterOptionsV1):AttesterV1{
 const heartbeat=options.heartbeatMilliseconds??5000,stable=options.stableMilliseconds??5000,spacing=Math.min(1000,heartbeat),probe=createCodeProbeV1(options.codeRoot??codeRootV1),watching=options.watchRoot===undefined?probe:createCodeProbeV1(options.watchRoot),platform=options.platform??platformV1(),viewEvery=options.viewMilliseconds??120_000,viewQuiet=options.viewDebounceMilliseconds??5000,deltaQuiet=options.deltaDebounceMilliseconds??1000,deltaLimit=options.deltaLimit??1500,guardsEvery=options.guardsMilliseconds??300_000,guardsMinimum=options.guardsMinimumMilliseconds??10_000;
 let started="",bootAt=0,reported=false,adopt:{digest:string;since:number}|undefined,admission:{digest:string;ok:boolean;reason:string;at:number}|undefined,admitting:Promise<void>|undefined,verdict:{digest:string;ok:boolean;reason:string;retryAt?:number}|undefined,checking:Promise<void>|undefined,signature="",wrote=-Infinity,gitAt=-Infinity,gitState:AttesterGitV1|undefined,gitBusy:Promise<void>|undefined,gitFailure:GitStageObservationV1|undefined,codeAt=-Infinity;
 let holdGitSince:number|undefined,currentBuckets:Readonly<{digest:string;buckets:string}>|undefined,manifestCheckpoint:FleetManifestCheckpointV1|undefined,gitCheckpoint:FleetGitCheckpointV1|undefined,viewCheckpoint:FleetViewCheckpointV1|undefined,lastCut:FleetCutV1|undefined,gitPending:Readonly<{before:FleetCutV1;wallMs:number;head:string|null;indexTree:string|null}>|undefined,viewPending:Readonly<{before:FleetCutV1;wallMs:number;digest:string;buckets:string}>|undefined;
 let guardsState:FleetGuardsV1|null|undefined,guardsBusy:Promise<void>|undefined,guardsAt=-Infinity,guardsDigest:string|undefined;
 let viewStale=false,viewSeeded=false,viewBase=false,viewState:AttesterViewV1|null|undefined,viewBusy:Promise<void>|undefined,viewFlight:boolean|undefined,viewAt=-Infinity,viewSpacing=viewEvery,viewDemandSpacing=0,viewDeltaAt=-Infinity,viewDeltaSpacing=0,viewDirty:number|undefined,viewTouched=0,viewWatch="",viewStored:string|undefined,viewKey="",viewTree:string|null=null,viewSeen="",viewPaths=new Set<string>(),viewOverflow=false,deltaDirty:number|undefined,deltaTouched=0;
 const askView=(signal:AbortSignal|undefined,mode:"verify"|"demand"|"delta",key:string,tree:string|null):Promise<void>=>{
  const began=options.now(),before=lastCut,taken=[...viewPaths],overflowed=viewOverflow,previous=viewState,request=mode==="delta"&&viewBase&&previous!==null&&previous!==undefined?{previous,paths:taken,restage:key!==viewKey?{before:viewTree,after:tree}:null}:undefined;
  viewPaths.clear();viewOverflow=false;viewFlight=mode!=="verify";
  if(request===undefined){viewAt=began;viewDirty=undefined;}else viewDeltaAt=began;
  return viewBusy??=(options.view===undefined?Promise.resolve(null):options.view(signal,request)).then(state=>{
   viewFlight=undefined;
   if(signal?.aborted===true)return;
   if(state===null||state===undefined){if(request===undefined){if(taken.length>0||overflowed)viewOverflow=true;}else for(const path of taken)viewPaths.add(path);viewState??=null;return;}
   viewState=state;viewBase=true;viewSeeded=false;viewKey=key;viewTree=tree;
   viewPending=before===undefined?undefined:{before,wallMs:began,digest:state.digest,buckets:viewBucketsV1(state.lines)};
   options.onView?.({staged:state.staged,modified:state.modified,deleted:state.deleted,untracked:state.untracked,digest:state.digest});
  },()=>{viewFlight=undefined;if(request===undefined){if(taken.length>0||overflowed)viewOverflow=true;}else for(const path of taken)viewPaths.add(path);viewState??=null;}).finally(()=>{
   viewBusy=undefined;viewFlight=undefined;const cost=Math.max(0,options.now()-began);
   if(request===undefined){viewSpacing=nextDelayV1(viewEvery,cost,5);viewDemandSpacing=nextDelayV1(0,cost,25);if(signal?.aborted!==true&&options.view!==undefined)options.timed?.("view",cost,viewSpacing);}
   else viewDeltaSpacing=nextDelayV1(deltaQuiet,cost,10);
  });
 };
 const askGuards=(signal?:AbortSignal):Promise<void>=>{
  guardsAt=options.now();guardsDigest=viewState?.digest;
  return guardsBusy??=(options.guards===undefined?Promise.resolve(null):options.guards(signal)).then(state=>{if(signal?.aborted!==true)guardsState=state??guardsState??null;},()=>{guardsState??=null;}).finally(()=>{guardsBusy=undefined;});
 };
 const askGit=(signal?:AbortSignal):Promise<void>=>{
  gitAt=options.now();
  const before=lastCut,wallMs=gitAt;
  const observed=(state:AttesterGitV1):void=>{
   if(signal?.aborted===true)return;
   gitState=state;
   const unavailable=gitUnavailable(state);
   if(unavailable)gitCheckpoint=undefined;
   gitPending=before===undefined||unavailable?undefined:{before,wallMs,head:state.present?state.commit:null,indexTree:state.present?state.indexTree:null};
   const reason=state.observation===undefined?undefined:gitObservationKinds.has(state.observation)?state.observation:"observation-failed";
   if(reason===gitFailure)return;
   gitFailure=reason;
   if(gitFailure!==undefined)options.gitObservationFailed?.(gitFailure);
  };
  return gitBusy??=Promise.resolve().then(()=>options.git(signal)).then(observed,()=>observed({present:false,head:null,commit:null,indexTree:null,observation:"observation-failed"})).finally(()=>{gitBusy=undefined;});
 };
 const shown=():Readonly<{code:string;ok:false;reason:string}>|null=>verdict?.ok===false?{code:verdict.digest,ok:false,reason:verdict.reason}:adopt!==undefined&&admission?.digest===adopt.digest&&!admission.ok?{code:admission.digest,ok:false,reason:`waiting: ${admission.reason}`}:null;
 return {
  pending:shown,
  running:()=>started,
  async start(signal){started=await probe();bootAt=options.now();await askGit(signal);},
  async tick(snapshot,signal){
   const hold=options.unsettled?.()??[],now=options.now(),named=hold.some(path=>path===".git/index"||gitPathV1(path));
   if(!named)holdGitSince=undefined;else holdGitSince??=now;
   const holdGit=named&&now-holdGitSince!<holdGitMilliseconds,spanned=snapshot.buckets!==undefined,shot=spanned?cutOfV1(snapshot.cursors):undefined,eligible=shot!==undefined&&snapshot.sync&&snapshot.incoming===0&&snapshot.busy!==true;
   if(eligible){manifestCheckpoint={cursors:shot!,wallMs:now,files:snapshot.files,manifest:snapshot.manifest,buckets:snapshot.buckets!};lastCut=shot;}
   if(shot!==undefined&&gitPending!==undefined){if(!holdGit)gitCheckpoint={cursors:gitPending.before,...sameCutV1(gitPending.before,shot)?{}:{until:shot},wallMs:gitPending.wallMs,head:gitPending.head,indexTree:gitPending.indexTree};gitPending=undefined;}
   if(now-codeAt>=heartbeat){
    codeAt=now;
    const digest=await watching();
    if(digest===started&&options.fallback===undefined){adopt=undefined;verdict=undefined;admission=undefined;}
    else if(adopt?.digest!==digest){
     adopt={digest,since:now};verdict=undefined;admission=undefined;
     const why=options.suspect?.(digest,started);
     if(why!==undefined){verdict={digest,ok:false,reason:`not adopted automatically: ${why}`};options.suspicious?.(digest,why);}
    }
   }
   if(adopt!==undefined&&now-adopt.since>=stable){
    const digest=adopt.digest,refuse=(reason:string):void=>{if(adopt?.digest===digest){verdict={digest,ok:false,reason,...retryableCheckV1(reason)?{retryAt:options.now()+(options.retryMilliseconds??300_000)}:{}};options.refused?.(digest,reason);}};
    if(verdict?.ok===false&&verdict.retryAt!==undefined&&verdict.digest===digest&&now>=verdict.retryAt)verdict=undefined;
    if(verdict===undefined&&checking===undefined&&options.admit!==undefined&&admitting===undefined&&(admission?.digest!==digest||!admission.ok&&now-admission.at>=heartbeat)){
     admitting=Promise.resolve().then(()=>options.admit!(digest,signal)).then(result=>{if(adopt?.digest===digest)admission={digest,ok:result.ok,reason:result.ok?"":result.reason,at:options.now()};},error=>{if(adopt?.digest===digest)admission={digest,ok:false,reason:error instanceof Error?error.message:String(error),at:options.now()};}).finally(()=>{admitting=undefined;});
    }
    if(verdict===undefined&&checking===undefined&&(options.admit===undefined||admission?.digest===digest&&admission.ok)){
     checking=(options.check===undefined?Promise.resolve({ok:true} as RestartCheckV1):options.check(digest,signal)).then(result=>{
      if(adopt?.digest!==digest)return;
      if(result.ok)verdict={digest,ok:true,reason:""};else if(result.again!==true)refuse(result.reason);
     },error=>{refuse(error instanceof Error?error.message:String(error));}).finally(()=>{checking=undefined;});
    }
    if(verdict?.ok===true&&verdict.digest===digest){
     const span=options.jitterSpanMilliseconds??60_000,at=adopt.since+stable+restartJitterMillisecondsV1(options.originId,digest,span);
     if(now>=at&&now-bootAt>=Math.max(options.minimumUptimeMilliseconds??30_000,digest===started?options.returnMilliseconds??300_000:0)&&(options.busy?.()!==true||now-at>=(options.busyCapMilliseconds??300_000)))return "restart";
    }
   }
   const unavailable=gitUnavailable(gitState);
   if((now-gitAt>=heartbeat||!unavailable&&spanned&&gitCheckpoint===undefined&&gitPending===undefined&&now-gitAt>=spacing)&&gitBusy===undefined)void askGit(signal);
   const changes=options.changes?.();
   if(changes!==undefined&&(changes.overflow||changes.paths.length>0)){
    if(changes.overflow)viewOverflow=true;
    for(const path of changes.paths)viewPaths.add(path);
    if(viewPaths.size>deltaLimit){viewOverflow=true;viewPaths.clear();}
    deltaTouched=now;deltaDirty??=now;
   }
   if(viewState===undefined&&!viewSeeded){const seed=options.viewSeed?.();if(seed!==undefined){viewState={...seed,lines:[]};viewSeeded=true;viewStored=seed.digest;}}
   if(unavailable){viewStale=true;viewCheckpoint=undefined;viewPending=undefined;}
   else if(gitState!==undefined&&!gitState.present){viewState=null;viewCheckpoint=undefined;viewPending=undefined;viewStale=false;viewSeeded=false;viewBase=false;viewPaths.clear();viewOverflow=false;deltaDirty=undefined;guardsState=null;}
   else if(gitState!==undefined){
    const key=`${gitState.commit}|${gitState.indexTree}`,watch=JSON.stringify([snapshot.files,snapshot.manifest,snapshot.cursors,snapshot.activity,gitState.commit,gitState.indexTree]);
    if(key!==viewSeen){viewSeen=key;if(viewBase){deltaTouched=now;deltaDirty??=now;}}
    if(watch!==viewWatch){viewWatch=watch;viewTouched=now;viewDirty??=now;}
    const restage=viewBase&&key!==viewKey,pending=viewPaths.size>0||viewOverflow||restage;
    if(!pending)deltaDirty=undefined;
    if(shot!==undefined&&viewPending!==undefined){viewCheckpoint={cursors:viewPending.before,...sameCutV1(viewPending.before,shot)?{}:{until:shot},wallMs:viewPending.wallMs,digest:viewPending.digest,buckets:viewPending.buckets,pending:viewOverflow||restage||holdGit?everyBucketMaskV1:bucketMaskV1([...viewPaths,...hold].map(bucketOfV1))};viewPending=undefined;}
    if(viewBusy===undefined){
     if(viewState===undefined||viewSeeded)void askView(signal,"demand",key,gitState.indexTree);
     else if(pending&&(viewOverflow||!viewBase)){if(now-viewAt>=viewDemandSpacing&&(now-deltaTouched>=viewQuiet||now-(deltaDirty??now)>=30_000))void askView(signal,"demand",key,gitState.indexTree);}
     else if(pending){if(now-viewDeltaAt>=viewDeltaSpacing&&(now-deltaTouched>=deltaQuiet||now-(deltaDirty??now)>=viewQuiet))void askView(signal,"delta",key,gitState.indexTree);}
     else if(viewDirty!==undefined&&now-viewAt>=viewSpacing&&(now-viewTouched>=viewQuiet||now-viewDirty>=viewSpacing))void askView(signal,"verify",key,gitState.indexTree);
    }
    viewStale=viewSeeded||viewPaths.size>0||viewOverflow||(viewBase&&key!==viewKey)||(viewBusy!==undefined&&viewFlight===true);
    if(eligible&&!viewStale&&!holdGit&&viewBase&&viewState!==null&&viewState!==undefined&&viewPending===undefined){
     if(currentBuckets?.digest!==viewState.digest)currentBuckets={digest:viewState.digest,buckets:viewBucketsV1(viewState.lines)};
     viewCheckpoint={cursors:shot!,wallMs:now,digest:viewState.digest,buckets:currentBuckets.buckets,pending:bucketMaskV1(hold.map(bucketOfV1))};
    }
   }
   if(!unavailable&&gitState?.present===true&&guardsBusy===undefined&&(guardsState===undefined||now-guardsAt>=guardsEvery||viewState!==undefined&&viewState!==null&&viewState.digest!==guardsDigest&&!viewStale&&now-guardsAt>=guardsMinimum))void askGuards(signal);
   const checkpoints:FleetCheckpointsV1|undefined=spanned?{...manifestCheckpoint===undefined?{}:{manifest:manifestCheckpoint},...gitCheckpoint===undefined?{}:{git:gitCheckpoint},...viewCheckpoint===undefined||gitState?.present!==true?{}:{view:viewCheckpoint}}:undefined;
   const shownNow=shown(),refusal=shownNow===null?undefined:{code:shownNow.code,ok:false as const,reason:shownNow.reason.slice(0,512)},failures=options.failures?.()??[];
   const next=JSON.stringify([options.fallback,snapshot.sync,snapshot.files,snapshot.manifest,snapshot.cursors,snapshot.incoming,started,refusal,failures,gitState?.commit,gitState?.indexTree,gitState?.head,viewState?.digest,viewStale,guardsState,checkpoints===undefined?undefined:[manifestCheckpoint?.cursors,manifestCheckpoint?.manifest,gitCheckpoint?.cursors,gitCheckpoint?.until,gitCheckpoint?.head,gitCheckpoint?.indexTree,viewCheckpoint?.cursors,viewCheckpoint?.until,viewCheckpoint?.digest,viewCheckpoint?.pending]]);
   const due=now-wrote>=heartbeat||(next!==signature&&now-wrote>=spacing);
   if(!due||started===""||gitState===undefined&&now-bootAt<heartbeat)return "idle";
   if(!await options.reachable())return "idle";
   const write=async():Promise<void>=>{
    await options.prepare?.();
    const view=viewState??null;
    if(view!==null&&view.digest!==viewStored){await writeViewLinesV1(options.storeRoot,options.originId,view.lines);viewStored=view.digest;}
    const git=gitState,failed=git===undefined||gitUnavailable(git);
    const currentCheckpoints=failed&&checkpoints!==undefined?{...checkpoints.manifest===undefined?{}:{manifest:checkpoints.manifest}}:checkpoints;
     const commit=git?.commit,head=git?.present&&commit!=null?{ref:git.head??"HEAD",commit}:null;
     await writeAttestationV1(options.storeRoot,{version:1,originId:options.originId,groupId:options.groupId,platform,wallMs:now,hlc:snapshot.hlc,sync:snapshot.sync,code:started,cursors:snapshot.cursors,files:snapshot.files,manifest:snapshot.manifest,head,indexTree:git?.present&&!failed?git.indexTree:null,view:view===null?null:{staged:view.staged,modified:view.modified,deleted:view.deleted,untracked:view.untracked,digest:view.digest,...(viewStale||failed?{stale:true}:{})},guards:guardsState??null,latency:options.latency?.()??silentLatencyV1(),queue:{incoming:snapshot.incoming},...currentCheckpoints===undefined?{}:{checkpoints:currentCheckpoints},...refusal===undefined?{}:{pending:refusal},...failures.length===0?{}:{failures},...options.fallback===undefined?{}:{fallback:options.fallback}});
   };
   if(options.store===undefined)await write();
   else{try{await options.store(write);}catch(error){if(error instanceof StoreStalledV1)return "idle";throw error;}}
   signature=next;wrote=now;
   if(!reported){reported=true;options.ran?.(started);}
   return "written";
  }
 };
}
