import assert from "node:assert/strict";
import * as api from "../../../🟦️.ts";
import {withIssuedNativeImageV1,readIssuedNativeImageV1,issuedNativeStoragePathV1,type IssuedNativeImageV1} from "../../../../../../📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📥️issuance/🟦️.ts";
import type {NativeFixedPlatformOptionsV1} from "../../../../../../📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../🎛️owned-execution/🟦️.ts";
import {resourceLeasePathV1,type ResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";
import type {ScriptInvocation} from "../../../../🟦️.ts";

const refused=(run:()=>unknown):boolean=>{try{run();return false;}catch{return true;}};
const next=()=>new Promise<void>(done=>setImmediate(done));
/** 🧪️ Runs each actual process birth in its own owned child; all accepted operations settle before returning. */
export async function proveAcceptedOriginalDrainV1(mode:string,options:(invocation:ScriptInvocation,control:ResourceLeaseControl)=>NativeFixedPlatformOptionsV1):Promise<Record<string,boolean>>{
 const result:Record<string,boolean>={};
 if(mode==="issuer-canonical-drain"){
  const{proveCanonicalRootControlV1}=await import("../../../🧪️tests/📥️canonical/🟦️.ts");await proveCanonicalRootControlV1();
 }else if(mode==="issuer-drain-failure"){
  const unhandled:unknown[]=[],observe=(error:unknown)=>{unhandled.push(error);};process.on("unhandledRejection",observe);
  let release!:()=>void,entered!:()=>void,failure:unknown;const ready=new Promise<void>(done=>{entered=done;});
  const parent=api.receiveScriptProcessInvocation(process.env,async original=>{void api.withIssuedResourceControlV1(original,"lease:unhandled-failure",10000,async()=>{throw Error("Detached accepted failure");});void api.withIssuedResourceControlV1(original,"lease:blocked-sibling",10000,async()=>{entered();await new Promise<void>(done=>{release=done;});});});void parent.catch(error=>{failure=error;});
  try{await ready;await next();await next();result.detachedPublicFailureNeverUnhandled=unhandled.length===0;assert.equal(result.detachedPublicFailureNeverUnhandled,true,"exact returned accepted promise must be observed while sibling drains");}
  finally{release();await parent.catch(error=>{failure=error;});process.removeListener("unhandledRejection",observe);}
  result.detachedFailureRejectsOriginal=String(failure).includes("Detached accepted failure");
 }else if(["issuer-drain","issuer-drain-interrupt"].includes(mode)){
  let release!:()=>void,entered!:()=>void,settled=false,stopped=false,held!:ResourceLeaseControl,birth!:ScriptInvocation,child!:Promise<void>,failure:unknown;
  const ready=new Promise<void>(done=>{entered=done;}),listeners=process.listenerCount("SIGINT"),terminators=process.listenerCount("SIGTERM"),parent=api.receiveScriptProcessInvocation(process.env,async original=>{birth=original;child=api.withIssuedResourceControlV1(original,"lease:detached-proof",10000,async control=>{held=control;try{entered();await new Promise<void>(done=>{release=done;});api.requireIssuedResourceControlV1(control);if(mode==="issuer-drain-failure")throw Error("Detached accepted failure");}finally{stopped=true;}});void child.catch(()=>{});});
  void parent.then(()=>{settled=true;},error=>{failure=error;settled=true;});
  try{
   await ready;await next();result.parentWaitsForAcceptedChild=!settled;result.acceptedChildRemainsLiveDuringDrain=!refused(()=>api.requireIssuedResourceControlV1(held));
   try{await api.withIssuedResourceControlV1(birth,"lease:late-proof",10000,async()=>{});}catch{result.delayedNewAdmissionRefused=true;}
   if(mode==="issuer-drain-interrupt"){const retained=process.listenerCount("SIGINT")===listeners+1;process.emit("SIGINT");await next();result.interruptionDrainRetainsHandlersUntilChildStop=retained&&process.listenerCount("SIGTERM")===terminators+1&&!settled&&birth.control.signal.aborted;}
   assert.equal(result.parentWaitsForAcceptedChild,true,"accepted child must retain Original until actual settlement");
  }finally{release();await child.catch(()=>{});await parent.catch(()=>{});}
  result.acceptedChildSettlesBeforeOriginalRetires=stopped&&refused(()=>api.requireIssuedScriptInvocationV1(birth));
  if(mode==="issuer-drain-interrupt")result.interruptionDrainSettlesWithOriginalFailure=String(failure).includes("Original Script process interrupted")&&process.listenerCount("SIGINT")===listeners&&process.listenerCount("SIGTERM")===terminators;
  if(mode==="issuer-drain")assert.equal(failure,undefined);
 }else if(mode==="issuer-nested-drain"){
  await api.receiveScriptProcessInvocation(process.env,async original=>{
   let release!:()=>void,entered!:()=>void,parentControl!:ResourceLeaseControl,childControl!:ResourceLeaseControl,settled=false,stopped=false,child!:Promise<void>;
   const ready=new Promise<void>(done=>{entered=done;}),parent=api.withIssuedResourceControlV1(original,"lease:nested-parent",10000,async control=>{parentControl=control;child=api.withRequestedIssuedResourceControlV1(control,0,async nested=>{childControl=nested;entered();await new Promise<void>(done=>{release=done;});api.requireIssuedResourceControlV1(nested);stopped=true;});void child.catch(()=>{});});
   void parent.then(()=>{settled=true;},()=>{settled=true;});
   try{await ready;await next();result.nestedParentWaitsForAcceptedChild=!settled&&!refused(()=>api.requireIssuedResourceControlV1(childControl));try{await api.withRequestedIssuedResourceControlV1(parentControl,0,async()=>{});}catch{result.nestedLateAdmissionRefused=true;}assert.equal(result.nestedParentWaitsForAcceptedChild,true);}
   finally{release();await child;await parent;}
   result.nestedChildSettlesBeforeParentRetires=stopped&&refused(()=>api.requireIssuedResourceControlV1(parentControl));
  });
 }else if(mode==="issuer-image-failure-drain"){
  const unhandled:unknown[]=[],observe=(error:unknown)=>{unhandled.push(error);};process.on("unhandledRejection",observe);
  let release!:()=>void,entered!:()=>void,failure:unknown,settled=false;
  const ready=new Promise<void>(done=>{entered=done;}),parent=api.receiveScriptProcessInvocation(process.env,original=>api.withIssuedResourceControlV1(original,"lease:path-failure",10000,control=>withIssuedNativeImageV1(options(original,control),async loan=>{void issuedNativeStoragePathV1(loan,"/outside-original-image");entered();await new Promise<void>(done=>{release=done;});})));
  void parent.then(()=>{settled=true;},error=>{failure=error;settled=true;});
  try{await ready;await next();await next();result.detachedPathFailureNeverUnhandled=unhandled.length===0;result.detachedPathImageRetainedUntilCallbackStops=!settled;assert.equal(result.detachedPathFailureNeverUnhandled,true,"exact path-operation promise must remain observed");assert.equal(result.detachedPathImageRetainedUntilCallbackStops,true);}
  finally{release?.();await parent.catch(error=>{failure=error;});process.removeListener("unhandledRejection",observe);}
  result.detachedPathFailureRejectsOriginal=String(failure).includes("Native storage path");
 }else if(mode==="issuer-image-drain"){
  await api.receiveScriptProcessInvocation(process.env,original=>api.withIssuedResourceControlV1(original,"lease:image-supervisor",10000,async supervisor=>{
   const declared=options(original,supervisor),path=resourceLeasePathV1(declared.leaseDirectory,"native-fixed-image:"+declared.directory),descriptors=async()=>{const text=await captureOwnedCommandV1("lsof",["-a","-p",String(process.pid),"-Fn"],declared.directory,{control:supervisor,output:{maximumBytes:1048576,maximumLines:8192}});return text.split("\n").filter(line=>line==="n"+path).length;},before=await descriptors();
   let release!:()=>void,entered!:()=>void,parentControl!:ResourceLeaseControl,image!:IssuedNativeImageV1,settled=false,observed=0,work!:Promise<void>;
   const reads:Promise<unknown>[]=[],ready=new Promise<void>(done=>{entered=done;}),parent=api.withIssuedResourceControlV1(original,"lease:image-holder",10000,async control=>{parentControl=control;work=withIssuedNativeImageV1(options(original,control),async loan=>{image=loan;entered();await new Promise<void>(done=>{release=done;});for(let n=0;n<16;n++){const reading=readIssuedNativeImageV1(loan).then(()=>{observed++;});reads.push(reading);void reading.catch(()=>{});}});void work.catch(error=>{entered();throw error;}).catch(()=>{});});
   void parent.then(()=>{settled=true;},()=>{settled=true;});
   try{await ready;await next();const during=await descriptors();result.imageAdmissionRetainedThroughCallbackAndClose=!settled&&during>before;try{await withIssuedNativeImageV1(options(original,parentControl),async()=>{});}catch{result.lateImageAdmissionRefused=true;}assert.equal(result.imageAdmissionRetainedThroughCallbackAndClose,true);}
   finally{release?.();await work;await parent;}
   await Promise.all(reads);result.acceptedImageObservationsDrained=observed===16;result.realImageDescriptorClosedBeforeParentSettles=await descriptors()===before;
   try{await readIssuedNativeImageV1(image);}catch{result.imageTokenRetiredAfterDrain=true;}
  }));
 }else throw Error("Declared actual issuance drain mode required");
 for(const[key,value]of Object.entries(result))assert.equal(value,true,key);
 return result;
}
