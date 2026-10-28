/** 🚦️ Predicate admission keeps original caller failures distinct from incomplete-source reports. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './progress-cases.json';
import schema from './progress.schema.json';
import {validateIfcAlignmentTerminal} from './📐️alignment/🟦️.ts';
import {validateIfcBodyTessellation} from './🔺️tessellation/🟦️.ts';
import {parseIfc4x3Step} from '../../../🚪️io/step.ts';
import {IfcStepError,type IfcIoControl} from '../../../🚪️io/control.ts';
import {IfcEditError} from '../../🔺️diff/🟦️.ts';
test('predicate caller failure vectors have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('original predicate '+row.concept+' '+row.stage+' '+row.fault,async()=>{
 const source=await parseIfc4x3Step(await readFile(new URL('../../../🚪️io/🧪️tests/triangulated.ifc',import.meta.url),'utf8'),{batchSize:4096}),before=structuredClone(source),failure=row.fault==='edit-error'?new IfcEditError('invalidValue','caller'):row.fault==='edit-cancelled'?new IfcEditError('cancelled','caller'):row.fault==='step-error'?new IfcStepError('predicate.caller','caller'):row.fault==='error'?new Error('caller'):row.fault==='number'?17:undefined;
 const oracle=new Ajv({strict:true}).addKeyword({keyword:'callerFault',validate:()=>{throw failure;}}).compile({type:'number',callerFault:true});let oracleCaught=false,oracleError:unknown;try{oracle(1);}catch(error){oracleCaught=true;oracleError=error;}expect(oracleCaught).toBe(true);expect(oracleError).toBe(failure);
 const prefix=row.concept==='alignment'?'alignment':'mvd',phase=prefix+'-'+(row.stage==='request'?'request':'capture')+'-copyOwned';let reached=false,replacementCalls=0,caught=false;const replacement=new AbortController();
 const control:IfcIoControl={batchSize:4096,progress(event){if(event.phase!==phase)return;reached=true;control.signal=replacement.signal;control.progress=()=>replacementCalls++;throw failure;}};
 try{if(row.concept==='alignment')await validateIfcAlignmentTerminal(source,{targets:[{layout:'1',kind:'horizontal',applicable:true}]},control);else await validateIfcBodyTessellation(source,{targets:[{representation:'1020',applicable:true}]},control);}catch(error){caught=true;expect(error).toBe(failure);}
 expect(reached).toBe(true);expect(caught).toBe(true);expect(replacementCalls).toBe(0);expect(source).toEqual(before);
});
