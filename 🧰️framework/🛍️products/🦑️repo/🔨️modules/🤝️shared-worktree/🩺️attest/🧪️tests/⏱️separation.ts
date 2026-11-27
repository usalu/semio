import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {separateOutliersV1,type ApartPairV1} from "../🟦️.ts";

type CaseV1=Readonly<{apart:readonly [string,string][];alike:readonly [string,string][];pointed:number}>;
const named=(one:string):string=>`g1-${one}`;
const pairs=(list:readonly [string,string][]):[string,string][]=>list.map(([one,two])=>[named(one),named(two)]);
const wire=(list:readonly [string,string][]):ApartPairV1[]=>list.map(([one,two])=>({one,two,left:`${one}-value`,right:`${two}-value`}));
const mine=(item:CaseV1)=>{
 const found=separateOutliersV1(wire(item.apart),item.alike.map(([one,two])=>({one,two})),item.pointed);
 return {victims:found.victims.map(entry=>[entry.origin,[entry.first.one,entry.first.two]]),ambiguous:[...found.ambiguous]};
};

const clique=["m01","m02","m03","m06","m07","server"];
const hang:CaseV1={
 pointed:10,
 alike:pairs(clique.flatMap((one,index)=>clique.slice(index+1).map((two):[string,string]=>[one,two]))),
 apart:pairs([...clique.flatMap((origin):[string,string][]=>[[origin,"m04"],[origin,"m10"]]),["m04","m10"]]),
};

test("two outliers that differ from a majority and from each other are both named and the pass ends (it repeated forever on the real fleet and held the event loop for good)",()=>{
 const started=performance.now(),found=mine(hang);
 expect(performance.now()-started).toBeLessThan(50);
 expect(found.victims.map(entry=>entry[0]).sort()).toEqual(["g1-m04","g1-m10"]);
 expect(found.ambiguous.sort()).toEqual(["g1-m04","g1-m10"]);
});

test("a 2 against 2 split stays ambiguous and names nobody, a single outlier is named without ambiguity",()=>{
 const split=mine({pointed:4,alike:[["a","b"],["c","d"]],apart:[["a","c"],["a","d"],["b","c"],["b","d"]]});
 expect(split.ambiguous.sort()).toEqual(["a","b","c","d"]);
 const single=mine({pointed:4,alike:[["a","b"],["a","c"],["b","c"]],apart:[["a","x"],["b","x"],["c","x"]]});
 expect(single.victims.map(entry=>entry[0])).toEqual(["x"]);
 expect(single.ambiguous).toEqual([]);
});

test("every random disagreement graph ends within a bounded number of removed pairs and agrees with an independent python implementation",()=>{
 let seed=12345;
 const next=():number=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
 const cases:CaseV1[]=[hang];
 for(let index=0;index<1500;index++){
  const size=2+Math.floor(next()*10),origins=Array.from({length:size},(_,at)=>`o${String(at).padStart(2,"0")}`),apart:[string,string][]=[],alike:[string,string][]=[];
  for(let one=0;one<size;one++)for(let two=one+1;two<size;two++){const roll=next();if(roll<0.45)apart.push([origins[one]!,origins[two]!]);else if(roll<0.9)alike.push([origins[one]!,origins[two]!]);}
  cases.push({pointed:size,alike,apart});
 }
 const started=performance.now(),answers=cases.map(mine),spent=performance.now()-started;
 expect(spent).toBeLessThan(5000);
 cases.forEach((item,at)=>{expect(answers[at]!.victims.length).toBeLessThanOrEqual(item.apart.length);});
 const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️separation.py")],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<28});
 expect(oracle.status,oracle.stderr).toBe(0);
 const expected=JSON.parse(oracle.stdout) as {victims:unknown[];ambiguous:string[]}[];
 cases.forEach((_,at)=>{expect<unknown>(answers[at],`case ${at}`).toEqual(expected[at]!);});
});
