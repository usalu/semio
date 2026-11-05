export interface ChildEditorGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface ChildEditorDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface ChildEditorReceipt{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
export interface ChildEditorFields{parentArtifact:string;owner:string;slot:string;childId:string;artifactId:string;artifactKind:string;standard:string;subset:string;editorId:string;commandId:string;parentRevision:Uint8Array;childRevision:Uint8Array}
export interface ChildEditorRequest{identity:unknown;expectedReferenceIdentity:unknown;fields:ChildEditorFields}
export interface ChildEditorCapability{artifactKind:string;standard:string;subset:string;editorId:string;commandId:string}
export interface ChildEditorCurrent{parentIdentity:unknown;childIdentity:unknown;fields:ChildEditorFields;capability:ChildEditorCapability}
const keys=['parentArtifact','owner','slot','childId','artifactId','artifactKind','standard','subset','editorId','commandId'] as const;
const capabilityKeys=['artifactKind','standard','subset','editorId','commandId'] as const;
const STATE_BYTES=88,STAMP_BYTES=104;
class Stamp{readonly #qualified=true;constructor(readonly commandIdentity:unknown,readonly expectedReferenceIdentity:unknown,readonly parentIdentity:unknown,readonly childIdentity:unknown,readonly capabilityIdentity:unknown,readonly parentRevision:Uint8Array,readonly childRevision:Uint8Array){}qualified(){return this.#qualified;}childAdmissionStamp(){return{parentIdentity:this.parentIdentity,expectedReferenceIdentity:this.expectedReferenceIdentity,childIdentity:this.childIdentity,capabilityIdentity:this.capabilityIdentity,requestIdentity:this.commandIdentity,parentRevision:this.parentRevision,childRevision:this.childRevision};}}
export type ChildEditorStamp=Stamp;
export type ChildEditorSourceStep={kind:'pending'}|{kind:'ready';stamp:ChildEditorStamp};
/** 🧭️ One original immutable command and two original read identities qualify one exact member/editor route. */
export class ChildEditorSourceCursor{
 private phase=0;private field=0;private offset=0;private command:unknown;private expected:unknown;private parent:unknown;private child:unknown;private capability:unknown;private bound=false;private ticks=0;private requestFields:ChildEditorFields|undefined;private currentFields:ChildEditorFields|undefined;
 private receipt:ChildEditorReceipt={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
 get progress(){return this.ticks;}get lastReceipt():Readonly<ChildEditorReceipt>{return this.receipt;}
 nextDemand():ChildEditorDemand{const key=this.phase===1?keys[this.field]:this.phase===4?capabilityKeys[this.field]:undefined,text=key?this.currentFields?.[key]:undefined,pairs=this.phase===2||this.phase===3?64:text===undefined?0:Math.min(64,Math.max(0,text.length-this.offset)*3)*2;return{copyBytes:STATE_BYTES+pairs+(this.phase===5?STAMP_BYTES:0),capacityBytes:0,releaseBytes:0,depth:1};}
 private compare(a:string,b:string):boolean{if(a.length!==b.length)throw Error('child editor identity differs');let bytes=0;for(let count=0;count<16&&this.offset<a.length;count++){const point=a.codePointAt(this.offset)!;const width=point<128?1:point<2048?2:point<65536?3:4;this.receipt.copiedBytes+=width*2;if(point>=55296&&point<=57343||point!==b.codePointAt(this.offset))throw Error('child editor identity differs');bytes+=width;this.offset+=point>65535?2:1;}if(bytes>64)throw Error('child editor identity block extent');if(this.offset===a.length){this.offset=0;return true;}return false;}
 advance(request:ChildEditorRequest,current:ChildEditorCurrent,grant:ChildEditorGrant):ChildEditorSourceStep{
  this.receipt={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};if(this.phase>=6)throw Error('child editor source cancelled');const d=this.nextDemand();if(grant.maximumItems===0||grant.maximumDepth<d.depth||grant.maximumCopyBytes<d.copyBytes)return{kind:'pending'};
  if(this.bound&&(this.command!==request.identity||this.expected!==request.expectedReferenceIdentity||this.parent!==current.parentIdentity||this.child!==current.childIdentity||this.capability!==current.capability||this.requestFields!==request.fields||this.currentFields!==current.fields))throw Error('child editor source changed');
  if(request.fields.parentRevision.length!==32||request.fields.childRevision.length!==32||current.fields.parentRevision.length!==32||current.fields.childRevision.length!==32)throw Error('child editor revision length');
  this.receipt={copiedItems:1,copiedBytes:STATE_BYTES+(this.phase===5?STAMP_BYTES:0),retainedCapacityBytes:0,releasedBytes:0};let stamp:ChildEditorStamp|undefined;
  if(this.phase===0){for(const source of [request.identity,request.expectedReferenceIdentity,current.parentIdentity,current.childIdentity])if(typeof source!=='object'||source===null)throw Error('child editor source missing');this.capability=current.capability;this.command=request.identity;this.expected=request.expectedReferenceIdentity;this.parent=current.parentIdentity;this.child=current.childIdentity;this.bound=true;this.requestFields=request.fields;this.currentFields=current.fields;this.phase=1;}
  else if(this.phase===1){const key=keys[this.field]!;if(this.compare(request.fields[key],current.fields[key])){this.field++;if(this.field===keys.length){this.phase=2;this.field=0;}}}
  else if(this.phase===2||this.phase===3){const key=this.phase===2?'parentRevision':'childRevision';this.receipt.copiedBytes+=64;for(let at=0;at<32;at++)if(request.fields[key][at]!==current.fields[key][at])throw Error('child editor '+key+' differs');this.offset=0;this.phase++;if(this.phase===4)this.field=0;}
  else if(this.phase===4){const key=capabilityKeys[this.field]!;if(this.compare(current.fields[key],current.capability[key])){this.field++;if(this.field===capabilityKeys.length)this.phase=5;}}
  else stamp=new Stamp(this.command,this.expected,this.parent,this.child,this.capability,current.fields.parentRevision,current.fields.childRevision);
  this.ticks++;return stamp?{kind:'ready',stamp}:{kind:'pending'};
 }
 cancel(){this.phase=6;}
 closeOne(grant:ChildEditorGrant):boolean{this.receipt={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};if(this.phase===7)return true;if(grant.maximumItems===0||grant.maximumCopyBytes<STATE_BYTES||grant.maximumDepth<1)return false;this.command=undefined;this.expected=undefined;this.parent=undefined;this.child=undefined;this.capability=undefined;this.bound=false;this.requestFields=undefined;this.currentFields=undefined;this.phase=7;this.receipt={copiedItems:1,copiedBytes:STATE_BYTES,retainedCapacityBytes:0,releasedBytes:0};return true;}
 terminalIsEmpty(){return this.phase===7;}
}
