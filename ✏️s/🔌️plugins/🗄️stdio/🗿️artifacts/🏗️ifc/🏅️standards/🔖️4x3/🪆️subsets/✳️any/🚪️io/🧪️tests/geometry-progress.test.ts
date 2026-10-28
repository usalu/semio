/** 🚨️ Original caller failures are refusals, never unsupported geometry diagnostics. */
import {expect,test} from 'bun:test';
import {readFile} from 'node:fs/promises';
import Ajv from 'ajv/dist/2020';
import cases from './geometry-progress-cases.json';
import schema from './geometry-progress.schema.json';
import {parseIfc4x3Step} from '../step.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {extractIfc4x3Triangles,IfcGeometryGraph} from '../geometry.ts';
import {ifc4x3ToGltf} from '../gltf.ts';
import {IfcEditError} from '../../🧬️schema/🔺️diff/🟦️.ts';
test('geometry progress refusal cases are closed language-neutral data',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('original geometry callback '+row.endpoint+' '+row.phase+' '+row.fault,async()=>{
 const source=await parseIfc4x3Step(await readFile(new URL('triangulated.ifc',import.meta.url),'utf8'),{batchSize:4096}),before=structuredClone(source),fault=row.fault==='error'?new Error('caller'):row.fault==='step-error'?new IfcStepError('geometry.caller','caller'):row.fault==='edit-error'||row.fault==='edit-cancelled'?new IfcEditError(row.fault==='edit-error'?'invalidValue':'cancelled','caller'):row.fault==='number'?17:undefined;
 const oracle=new Ajv({strict:true}).addKeyword({keyword:'callerFault',schemaType:'boolean',validate:()=>{throw fault;}}).compile({type:'number',callerFault:true});let oracleCaught=false,oracleError:unknown;
 try{oracle(1);}catch(error){oracleCaught=true;oracleError=error;}expect(oracleCaught).toBe(true);expect(oracleError).toBe(fault);
 const replacement=new AbortController();let reached=false,caught=false,replacementCalls=0;
 const control:IfcIoControl={batchSize:4096,progress:event=>{if(event.phase!==row.phase)return;reached=true;control.signal=replacement.signal;control.progress=()=>replacementCalls++;throw fault;}};
 try{await (row.endpoint==='source-capture'?IfcGeometryGraph.capture(source,control):row.endpoint==='triangles'?extractIfc4x3Triangles(source,control):ifc4x3ToGltf(source,control));}catch(error){caught=true;expect(error).toBe(oracleError);}expect(reached).toBe(true);expect(caught).toBe(true);expect(replacementCalls).toBe(0);expect(source).toEqual(before);
});
