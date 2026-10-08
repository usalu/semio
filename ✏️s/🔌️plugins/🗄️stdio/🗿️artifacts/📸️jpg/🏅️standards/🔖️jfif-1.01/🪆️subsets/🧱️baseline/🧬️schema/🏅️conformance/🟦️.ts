/** 🛡️ Baseline conformance rules over immutable owned native facts. */
import type {JpgBaselineFacts} from './🧮️facts/🟦️.ts';
export interface JpgBaselineFactControl {signal?:AbortSignal;onProgress?:(completed:number,total:number)=>void|Promise<void>}
export async function checkJpgBaselineFacts(facts:JpgBaselineFacts,control:JpgBaselineFactControl={}):Promise<string[]>{
 const checkpoint=async(position:number)=>{await control.onProgress?.(position,facts.components.length);if(control.signal?.aborted)throw new Error('JPEG baseline conformance canceled');};
 await checkpoint(0);const codes:string[]=[];const add=(code:string)=>codes.push('stdio.jpg.baseline.'+code);
 if(!facts.hasFrame){add('no-frame');return codes;}
 if(!facts.baselineSequential)add('sof-marker');if(facts.samplePrecision!==8)add('precision');if(facts.arithmeticConditioning)add('arithmetic-conditioning-present');
 if(facts.dcTableCount>2||facts.acTableCount>2)add('huffman-table-count');if(facts.components.length>4)add('component-sampling');
 for(let position=0;position<facts.components.length;position++){await checkpoint(position);const component=facts.components[position]!;if(component.horizontal<1||component.horizontal>4||component.vertical<1||component.vertical>4)add('component-sampling');}
 await checkpoint(facts.components.length);return codes;
}
