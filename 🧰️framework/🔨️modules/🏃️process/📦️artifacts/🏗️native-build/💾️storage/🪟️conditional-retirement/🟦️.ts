import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{opendir,lstat}from"node:fs/promises";
import{win32}from"node:path";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../🔒️leases/🟦️.ts";
import{requireIssuedResourceControlV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{openConditionalWindowsSystemPortV1,type ConditionalWindowsPinV1,type ConditionalWindowsIdentityV1}from"./🔌️port/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=schema.$defs.Policy.const;
/** 🧮️ Distinguishes accepted kernel effects from observed namespace settlement. */
export function conditionalWindowsOutcomeV1(accepted:number,removed:number,refused:boolean):"complete"|"pending"|"refused"|"partial"{if(!Number.isSafeInteger(accepted)||!Number.isSafeInteger(removed)||accepted<0||removed<0||removed>accepted)throw Error("Exact conditional retirement counters required");return refused?accepted===0?"refused":"partial":removed===accepted?"complete":"pending";}
/** 🧾️ Records only conditional kernel acceptance and observed names, never a census or reclaimed-byte estimate. */
export type ConditionalWindowsReceiptV1=Readonly<{kind:"windows-kernel-conditional";acceptedEntries:number;removedEntries:number;acceptedLogicalBytes:number;removedLogicalBytes:number;handlesSettled:boolean;outcome:"complete"|"pending"|"refused"|"partial"}>;
/** 🚧️ Carries every accepted effect through refusal and cancellation. */
export class ConditionalWindowsRetirementFailureV1 extends Error{readonly receipt:ConditionalWindowsReceiptV1;constructor(receipt:ConditionalWindowsReceiptV1,cause:unknown){super("Windows conditional retirement "+receipt.outcome,{cause});this.receipt=receipt;}}
/** 📂️ Keeps namespace observations behind the same firstparty system boundary as the kernel calls. */
export type ConditionalWindowsTreeIOV1=Readonly<{forEachChild(path:string,consume:(name:string)=>Promise<void>):Promise<void>;exists(path:string):Promise<boolean>}>;
const systemIO:ConditionalWindowsTreeIOV1={forEachChild:async(path,consume)=>{const directory=await opendir(path,{bufferSize:32});let primary:unknown,failed=false;try{for(let entry=await directory.read();entry;entry=await directory.read())await consume(entry.name);}catch(error){failed=true;primary=error;}try{await directory.close();}catch(error){if(failed)throw new AggregateError([primary,error],"Conditional directory iteration and close failed",{cause:primary});throw error;}if(failed)throw primary;},exists:async path=>{try{await lstat(path);return true;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}}};
const sameObject=(a:ConditionalWindowsIdentityV1,b:ConditionalWindowsIdentityV1)=>a.volume===b.volume&&a.file===b.file&&a.directory===b.directory;
type WindowsConditionalRequestV1=Readonly<{root:string;unit:string;maximumLogicalBytes:number;control:ResourceLeaseControl}>;
function conditionalTreePathV1(request:WindowsConditionalRequestV1):Readonly<{root:string;target:string}>{
 const{control}=request;requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);if(Object.keys(request).sort().join(",")!=="control,maximumLogicalBytes,root,unit"||validateJsonSchemaSubset(schema.$defs.Request,{root:request.root,unit:request.unit,maximumLogicalBytes:request.maximumLogicalBytes},schema).length)throw Error("Closed conditional Windows request required");
 const root=win32.resolve(request.root),parts=request.unit.replaceAll("/","\\").split("\\");
 if(root!==request.root||!/^([A-Za-z]):\\/u.test(root)||root.length>policy.maximumPathLength||parts.some(part=>!part||part==="."||part===".."||/[\\/:\u0000-\u001f]/u.test(part))||!Number.isSafeInteger(request.maximumLogicalBytes)||request.maximumLogicalBytes<0)throw Error("Closed local Windows conditional tree request required");
 const target=win32.join(root,...parts);if(target.length>policy.maximumPathLength||!target.startsWith(root.endsWith("\\")?root:root+"\\"))throw Error("Conditional tree escaped its exact root");
 return{root,target};
}
/** ♻️ Conditionally retires one exact tree while preserving foreign immutable readers and all partial effects. */
export async function retireNativeWindowsConditionalTreeV1(request:Readonly<{root:string;unit:string;maximumLogicalBytes:number;control:ResourceLeaseControl}>,port?:ReturnType<typeof openConditionalWindowsSystemPortV1>,io:ConditionalWindowsTreeIOV1=systemIO):Promise<ConditionalWindowsReceiptV1>{
 request=Object.freeze({...request});const{control}=request,{root,target}=conditionalTreePathV1(request);
 const owner=port??openConditionalWindowsSystemPortV1(),ancestorPins:ConditionalWindowsPinV1[]=[],directoryPins=new Map<string,ConditionalWindowsPinV1>();
 const held:ConditionalWindowsPinV1[]=[],entries:{path:string;pin:ConditionalWindowsPinV1;identity:ConditionalWindowsIdentityV1}[]=[],directory=win32.dirname(target);let count=1,operations=0,lastProgress=Number.NEGATIVE_INFINITY,started=performance.now(),logical=0,accepted=0,removed=0,acceptedBytes=0,removedBytes=0,primary:unknown,failed=false,result:ConditionalWindowsReceiptV1|undefined;
 const receipt=(refused:boolean,handlesSettled:boolean=true):ConditionalWindowsReceiptV1=>Object.freeze({kind:policy.kind as ConditionalWindowsReceiptV1["kind"],acceptedEntries:accepted,removedEntries:removed,acceptedLogicalBytes:acceptedBytes,removedLogicalBytes:removedBytes,handlesSettled,outcome:conditionalWindowsOutcomeV1(accepted,removed,refused)});
 const retain=(path:string,candidate:boolean)=>{checkResourceLeaseControl(control);const pin=owner.pin(path,candidate);held.push(pin);return pin;};
 const checkpoint=async(path:string)=>{checkResourceLeaseControl(control);if(++operations%policy.maximumFilesPerYield===0){const now=performance.now();if(now-lastProgress>=1000){lastProgress=now;await control.publish({resource:"windows-conditional-retirement:"+target,mode:"exclusive",elapsedMs:now-started});}await control.yieldContinuation();checkResourceLeaseControl(control);}};
 try{
  const ancestry:string[]=[];for(let path=directory;;path=win32.dirname(path)){ancestry.push(path);if(win32.dirname(path)===path)break;}
  let volume:string|undefined;for(const path of ancestry.reverse()){const pin=retain(path,false);if(!pin.identity.directory||volume!==undefined&&pin.identity.volume!==volume)throw Error("One pinned Windows volume ancestry required");volume=pin.identity.volume;ancestorPins.push(pin);}
  const walk=async(path:string,depth:number):Promise<void>=>{
   if(depth>policy.maximumDepth||path.length>policy.maximumPathLength)throw Error("Conditional tree depth/path bound exceeded");await checkpoint(path);const pin=retain(path,false),identity=pin.identity;if(identity.volume!==volume)throw Error("Conditional tree changed its pinned volume");
   if(identity.directory){directoryPins.set(path,pin);const names:string[]=[],seen=new Set<string>();await io.forEachChild(path,async name=>{checkResourceLeaseControl(control);if(++count>policy.maximumEntries||seen.has(name))throw Error("Conditional tree membership bound exceeded");seen.add(name);names.push(name);await checkpoint(path);});names.sort();pin.check();for(const name of names){if(!name||name==="."||name===".."||/[\\/:\u0000-\u001f]/u.test(name))throw Error("Exact conditional child name required");await walk(win32.join(path,name),depth+1);}entries.push({path,pin,identity});}
   else{logical+=identity.bytes;if(!Number.isSafeInteger(logical)||logical>request.maximumLogicalBytes)throw Error("Conditional tree exceeds its accepted logical bound");pin.close();const exclusive=retain(path,true);if(!sameObject(identity,exclusive.identity)||identity.bytes!==exclusive.identity.bytes||identity.write!==exclusive.identity.write||identity.change!==exclusive.identity.change)throw Error("Conditional file changed before exclusion");entries.push({path,pin:exclusive,identity});}
  };
  await walk(target,0);if(!entries.length||logical!==request.maximumLogicalBytes)throw Error("Exact conditional unit membership and logical extent required");
  for(const entry of entries){
   checkResourceLeaseControl(control);for(const ancestor of ancestorPins)ancestor.check();for(let path=win32.dirname(entry.path);path!==win32.dirname(target);path=win32.dirname(path)){const parent=directoryPins.get(path);if(!parent)throw Error("Pinned conditional parent required");parent.check();}
   let pin=entry.pin;if(entry.identity.directory){pin.check();pin.close();pin=retain(entry.path,true);if(!sameObject(entry.identity,pin.identity))throw Error("Conditional directory changed before disposition");}
   pin.remove();accepted++;acceptedBytes+=entry.identity.bytes;pin.close();
   if(!await io.exists(entry.path)){removed++;removedBytes+=entry.identity.bytes;}
   await checkpoint(entry.path);
  }
  result=receipt(false);
 }catch(error){failed=true;primary=error;}
 const closeErrors:unknown[]=[];for(const pin of [...held].reverse())try{pin.close();}catch(error){closeErrors.push(error);}try{owner.close();}catch(error){closeErrors.push(error);}
 if(closeErrors.length){primary=failed?new AggregateError([primary,...closeErrors],"Conditional retirement and handle settlement failed",{cause:primary}):new AggregateError(closeErrors,"Conditional retirement handle settlement failed");failed=true;}
 if(failed)throw new ConditionalWindowsRetirementFailureV1(receipt(true,closeErrors.length===0),primary);
 return result!;
}

/** 📏️ Observes the complete physical unit namespace before Windows retirement selection. */
export async function observeNativeWindowsConditionalTreeV1(request:WindowsConditionalRequestV1,port?:ReturnType<typeof openConditionalWindowsSystemPortV1>,io:ConditionalWindowsTreeIOV1=systemIO):Promise<Readonly<{kind:"windows-physical-unit-observation";logicalBytes:number;entries:number;recencyMs:number}>>{
 request=Object.freeze({...request});const{control}=request,{target}=conditionalTreePathV1(request),owner=port??openConditionalWindowsSystemPortV1(),held=new Set<ConditionalWindowsPinV1>();let entries=0,logicalBytes=0,recencyMs=Number.NEGATIVE_INFINITY,volume:string|undefined,operations=0,lastProgress=Number.NEGATIVE_INFINITY,started=performance.now(),failed=false,primary:unknown;
 const pin=(path:string)=>{checkResourceLeaseControl(control);const value=owner.pin(path,false);held.add(value);return value;},close=(value:ConditionalWindowsPinV1)=>{value.close();held.delete(value);},checkpoint=async()=>{checkResourceLeaseControl(control);if(++operations%policy.maximumFilesPerYield===0){const now=performance.now();if(now-lastProgress>=1000){lastProgress=now;await control.publish({resource:"windows-conditional-observation:"+target,mode:"shared",elapsedMs:now-started});}await control.yieldContinuation();checkResourceLeaseControl(control);}},timestamp=(value:string)=>{const delta=BigInt(value)-116444736000000000n;return Number(delta/10000n)+Number(delta%10000n)/10000;};
 let result:Readonly<{kind:"windows-physical-unit-observation";logicalBytes:number;entries:number;recencyMs:number}>|undefined;
 try{
  const ancestry:string[]=[];for(let path=win32.dirname(target);;path=win32.dirname(path)){ancestry.push(path);if(win32.dirname(path)===path)break;}
  for(const path of ancestry.reverse()){const current=pin(path);if(!current.identity.directory||volume!==undefined&&volume!==current.identity.volume)throw Error("One physical Windows observation volume required");volume=current.identity.volume;}
  const walk=async(path:string,depth:number):Promise<void>=>{
   if(++entries>policy.maximumEntries||depth>policy.maximumDepth||path.length>policy.maximumPathLength)throw Error("Bounded Windows physical unit observation required");await checkpoint();const current=pin(path),identity=current.identity;if(identity.volume!==volume)throw Error("Windows physical observation changed volume");
   if(identity.directory){const seen=new Set<string>();await io.forEachChild(path,async name=>{if(!name||name==="."||name===".."||/[\\/:\u0000-\u001f]/u.test(name)||seen.has(name)||seen.size>=policy.maximumEntries)throw Error("Exact bounded Windows observed child required");seen.add(name);await walk(win32.join(path,name),depth+1);});}
   else{logicalBytes+=identity.bytes;if(!Number.isSafeInteger(logicalBytes)||logicalBytes>request.maximumLogicalBytes)throw Error("Windows physical observation exceeds its finite extent");recencyMs=Math.max(recencyMs,timestamp(identity.access),timestamp(identity.write));}
   current.check();close(current);
  };
  await walk(target,0);for(const current of held)current.check();result=Object.freeze({kind:policy.observationKind as"windows-physical-unit-observation",logicalBytes,entries,recencyMs:Number.isFinite(recencyMs)?recencyMs:0});
 }catch(error){failed=true;primary=error;}
 const failures:unknown[]=[];for(const current of [...held].reverse())try{close(current);}catch(error){failures.push(error);}try{owner.close();}catch(error){failures.push(error);}if(failures.length)throw new AggregateError(failed?[primary,...failures]:failures,"Windows physical observation handle settlement failed",failed?{cause:primary}:undefined);if(failed)throw primary;return result!;
}
