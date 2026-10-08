/** 🧵️ Portable JSON, UTF8 and RFC6902 independently define borrowed diagnostic composition. */
import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";

const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const bytes=(value:string)=>Buffer.byteLength(value,"utf8");

test("🧵️ borrowed diagnostic code bytes respect the producer limit",()=>{
  for(const row of law.cases)expect(bytes(row.code)).toBeLessThanOrEqual(law.maximumCodeBytes);
});

test("🧵️ borrowed diagnostic text agrees with JSON quoting, UTF8 prefixes and independent JSON Patch",()=>{
  for(const row of law.cases){
    const targets=row.targets.map((target:any)=>target.text.repeat(target.repeat));
    let remaining=row.limit-law.messageOwnerBytes-bytes(row.code);
    const targetIndices:number[]=[];
    targets.forEach((target:string,index:number)=>{const demand=law.targetOwnerBytes+bytes(target);if(demand<=remaining){targetIndices.push(index);remaining-=demand;}});
    const rendered=row.fragments.map((fragment:any)=>fragment.kind==="quoted"?JSON.stringify(fragment.text.repeat(fragment.repeat)):fragment.kind==="unsigned"?BigInt(fragment.text).toString():fragment.text.repeat(fragment.repeat)).join("");
    let text="";
    for(const scalar of rendered){if(bytes(text)+bytes(scalar)>remaining)break;text+=scalar;}
    const expected={targetIndices,text,truncated:bytes(text)!==bytes(rendered),entryBytes:law.messageOwnerBytes+bytes(row.code)+bytes(text)+targetIndices.reduce((total,index)=>total+law.targetOwnerBytes+bytes(targets[index]),0)};
    expect(expected,row.name).toEqual(row.expected);
    const baseline={level:row.level,code:row.code,message:"",target:[]};
    const candidate=applyPatch(structuredClone(baseline),[{op:"replace",path:"/message",value:text},{op:"replace",path:"/target",value:targetIndices.map(index=>targets[index])}],true).newDocument;
    expect(candidate.level).toBe(row.level);
    expect(candidate.code).toBe(row.code);
    expect(baseline).toEqual({level:row.level,code:row.code,message:"",target:[]});
  }
  console.log(`[DEBUG] borrowed diagnostics ${law.cases.length} rows match JSON quoting, UTF8 and RFC6902; native producer and source custody remain separately required`);
});
