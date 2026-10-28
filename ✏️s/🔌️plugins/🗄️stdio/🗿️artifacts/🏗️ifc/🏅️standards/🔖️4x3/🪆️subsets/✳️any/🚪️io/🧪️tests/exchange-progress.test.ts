/** 🚦️ Every spatial and analytical endpoint preserves its original late caller refusal. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './exchange-progress-cases.json';
import schema from './exchange-progress.schema.json';
import {ifcExchangeControlEndpoint} from './exchange-control-fixtures.ts';
import {femConnectivityFixture} from './fem-isolated-fixtures.ts';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from '../fem.ts';
import {IfcStepError,type IfcIoControl} from '../control.ts';
import {IfcEditError} from '../../🧬️schema/🔺️diff/🟦️.ts';

test('exchange progress caller refusal vectors are closed',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test(row.route+' preserves '+row.fault+' at '+row.phase,async()=>{
 const model=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:[],loads:[]});
 const source=row.route==='ifc-to-fem'?(await fem3dToIfc4x3(model,{batchSize:4096})).artifact!:undefined;
 const run=row.route==='fem-to-ifc'?(control:IfcIoControl)=>fem3dToIfc4x3(model,control):row.route==='ifc-to-fem'?(control:IfcIoControl)=>ifc4x3ToFem3d(source!,control):await ifcExchangeControlEndpoint(row.route);
 const before=structuredClone(source??model);
 const fault=row.fault==='error'?new Error('caller'):row.fault==='step-error'?new IfcStepError('exchange.number','caller'):row.fault==='step-cancelled'?new IfcStepError('io.cancelled','caller'):row.fault==='edit-error'?new IfcEditError('invalidValue','caller'):row.fault==='number'?17:undefined;
 const ajv=new Ajv({strict:true});ajv.addKeyword({keyword:'callerFault',validate:()=>{throw fault;}});
 let oracleCaught=false,oracleError:unknown;try{ajv.compile({type:'number',callerFault:true})(1);}catch(error){oracleCaught=true;oracleError=error;}expect(oracleCaught).toBe(true);expect(oracleError).toBe(fault);
 let calls=0,replacementCalls=0,caught=false;const replacement=new AbortController();
 const control:IfcIoControl={batchSize:4096,progress(event){if(event.phase!==row.phase||++calls!==row.occurrence)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;throw fault;}};
 try{await run(control);}catch(error){caught=true;expect(error).toBe(oracleError);}
 expect(calls).toBe(row.occurrence);expect(caught).toBe(true);expect(replacementCalls).toBe(0);expect(source??model).toEqual(before);
});

