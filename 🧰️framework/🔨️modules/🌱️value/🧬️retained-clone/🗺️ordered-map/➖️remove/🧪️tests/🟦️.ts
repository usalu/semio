import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🔣️.json" with {type:"json"};
import {OrderedMapRemoval} from "../🟦️.ts";
const valid=new Ajv({strict:true}).compile(schema);
const grant={comparisonItems:1,comparisonBytes:8,movedItems:32,movedBytes:4096};
function pages(count:number):Array<Array<[number,null]>>{const entries=Array.from({length:count},(_,key)=>[key,null] as [number,null]);return Array.from({length:Math.ceil(count/16)},(_,page)=>entries.slice(page*16,(page+1)*16))}
test("owned paged removal matches independent Map and Ajv for complete and interrupted candidates",()=>{
 for(const row of fixture.cases)for(const interruption of [...fixture.interruptionTurns,null]){
  expect(valid({key:row.key})).toBe(true);const original=pages(row.count);const owner=new OrderedMapRemoval(original,row.key,8);let output:Array<Array<[number,null]>>|null=null;expect(owner.outputReady()).toBe(false);
  for(let turn=0;turn<10000;turn++){if(turn===interruption)break;const step=owner.advance(grant);expect(step.progress.comparisonItems).toBeLessThanOrEqual(grant.comparisonItems);expect(step.progress.comparisonBytes).toBeLessThanOrEqual(grant.comparisonBytes);expect(step.progress.movedItems).toBeLessThanOrEqual(grant.movedItems);expect(step.progress.movedBytes).toBeLessThanOrEqual(grant.movedBytes);if(step.complete){expect(step.removed).toBe(row.expectedRemoved);expect(owner.outputReady()).toBe(true);output=owner.take();expect(owner.outputReady()).toBe(false);break}}
  if(interruption===null){const oracle=new Map(Array.from({length:row.count},(_,key)=>[key,null]));oracle.delete(row.key);expect(output).toBe(original);expect(output!.flat().map(([key])=>key)).toEqual([...oracle.keys()])}
  owner.beginClose();const custody=owner.takeRetainedCustody();if(output===null)expect(custody.pages).toBe(original);const actual=[...(output??custody.pages??[]).flat(),...(custody.removed?[custody.removed]:[])];expect(actual.map(([key])=>key).sort((a,b)=>a-b)).toEqual(Array.from({length:row.count},(_,key)=>key));
 }
});
test("denied comparison and movement preserve page identity and exact entries",()=>{
 const original=pages(33);const owner=new OrderedMapRemoval(original,16,8);const before=JSON.stringify(original);expect(owner.advance({...grant,comparisonItems:0}).progress).toEqual({comparisonItems:0,comparisonBytes:0,movedItems:0,movedBytes:0});expect(JSON.stringify(original)).toBe(before);
 for(let turn=0;turn<100;turn++){const step=owner.advance({...grant,movedItems:0,movedBytes:0});if(step.progress.comparisonItems===0)break}expect(JSON.stringify(original)).toBe(before);owner.beginClose();expect(owner.takeRetainedCustody().pages).toBe(original);
});
test("Unicode scalar ordering agrees with independent UTF-8 byte comparison",()=>{
 const keys=[...fixture.stringKeys].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
 for(const key of fixture.stringTargets){const entries=keys.map(value=>[value,value.repeat(3)] as [string,string]);const original=[entries];const owner=new OrderedMapRemoval(original,key,48);let output:Array<Array<[string,string]>>|null=null;
  for(let turn=0;turn<10000;turn++){const step=owner.advance({...grant,comparisonBytes:4});if(step.complete){expect(owner.outputReady()).toBe(true);output=owner.take();expect(owner.outputReady()).toBe(false);break}}
  const oracle=new Map(keys.map(value=>[value,value.repeat(3)]));oracle.delete(key);expect(output?.flat()).toEqual([...oracle.entries()]);owner.beginClose();const custody=owner.takeRetainedCustody();expect(custody.removed?.[0]??null).toBe(keys.includes(key)?key:null);
 }
});
