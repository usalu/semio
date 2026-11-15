/** 🟰️ Contract: `sameFactV1` is a sound shortcut for key equality. It answers true only for facts whose canonical texts are equal (so an unchanged row of a patch needs no canonical text), and false wherever it cannot tell (NaN, `toJSON` objects), which the caller settles with the keys. Checked against the canonical key on random facts, key order and absent members included, and against an independent Python model of structural equality. */
import{expect,test}from"bun:test";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{rowKeyV1,sameFactV1}from"../../🧮️fact-rows/🟨️.mjs";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
let seed=7;const next=(n:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%n;};
const leaf=():unknown=>[null,true,false,0,1,-3,1.5,"a","b","🧰️x",""][next(11)];
const make=(depth:number):unknown=>{
 if(depth===0||next(3)===0)return leaf();
 if(next(2)===0)return Array.from({length:next(4)},()=>make(depth-1));
 const out:Record<string,unknown>={};for(let at=0;at<next(4);at++)out[["k","l","m","n"][next(4)]!]=make(depth-1);return out;
};
const shuffle=(value:unknown):unknown=>Array.isArray(value)?value.map(shuffle):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value as Record<string,unknown>).reverse().map(([key,item])=>[key,shuffle(item)])):value;
const key=(fact:unknown)=>rowKeyV1({kind:"analysis",path:"p",fact});

test("equal facts are the same fact whatever the key order, different facts are not, and the answer implies key equality",()=>{
 let same=0,different=0;
 for(let round=0;round<400;round++){
  const a=make(4),b=next(2)?shuffle(a):make(4);
  const verdict=sameFactV1(a,b);
  if(verdict){same++;expect(key(a)).toBe(key(b));}else different++;
  if(key(a)===key(b))expect(verdict).toBe(true);
 }
 expect(same).toBeGreaterThan(50);expect(different).toBeGreaterThan(50);
});

test("what cannot be told is left to the keys: NaN, objects with toJSON, members that are undefined on one side only",()=>{
 expect(sameFactV1(Number.NaN,Number.NaN)).toBe(false);
 expect(sameFactV1({at:new Date(0)},{at:new Date(0)})).toBe(false);
 expect(sameFactV1({a:1,b:undefined},{a:1})).toBe(false);
 expect(sameFactV1({a:1},{a:1,b:undefined})).toBe(false);
 expect(sameFactV1([1,2],{0:1,1:2})).toBe(false);
 expect(sameFactV1(null,undefined)).toBe(false);
 expect(sameFactV1(0,-0)).toBe(true);
});

test("the verdicts equal the independent Python model of structural equality",()=>{
 const pairs=Array.from({length:300},()=>{const a=make(4);return[a,next(2)?shuffle(a):make(4)];});
 if(!python)return;
 const out=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify(pairs))});
 expect(out.exitCode).toBe(0);
 expect(pairs.map(([a,b])=>sameFactV1(a,b))).toEqual(JSON.parse(out.stdout.toString()));
});
