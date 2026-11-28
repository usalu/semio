import{lstatSync,readFileSync,readdirSync,statfsSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageShardCapBytesV1,nativeImageShardFoldV1,nativeImageShardLayoutsV1,readNativeImageShardLedgerV1}from"../🧩️shard/🟦️.ts";
const protocol=schema.definitions.Protocol.const;
const alignDown=(value:number)=>Math.floor(value/protocol.alignBytes)*protocol.alignBytes,alignUp=(value:number)=>Math.ceil(value/protocol.alignBytes)*protocol.alignBytes;
/** 🧮️ The authored protocol of the host-wide commitment budget: safety reserve, overcommit and texts.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_COMMITMENT_V1=protocol;
/** 🧾️ One image the host is asked to back: its declared capacity and the host blocks it holds now. */
export type NativeImageCommitmentRowV1=Readonly<{id:string;capacityBytes:number;allocatedBytes:number}>;
/** 🌡️ What the host promised and can promise: the sums over all images, the ceiling and the room left under it. */
export type NativeImageCommitmentV1=Readonly<{hostCapacityBytes:number;hostFreeBytes:number;allocatedBytes:number;declaredBytes:number;committedBytes:number;ceilingBytes:number;roomBytes:number;rows:readonly NativeImageCommitmentRowV1[]}>;
/** 🛡️ The bytes of the host volume that stay free of any promise: the larger of the floor and the percentage of the volume, aligned up. Pure and language-agnostic. */
export function nativeImageCommitmentReserveBytesV1(hostCapacityBytes:number):number{return Math.max(protocol.reserveFloorBytes,alignUp(Math.floor(hostCapacityBytes*protocol.reservePercent/100)));}
/** 🏔️ The most the declared capacities of all images together may add up to: the smaller of the volume less its reserve and the blocks the images hold now plus the free bytes above the reserve times the overcommit, aligned down; 0 when the host is below its reserve. Pure and language-agnostic. */
export function nativeImageCommitmentCeilingV1(facts:Readonly<{hostCapacityBytes:number;hostFreeBytes:number;allocatedBytes:number}>):number{
 const reserve=nativeImageCommitmentReserveBytesV1(facts.hostCapacityBytes),hard=facts.hostCapacityBytes-reserve,soft=facts.allocatedBytes+Math.floor(Math.max(0,facts.hostFreeBytes-reserve)*protocol.overcommitPercent/100);
 return alignDown(Math.max(0,Math.min(hard,soft)));
}
/** 🚪️ Admits an additional declaration (a birth, or the growth of an image by the difference): it fits when everything declared plus the request stays within the ceiling. Answers the room left, and the shortfall that reclaiming has to cover before the request is refused. Pure and language-agnostic. */
export function nativeImageCommitmentAdmitV1(facts:Readonly<{ceilingBytes:number;committedBytes:number;requestBytes:number}>):Readonly<{admitted:boolean;roomBytes:number;shortfallBytes:number}>{
 const shortfall=Math.max(0,facts.committedBytes+facts.requestBytes-facts.ceilingBytes);
 return Object.freeze({admitted:shortfall===0,roomBytes:Math.max(0,facts.ceilingBytes-facts.committedBytes),shortfallBytes:shortfall});
}
/** 📏️ Whether one image keeps the layout invariant of its platform: its declared capacity is at most the cap the platform returns freed blocks from (2^35 on macOS), so no image can strand slack. Pure and language-agnostic. */
export function nativeImageCommitmentLayoutFitsV1(platform:string,capacityBytes:number):boolean{const cap=nativeImageShardCapBytesV1(platform);return cap>0&&capacityBytes<=cap;}
/** 🪙️ What one image promises the host: the blocks it holds plus the growth reserve, never more than it declares. Pure and language-agnostic. */
export function nativeImageCommitmentRowCostV1(row:Readonly<{capacityBytes:number;allocatedBytes:number}>):number{return Math.min(row.capacityBytes,row.allocatedBytes+protocol.growthReserveBytes);}
/** 🌱️ What a new image promises the host at its birth: its growth reserve, never more than it declares, the unit every standing image is counted in (`nativeImageCommitmentRowCostV1` with nothing written). A request is admitted against the room in this unit, never by its declared capacity. Pure and language-agnostic. */
export function nativeImageCommitmentRequestCostBytesV1(declaredBytes:number):number{return Math.min(declaredBytes,protocol.growthReserveBytes);}
/** 🏷️ The most a birth may declare for a room under the ceiling: nothing bounds it (`undefined`) while the room holds the growth reserve every birth promises, a smaller room bounds it to the room, aligned down. Pure and language-agnostic. */
export function nativeImageCommitmentDeclarableBytesV1(roomBytes:number):number|undefined{return roomBytes>=protocol.growthReserveBytes?undefined:alignDown(Math.max(0,roomBytes));}
/** 🧮️ Sums the rows into the commitment the host carries (what each image promises, see `nativeImageCommitmentRowCostV1`; the plain sum of the declarations is `declaredBytes`) and the room under its ceiling. Pure and language-agnostic. */
export function nativeImageCommitmentSumV1(host:Readonly<{hostCapacityBytes:number;hostFreeBytes:number}>,rows:readonly NativeImageCommitmentRowV1[]):NativeImageCommitmentV1{
 let committed=0,declared=0,allocated=0;for(const row of rows){committed+=nativeImageCommitmentRowCostV1(row);declared+=row.capacityBytes;allocated+=row.allocatedBytes;}
 const ceiling=nativeImageCommitmentCeilingV1({...host,allocatedBytes:allocated});
 return Object.freeze({...host,allocatedBytes:allocated,declaredBytes:declared,committedBytes:committed,ceilingBytes:ceiling,roomBytes:Math.max(0,ceiling-committed),rows:Object.freeze([...rows])});
}
const blocksOf=(path:string):number=>{try{return lstatSync(path).blocks*512;}catch{return 0;}};
const receiptBytesOf=(directory:string):number=>{try{const size=(JSON.parse(readFileSync(join(directory,protocol.receiptName),"utf8")) as {maximumPhysicalBytes?:unknown}).maximumPhysicalBytes;return Number.isSafeInteger(size)&&(size as number)>0?size as number:0;}catch{return 0;}};
/** 🔭️ Measures every image below a cache base (`.cache/b`): the home image of each partition by the capacity its receipt pins and every live shard by its ledger, with the host blocks each holds, against the volume that holds the base. Reads receipts, ledgers and `stat` only. */
export function measureNativeImageCommitmentV1(cacheBase:string):NativeImageCommitmentV1{
 const row=statfsSync(cacheBase),rows:NativeImageCommitmentRowV1[]=[];
 let names:string[]=[];try{names=readdirSync(cacheBase).sort();}catch{}
 for(const name of names){
  const directory=join(cacheBase,name),home=join(directory,protocol.imageName),capacityBytes=receiptBytesOf(directory);
  if(capacityBytes>0)rows.push({id:name,capacityBytes,allocatedBytes:blocksOf(home)});
  try{const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory));for(const layout of nativeImageShardLayoutsV1(directory,live).slice(1)){const record=live.get(layout.id);if(record!==undefined)rows.push({id:name+"/"+layout.id,capacityBytes:record.capacityBytes,allocatedBytes:blocksOf(layout.image)});}}catch{}
 }
 return nativeImageCommitmentSumV1({hostCapacityBytes:Number(row.blocks)*row.bsize,hostFreeBytes:Number(row.bavail)*row.bsize},rows);
}
/** 🗣️ The refusal of a request the commitment budget cannot admit, in one language. */
export function nativeImageCommitmentRefusedTextV1(language:"en"|"de",values:Readonly<{need:string;committed:string;ceiling:string}>):string{return protocol.messages.refused[language].replace(/\{(\w+)\}/gu,(_,name:string)=>values[name as keyof typeof values]??"");}
