import corpus from '../🧪️fixtures/🔣️.json';
import retainedCorpus from '../🧳️retained/🧪️fixtures/🔣️.json';
import versionCorpus from '../🔢️version/🧳️retained/🧪️fixtures/🔣️.json';
import {IdmlChangeCursor} from '../🔺️change/🟦️.ts';
import { IdmlNameLookup, IdmlVersionDecoder, admitIdmlParts, IdmlAdmissionCursor, type IdmlAdmissionSource } from '../🟦️.ts';
import type { IdmlSnapshot } from '../../../🧬️schema/📸️snapshot/🟦️.ts';
/** 🔮 The independent ZIP/XML engine sees only owned package scalars and bytes. */
export interface IdmlAdmissionOracle { valid(snapshot:IdmlSnapshot):boolean;versionValue(source:readonly number[]):string;names(snapshot:IdmlSnapshot):readonly string[] }
import { mutationIdmlDiff,applyIdmlDiff,inverseIdmlMutation } from '../../../🟦️.ts';
const encoder=new TextEncoder();
const close=(cursor:IdmlAdmissionCursor)=>{cursor.beginClose();while(cursor.nextReleaseBytes()){const demand=cursor.nextReleaseBytes();if(cursor.closeOne(demand-1).releasedBytes)throw new Error('underfunded release');if(cursor.closeOne(demand).releasedBytes!==demand)throw new Error('exact release');}if(!cursor.terminalIsEmpty())throw new Error('admission close');};
/** 🧪 Syntax, indexed names, metadata grants, overlays and retirement laws. */
export function verifyIdmlAdmission(oracle:IdmlAdmissionOracle):{cases:number;manyParts:number;cuts:number;replacementCases:number;versionFields:number}{
  for(const row of corpus.cases){
    const snapshot:IdmlSnapshot={domVersion:row.version,parts:row.parts.map(([path,text])=>({path,bytes:[...encoder.encode(text==='$mime'?corpus.mime:text)]}))};
    if(oracle.valid(snapshot)!==row.valid)throw new Error('independent admission '+row.id);
    let cold:string|undefined;try{cold=admitIdmlParts(snapshot.parts);}catch{}if((cold===row.version)!==row.valid)throw new Error('production cold admission '+row.id);
    if(row.valid){const mutation={mutation:'replacePart' as const,path:'designmap.xml',bytes:[...encoder.encode('<Document DOMVersion="9.0"/>')]};const diff=mutationIdmlDiff(snapshot,mutation);const after=applyIdmlDiff(snapshot,diff);if(after.domVersion!=='9.0'||!oracle.valid(after))throw new Error('zero-touch typed mutation '+row.id);const inverse=inverseIdmlMutation(snapshot,mutation);if(inverse.length!==1||JSON.stringify(applyIdmlDiff(after,mutationIdmlDiff(after,inverse[0])))!==JSON.stringify(snapshot))throw new Error('zero-touch inverse '+row.id);}

    for(const grant of corpus.lifecycle.workGrants){const cursor=new IdmlAdmissionCursor(),source={snapshot};let step,turns=0;
      do{step=cursor.advance(source,{maximumWork:grant,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.work>grant || step.copiedBytes>2048 || step.retainedCapacityBytes>1024)throw new Error('grant');if(++turns>1000000)throw new Error('stalled '+row.id);}while(!step.done&&!step.error);
      if((!step.error)!==row.valid)throw new Error('admission '+row.id+': '+step.error);
      if(!row.valid && step.error!==row.error)throw new Error('refusal '+row.id+': '+step.error);
      if(row.valid && step.version?.decodedLength!==row.version.length)throw new Error('version view');
      if(row.valid){const view=step.version!,decoder=new IdmlVersionDecoder(view),data=snapshot.parts[view.partIndex].bytes;let text='';for(;;){const step=decoder.advance(data,grant);if(step.work>grant||step.error)throw new Error('version decoder');if(step.byte!==undefined)text+=String.fromCharCode(step.byte);if(step.done)break;}if(text!==row.version)throw new Error('decoded version');
        const names=oracle.names(snapshot);for(const keyValue of corpus.lifecycle.lookupKeys){const key={value:keyValue},lookup=new IdmlNameLookup();if(cursor.lookup(source,key,lookup,0).work)throw new Error('zero lookup grant');let step;do{step=cursor.lookup(source,key,lookup,grant);if(step.error||step.work>grant)throw new Error('lookup grant');}while(!step.done);const expected=names.indexOf(keyValue);if(step.index!==(expected<0?undefined:expected))throw new Error('lookup '+keyValue);}
      }close(cursor);
    }
  }
  const snapshot:IdmlSnapshot={domVersion:'8.0',parts:[{path:'mimetype',bytes:[...encoder.encode(corpus.mime)]},{path:'designmap.xml',bytes:[...encoder.encode('<Document DOMVersion="8.0"/>')]}]};
  for(let i=0;i<corpus.lifecycle.additionalParts;i++)snapshot.parts.push({path:'Stories/'+i+'.xml',bytes:[...encoder.encode('<Story/>')]});
  if(!oracle.valid(snapshot))throw new Error('many independent');
  const source={snapshot},cursor=new IdmlAdmissionCursor();let step;
  do{step=cursor.advance(source,{maximumWork:7,maximumCopyBytes:2048,maximumCapacityBytes:1023,maximumReleaseBytes:0});if(step.error)throw new Error(step.error);}while(!cursor.nextCapacityBytes());
  if(cursor.advance(source,{maximumWork:7,maximumCopyBytes:2047,maximumCapacityBytes:1024,maximumReleaseBytes:0}).retainedCapacityBytes)throw new Error('underfunded copy');
  do{step=cursor.advance(source,{maximumWork:7,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.error)throw new Error(step.error); }while(!step.done);
  const names=oracle.names(snapshot);for(const value of corpus.lifecycle.lookupKeys){const key={value},lookup=new IdmlNameLookup();let step;do{step=cursor.lookup(source,key,lookup,7);if(step.error||step.work>7)throw new Error('many lookup');}while(!step.done);const expected=names.indexOf(value);if(step.index!==(expected<0?undefined:expected))throw new Error('many lookup result');}close(cursor);
  for(const cut of corpus.lifecycle.cancellationCuts){const cursor=new IdmlAdmissionCursor();for(let i=0;i<cut;i++)cursor.advance(source,{maximumWork:7,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});close(cursor);}
  const overlay:IdmlAdmissionSource={snapshot,replacement:{index:1,bytes:[...encoder.encode('<Document DOMVersion="'+corpus.lifecycle.changedVersionOverlay+'"/>')]},compareVersion:false};
  const over=new IdmlAdmissionCursor();do{step=over.advance(overlay,{maximumWork:64,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.error)throw new Error(step.error);}while(!step.done);if(step.version?.decodedLength!==3)throw new Error('overlay version');close(over);
  for(const row of retainedCorpus.replacements){
    const snapshot:IdmlSnapshot={domVersion:'8.0',parts:[{path:'mimetype',bytes:[...encoder.encode(corpus.mime)]},{path:'designmap.xml',bytes:[...encoder.encode('<Document DOMVersion="8.0"/>')]}]},original=JSON.stringify(snapshot),replacement=[...encoder.encode(row.bytes)],saved=JSON.stringify(replacement),source:IdmlAdmissionSource={snapshot,replacement:{index:row.index,bytes:replacement},compareVersion:false},target=new IdmlAdmissionCursor();let step;
    do{step=target.advance(source,{maximumWork:1,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(step.work>1)throw new Error('replacement grant');}while(!step.done&&!step.error);
    if((!step.error)!==row.valid||JSON.stringify(snapshot)!==original||JSON.stringify(replacement)!==saved)throw new Error('borrowed replacement '+row.id);
    if(row.valid){const parts=snapshot.parts.map((part,index)=>({path:part.path,bytes:index===row.index?replacement:part.bytes})),version=admitIdmlParts(parts);if(!oracle.valid({domVersion:version,parts}))throw new Error('independent replacement');const baseSource={snapshot},base=new IdmlAdmissionCursor();let admitted;do{admitted=base.advance(baseSource,{maximumWork:7,maximumCopyBytes:2048,maximumCapacityBytes:1024,maximumReleaseBytes:0});if(admitted.error)throw new Error(admitted.error);}while(!admitted.done);const cursor=new IdmlChangeCursor(),events=[];for(;;){const step=cursor.advance(baseSource,source,base,target,1);if(step.error||step.work>1)throw new Error('replacement planning');if(step.event)events.push(step.event);if(step.done)break;}if(JSON.stringify(events)!==JSON.stringify(row.events))throw new Error('replacement original coordinates');cursor.close();close(base);}close(target);
  }
  for(const row of versionCorpus.cases){const bytes=[...encoder.encode(row.source)];if(oracle.versionValue(bytes)!==row.expected)throw new Error('independent version field');for(const grant of corpus.lifecycle.workGrants){const decoder=new IdmlVersionDecoder({partIndex:0,value:{start:row.start,end:row.end},decodedLength:row.decodedLength});let text='';for(;;){const step=decoder.advance(bytes,grant);if(step.error||step.work>grant)throw new Error('version field source');if(step.byte!==undefined)text+=String.fromCharCode(step.byte);if(step.done)break;}if(text!==row.expected)throw new Error('version field witness');}}
  return {cases:corpus.cases.length,manyParts:snapshot.parts.length,cuts:corpus.lifecycle.cancellationCuts.length,replacementCases:retainedCorpus.replacements.length,versionFields:versionCorpus.cases.length};
}
