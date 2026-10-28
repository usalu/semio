import {IdmlAdmissionCursor,IdmlNameLookup,IdmlVersionDecoder,type IdmlAdmissionSource} from '../🟦️.ts';
/** 🔺 Original source coordinates identify typed sparse fields before any field clone. */
export type IdmlChangeEvent={kind:'version'}|{kind:'order'}|{kind:'remove';before:number}|{kind:'part';next:number;before?:number};
export interface IdmlChangeStep {work:number;done:boolean;event?:IdmlChangeEvent;error?:string}
const bytes=(source:IdmlAdmissionSource,index:number)=>source.replacement?.index===index?source.replacement.bytes:source.snapshot.parts[index].bytes;
/** 🧭 A read-only numeric change stream reuses both admitted member indexes. */
export class IdmlChangeCursor {
  private base?:IdmlAdmissionSource;
  private next?:IdmlAdmissionSource;
  private mode='init';
  private index=0;
  private offset=0;
  private before=0;
  private baseDecoder?:IdmlVersionDecoder;
  private nextDecoder?:IdmlVersionDecoder;
  private baseByte?:number;
  private nextByte?:number;
  private baseDone=false;
  private nextDone=false;
  private lookup=new IdmlNameLookup();
  private key={value:''};
  private active=false;
  close():void {this.mode='closed';this.base=undefined;this.next=undefined;this.key.value='';this.lookup.restart();this.baseDecoder=undefined;this.nextDecoder=undefined;}
  terminalIsEmpty():boolean{return this.mode==='closed';}
  advance(base:IdmlAdmissionSource,next:IdmlAdmissionSource,baseIndex:IdmlAdmissionCursor,nextIndex:IdmlAdmissionCursor,maximumWork:number):IdmlChangeStep {
    if(!Number.isSafeInteger(maximumWork)||maximumWork<0)throw new Error('stdio.idml.change-grant');
    if(this.mode==='closed')return{work:0,done:false,error:'stdio.idml.change-closed'};
    if(!this.base){this.base=base;this.next=next;}else if(this.base!==base||this.next!==next)return{work:0,done:false,error:'stdio.idml.change-source'};
    if(!baseIndex.admitsSource(base)||!nextIndex.admitsSource(next))return{work:0,done:false,error:'stdio.idml.change-admission'};
    let work=0;
    while(work<maximumWork&&this.mode!=='done'){
      if(this.mode==='init'){this.baseDecoder=new IdmlVersionDecoder(baseIndex.versionView()!);this.nextDecoder=new IdmlVersionDecoder(nextIndex.versionView()!);this.mode='version';work++;continue;}
      if(this.mode==='version'){
        if(this.baseByte===undefined&&!this.baseDone){const view=baseIndex.versionView()!,step=this.baseDecoder!.advance(bytes(base,view.partIndex),maximumWork-work);work+=step.work;if(step.error)return{work,done:false,error:step.error};this.baseByte=step.byte;this.baseDone=step.done;continue;}
        if(this.nextByte===undefined&&!this.nextDone){const view=nextIndex.versionView()!,step=this.nextDecoder!.advance(bytes(next,view.partIndex),maximumWork-work);work+=step.work;if(step.error)return{work,done:false,error:step.error};this.nextByte=step.byte;this.nextDone=step.done;continue;}
        if(this.baseByte!==this.nextByte||this.baseDone!==this.nextDone){this.mode='order-init';work++;return{work,done:false,event:{kind:'version'}};}
        if(this.baseDone&&this.nextDone)this.mode='order-init';else{this.baseByte=undefined;this.nextByte=undefined;}work++;continue;
      }
      if(this.mode==='order-init'){this.index=0;this.offset=0;work++;if(base.snapshot.parts.length!==next.snapshot.parts.length){this.mode='remove';return{work,done:false,event:{kind:'order'}};}this.mode='order';continue;}
      if(this.mode==='order'){
        if(this.index===base.snapshot.parts.length){this.index=0;this.mode='remove';work++;continue;}
        const a=base.snapshot.parts[this.index].path,b=next.snapshot.parts[this.index].path;
        if(a.length!==b.length){this.index=0;this.mode='remove';work++;return{work,done:false,event:{kind:'order'}};}
        if(this.offset===a.length){this.index++;this.offset=0;work++;continue;}
        if(a.charCodeAt(this.offset)!==b.charCodeAt(this.offset)){this.index=0;this.offset=0;this.mode='remove';work++;return{work,done:false,event:{kind:'order'}};}this.offset++;work++;continue;
      }
      if(this.mode==='remove'){
        if(this.index===base.snapshot.parts.length){this.index=0;this.mode='update';work++;continue;}
        if(!this.active){this.lookup.restart();this.key.value=base.snapshot.parts[this.index].path;this.active=true;work++;continue;}
        const step=nextIndex.lookup(next,this.key,this.lookup,maximumWork-work);work+=step.work;if(step.error)return{work,done:false,error:step.error};
        if(step.done){this.active=false;const before=this.index++;if(step.index===undefined)return{work,done:false,event:{kind:'remove',before}};}continue;
      }
      if(this.mode==='update'){
        if(this.index===next.snapshot.parts.length){this.mode='done';work++;continue;}
        if(!this.active){this.lookup.restart();this.key.value=next.snapshot.parts[this.index].path;this.active=true;work++;continue;}
        const step=baseIndex.lookup(base,this.key,this.lookup,maximumWork-work);work+=step.work;if(step.error)return{work,done:false,error:step.error};
        if(step.done){this.active=false;if(step.index===undefined){const at=this.index++;return{work,done:false,event:{kind:'part',next:at}};}this.before=step.index;this.offset=0;this.mode='bytes';}continue;
      }
      if(this.mode==='bytes'){
        const a=bytes(base,this.before),b=bytes(next,this.index);
        if(a.length!==b.length){const at=this.index++;this.mode='update';work++;return{work,done:false,event:{kind:'part',next:at,before:this.before}};}
        if(this.offset===a.length){this.index++;this.mode='update';work++;continue;}
        if(a[this.offset]!==b[this.offset]){const at=this.index++;this.mode='update';work++;return{work,done:false,event:{kind:'part',next:at,before:this.before}};}this.offset++;work++;continue;
      }
      return{work,done:false,error:'stdio.idml.change-state'};
    }
    return{work,done:this.mode==='done'};
  }
}
