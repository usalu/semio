import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {applyPatch} from "fast-json-patch";
import contract from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

type Value={kind:string;value?:boolean|string;items?:Value[];members?:{name:string;value:Value}[]};
function words(value:number):Buffer{const bytes:number[]=[];do{bytes.push((value&127)|(value>=128?128:0));value=Math.floor(value/128);}while(value);return Buffer.from(bytes);}
function text(value:string):Buffer{const bytes=Buffer.from(value);return Buffer.concat([words(bytes.length),bytes]);}
function body(source:Value):Buffer{
 const symbols=new Map<string,number>();const visit=(node:Value)=>{if(node.kind==="text"){const value=node.value as string;symbols.set(value,(symbols.get(value)??0)+1);}for(const child of node.items??[])visit(child);for(const member of node.members??[])visit(member.value);};visit(source);
 const dictionary=[...symbols].filter(([value,count])=>Buffer.byteLength(value)<=128||count>=2).map(([value])=>value).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
 const encode=(node:Value):Buffer=>{
  if(node.kind==="null")return Buffer.from([18]);if(node.kind==="boolean")return Buffer.from([node.value?2:1]);if(node.kind==="text"){const value=node.value as string,index=dictionary.indexOf(value);return index<0?Buffer.concat([Buffer.from([7]),text(value)]):Buffer.concat([Buffer.from([6]),words(index)]);}
  if(node.kind==="array")return Buffer.concat([Buffer.from([12]),words(node.items!.length),...node.items!.map(encode)]);
  return Buffer.concat([Buffer.from([16]),words(node.members!.length),...node.members!.flatMap(member=>[Buffer.from([7]),text(member.name),encode(member.value)])]);
 };
 return Buffer.concat([words(dictionary.length),...dictionary.map(text),Buffer.from([1,1,17]),encode(source)]);
}

test("strict intrinsic occurrence contract preserves original ordinal wire independently",()=>{
 const validate=new Ajv({strict:true}).compile(contract);expect(validate(fixture)).toBe(true);expect(new Set(fixture.cases.map(row=>row.id)).size).toBe(5);
 for(const row of fixture.cases){expect(body(row.source as Value).toString("hex")).toBe(row.bodyHex);expect(applyPatch({},[{op:"add",path:"/source",value:structuredClone(row.source)}],true).newDocument).toEqual({source:row.source});}
 for(const operation of[
  {op:"replace",path:"/ordering",value:"utf8Bytes"},
  {op:"replace",path:"/equalKeys",value:"discardDuplicates"},
  {op:"replace",path:"/version",value:0},
  {op:"add",path:"/legacy",value:true},
  {op:"remove",path:"/cases/0/bodyHex"},
 ])expect(validate(applyPatch(structuredClone(fixture),[operation],true).newDocument)).toBe(false);
 const unicode=fixture.cases.find(row=>row.id==="utf8-versus-utf16")!;const names=unicode.source.members!.map(member=>member.name);expect([...names].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)))).not.toEqual(names);
 expect(fixture.cases[0]!.bodyHex).not.toBe(fixture.cases[1]!.bodyHex);expect(fixture.cases[4]!.source.members!.map(member=>member.name)).toEqual(["z","a","a"]);
 console.log("[DEBUG] intrinsic occurrence order5 exact independent Buffer wire RFC6902 clone and5 strict Ajv negative authorities");
});

test("original receipt order fold 1 keeps plain trials outside schema authority",async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);console.log("[DEBUG] Original receipt/order/fold trial has no whole-corpus schema authority");});
