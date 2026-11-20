import {lstatSync,statfsSync} from "node:fs";
import {join} from "node:path";
import {machinePathV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import budgetPolicy from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {diskTextV1,type DiskLanguageV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🖼️ What the gate may return from the fixed cache images: the floors of a candidate, the filesystem overhead that is not slack, the elastic partitions that the product's regrow may retire and the command that does it. */
export type ImageSlackPolicyV1=Readonly<{version:1;minimumSlackBytes:number;minimumSlackPercent:number;filesystemOverheadBytes:number;regrowProfiles:readonly string[];command:string;reclaimCommand:string;reclaimProfiles:readonly string[]}>;
export const IMAGE_SLACK_V1:ImageSlackPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.ImageSlack.const)));

/** 📏️ One fixed image of the machine: the bytes it holds on the host volume, the bytes its filesystem uses inside, their difference less the filesystem overhead (the slack no collector inside the image can return) and whether it is attached. */
export type ImageRowV1=Readonly<{id:string;directory:string;device?:number;mounted:boolean;allocatedBytes:number;usedBytes:number;slackBytes:number}>;
/** 🏁️ What the in-place reclaim of one image answered: `reclaimed` (the cycle reduced backing allocation), `skipped` (the image is held, retiring, not bloated or the platform has no such mechanism), `unchanged` (settled without reducing backing allocation), `unconfirmed` (no settled allocation evidence) or `failed`. */
export type ImageReclaimOutcomeV1=Readonly<{kind:"reclaimed"|"skipped"|"unchanged"|"unconfirmed"|"failed";message?:string}>;
/** 📏️ The slack below which an image is not worth a cycle: the larger of the bytes floor and the percentage of its allocation. */
export function slackFloorV1(row:ImageRowV1,policy:ImageSlackPolicyV1=IMAGE_SLACK_V1):number{return Math.max(policy.minimumSlackBytes,Math.ceil(row.allocatedBytes*policy.minimumSlackPercent/100));}
/** 🧭️ What the gate does with one image and why: `retire` it, or skip it because it is not an elastic recomputable partition (`not-elastic`), its slack is below the floor (`below-floor`), the volume is already served by larger ones (`not-needed`) or it is not attached so its inside is unknown (`unmounted`). */
export type ImageVerdictV1=Readonly<{id:string;slackBytes:number;action:"retire"|"skip";reason:"slack"|"not-elastic"|"below-floor"|"not-needed"|"unmounted"}>;
/** 🏁️ What the product's regrow answered for one image: `retired` (a fresh image is born by the next opener), `held` (a command holds the image mapped, the exclusive non-waiting try ended silently), `journal` (the journal does not admit whole retirement) or `failed`. */
export type ImageOutcomeV1=Readonly<{kind:"retired"|"held"|"journal"|"skipped"|"failed";message?:string}>;

/** 📐️ Measures every fixed image below the cache root of a workspace by the product's own partition table: `statfs` of the mounted filesystem for the bytes inside, the allocated blocks of `fixed.dmg` for the bytes on the host; an absent image is left out, a detached one answers `mounted: false` with no slack. */
export function measureImagesV1(workspace:string,policy:ImageSlackPolicyV1=IMAGE_SLACK_V1,partitions:Readonly<Record<string,Readonly<{directory:string}>>>=budgetPolicy.partitions):readonly ImageRowV1[]{
 const rows:ImageRowV1[]=[];
 for(const [id,declared] of Object.entries(partitions)){
  const directory=machinePathV1(workspace,declared.directory);let allocatedBytes:number,device:number;
  try{
   const root=lstatSync(directory),backing=lstatSync(join(directory,"fixed.dmg"));
   if(!root.isDirectory()||root.isSymbolicLink()||!backing.isFile()||backing.isSymbolicLink()||backing.nlink!==1||backing.dev!==root.dev)continue;
   allocatedBytes=backing.blocks*512;device=backing.dev;
  }catch{continue;}
  try{
   const mount=join(directory,"m");
   const root=lstatSync(mount);if(!root.isDirectory()||root.isSymbolicLink()||root.dev===device){rows.push({id,directory,device,mounted:false,allocatedBytes,usedBytes:0,slackBytes:0});continue;}
   const inside=statfsSync(mount),usedBytes=(inside.blocks-inside.bfree)*inside.bsize;
   rows.push({id,directory,device,mounted:true,allocatedBytes,usedBytes,slackBytes:Math.max(0,allocatedBytes-usedBytes-policy.filesystemOverheadBytes)});
  }catch{rows.push({id,directory,device,mounted:false,allocatedBytes,usedBytes:0,slackBytes:0});}
 }
 return Object.freeze(rows);
}

/** ⚖️ Decides which images to retire so that `needBytes` come back: the elastic partitions with at least the floor of slack (the bytes and the percentage of their allocation, whichever is larger), the largest slack first, until the sum of their slack covers the need. Pure; the Python oracle of the tests agrees. */
export function decideImageSlackV1(rows:readonly ImageRowV1[],needBytes:number,policy:ImageSlackPolicyV1=IMAGE_SLACK_V1):readonly ImageVerdictV1[]{
 let remaining=needBytes;const verdicts:ImageVerdictV1[]=[];
 for(const row of [...rows].sort((first,second)=>second.slackBytes-first.slackBytes||(first.id<second.id?-1:1))){
  const base={id:row.id,slackBytes:row.slackBytes},floor=Math.max(policy.minimumSlackBytes,Math.ceil(row.allocatedBytes*policy.minimumSlackPercent/100));
  if(!policy.regrowProfiles.includes(row.id))verdicts.push({...base,action:"skip",reason:"not-elastic"});
  else if(!row.mounted)verdicts.push({...base,action:"skip",reason:"unmounted"});
  else if(row.slackBytes<floor)verdicts.push({...base,action:"skip",reason:"below-floor"});
  else if(remaining<=0)verdicts.push({...base,action:"skip",reason:"not-needed"});
  else{verdicts.push({...base,action:"retire",reason:"slack"});remaining-=row.slackBytes;}
 }
 return Object.freeze(verdicts);
}

/** 🧹️ Retires the chosen images one at a time through the port (the product's exclusive non-waiting regrow), reads the volume again after each one because the claim of a collector is not trusted, stops as soon as the volume has the free bytes it was asked for, names every skip with its reason in the language asked for, and stops between images when cancelled. A killed or failing retirement leaves the image it did not finish as it was: the port retires whole images or none. */
export async function collectImageSlackV1(input:Readonly<{rows:readonly ImageRowV1[];targetFreeBytes:number;pressureDevice?:number;freeBytes():number|undefined;retire(id:string,signal?:AbortSignal):Promise<ImageOutcomeV1>;reclaim?(id:string,signal?:AbortSignal):Promise<ImageReclaimOutcomeV1>;language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;policy?:ImageSlackPolicyV1}>):Promise<Readonly<{reclaimed:readonly string[];retired:readonly string[];held:readonly string[];freedBytes:number}>>{
 const write=input.report??(()=>undefined),text=(key:string,values:Record<string,string|number>)=>diskTextV1(key,input.language,values),retired:string[]=[],held:string[]=[],reclaimed:string[]=[];
 const start=input.freeBytes();let free=start;
 const policy=input.policy??IMAGE_SLACK_V1;
 const rows=input.pressureDevice===undefined?input.rows:input.rows.filter(row=>row.device===input.pressureDevice);
 for(const row of [...rows].sort((first,second)=>second.slackBytes-first.slackBytes||(!first.mounted&&!second.mounted?second.allocatedBytes-first.allocatedBytes:0)||(first.id<second.id?-1:1))){
  input.signal?.throwIfAborted();
  if(input.reclaim!==undefined&&row.mounted&&(input.policy??IMAGE_SLACK_V1).reclaimProfiles.includes(row.id)&&row.slackBytes>=slackFloorV1(row,input.policy)&&(free===undefined||free<input.targetFreeBytes)){
   const earlier=free??0,answer=await input.reclaim(row.id,input.signal).catch(error=>{if(input.signal?.aborted)throw error;return{kind:"failed",message:String((error as Error)?.message??error).slice(0,200)} as ImageReclaimOutcomeV1;});
   free=input.freeBytes()??free;input.signal?.throwIfAborted();
   if(answer.kind==="reclaimed"){reclaimed.push(row.id);write(text("image.reclaimed",{id:row.id,slack:formatDiskBytesV1(row.slackBytes),freed:formatDiskBytesV1(Math.max(0,(free??0)-earlier))}));if(free!==undefined&&free>=input.targetFreeBytes)break;continue;}
   const context:Record<string,string>=answer.kind==="failed"?{message:answer.message??""}:answer.kind==="skipped"?{slack:formatDiskBytesV1(row.slackBytes)}:{};
   write(text("image.reclaim-"+answer.kind,{id:row.id,...context}));
   if(answer.kind==="unconfirmed"){if(free!==undefined&&free>=input.targetFreeBytes)break;continue;}
  }
  const need=Math.max(0,input.targetFreeBytes-(free??0)),verdict=decideImageSlackV1([row],need,input.policy)[0];
  const inspectDetached=input.pressureDevice!==undefined&&free!==undefined&&need>0&&!row.mounted&&policy.regrowProfiles.includes(row.id)&&row.allocatedBytes>=policy.filesystemOverheadBytes+slackFloorV1(row,policy);
  if(verdict.action!=="retire"&&!inspectDetached)continue;
  const before=free??0,outcome=await input.retire(row.id,input.signal).catch(error=>{if(input.signal?.aborted)throw error;return{kind:"failed",message:String((error as Error)?.message??error).slice(0,200)} as ImageOutcomeV1;});
  free=input.freeBytes()??free;
  const slack=formatDiskBytesV1(row.slackBytes);
  if(outcome.kind==="retired"){retired.push(row.id);write(text("image.retired",{id:row.id,slack,freed:formatDiskBytesV1(Math.max(0,(free??0)-before))}));}
  else if(outcome.kind==="held"){held.push(row.id);write(text("image.held",{id:row.id,slack}));}
  else if(outcome.kind==="journal")write(text("image.journal",{id:row.id,slack}));
  else if(outcome.kind==="skipped")write(text("image.not-eligible",{id:row.id}));
  else write(text("image.failed",{id:row.id,message:outcome.message??""}));
  if(free!==undefined&&free>=input.targetFreeBytes)break;
 }
 return Object.freeze({reclaimed:Object.freeze(reclaimed),retired:Object.freeze(retired),held:Object.freeze(held),freedBytes:start===undefined||free===undefined?0:Math.max(0,free-start)});
}

/** 🔎️ What the refusal names of the images: the slack that a retirement can return (the elastic images above their floor), the root with every image's slack for context, and the exact command for the largest retirable one (`repo cache gc` when none is retirable). */
export function describeImageSlackV1(rows:readonly ImageRowV1[],policy:ImageSlackPolicyV1=IMAGE_SLACK_V1):Readonly<{bytes:number;root:string;command:string}>{
 const chosen=decideImageSlackV1(rows,Number.MAX_SAFE_INTEGER,policy).filter(item=>item.action==="retire"),inPlace=rows.filter(item=>item.mounted&&policy.reclaimProfiles.includes(item.id)&&item.slackBytes>=slackFloorV1(item,policy)&&!chosen.some(other=>other.id===item.id)),bytes=chosen.reduce((sum,item)=>sum+item.slackBytes,0)+inPlace.reduce((sum,item)=>sum+item.slackBytes,0),first=rows.find(item=>item.id===chosen[0]?.id),second=[...inPlace].sort((left,right)=>right.slackBytes-left.slackBytes)[0];
 const detail=rows.filter(item=>item.slackBytes>0).sort((left,right)=>right.slackBytes-left.slackBytes).map(item=>`${item.id} ${formatDiskBytesV1(item.slackBytes)} of ${formatDiskBytesV1(item.allocatedBytes)}`).join(", ");
 return Object.freeze({bytes,root:((first??second)?.directory??"")+" ("+detail+")",command:first?policy.reclaimCommand.replace("{partition}",first.id)+" or/oder "+policy.command.replace("{partition}",first.id):second?policy.reclaimCommand.replace("{partition}",second.id):"repo cache gc"});
}
