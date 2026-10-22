/** 📏️ Contract: the measure union keeps the largest bytes and the largest lines per path across merged closures, never counts excluded paths, starts from its floor and answers `peek` exactly as `add` would; the corpus is language-agnostic and fast-check (seeded) compares random unions with a brute-force model and the order-free total. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import {MeasureUnion,totalsFit} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const partsOf=(rows:unknown[][][]):Map<string,{bytes:number;lines:number}>[]=>rows.map(part=>new Map(part.map(([path,bytes,lines])=>[path as string,{bytes:bytes as number,lines:lines as number}] as [string,{bytes:number;lines:number}])));
const model=(floor:{files:number;bytes:number;lines:number},excluded:Set<string>,parts:Map<string,{bytes:number;lines:number}>[]):{files:number;bytes:number;lines:number}=>{
 const best=new Map<string,{bytes:number;lines:number}>();for(const part of parts)for(const[path,measure]of part){if(excluded.has(path))continue;const current=best.get(path);best.set(path,{bytes:Math.max(current?.bytes??0,measure.bytes),lines:Math.max(current?.lines??0,measure.lines)});}
 let files=floor.files,bytes=floor.bytes,lines=floor.lines;for(const measure of best.values()){files+=1;bytes+=measure.bytes;lines+=measure.lines;}return{files,bytes,lines};
};
for(const row of corpus.cases)test("union: "+row.name,()=>{
 const union=new MeasureUnion(new Set(row.excluded),row.floor),parts=partsOf(row.parts as unknown[][][]);expect(union.totals()).toEqual(row.floor);
 const peeked=union.peek(parts);union.add(parts);expect(union.totals()).toEqual(row.totals);expect(peeked).toEqual(row.totals);
});
for(const row of corpus.fits)test("totals fit "+JSON.stringify(row.totals)+" in "+JSON.stringify(row.budget),()=>{expect(totalsFit(row.totals,row.budget)).toBe(row.expected);});
test("peeking never keeps the parts",()=>{const union=new MeasureUnion(),before=union.totals();union.peek([new Map([["a",{bytes:5,lines:5}]])]);expect(union.totals()).toEqual(before);});
test("random unions agree with the brute-force model and with peek (fast-check)",()=>{
 const measure=fc.record({bytes:fc.integer({min:0,max:500}),lines:fc.integer({min:0,max:50})}),part=fc.array(fc.tuple(fc.constantFrom("a","b","c","d","mirror"),measure),{maxLength:6}).map(rows=>new Map(rows));
 fc.assert(fc.property(fc.array(part,{maxLength:5}),fc.array(part,{maxLength:5}),fc.integer({min:0,max:3}),(first,second,floorFiles)=>{
  const floor={files:floorFiles,bytes:floorFiles*7,lines:floorFiles},excluded=new Set(["mirror"]),union=new MeasureUnion(excluded,floor);
  union.add(first);const peeked=union.peek(second);expect(peeked).toEqual(model(floor,excluded,[...first,...second]));
  union.add(second);expect(union.totals()).toEqual(peeked);expect(union.totals()).toEqual(model(floor,excluded,[...first,...second]));
  const fresh=new MeasureUnion(excluded,floor);fresh.add([...second,...first]);expect(fresh.totals()).toEqual(union.totals());
 }),{numRuns:corpus.property.runs,seed:corpus.property.seed});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
