import {expect,test} from "bun:test";
import {rowOwnerNamesV1} from "../../🧮️fact-rows/🟨️.mjs";
import {isDerivedOwnerV1} from "../../✂️cutoff/🟨️.mjs";

/** 🧭️ The owners named behind a stale row come from one pass over the classes: the result equals the former search of every class per row on random indexes (the oracle is that search, copied), and a stale set of tens of thousands of rows over tens of thousands of classes costs one pass. */
function random(seed:number):()=>number{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}
const former=(index:any,classes:number[],id:number)=>{
 const names:string[]=[];
 for(const at of classes){const cls=index.classes[at];if(cls.r.includes(id))for(const owner of cls.o){const name=index.owners[owner];if(!isDerivedOwnerV1(name)&&!names.includes(name))names.push(name);if(names.length>=3)break;}if(names.length>=3)break;}
 return names;
};
const build=(seed:number,rows:number,owners:number,classCount:number)=>{
 const next=random(seed),names=Array.from({length:owners},(_,at)=>next()<0.2?"@derived:"+at:"owner"+at);
 const classes=Array.from({length:classCount},()=>({r:Array.from(new Set(Array.from({length:1+Math.floor(next()*6)},()=>Math.floor(next()*rows)))).sort((a,b)=>a-b),o:Array.from(new Set(Array.from({length:1+Math.floor(next()*5)},()=>Math.floor(next()*owners)))).sort((a,b)=>a-b)}));
 return{owners:names,classes};
};

test("one pass equals the search of every class per row over 40 random indexes",()=>{
 for(let seed=1;seed<=40;seed++){
  const index=build(seed,60,12,25),classes=[...index.classes.keys()].filter(at=>at%7!==3),wanted=Array.from({length:60},(_,id)=>id).filter(id=>id%2===seed%2);
  const found=rowOwnerNamesV1(index,classes,wanted,isDerivedOwnerV1);
  for(const id of wanted)expect([seed,id,found.get(id)??[]]).toEqual([seed,id,former(index,classes,id)]);
 }
});

test("a stale set of 40000 rows over 20000 classes is one pass",()=>{
 const index=build(7,200000,3000,20000),classes=[...index.classes.keys()],wanted=Array.from({length:40000},(_,id)=>id*5),started=performance.now();
 const found=rowOwnerNamesV1(index,classes,wanted,isDerivedOwnerV1);
 expect(performance.now()-started).toBeLessThan(2000);
 expect(found.size).toBeGreaterThan(1000);
 for(const id of wanted.slice(0,25))expect(found.get(id)??[]).toEqual(former(index,classes,id));
});
