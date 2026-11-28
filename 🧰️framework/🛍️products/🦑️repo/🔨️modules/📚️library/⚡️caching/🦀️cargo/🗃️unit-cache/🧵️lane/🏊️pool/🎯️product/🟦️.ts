import {closeSync,existsSync,fstatSync,fsyncSync,lstatSync,mkdirSync,openSync,readSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {sleepV1,tryClaimV1} from "../../../🔒️lock/🟦️.ts";
import {laneMessageV1} from "../../../🚪️launch/🟦️.ts";
import {crateKeyV1,enterLaneV1,lanePathsV1,type LaneOwnerV1} from "../../🟦️.ts";
import {admitNewLaneV1,boundedPoolV1,laneHoldersV1,poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1,type PoolTrimContextV1} from "../🟦️.ts";
import {ensureLaneImageV1,laneImagePortsV1,repairLaneImagesV1,retireClampedLaneImagesV1,retireIdleLaneImagesV1} from "../🧩️image/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.pool,layout=UNIT_CACHE_POLICY_V1.layout;
/** 🎯️ A stable product workspace remains exclusively held by its current caller, never by a waiter's authority. A retained product lane (idle, inside its retention window) is never a trim victim, so it is not counted against the live bound either (`boundedPoolV1`): counting it let thirty-two retained products fill the pool for twelve hours and every later product wait here for ever without a word (the artifact-plan receiver stalled its whole loan in `cargo.product.claim`); a pool full of lanes that are really live now says so once (`poolBusy`) and keeps waiting, cancellable. */
export async function acquireProductLaneV1(root:string,request:Readonly<{key:string;crates:readonly string[];quotaBytes:number;commitmentBytes?:number;owner:LaneOwnerV1;flavour?:string}>,options:Readonly<{signal:AbortSignal;report?:PoolReportV1;volume?:PoolTrimContextV1["volume"];onHeld?:"wait"|"refuse"}>):Promise<Readonly<{paths:ReturnType<typeof lanePathsV1>;release():void}>>{
 if(!/^[a-f0-9]{64}$/u.test(request.key))throw Error("Full Cargo product namespace identity required");
 const id=policy.product.prefix+request.key.slice(0,policy.product.digestDigits),paths=lanePathsV1(root,id),locks=join(root,layout.locks),bounds=poolBoundsV1(request.quotaBytes),trimContext:PoolTrimContextV1={requested:[id],...request.flavour===undefined?{}:{flavour:request.flavour},...options.volume===undefined?{}:{volume:options.volume}};let poll=policy.pollMilliseconds,reported=false;const started=Date.now();let noticed=started;
 mkdirSync(join(root,layout.lanes),{recursive:true});
 const lanePorts=await laneImagePortsV1(root,options.signal,request.commitmentBytes,crateKeyV1(request.crates));
 if(lanePorts!==undefined)try{await repairLaneImagesV1(lanePorts);await retireIdleLaneImagesV1(root,lanePorts,Date.now());await retireClampedLaneImagesV1(root,lanePorts,Date.now());}catch{}
 for(;;){
  options.signal.throwIfAborted();const catalog=tryClaimV1(locks,policy.catalogClaim);
  if(!catalog){await sleepV1(poll,options.signal);continue;}
  const acquired=await(async()=>{try{
  let lanes=readPoolV1(root,Date.now());
  if(!lanes.some(lane=>lane.id===id)&&boundedPoolV1(lanes).length>=bounds.maximumLiveLanes){trimPoolV1(root,bounds,Date.now(),options.report,trimContext);lanes=readPoolV1(root,Date.now());const bounded=boundedPoolV1(lanes);if(bounded.length>=bounds.maximumLiveLanes){if(!reported){reported=true;options.report?.("poolBusy",{live:bounded.length,maximum:bounds.maximumLiveLanes,holders:laneHoldersV1(root,bounded.map(lane=>lane.id),Date.now())});}if(options.onHeld==="refuse")throw new ProductLaneBusyErrorV1(root,request.key,id,laneHoldersV1(root,bounded.map(lane=>lane.id),Date.now()));return undefined;}}
  const claim=tryClaimV1(locks,"lane-"+id);
  if(!claim){if(!reported){reported=true;options.report?.("productLaneHeld",{lane:id,live:lanes.filter(row=>row.claimed).length,maximum:bounds.maximumLiveLanes,holders:laneHoldersV1(root,[id],Date.now())});}if(options.onHeld==="refuse")throw new ProductLaneBusyErrorV1(root,request.key,id,laneHoldersV1(root,[id],Date.now()));return undefined;}
  try{
   options.signal.throwIfAborted();{if(!existsSync(paths.base))await admitNewLaneV1(root,id,lanePorts,Date.now,{...options.report===undefined?{}:{report:options.report}});const image=await ensureLaneImageV1(root,id,lanePorts);if(image.kind==="birth"||image.kind==="failed"||image.kind==="refused")process.stderr.write(`[lane] image of lane ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"} / Image der Spur ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"}\n`);}
   const receipt=join(paths.base,policy.product.receipt);
   if(existsSync(paths.base)&&!existsSync(receipt)){const directory=lstatSync(paths.base);if(!directory.isDirectory()||directory.isSymbolicLink())throw Error("Actual Cargo product namespace directory required");}
   if(existsSync(receipt)){
    const directory=lstatSync(paths.base),before=lstatSync(receipt);if(!directory.isDirectory()||directory.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1||before.size>policy.product.maximumReceiptBytes)throw Error("Actual Cargo product namespace receipt required");
    const file=openSync(receipt,"r");try{const opened=fstatSync(file);if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size||!opened.isFile()||opened.nlink!==1)throw Error("Changed Cargo product namespace receipt");const bytes=Buffer.alloc(opened.size+1),count=readSync(file,bytes,0,bytes.byteLength,0);if(count!==opened.size)throw Error("Changed Cargo product namespace receipt");const row=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(0,count))),after=fstatSync(file),named=lstatSync(receipt);if(after.dev!==opened.dev||after.ino!==opened.ino||after.size!==opened.size||after.mtimeMs!==opened.mtimeMs||after.ctimeMs!==opened.ctimeMs||named.dev!==opened.dev||named.ino!==opened.ino)throw Error("Changed Cargo product namespace receipt");if(!row||Object.keys(row).sort().join(",")!=="key,version"||row.version!==1||row.key!==request.key)throw Error("Cargo product namespace identity collision");}finally{closeSync(file);}
   }else{
    mkdirSync(paths.base,{recursive:true});const file=openSync(receipt,"wx");try{writeFileSync(file,JSON.stringify({version:1,key:request.key})+"\n");fsyncSync(file);}finally{closeSync(file);}
   }
   const entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);try{trimPoolV1(root,bounds,Date.now(),options.report,trimContext);}catch(error){entered.release();throw error;}
   return Object.freeze({paths:entered.paths,release(){try{entered.release();}finally{claim.release();}}});
  }catch(error){claim.release();throw error;}
  }finally{catalog.release();}})();
  if(acquired)return acquired;
  const waited=Date.now()-started;
  if(waited>=policy.product.waitBoundMilliseconds)throw new ProductLaneWaitBoundErrorV1(id,laneHoldersV1(root,[id],Date.now()),Math.round(waited/1000));
  if(Date.now()-noticed>=policy.product.waitNoticeMilliseconds){noticed=Date.now();options.report?.("productLaneWaiting",{lane:id,holders:laneHoldersV1(root,[id],Date.now()),seconds:Math.round(waited/1000),bound:Math.round(policy.product.waitBoundMilliseconds/1000)});}
  await sleepV1(poll,options.signal);poll=Math.min(policy.pollMaximumMilliseconds,Math.ceil(poll*policy.pollBackoffBasisPoints/10000));
 }
}

/** 🚧️ Raised by an acquisition that must not wait (`onHeld: "refuse"`) because the product lane is held: the caller leaves every scope it holds, waits for the lane with `awaitProductLaneFreeV1` and tries again, so nothing is held while it queues. */
export class ProductLaneBusyErrorV1 extends Error{
 readonly code="SEMIO_PRODUCT_LANE_BUSY";
 constructor(readonly root:string,readonly key:string,readonly lane:string,readonly holders:string){super(`product lane ${lane} is held by ${holders}`);this.name="ProductLaneBusyErrorV1";}
}

/** ⏱️ Raised when a product lane stays held longer than `pool.product.waitBoundMilliseconds`; names the lane, the holders and the bound in both languages. */
export class ProductLaneWaitBoundErrorV1 extends Error{
 readonly code="SEMIO_PRODUCT_LANE_WAIT_BOUND";
 constructor(readonly lane:string,readonly holders:string,readonly seconds:number,boundMilliseconds:number=policy.product.waitBoundMilliseconds){super((["en","de"] as const).map(language=>laneMessageV1("productLaneBound",{lane,holders,seconds,bound:Math.round(boundMilliseconds/1000)},language)).join("\n"));this.name="ProductLaneWaitBoundErrorV1";}
}

/** 🕰️ Waits, holding nothing, until the product lane of `key` is free and reports who holds it every `waitNoticeMilliseconds`. A waiter that holds nothing harms nobody, so it is bounded only when the caller passes `boundMilliseconds`. The lane is only probed and released at once, so the retry that follows takes it in the order every path uses. */
export async function awaitProductLaneFreeV1(root:string,key:string,options:Readonly<{signal:AbortSignal;report?:PoolReportV1;now?:()=>number;boundMilliseconds?:number;noticeMilliseconds?:number}>):Promise<void>{
 const id=policy.product.prefix+key.slice(0,policy.product.digestDigits),locks=join(root,layout.locks),now=options.now??Date.now,started=now(),bound=options.boundMilliseconds??Infinity,notice=options.noticeMilliseconds??policy.product.waitNoticeMilliseconds;
 let poll=policy.pollMilliseconds,noticed=started;
 for(;;){
  options.signal.throwIfAborted();
  const probe=tryClaimV1(locks,"lane-"+id);if(probe){probe.release();return;}
  const waited=now()-started,holders=laneHoldersV1(root,[id],now());
  if(waited>=bound)throw new ProductLaneWaitBoundErrorV1(id,holders,Math.round(waited/1000),bound);
  if(now()-noticed>=notice){noticed=now();options.report?.(bound===Infinity?"productLaneQueued":"productLaneWaiting",{lane:id,holders,seconds:Math.round(waited/1000),bound:Math.round(bound/1000)});}
  await sleepV1(Math.min(poll,Math.max(1,bound-waited)),options.signal);poll=Math.min(policy.pollMaximumMilliseconds,Math.ceil(poll*policy.pollBackoffBasisPoints/10000));
 }
}

/** 🔁️ Runs `attempt` (which enters its scopes and takes the lane with `onHeld: "refuse"`) and, whenever it left because the lane is held, waits for the lane with nothing held and runs it again: the one lock order is lane first, scopes second, by construction. */
export async function retryWhileProductLaneBusyV1<T>(attempt:()=>Promise<T>,options:Readonly<{signal:AbortSignal;report?:PoolReportV1;boundMilliseconds?:number;noticeMilliseconds?:number}>):Promise<T>{
 for(;;){
  try{return await attempt();}catch(error){
   if(!(error instanceof ProductLaneBusyErrorV1))throw error;
   await awaitProductLaneFreeV1(error.root,error.key,options);
  }
 }
}
