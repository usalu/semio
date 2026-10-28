import fields from '../🧳️part/🧪️fixtures/🔣️.json';
import corpus from '../🧪️fixtures/🔣️.json';
import {IdmlChangeCursor,type IdmlChangeEvent} from '../🟦️.ts';
import {IdmlAdmissionCursor} from '../../🟦️.ts';
import {applyIdmlDiff,type IdmlSnapshot,type IdmlPatch} from '../../../../🟦️.ts';
/** 🔮 Independent physical admission and RFC6902 result behind an owned test interface. */
export interface IdmlChangeOracle {valid(snapshot:IdmlSnapshot):boolean;apply(before:IdmlSnapshot,after:IdmlSnapshot):IdmlSnapshot}
interface ChangeOperation {kind:string;index?:number;text?:string;path?:string;bytes?:number[];indices?:number[];value?:string}
const encoder=new TextEncoder();
const close=(cursor:IdmlAdmissionCursor)=>{cursor.beginClose();while(cursor.nextReleaseBytes())cursor.closeOne(cursor.nextReleaseBytes());if(!cursor.terminalIsEmpty())throw new Error('index close');};
const index=(snapshot:IdmlSnapshot)=>{const source={snapshot},cursor=new IdmlAdmissionCursor();let step;do{step=cursor.advance(source,{maximumWork:7,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.error)throw new Error(step.error);}while(!step.done);return{source,cursor};};
/** 🧪 Original numeric field events compile to the existing typed sparse diff. */
export function verifyIdmlChanges(oracle:IdmlChangeOracle):{cases:number;grants:number;cuts:number;fieldCases:number}{
  for(const row of corpus.cases){
    const before:IdmlSnapshot={domVersion:corpus.base.version,parts:corpus.base.parts.map(([path,text])=>({path,bytes:[...encoder.encode(text==='$mime'?'application/vnd.adobe.indesign-idml-package':text)]}))};
    const after=structuredClone(before);
    const operations:readonly ChangeOperation[]=row.operations;
    for(const operation of operations){if(operation.kind==='replace')after.parts[operation.index!].bytes=[...encoder.encode(operation.text!)];else if(operation.kind==='append')after.parts.push({path:operation.path!,bytes:operation.bytes?[...operation.bytes]:[...encoder.encode(operation.text!)]});else if(operation.kind==='remove')after.parts.splice(operation.index!,1);else if(operation.kind==='order'){const original=after.parts;after.parts=operation.indices!.map(index=>original[index]);}else if(operation.kind==='version')after.domVersion=operation.value!;}
    if(!oracle.valid(before)||!oracle.valid(after))throw new Error('independent change input '+row.id);
    const base=index(before),next=index(after),beforeJson=JSON.stringify(before),afterJson=JSON.stringify(after);
    for(const grant of corpus.lifecycle.workGrants){const cursor=new IdmlChangeCursor(),events:IdmlChangeEvent[]=[];let step;do{step=cursor.advance(base.source,next.source,base.cursor,next.cursor,grant);if(step.error||step.work>grant)throw new Error('change '+row.id+': '+step.error);if(step.event)events.push(step.event);}while(!step.done);
      if(JSON.stringify(events)!==JSON.stringify(row.expected))throw new Error('numeric changes '+row.id+' '+JSON.stringify(events));
      const patch:IdmlPatch={parts:[]};for(const event of events){if(event.kind==='version')patch.domVersion=after.domVersion;else if(event.kind==='order')patch.partOrder=after.parts.map(part=>part.path);else if(event.kind==='remove')patch.parts.push({path:before.parts[event.before].path,bytes:null});else patch.parts.push({path:after.parts[event.next].path,bytes:[...after.parts[event.next].bytes]});}
      const actual=applyIdmlDiff(before,{steps:events.length?[patch]:[]}),independent=oracle.apply(before,after);
      if(JSON.stringify(actual)!==JSON.stringify(independent)||!oracle.valid(actual))throw new Error('typed sparse result '+row.id);
      if(JSON.stringify(before)!==beforeJson||JSON.stringify(after)!==afterJson)throw new Error('original mutated');cursor.close();if(!cursor.terminalIsEmpty())throw new Error('change close');
    }
    for(const cut of corpus.lifecycle.cancellationCuts){const cursor=new IdmlChangeCursor();for(let i=0;i<cut;i++){const step=cursor.advance(base.source,next.source,base.cursor,next.cursor,1);if(step.error)throw new Error(step.error);if(step.done)break;}cursor.close();if(!cursor.terminalIsEmpty()||JSON.stringify(before)!==beforeJson||JSON.stringify(after)!==afterJson)throw new Error('change cancellation');}
    close(base.cursor);close(next.cursor);
  }
  for(const row of fields.cases){const before:IdmlSnapshot={domVersion:corpus.base.version,parts:corpus.base.parts.map(([path,text])=>({path,bytes:[...encoder.encode(text==='$mime'?'application/vnd.adobe.indesign-idml-package':text)]}))};const bytes='text'in row?[...encoder.encode(row.text!)]:row.bytes===null?null:[...row.bytes!];const after=structuredClone(before);const found=after.parts.findIndex(part=>part.path===row.path);if(bytes===null){if(found>=0)after.parts.splice(found,1);}else if(found>=0)after.parts[found].bytes=bytes;else after.parts.push({path:row.path,bytes});const actual=applyIdmlDiff(before,{steps:[{parts:[{path:row.path,bytes}]}]});if(JSON.stringify(actual)!==JSON.stringify(oracle.apply(before,after))||!oracle.valid(actual))throw Error('original field projection '+row.id);}
  return{fieldCases:fields.cases.length,cases:corpus.cases.length,grants:corpus.lifecycle.workGrants.length,cuts:corpus.lifecycle.cancellationCuts.length};
}
