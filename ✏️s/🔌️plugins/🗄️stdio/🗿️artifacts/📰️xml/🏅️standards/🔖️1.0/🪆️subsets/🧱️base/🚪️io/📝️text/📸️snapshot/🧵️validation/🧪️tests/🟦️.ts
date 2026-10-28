import corpus from '../🧪️fixtures/🔣️.json';
import { XmlValidationCursor } from '../🟦️.ts';
/** 🔮 External syntax engines enter through this owned byte-only test port. */
export interface XmlValidationOracle { valid(source: Uint8Array): boolean }
/** 🧪 Portable syntax, grant, source-span and cancellation laws. */
export function verifyXmlValidation(oracle: XmlValidationOracle): {cases:number;grants:number;cuts:number;depth:number} {
  let cases=0;
  const encoder=new TextEncoder(),decoder=new TextDecoder();
  for(const row of corpus.cases){
    const source='bytes' in row && row.bytes ? new Uint8Array(row.bytes):encoder.encode(row.xml!);
    if(oracle.valid(source)!==row.valid)throw new Error('independent XML witness '+row.id);
    for(const grant of corpus.lifecycle.workGrants){
      const cursor=new XmlValidationCursor();let last,turns=0;
      do {last=cursor.advance(source,grant);if(last.work>grant)throw new Error('grant '+row.id);if(++turns>source.length*1100+2000)throw new Error('stalled '+row.id);}
      while(!last.done && !last.error);
      if(!last.error!==row.valid)throw new Error('validity '+row.id+': '+last.error);
      if(!row.valid && last.error!==row.error)throw new Error('refusal '+row.id+': '+last.error);
      const root=cursor.rootName();if(row.valid && (!root || decoder.decode(source.subarray(root.start,root.end))!==row.root))throw new Error('root '+row.id);
      cursor.close();if(!cursor.terminalIsEmpty())throw new Error('closure '+row.id);
    }
    cases++;
  }
  const long=encoder.encode('<root>'+('text 😀 &amp; '.repeat(corpus.lifecycle.longTextRepeat))+'</root>');
  for(const grant of corpus.lifecycle.workGrants){const cursor=new XmlValidationCursor();let step;do{step=cursor.advance(long,grant);if(step.work>grant || step.error)throw new Error('long text');}while(!step.done);cursor.close();}
  for(const cut of corpus.lifecycle.cancellationCuts){const cursor=new XmlValidationCursor();for(let i=0;i<cut;i++)cursor.advance(long,7);cursor.close();if(!cursor.terminalIsEmpty() || cursor.advance(long,7).error!=='xml.validation.closed')throw new Error('cancel');}
  for(const depth of [corpus.lifecycle.deepAccepted,corpus.lifecycle.deepRefused]){const source=encoder.encode('<a>'.repeat(depth)+'</a>'.repeat(depth));const cursor=new XmlValidationCursor();let step;do{step=cursor.advance(source,7);}while(!step.done && !step.error);if((!step.error)!==(depth===256))throw new Error('depth');cursor.close();}
  for(const count of [corpus.lifecycle.attributeAccepted,corpus.lifecycle.attributeRefused]){const source=encoder.encode('<a '+Array.from({length:count},(_,i)=>'a'+i+'=""').join(' ')+'/>');const cursor=new XmlValidationCursor();let step;do{step=cursor.advance(source,4096);}while(!step.done && !step.error);if((!step.error)!==(count===1024))throw new Error('attribute limit');cursor.close();}
  const name='a'.repeat(corpus.lifecycle.longNameBytes);const named=encoder.encode('<'+name+' '+name+'="" '+name+'=""/>');const namedCursor=new XmlValidationCursor();let namedStep;do{namedStep=namedCursor.advance(named,7);if(namedStep.work>7)throw new Error('name grant');}while(!namedStep.done && !namedStep.error);if(namedStep.error!=='xml.validation.duplicate-attribute')throw new Error('long name comparison');namedCursor.close();
  let random=corpus.lifecycle.differential.seed;const next=()=>{random=(Math.imul(random,1664525)+1013904223)>>>0;return random;};
  const valid=corpus.cases.filter(row=>row.valid && row.xml).map(row=>row.xml!);
  for(let i=0;i<corpus.lifecycle.differential.mutations;i++){const original=valid[next()%valid.length];const at=next()%(original.length+1);const text=original.slice(0,at)+corpus.lifecycle.differential.alphabet[next()%corpus.lifecycle.differential.alphabet.length]+original.slice(at+(next()%2));const source=encoder.encode(text),cursor=new XmlValidationCursor();let step;do{step=cursor.advance(source,7);}while(!step.done && !step.error);if((!step.error)!==oracle.valid(source))throw new Error('differential '+i+' '+JSON.stringify(text)+' '+step.error);cursor.close();}
  const source=encoder.encode('<a/>'),cursor=new XmlValidationCursor();if(cursor.advance(source,0).work!==0)throw new Error('zero grant');if(cursor.advance(new Uint8Array(source),1).error!=='xml.validation.source')throw new Error('source binding');cursor.close();
  return {cases,grants:corpus.lifecycle.workGrants.length,cuts:corpus.lifecycle.cancellationCuts.length,depth:256};
}
