/** 🚦️ Every public spatial and analytical exchange keeps its original operation controls. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './exchange-control-cases.json';
import schema from './exchange-control.schema.json';
import {ifcExchangeControlEndpoint as endpoint} from './exchange-control-fixtures.ts';
import type {IfcIoControl} from '../control.ts';


test('neutral exchange endpoint controls have a closed schema',()=>{
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true);
});
for(const name of cases.endpoints){
 test(`${name} preserves cancellation when caller options are replaced`,async()=>{
  const run=await endpoint(name),original=new AbortController(),replacement=new AbortController();let calls=0,replacementCalls=0;
  const control:IfcIoControl={signal:original.signal,progress:()=>{calls++;control.signal=replacement.signal;control.progress=()=>replacementCalls++;original.abort();}};
  await expect(run(control)).rejects.toHaveProperty('code',cases.code);
  expect(calls).toBe(cases.callbacksBeforeCancellation);expect(replacementCalls).toBe(0);
 });
 test(`${name} admits controls before semantic inspection`,async()=>{
  const run=await endpoint(name);
  for(const batchSize of cases.invalidBatchSizes)await expect(run({batchSize})).rejects.toHaveProperty('code','io.control');
 });
}
