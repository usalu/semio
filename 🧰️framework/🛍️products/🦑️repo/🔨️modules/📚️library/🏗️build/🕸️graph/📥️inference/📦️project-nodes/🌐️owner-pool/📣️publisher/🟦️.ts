import{randomBytes}from"node:crypto";
import{hostname}from"node:os";
import{serialize}from"node:v8";
import{graphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{projectRecordKeyTextV1}from"../../../../../👁️observation/🟨️.mjs";
import{createOwnerPoolStatusV1}from"../📊️status/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolIdV1,ownerPoolKeepAliveV1,ownerPoolRepublishV1,publishOwnerPoolV1,type OwnerPoolHeadV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1}from"../🟦️.ts";

/** 📣️ What the machine that runs a wave hands over about it, evaluated only when the wave proves cold. */
export type OwnerPoolPublisherInputV1=Readonly<{workspaceRoot:string;environment:Readonly<Record<string,string|undefined>>;identity:string;globals:OwnerPoolHeadV1["globals"];session():unknown;units():readonly OwnerPoolUnitV1[];threshold?:number;store?:string|undefined;origin?:string;say?(line:string):void;publish?:typeof publishOwnerPoolV1;now?:()=>number}>;

/** 📣️ Publishes the owner pool of a wave to the fleet as early as the reader knows its owners, and again as it learns more: `announce` once the unit list is known (before any owner is taken or inferred, so the pools of every machine start within seconds of a read starting anywhere), `learn` while the take pass shrinks the open owners (by `republishShrinkBasisPoints`, at most every `republishMinimumMilliseconds`), `settle` when the take pass ended (the exact open list, a changed list is a new wave), renews the accepted wave between membership updates, and publishes on `miss` as before once a wave has missed `threshold` owners. The work runs after the current turn and one publication at a time, every fault ends in "this machine infers every owner itself", and `SEMIO_OWNER_POOL=off` publishes nothing.
 * @see ./../🧪️tests/🌊️early-wave/🟦️.ts */
export function createOwnerPoolPublisherV1(input:OwnerPoolPublisherInputV1):Readonly<{announce(count:number):void;learn(count:number):void;settle(count:number):void;miss(count:number):void;ask(units:readonly OwnerPoolUnitV1[],forced?:boolean):Promise<boolean>;published():Promise<boolean>;finish(wave:Readonly<{owners:number;seconds:number;consumedByHost:Readonly<Record<string,number>>}>):void}>{
 const now=input.now??Date.now;
 let chain:Promise<boolean>|undefined,waveId:string|null=null,count=0,tried=0,pending=0,retryable=false,over=false,last:{manifest:OwnerPoolManifestV1;at:number}|undefined,asked:{id:string;at:number}|undefined,askChain:Promise<boolean>=Promise.resolve(false);
 const status=input.origin===undefined?undefined:createOwnerPoolStatusV1({store:input.store,origin:input.origin,role:"publisher"});
 const settings=input.environment[P.environmentOff]==="off"?undefined:graphFleetSettingsV1(input.workspaceRoot,input.environment);
 const run=async(forced:boolean,only?:readonly OwnerPoolUnitV1[]):Promise<boolean>=>{
  if(only===undefined){tried=now();retryable=settings?.upload===true;}
  try{
   if(settings===undefined||!settings.upload)return false;
   const renewal=only===undefined&&forced?last?.manifest:undefined;
   const client=graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(input.environment,{root:input.workspaceRoot})}),units=renewal?.units??only??input.units(),partial=renewal?.head??{version:1 as const,revision:projectRecordKeyTextV1(null),identity:input.identity,globals:input.globals,publisher:`${hostname()}:${process.pid}:${randomBytes(3).toString("hex")}`.replace(/[^A-Za-z0-9._:@/+-]/gu,"-").slice(0,160)};
   const id=renewal?.head.id??ownerPoolIdV1(partial,units);if(only!==undefined){if(!forced&&asked?.id===id&&now()-asked.at<P.republishKeepAliveMilliseconds)return true;}else if(!forced&&last?.manifest.head.id===id)return true;
   if(only===undefined){waveId=id;count=units.length;}const head:OwnerPoolHeadV1={...partial,id,createdMs:now()};
   const manifest:OwnerPoolManifestV1={head,session:renewal?.session??serialize(input.session()),units};
   const stored=await(input.publish??publishOwnerPoolV1)({
    client,owner:head.publisher,manifest,
    onRefused:refusal=>{
     if(only===undefined)retryable=refusal.transient;
     input.say?.("owner pool refusal "+JSON.stringify({wave:id.slice(0,12),...refusal}));
    },
   });
   if(stored){if(only===undefined)last={manifest,at:now()};else asked={id,at:now()};}
   input.say?.(`owner pool ${id.slice(0,12)}: ${units.length} owners ${stored?"published to":"not accepted by"} the fleet / ${units.length} Owner ${stored?"an die Flotte veröffentlicht":"von der Flotte nicht angenommen"}`);
   if(only===undefined)status?.update({state:"pulling",wave:id,units:units.length,published:0,lastError:stored?null:"the fleet did not accept the pool"});
   return stored;
  }catch{return false;}
 };
 const start=(forced=false):Promise<boolean>=>{
  pending++;
  return chain=(chain??Promise.resolve(false)).then(()=>new Promise<void>(done=>setTimeout(done,0))).then(()=>run(forced)).finally(()=>{pending--;});
 };
 const alive=(owners:number):void=>{
  if(settings===undefined||over||pending!==0)return;
  if(last!==undefined){
   if(ownerPoolKeepAliveV1({published:last.manifest.units.length,sinceMs:now()-Math.max(last.at,tried)}))void start(true);
  }else if(chain!==undefined&&retryable&&owners>=(input.threshold??8)&&now()-tried>=P.faultRetryMilliseconds)void start();
 };
 return Object.freeze({
  announce(owners){if(chain===undefined&&settings!==undefined&&ownerPoolRepublishV1({published:undefined,count:owners,sinceMs:0}))void start();},
  learn(owners){if(last!==undefined&&settings!==undefined&&ownerPoolRepublishV1({published:last.manifest.units.length,count:owners,sinceMs:now()-last.at}))void start();else alive(owners);},
  settle(owners){if(settings===undefined)return;if(chain!==undefined&&last===undefined){void start();return;}if(last===undefined){if(owners>=(input.threshold??8))void start();return;}if(owners<last.manifest.units.length)void start();},
  miss(owners){if(chain===undefined&&settings!==undefined&&owners>=(input.threshold??8))void start();else alive(owners);},
  ask(units,forced=false){if(settings===undefined||units.length===0)return Promise.resolve(false);return askChain=askChain.then(()=>run(forced,units));},
  published:()=>chain??Promise.resolve(false),
  finish(wave){over=true;if(chain===undefined)return;status?.update({state:"idle",wave:waveId,units:count,stored:Object.values(wave.consumedByHost).reduce((sum,each)=>sum+each,0),consumedByHost:{...wave.consumedByHost},waveSeconds:Math.round(wave.seconds*10)/10,ownersPerMinute:wave.seconds>0?Math.round(wave.owners/wave.seconds*600)/10:0});},
 });
}
