import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {readAttestationV1} from "../../../../../../🤝️shared-worktree/🩺️attest/🟦️.ts";
import {readFleetMachineV1} from "../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {readReadyWorkersV1} from "../../../🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts";
import {CHECKOUT_TEST_POLICY_V1,checkoutTestCoversV1,type CheckoutTestCutV1} from "../📐️policy/🟦️.ts";

/** 🧭️ What this machine knows about its own place in the fleet: who it is, which replica cut it has applied (`undefined` while it has published none), whether it is Ready, the digest of a checkout file (`undefined` when it has none) and the other machines of its platform that are Ready now (`peers`, in the order to try them; absent where the fleet is not asked, then the scheduler picks). The one seam of the checkout test offload that touches the replica and the readiness records, so tests replace it. */
export type CheckoutTestReplicaV1=Readonly<{origin:string;cut():Promise<CheckoutTestCutV1|undefined>;ready():Promise<Readonly<{ready:boolean;detail:string}>>;digest(file:string):string|undefined;peers?():Promise<readonly string[]>}>;

/** 🔏️ The SHA-256 of one file of a checkout, `undefined` when it cannot be read. */
export function checkoutTestFileDigestV1(root:string,file:string):string|undefined{
 try{return createHash("sha256").update(readFileSync(resolve(root,...file.split("/")))).digest("hex");}catch{return undefined;}
}

/** 🏠️ The replica view of this machine: its origin and store from `.cache/f/machine.json`, its applied cut from its own attestation (the cut every other machine compares), its readiness from its own record of the worker ready stages. `undefined` when the machine is no member of a fleet. */
export function fleetCheckoutReplicaV1(root:string,now:()=>number=Date.now):CheckoutTestReplicaV1|undefined{
 const machine=readFleetMachineV1(root);
 if(machine===undefined||machine.storeRoot===undefined)return undefined;
 const store=machine.storeRoot,origin=machine.originId;
 return Object.freeze({
  origin,
  async cut(){return (await readAttestationV1(store,origin))?.cursors;},
  async ready(){
   const row=readReadyWorkersV1({store,nowMs:now()}).find(item=>item.origin===origin);
   return row===undefined?{ready:false,detail:"no readiness record"}:{ready:row.liveness==="ready",detail:row.liveness};
  },
  digest:file=>checkoutTestFileDigestV1(root,file),
  async peers(){
   const own=(await readAttestationV1(store,origin))?.platform,rows=readReadyWorkersV1({store,nowMs:now()}).filter(row=>row.origin!==origin&&row.liveness==="ready"&&(own===undefined||row.platform===null||row.platform===own)).map(row=>row.origin);
   for(let at=rows.length-1;at>0;at--){const other=Math.floor(Math.random()*(at+1));[rows[at],rows[other]]=[rows[other]!,rows[at]!];}
   return rows;
  },
 });
}

/** ⏳️ Waits until this replica holds the revision of a request: its cut covers the requester's cut and its copy of the file has the requester's digest. Polls every `pollMilliseconds` for at most `waitMilliseconds`; a replica that stays behind is not served, never guessed. The answer names what is missing in English and German-neutral terms (origins, file). */
export async function awaitCheckoutRevisionV1(input:Readonly<{replica:CheckoutTestReplicaV1;need:CheckoutTestCutV1;file:string;fileSha256:string;waitMilliseconds?:number;pollMilliseconds?:number;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;signal?:AbortSignal}>):Promise<Readonly<{held:true}>|Readonly<{held:false;reason:"behind"|"file";detail:string}>>{
 const wait=input.waitMilliseconds??CHECKOUT_TEST_POLICY_V1.wait.replicaMilliseconds,poll=input.pollMilliseconds??CHECKOUT_TEST_POLICY_V1.wait.pollMilliseconds,now=input.now??Date.now,sleep=input.sleep??((milliseconds:number)=>new Promise<void>(done=>{setTimeout(done,milliseconds);})),until=now()+wait;
 let last:Readonly<{held:false;reason:"behind"|"file";detail:string}>={held:false,reason:"behind",detail:"no cut published"};
 for(;;){
  input.signal?.throwIfAborted();
  const have=await input.replica.cut(),covers=have===undefined?undefined:checkoutTestCoversV1(input.need,have);
  if(covers?.covered===true){
   const digest=input.replica.digest(input.file);
   if(digest===input.fileSha256)return{held:true};
   last={held:false,reason:"file",detail:input.file};
  }else last={held:false,reason:"behind",detail:covers===undefined?"no cut published":covers.behind.slice(0,6).join(", ")};
  if(now()+poll>until)return last;
  await sleep(poll,input.signal);
 }
}
