/** 🪃️ IFC work yields through the existing shared host scheduler under unchanged original controls. */
import {expect,test,spyOn} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {compare} from 'fast-json-patch';
import {hostContinuations} from '@semio-tech/framework-async/continuation';
import cases from './continuation-cases.json';
import schema from './continuation.schema.json';
import {ifcCheckpoint,runIfcIoSteps,type IfcIoSteps} from '../control.ts';
import {runIfcEditAsync,type IfcEditSteps} from '../../🧬️schema/🔺️diff/🟦️.ts';
const ioSteps=function*(count:number):IfcIoSteps<number>{for(let at=0;at<count;at++)yield;return count;};
const editSteps=function*(count:number):IfcEditSteps<number>{for(let at=0;at<count;at++)yield{phase:'copyOwned',completed:at+1,total:count};return count;};
test('continuation laws have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const route of cases.routes)test(route+' uses the shared continuation host without local zero timers',async()=>{
 const timer=spyOn(globalThis,'setTimeout'),events:number[]=[];
 try{
  if(route==='io'){await ifcCheckpoint({progress:event=>events.push(event.completed)},'fixture',1,1);expect(compare([1],events)).toEqual([]);}
  else{expect(await runIfcEditAsync(editSteps(cases.steps),{yieldEvery:1,onProgress:event=>events.push(event.completed)})).toBe(cases.steps);expect(compare(Array.from({length:cases.steps},(_,at)=>at+1),events)).toEqual([]);}
  expect(timer.mock.calls.filter(args=>args[1]===0||args[1]===undefined)).toHaveLength(0);
 }finally{timer.mockRestore();}
});
for(const route of cases.routes)test(route+' yields to a real host cancellation task',async()=>{
 const controller=new AbortController();let closed=false,turns=0;const original=route==='io'?ioSteps(4096):editSteps(4096),steps=(function*(){try{return yield* original;}finally{closed=true;}})();
 const cancel=hostContinuations.schedule(()=>controller.abort(),0);
 try{
  const work=route==='io'?runIfcIoSteps(steps as IfcIoSteps<number>,{signal:controller.signal,batchSize:1,progress:()=>turns++},'fixture'):runIfcEditAsync(steps as IfcEditSteps<number>,{signal:controller.signal,yieldEvery:1,onProgress:()=>turns++});
  await expect(work).rejects.toMatchObject({code:route==='io'?'io.cancelled':'cancelled'});expect(turns).toBeGreaterThan(0);expect(turns).toBeLessThan(4096);
  expect(closed).toBe(route==='edit');
 }finally{cancel();}
});
for(const route of cases.routes)test(route+' does not starve an original timer cancellation',async()=>{
 const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),0);let turns=0;
 try{await expect(route==='io'?runIfcIoSteps(ioSteps(100000),{signal:controller.signal,batchSize:1,progress:()=>turns++},'fixture'):runIfcEditAsync(editSteps(100000),{signal:controller.signal,yieldEvery:1,onProgress:()=>turns++})).rejects.toMatchObject({code:route==='io'?'io.cancelled':'cancelled'});expect(turns).toBeGreaterThan(0);expect(turns).toBeLessThan(100000);}finally{clearTimeout(timer);}
});
for(const route of cases.routes)for(const failure of [new Error('original'),undefined,23])test(route+' preserves exact caller refusal '+String(failure),async()=>{
 let actual:unknown=Symbol('missing'),closed=false;const original=route==='io'?ioSteps(4):editSteps(4),steps=(function*(){try{return yield* original;}finally{closed=true;}})();
 try{if(route==='io')await runIfcIoSteps(steps as IfcIoSteps<number>,{batchSize:1,progress:event=>{if(event.completed>0)throw failure;}},'fixture');else await runIfcEditAsync(steps as IfcEditSteps<number>,{yieldEvery:1,onProgress:()=>{throw failure;}});}catch(error){actual=error;}
 expect(actual).toBe(failure);expect(closed).toBe(true);
});
