import {closeSync,existsSync,fstatSync,fsyncSync,lstatSync,mkdirSync,openSync,readSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {sleepV1,tryClaimV1} from "../../../🔒️lock/🟦️.ts";
import {enterLaneV1,lanePathsV1,type LaneOwnerV1} from "../../🟦️.ts";
import {poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1} from "../🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.pool,layout=UNIT_CACHE_POLICY_V1.layout;
/** 🎯️ A stable product workspace remains exclusively held by its current caller, never by a waiter's authority. */
export async function acquireProductLaneV1(root:string,request:Readonly<{key:string;crates:readonly string[];quotaBytes:number;owner:LaneOwnerV1}>,options:Readonly<{signal:AbortSignal;report?:PoolReportV1}>):Promise<Readonly<{paths:ReturnType<typeof lanePathsV1>;release():void}>>{
 if(!/^[a-f0-9]{64}$/u.test(request.key))throw Error("Full Cargo product namespace identity required");
 const id=policy.product.prefix+request.key.slice(0,policy.product.digestDigits),paths=lanePathsV1(root,id),locks=join(root,layout.locks),bounds=poolBoundsV1(request.quotaBytes);let poll=policy.pollMilliseconds,reported=false;
 mkdirSync(join(root,layout.lanes),{recursive:true});
 for(;;){
  options.signal.throwIfAborted();const catalog=tryClaimV1(locks,policy.catalogClaim);
  if(!catalog){await sleepV1(poll,options.signal);continue;}
  const acquired=(()=>{try{
  let lanes=readPoolV1(root,Date.now());
  if(!lanes.some(lane=>lane.id===id)&&lanes.length>=bounds.maximumLiveLanes){trimPoolV1(root,bounds,Date.now(),options.report);lanes=readPoolV1(root,Date.now());if(lanes.length>=bounds.maximumLiveLanes)return undefined;}
  const claim=tryClaimV1(locks,"lane-"+id);
  if(!claim){if(!reported){reported=true;options.report?.("poolBusy",{live:lanes.filter(row=>row.claimed).length,maximum:bounds.maximumLiveLanes});}return undefined;}
  try{
   options.signal.throwIfAborted();const receipt=join(paths.base,policy.product.receipt);
   if(existsSync(paths.base)){
    const directory=lstatSync(paths.base),before=lstatSync(receipt);if(!directory.isDirectory()||directory.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1||before.size>policy.product.maximumReceiptBytes)throw Error("Actual Cargo product namespace receipt required");
    const file=openSync(receipt,"r");try{const opened=fstatSync(file);if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size||!opened.isFile()||opened.nlink!==1)throw Error("Changed Cargo product namespace receipt");const bytes=Buffer.alloc(opened.size+1),count=readSync(file,bytes,0,bytes.byteLength,0);if(count!==opened.size)throw Error("Changed Cargo product namespace receipt");const row=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(0,count))),after=fstatSync(file),named=lstatSync(receipt);if(after.dev!==opened.dev||after.ino!==opened.ino||after.size!==opened.size||after.mtimeMs!==opened.mtimeMs||after.ctimeMs!==opened.ctimeMs||named.dev!==opened.dev||named.ino!==opened.ino)throw Error("Changed Cargo product namespace receipt");if(!row||Object.keys(row).sort().join(",")!=="key,version"||row.version!==1||row.key!==request.key)throw Error("Cargo product namespace identity collision");}finally{closeSync(file);}
   }else{
    mkdirSync(paths.base);const file=openSync(receipt,"wx");try{writeFileSync(file,JSON.stringify({version:1,key:request.key})+"\n");fsyncSync(file);}finally{closeSync(file);}
   }
   const entered=enterLaneV1(root,id,request.owner,request.crates);try{trimPoolV1(root,bounds,Date.now(),options.report);}catch(error){entered.release();throw error;}
   return Object.freeze({paths:entered.paths,release(){try{entered.release();}finally{claim.release();}}});
  }catch(error){claim.release();throw error;}
  }finally{catalog.release();}})();
  if(acquired)return acquired;
  await sleepV1(poll,options.signal);poll=Math.min(policy.pollMaximumMilliseconds,Math.ceil(poll*policy.pollBackoffBasisPoints/10000));
 }
}
