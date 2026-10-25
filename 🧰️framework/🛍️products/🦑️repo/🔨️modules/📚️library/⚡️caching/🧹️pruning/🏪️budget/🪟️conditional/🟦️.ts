import{requireCompilerWriterCustodyV1}from"../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import{writeFile}from"node:fs/promises";
import{join}from"node:path";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{observeNativeWindowsConditionalTreeV1,retireNativeWindowsConditionalTreeV1,ConditionalWindowsRetirementFailureV1,type ConditionalWindowsReceiptV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪟️conditional-retirement/🟦️.ts";
import type{CacheUnit}from"../../🟦️.ts";
/** 🪟️ Exposes conditional kernel acceptance separately from completed units and fresh physical availability. */
export type CargoWindowsConditionalSummaryV1=Readonly<{kind:"windows-kernel-conditional";acceptedEntries:number;removedEntries:number;acceptedLogicalBytes:number;removedLogicalBytes:number;unsettledUnits:number;handlesSettled:boolean;headroomDeficitBytes:number}>;
/** 🧾️ Retains partial accepted effects even when the remaining Original cannot fund a fresh headroom read. */
export class CargoWindowsConditionalFailureV1 extends Error{readonly settlement:Readonly<{units:number;logicalBytes:number;availableBytes:number|null;conditional:Omit<CargoWindowsConditionalSummaryV1,"headroomDeficitBytes">&{headroomDeficitBytes:number|null}}>;constructor(settlement:Readonly<{units:number;logicalBytes:number;availableBytes:number|null;conditional:Omit<CargoWindowsConditionalSummaryV1,"headroomDeficitBytes">&{headroomDeficitBytes:number|null}}>,cause:unknown){super("Windows compiler retirement conditionally refused",{cause});this.settlement=settlement;}}
/** 🏪️ Applies the distinct Windows contract only inside the authenticated maintenance owner. */
export async function retireWindowsCargoUnitsV1(options:Readonly<{root:string;backingRoot:string;units:readonly CacheUnit[];control:ResourceLeaseControl;output:string;targetAvailableBytes:number;maximumDeletionBytes:number;recensus:()=>Promise<readonly CacheUnit[]>;available:()=>Promise<number>}>):Promise<Readonly<{units:number;logicalBytes:number;availableBytes:number;conditional:CargoWindowsConditionalSummaryV1}>>{
 options=Object.freeze({...options,units:Object.freeze(options.units.map(unit=>Object.freeze({...unit})))});const{control}=options;checkResourceLeaseControl(control);requireCompilerWriterCustodyV1(options.backingRoot,control);const current=new Map((await options.recensus()).map(unit=>[unit.path,unit]));
 for(const unit of options.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld)throw Error("Conditional Cargo unit changed before kernel custody");}
 let units=0,bytes=0,acceptedEntries=0,removedEntries=0,acceptedLogicalBytes=0,removedLogicalBytes=0,handlesSettled=true;
 const record=(receipt:ConditionalWindowsReceiptV1)=>{handlesSettled=handlesSettled&&receipt.handlesSettled;acceptedEntries+=receipt.acceptedEntries;removedEntries+=receipt.removedEntries;acceptedLogicalBytes+=receipt.acceptedLogicalBytes;removedLogicalBytes+=receipt.removedLogicalBytes;};
 try{
  for(const unit of options.units){
   checkResourceLeaseControl(control);if(unit.lockHeld||acceptedLogicalBytes+unit.bytes>options.maximumDeletionBytes)throw Error("Conditional compiler deletion bound changed");
   let receipt:ConditionalWindowsReceiptV1;try{receipt=await retireNativeWindowsConditionalTreeV1({root:options.root,unit:unit.path,maximumLogicalBytes:unit.bytes,control});}catch(error){if(error instanceof ConditionalWindowsRetirementFailureV1)record(error.receipt);throw error;}
   record(receipt);if(receipt.outcome==="complete"){units++;bytes+=unit.bytes;}
   await writeFile(join(options.output,"conditional-receipt.json"),JSON.stringify({version:1,units,logicalBytes:bytes,acceptedEntries,removedEntries,acceptedLogicalBytes,removedLogicalBytes,unsettledUnits:options.units.length-units,handlesSettled}));
  }
  const availableBytes=await options.available(),conditional:CargoWindowsConditionalSummaryV1={kind:"windows-kernel-conditional",acceptedEntries,removedEntries,acceptedLogicalBytes,removedLogicalBytes,unsettledUnits:options.units.length-units,handlesSettled,headroomDeficitBytes:Math.max(0,options.targetAvailableBytes-availableBytes)};
  await writeFile(join(options.output,"conditional-result.json"),JSON.stringify({version:1,units,logicalBytes:bytes,availableBytes,conditional}));return{units,logicalBytes:bytes,availableBytes,conditional};
 }catch(error){
  let availableBytes:number|null=null;if(handlesSettled)try{availableBytes=await options.available();}catch{}
  const settlement={units,logicalBytes:bytes,availableBytes,conditional:{kind:"windows-kernel-conditional"as const,acceptedEntries,removedEntries,acceptedLogicalBytes,removedLogicalBytes,unsettledUnits:options.units.length-units,handlesSettled,headroomDeficitBytes:availableBytes===null?null:Math.max(0,options.targetAvailableBytes-availableBytes)}};
  try{await writeFile(join(options.output,"conditional-result.json"),JSON.stringify({version:1,...settlement}));}catch(recordError){throw new CargoWindowsConditionalFailureV1(settlement,new AggregateError([error,recordError],"Conditional retirement and settlement recording failed",{cause:error}));}
  throw new CargoWindowsConditionalFailureV1(settlement,error);
 }
}

/** 🗂️ Projects full Windows physical units without changing POSIX session accounting. */
export async function observeWindowsCargoUnitsV1(options:Readonly<{root:string;units:readonly CacheUnit[];control:ResourceLeaseControl}>):Promise<readonly CacheUnit[]>{
 options=Object.freeze({...options,units:Object.freeze(options.units.map(unit=>Object.freeze({...unit})))});const rows:CacheUnit[]=[];for(const unit of options.units){checkResourceLeaseControl(options.control);const observation=await observeNativeWindowsConditionalTreeV1({root:options.root,unit:unit.path,maximumLogicalBytes:Number.MAX_SAFE_INTEGER,control:options.control});const{recencySource,...data}=unit;rows.push(Object.freeze({...data,bytes:observation.logicalBytes,recencyMs:observation.recencyMs,...(recencySource&&unit.recencyMs===observation.recencyMs?{recencySource}:{})}));}checkResourceLeaseControl(options.control);return Object.freeze(rows);
}
