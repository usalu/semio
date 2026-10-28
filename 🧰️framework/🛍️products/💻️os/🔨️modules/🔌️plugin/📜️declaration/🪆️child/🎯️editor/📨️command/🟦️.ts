import{childAdmissionAdmits,emptyChildAdmissionProgress,type ChildAdmissionSource,type ChildAdmissionGrant,type ChildAdmissionDemand,type ChildAdmissionProgress}from'../../../../../../../../🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts';
import type{ChildVectorReadStamp,ChildVectorMemberReadV1}from'../../📖️vector/🟦️.ts';
import{ChildEditorSourceCursor,type ChildEditorCurrent,type ChildEditorStamp}from'../🧭️source/🟦️.ts';
import type{ChildEditorRegistration}from'../📇️registration/🟦️.ts';
const header=512;
const fixed=():ChildAdmissionDemand=>({copyBytes:header,capacityBytes:0,releaseBytes:0,depth:1});
const paid=(p:ChildAdmissionProgress):ChildAdmissionProgress=>({...p,copiedItems:Math.max(1,p.copiedItems),copiedBytes:p.copiedBytes+header});
const sameSource=(a:ChildAdmissionSource,b:ChildAdmissionSource)=>a.identity===b.identity&&a.projection===b.projection&&a.value===b.value;
function fail(error:unknown,bytes=0):never{throw Object.assign(error instanceof Error?error:Error(String(error)),{retainedProgress:paid({copiedItems:1,copiedBytes:bytes,retainedCapacityBytes:0,releasedBytes:0})})}
/** 📨️ Logical custody mirrors native sealed source bindings; original issuer allocation is native-only. */
export class OriginalChildEditorCommandCursor{
 private issued:ChildEditorStamp|undefined;private parent:ChildAdmissionSource|undefined;private command:ChildAdmissionSource|undefined;private read:ChildVectorReadStamp|undefined;private capability:ChildEditorRegistration<unknown>['identity']|undefined;private cursor=new ChildEditorSourceCursor();private phase:'parent'|'command'|'qualify'|'closed'='parent';private closing=false;
 private constructor(){}
 static birthDemand(){return fixed()}
 static admit(g:ChildAdmissionGrant){if(!childAdmissionAdmits(g,fixed()))throw Error('Original child command birth is unfunded');return{cursor:new OriginalChildEditorCommandCursor(),progress:paid(emptyChildAdmissionProgress())}}
 nextDemand():ChildAdmissionDemand{const child=this.phase==='qualify'?this.cursor.nextDemand():{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};return{...child,copyBytes:child.copyBytes+header,depth:child.depth+1}}
 advance<S>(parent:ChildAdmissionSource,command:ChildAdmissionSource,qualified:ChildVectorReadStamp,read:ChildVectorMemberReadV1,parentArtifact:string,registration:ChildEditorRegistration<S>,g:ChildAdmissionGrant):{progress:ChildAdmissionProgress;stamp?:ChildEditorStamp}{
  const d=this.nextDemand();if(!childAdmissionAdmits(g,d))return{progress:emptyChildAdmissionProgress()};
  try{
   if(this.closing||this.phase==='closed')throw Error('Original child command is closing');
   const current=read.memberMetadata();if(!qualified.qualified()||!sameSource(parent,qualified.original)||qualified.parentIdentity!==current.parentIdentity||qualified.childIdentity!==current.childIdentity)throw Error('Original child command read qualification changed');
   if(current.parentRevision.length!==32||current.childRevision.length!==32)throw Error('Original child command revision width differs');for(let at=0;at<32;at++)if(qualified.parentRevision[at]!==current.parentRevision[at]||qualified.childRevision[at]!==current.childRevision[at])throw Error('Original child command revision changed');
   if(this.parent&&(!sameSource(this.parent,parent)||!sameSource(this.command!,command)||this.read!==qualified||this.capability!==registration.identity))throw Error('Original child command source changed');
   if(this.phase==='parent'){this.parent={identity:parent.identity,projection:parent.projection,value:parent.value};this.command={identity:command.identity,projection:command.projection,value:command.value};this.read=qualified;this.capability=registration.identity;this.phase='command';return{progress:paid(emptyChildAdmissionProgress())}}
   if(this.phase==='command'){this.phase='qualify';return{progress:paid(emptyChildAdmissionProgress())}}
   const fields={...current,parentArtifact,editorId:registration.identity.editorId,commandId:registration.identity.commandId,parentRevision:current.parentRevision as Uint8Array,childRevision:current.childRevision as Uint8Array};
   const actual:ChildEditorCurrent={parentIdentity:current.parentIdentity,childIdentity:current.childIdentity,fields,capability:registration.identity};
   const step=this.cursor.advance({identity:command.identity,expectedReferenceIdentity:qualified.referenceIdentity,fields},actual,{...g,maximumCopyBytes:g.maximumCopyBytes-header,maximumDepth:g.maximumDepth-1});if(step.kind==='ready')this.issued=step.stamp;return{progress:paid(this.cursor.lastReceipt),...(step.kind==='ready'?{stamp:step.stamp}:{})};
  }catch(error){fail(error,this.phase==='qualify'?d.copyBytes-header:0)}
 }
 takeAuthorityDemand():ChildAdmissionDemand{return{copyBytes:header+authorityHeader+72,capacityBytes:0,releaseBytes:0,depth:2}}
 takeAuthority(child:ChildAdmissionSource,read:OriginalChildEditorCommandRead,g:ChildAdmissionGrant):{authority:OriginalChildEditorCommandAuthority;progress:ChildAdmissionProgress}|undefined{const d=this.takeAuthorityDemand();if(!childAdmissionAdmits(g,d))return undefined;try{if(this.closing||this.phase!=='qualify'||!this.issued)throw Error('Original child authority is not ready');const current=read.memberMetadata(),source=read.originalChildSource();if(!sameSource(source,child)||this.issued.parentIdentity!==current.parentIdentity||this.issued.childIdentity!==current.childIdentity)throw Error('Original child typed source changed');for(let at=0;at<32;at++)if(this.issued.parentRevision[at]!==current.parentRevision[at]||this.issued.childRevision[at]!==current.childRevision[at])throw Error('Original child authority revision changed');const authority=new OriginalChildEditorCommandAuthority(issueToken,this.parent!,this.command!,{identity:child.identity,projection:child.projection,value:child.value},this.issued);this.cursor.closeOne({...g,maximumCopyBytes:g.maximumCopyBytes-header-authorityHeader,maximumDepth:g.maximumDepth-1});this.issued=undefined;this.parent=undefined;this.command=undefined;this.read=undefined;this.capability=undefined;this.phase='closed';return{authority,progress:{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:0}}}catch(error){fail(error)}}
 beginClose(){this.closing=true;this.cursor.cancel()}
 closeDemand(_body:number):ChildAdmissionDemand{return this.phase==='closed'?{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0}:{copyBytes:header+72,capacityBytes:0,releaseBytes:0,depth:2}}
 closeStep(g:ChildAdmissionGrant):ChildAdmissionProgress{if(!this.closing)throw Error('Original child command closure has not begun');const d=this.closeDemand(g.maximumCopyBytes);if(this.phase==='closed'||!childAdmissionAdmits(g,d))return emptyChildAdmissionProgress();this.cursor.closeOne({...g,maximumCopyBytes:g.maximumCopyBytes-header,maximumDepth:g.maximumDepth-1});this.issued=undefined;this.parent=undefined;this.command=undefined;this.read=undefined;this.capability=undefined;this.phase='closed';return paid(this.cursor.lastReceipt)}
 terminalIsEmpty(){return this.phase==='closed'&&this.parent===undefined&&this.command===undefined&&this.cursor.terminalIsEmpty()}
}

const issueToken=Symbol('Original child command authority');const authorityHeader=512;
export interface OriginalChildEditorCommandRead extends ChildVectorMemberReadV1{originalChildSource():ChildAdmissionSource}
/** 🔐️ The portable logical owner preserves all three original sources beside its authentic issued stamp. */
export class OriginalChildEditorCommandAuthority{
 private closing=false;
 constructor(token:typeof issueToken,private parent:ChildAdmissionSource|undefined,private command:ChildAdmissionSource|undefined,private child:ChildAdmissionSource|undefined,private issued:ChildEditorStamp|undefined){if(token!==issueToken)throw Error('Original child authority requires its original issuer')}
 stamp(){return this.closing?undefined:this.issued}
 beginClose(){this.closing=true}
 closeDemand(_body:number):ChildAdmissionDemand{return this.terminalIsEmpty()?{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0}:{copyBytes:authorityHeader,capacityBytes:0,releaseBytes:0,depth:2}}
 closeStep(g:ChildAdmissionGrant):ChildAdmissionProgress{if(!this.closing)throw Error('Original child authority closure has not begun');const d=this.closeDemand(g.maximumCopyBytes);if(this.terminalIsEmpty()||!childAdmissionAdmits(g,d))return emptyChildAdmissionProgress();if(this.child)this.child=undefined;else if(this.command)this.command=undefined;else if(this.parent)this.parent=undefined;else this.issued=undefined;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:0}}
 terminalIsEmpty(){return this.parent===undefined&&this.command===undefined&&this.child===undefined&&this.issued===undefined}
}
