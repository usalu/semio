import type { IdmlPart } from '../../🧬️schema/📸️snapshot/🟦️.ts';
import { XmlValidationCursor, type XmlSpan } from '../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧵️validation/🟦️.ts';
/** 🧷 An immutable original package with an optional borrowed byte replacement. */
export interface IdmlAdmissionSnapshot {domVersion:string;parts:readonly IdmlPart[]}
export interface IdmlAdmissionSource { snapshot: IdmlAdmissionSnapshot; replacement?: {index:number;bytes:readonly number[]}; compareVersion?: boolean }
export interface IdmlAdmissionGrant { maximumWork:number; maximumCopyBytes:number; maximumCapacityBytes:number; maximumReleaseBytes:number }
export interface IdmlVersionView { partIndex:number; value:XmlSpan; decodedLength:number }
export interface IdmlAdmissionStep { work:number; copiedBytes:number; retainedCapacityBytes:number; releasedBytes:number; done:boolean; error?:string; version?:IdmlVersionView }
const MIME='application/vnd.adobe.indesign-idml-package';
const PAGE=256;
const validGrant=(n:number)=>Number.isSafeInteger(n)&&n>=0;
const ascii=(data:readonly number[],span:XmlSpan,text:string)=>{if(span.end-span.start!==text.length)return false;for(let i=0;i<text.length;i++)if(data[span.start+i]!==text.charCodeAt(i))return false;return true;};
/** 🛂 Validates original members without cloning strings, XML trees, or member bytes. */
export class IdmlAdmissionCursor {
  private source?: IdmlAdmissionSource;
  private pages: (Uint32Array|undefined)[]=new Array(128);
  private pageCount=0;
  private pageNeed=0;
  private mask=0;
  private mode='init';
  private closing=false;
  private error?:string;
  private index=0;
  private offset=0;
  private bytes=0;
  private segment=0;
  private dots=true;
  private hash=2166136261;
  private slot=0;
  private collision=0;
  private xml=new XmlValidationCursor();
  private designmap=-1;
  private versionSpan?:XmlSpan;
  private versionOffset=0;
  private decoded=0;
  private major=0;
  private minor=0;
  private dot=false;
  private entity=0;
  private scalar=0;
  private digits=0;
  private result?:IdmlVersionView;
  private data(source: IdmlAdmissionSource,index:number):readonly number[] {return source.replacement?.index===index?source.replacement.bytes:source.snapshot.parts[index].bytes;}
  private fail(error:string):void {this.error=error;}
  /** 🧷 Admitted evidence belongs to this exact immutable original view. */
  admitsSource(source:IdmlAdmissionSource):boolean {return this.mode==='done'&&!this.error&&!this.closing&&this.source===source;}
  /** 🏷 Returns only an original span after the whole package is admitted. */
  versionView():IdmlVersionView|undefined {return this.mode==='done'&&!this.error&&!this.closing?this.result:undefined;}
  /** 🔎 Hashes, probes, and compares original names without rebuilding a collection. */
  lookup(source:IdmlAdmissionSource,key:IdmlLookupKey,cursor:IdmlNameLookup,maximumWork:number):IdmlLookupStep {
    if(!validGrant(maximumWork))throw new Error('stdio.idml.lookup-grant');
    if(this.mode!=='done'||this.error||this.closing)return{work:0,done:false,error:'stdio.idml.lookup-admission'};
    if(this.source!==source)return{work:0,done:false,error:'stdio.idml.lookup-source'};
    const state=cursor[IDML_LOOKUP_STATE];
    if(!state.key)state.key=key;else if(state.key!==key)state.error='stdio.idml.lookup-key';
    let work=0;
    while(work<maximumWork&&!state.error&&!state.done){
      if(state.phase==='hash'){
        if(state.offset===key.value.length){state.slot=state.hash&this.mask;state.phase='probe';}else state.hash=Math.imul(state.hash^key.value.charCodeAt(state.offset++),16777619)>>>0;work++;continue;
      }
      if(state.phase==='probe'){
        const row=this.pages[Math.floor(state.slot/PAGE)]![state.slot%PAGE];
        if(!row){state.done=true;work++;continue;}
        state.candidate=row-1;state.offset=0;state.phase='compare';work++;continue;
      }
      const candidate=source.snapshot.parts[state.candidate].path;
      if(candidate.length!==key.value.length){state.slot=(state.slot+1)&this.mask;state.phase='probe';work++;continue;}
      if(state.offset===candidate.length){state.done=true;state.index=state.candidate;work++;continue;}
      if(candidate.charCodeAt(state.offset)!==key.value.charCodeAt(state.offset)){state.slot=(state.slot+1)&this.mask;state.phase='probe';}else state.offset++;work++;
    }
    return{work,done:state.done,index:state.index,error:state.error};
  }
  nextCapacityBytes():number {return this.mode==='allocate' && this.pageCount<this.pageNeed?PAGE*4:0;}
  nextCopyBytes():number {return this.nextCapacityBytes()?PAGE*8:0;}
  nextReleaseBytes():number {return this.pageCount?PAGE*4:0;}
  beginClose():void {this.closing=true;this.source=undefined;this.xml.close();}
  closeOne(maximumReleaseBytes:number):IdmlAdmissionStep {
    let releasedBytes=0;
    if(!this.closing)throw new Error('stdio.idml.admission-close');
    if(this.pageCount && maximumReleaseBytes>=PAGE*4){this.pages[--this.pageCount]=undefined;releasedBytes=PAGE*4;}
    return {work:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes,done:this.pageCount===0};
  }
  terminalIsEmpty():boolean {return this.closing && !this.pageCount && !this.source && this.xml.terminalIsEmpty();}
  private emitVersion(c:number,source:IdmlAdmissionSource):void {
    if(c===46){if(this.dot || !this.major){this.fail('stdio.idml.dom-version');return;}this.dot=true;}
    else if(c>=48 && c<=57){if(this.dot)this.minor++;else this.major++;}
    else{this.fail('stdio.idml.dom-version');return;}
    if(source.compareVersion!==false && source.snapshot.domVersion.charCodeAt(this.decoded)!==c)this.fail('stdio.idml.dom-version-mismatch');
    this.decoded++;
  }
  advance(source:IdmlAdmissionSource,grant:IdmlAdmissionGrant):IdmlAdmissionStep {
    if(!validGrant(grant.maximumWork)||!validGrant(grant.maximumCopyBytes)||!validGrant(grant.maximumCapacityBytes)||!validGrant(grant.maximumReleaseBytes))throw new Error('stdio.idml.admission-grant');
    if(this.closing)return {work:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0,done:false,error:'stdio.idml.admission-closed'};
    if(!this.source)this.source=source;else if(this.source!==source)this.fail('stdio.idml.admission-source');
    let work=0,copiedBytes=0,retainedCapacityBytes=0;
    const parts=source.snapshot.parts;
    while(work<grant.maximumWork && !this.error && this.mode!=='done'){
      if(this.mode==='init'){
        if(parts.length>16384){this.fail('stdio.idml.member-limit');break;}
        if(source.replacement && (!Number.isSafeInteger(source.replacement.index)||source.replacement.index<0||source.replacement.index>=parts.length)){this.fail('stdio.idml.missing-member');break;}
        if(parts[0]?.path!=='mimetype' || this.data(source,0).length!==MIME.length){this.fail('stdio.idml.mimetype');break;}
        this.offset=0;this.mode='mime';work++;continue;
      }
      if(this.mode==='mime'){
        if(this.offset===MIME.length){let slots=1;while(slots<parts.length*2)slots*=2;this.mask=slots-1;this.pageNeed=Math.ceil(slots/PAGE);this.mode='allocate';work++;continue;}
        if(this.data(source,0)[this.offset]!==MIME.charCodeAt(this.offset))this.fail('stdio.idml.mimetype');this.offset++;work++;continue;
      }
      if(this.mode==='allocate'){
        if(this.pageCount===this.pageNeed){this.mode='part';work++;continue;}
        if(grant.maximumCapacityBytes-retainedCapacityBytes<PAGE*4 || grant.maximumCopyBytes-copiedBytes<PAGE*8)break;
        this.pages[this.pageCount++]=new Uint32Array(PAGE);retainedCapacityBytes+=PAGE*4;copiedBytes+=PAGE*8;work++;break;
      }
      if(this.mode==='part'){
        if(this.index===parts.length){if(this.designmap<0)this.fail('stdio.idml.designmap');else if(!this.versionSpan)this.fail('stdio.idml.dom-version');else{this.versionOffset=this.versionSpan.start;this.mode='version';}work++;continue;}
        const part=parts[this.index],data=this.data(source,this.index);
        if(!part.path){this.fail('stdio.idml.member-path');break;}
        if(part.path.endsWith('/') && data.length){this.fail('stdio.idml.directory-payload');break;}
        this.bytes+=data.length;if(this.bytes>8388608){this.fail('stdio.idml.byte-limit');break;}
        this.offset=0;this.segment=0;this.dots=true;this.hash=2166136261;this.mode='path';work++;continue;
      }
      if(this.mode==='path'){
        const name=parts[this.index].path;
        if(this.offset===name.length){if(this.segment && this.dots && this.segment<=2){this.fail('stdio.idml.member-path');break;}this.slot=this.hash&this.mask;this.mode='lookup';work++;continue;}
        const c=name.charCodeAt(this.offset++);
        if(c>=0xd800&&c<=0xdbff&&!(name.charCodeAt(this.offset)>=0xdc00&&name.charCodeAt(this.offset)<=0xdfff)||c>=0xdc00&&c<=0xdfff&&!(name.charCodeAt(this.offset-2)>=0xd800&&name.charCodeAt(this.offset-2)<=0xdbff)){this.fail('stdio.idml.member-path');break;}
        if(c===92 || c===58 || c===47 && (!this.segment || this.dots && this.segment<=2)){this.fail('stdio.idml.member-path');break;}
        if(c===47){this.segment=0;this.dots=true;}else{this.segment++;this.dots=this.dots && c===46;}
        this.hash=Math.imul(this.hash^c,16777619)>>>0;work++;continue;
      }
      if(this.mode==='lookup'){
        const page=this.pages[Math.floor(this.slot/PAGE)]!,at=this.slot%PAGE,previous=page[at];
        if(previous){this.collision=previous-1;this.offset=0;this.mode='collision';work++;continue;}
        page[at]=this.index+1;
        if(parts[this.index].path==='designmap.xml')this.designmap=this.index;
        if(parts[this.index].path.endsWith('.xml')){this.xml.restart();this.mode='xml';}else{this.offset=0;this.mode='bytes';}work++;continue;
      }
      if(this.mode==='collision'){
        const a=parts[this.collision].path,b=parts[this.index].path;
        if(a.length!==b.length){this.slot=(this.slot+1)&this.mask;this.mode='lookup';work++;continue;}
        if(this.offset===a.length){this.fail('stdio.idml.duplicate-member');break;}
        if(a.charCodeAt(this.offset)!==b.charCodeAt(this.offset)){this.slot=(this.slot+1)&this.mask;this.mode='lookup';}else this.offset++;work++;continue;
      }
      if(this.mode==='xml'){
        const data=this.data(source,this.index),step=this.xml.advance(data,grant.maximumWork-work);work+=step.work;
        if(step.error){this.fail(step.error);break;}
        if(this.index===this.designmap && step.event){const event=step.event;
          if(event.depth===1 && event.kind==='element'){if(!ascii(data,event.name,'Document'))this.fail('stdio.idml.designmap');}
          if(event.depth===1 && event.kind==='attribute' && ascii(data,event.name,'DOMVersion'))this.versionSpan={...event.value};
        }
        if(step.done){this.index++;this.mode='part';}continue;
      }
      if(this.mode==='bytes'){const data=this.data(source,this.index);if(this.offset===data.length){this.index++;this.mode='part';}else{const byte=data[this.offset++];if(!Number.isInteger(byte)||byte<0||byte>255)this.fail('stdio.idml.member-bytes');}work++;continue;}
      if(this.mode==='version'){
        const data=this.data(source,this.designmap),end=this.versionSpan!.end;
        if(this.versionOffset===end){
          if(this.entity || !this.dot || !this.major || !this.minor)this.fail('stdio.idml.dom-version');
          else if(source.compareVersion!==false && source.snapshot.domVersion.length!==this.decoded)this.fail('stdio.idml.dom-version-mismatch');
          else{this.result={partIndex:this.designmap,value:{...this.versionSpan!},decodedLength:this.decoded};this.mode='done';}work++;continue;
        }
        const c=data[this.versionOffset++];work++;
        if(!this.entity){if(c===38){this.entity=1;this.scalar=0;this.digits=0;}else this.emitVersion(c,source);continue;}
        if(this.entity===1){if(c!==35)this.fail('stdio.idml.dom-version');else this.entity=10;continue;}
        if(this.entity===10 && !this.digits && c===120){this.entity=16;continue;}
        if(c===59){if(!this.digits)this.fail('stdio.idml.dom-version');else this.emitVersion(this.scalar,source);this.entity=0;continue;}
        const digit=c>=48 && c<=57?c-48:this.entity===16 && c>=65 && c<=70?c-55:this.entity===16 && c>=97 && c<=102?c-87:-1;
        if(digit<0 || digit>=this.entity || this.scalar>0x10ffff)this.fail('stdio.idml.dom-version');else{this.scalar=this.scalar*this.entity+digit;this.digits++;}continue;
      }
      this.fail('stdio.idml.admission-state');
    }
    return {work,copiedBytes,retainedCapacityBytes,releasedBytes:0,done:this.mode==='done',error:this.error,version:this.result};
  }
}

/** 🔤 A single decoded version byte or one bounded lexical progress turn. */
export interface IdmlVersionDecodeStep {work:number;byte?:number;done:boolean;error?:string}
/** 🔢 Decodes admitted numeric references while keeping the original XML member borrowed. */
export class IdmlVersionDecoder {
  private at:number;
  private entity=0;
  private value=0;
  private digits=0;
  private count=0;
  private source?:readonly number[];
  private error?:string;
  constructor(private readonly view:IdmlVersionView){this.at=view.value.start;}
  advance(source:readonly number[],maximumWork:number):IdmlVersionDecodeStep {
    if(!validGrant(maximumWork))throw new Error('stdio.idml.version-grant');
    if(!this.source)this.source=source;else if(this.source!==source)this.error='stdio.idml.version-source';
    if(!Number.isSafeInteger(this.at)||this.at<0||!Number.isSafeInteger(this.view.value.end)||this.view.value.end>source.length||this.view.value.end<this.at||!Number.isSafeInteger(this.view.decodedLength)||this.view.decodedLength<0)this.error='stdio.idml.version-span';
    let work=0;
    while(work<maximumWork&&!this.error){
      if(this.at===this.view.value.end){if(this.entity||this.count!==this.view.decodedLength)this.error='stdio.idml.dom-version';return{work:work+1,done:!this.error,error:this.error};}
      const c=source[this.at++];work++;let output:number|undefined;
      if(!this.entity){if(c===38){this.entity=1;this.value=0;this.digits=0;}else output=c;}
      else if(this.entity===1){if(c!==35)this.error='stdio.idml.dom-version';else this.entity=10;}
      else if(this.entity===10&&!this.digits&&c===120)this.entity=16;
      else if(c===59){if(!this.digits)this.error='stdio.idml.dom-version';else output=this.value;this.entity=0;}
      else{const digit=c>=48&&c<=57?c-48:this.entity===16&&c>=65&&c<=70?c-55:this.entity===16&&c>=97&&c<=102?c-87:-1;if(digit<0||digit>=this.entity||this.value>0x10ffff)this.error='stdio.idml.dom-version';else{this.value=this.value*this.entity+digit;this.digits++;}}
      if(output!==undefined){if(!(output===46||output>=48&&output<=57)||this.count>=this.view.decodedLength)this.error='stdio.idml.dom-version';else{this.count++;return{work,byte:output,done:false};}}
    }
    return{work,done:false,error:this.error};
  }
}
/** 📖 Synchronous cold admission; retained callers use the two explicit cursors instead. */
export function admitIdmlParts(parts:readonly IdmlPart[]):string {
  const source:IdmlAdmissionSource={snapshot:{domVersion:'',parts},compareVersion:false},cursor=new IdmlAdmissionCursor();
  try{
    let step:IdmlAdmissionStep;
    do{step=cursor.advance(source,{maximumWork:65536,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.error)throw new Error(step.error);}while(!step.done);
    const view=step.version!,decoder=new IdmlVersionDecoder(view),data=parts[view.partIndex].bytes,bytes=new Uint8Array(view.decodedLength);let at=0;
    for(;;){const step=decoder.advance(data,65536);if(step.error)throw new Error(step.error);if(step.byte!==undefined)bytes[at++]=step.byte;if(step.done)break;}
    return new TextDecoder().decode(bytes);
  }finally{cursor.beginClose();while(cursor.nextReleaseBytes())cursor.closeOne(cursor.nextReleaseBytes());}
}
/** 🛂 The production admission owner for native TypeScript event algebra. */
export const IDML_ADMISSION:Readonly<{admit:(parts:readonly IdmlPart[])=>string}>={admit:admitIdmlParts};

/** 🗝 A stable original key view; callers keep its value immutable for the lookup. */
export interface IdmlLookupKey {readonly value:string}
export interface IdmlLookupStep {work:number;done:boolean;index?:number;error?:string}
/** 🔎 Fixed numeric member lookup state borrows the already admitted name index. */
const IDML_LOOKUP_STATE:unique symbol=Symbol('idml.lookup');
export class IdmlNameLookup {
  /** 🔄 Reuses fixed lookup state after its prior query has settled. */
  restart():void {const state=this[IDML_LOOKUP_STATE];state.key=undefined;state.phase='hash';state.offset=0;state.hash=2166136261;state.slot=0;state.candidate=0;state.done=false;state.index=undefined;state.error=undefined;}
  readonly [IDML_LOOKUP_STATE]:{key?:IdmlLookupKey;phase:string;offset:number;hash:number;slot:number;candidate:number;done:boolean;index?:number;error?:string}={phase:'hash',offset:0,hash:2166136261,slot:0,candidate:0,done:false};
}
