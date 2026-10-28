import {test,expect} from "bun:test";
import {applyPatch,getValueByPointer} from "fast-json-patch";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {declaredValueMutation} from "./🟦️.ts";
import {ifc4x3Catalog} from "../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts";
import {ifcSelectAccepts,ifcSubtype} from "../📚️catalog/🟦️.ts";
import {applyIfcMutation,inverseIfcMutation} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
import type {IfcSnapshot} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
const contract=JSON.parse(readFileSync(new URL("./🧪️tests/constituent-value.json",import.meta.url),"utf8"));
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==="bigint"?item.toString():item);
const real=(value:number)=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return {bits:view.getBigUint64(0)};};
function native(value:any):any{if(Array.isArray(value))return value.map(native);if(value&&typeof value==="object"){if(value.kind==="real")return {...value,value:real(value.value)};if(value.kind==="integer")return {...value,value:BigInt(value.value)};return Object.fromEntries(Object.entries(value).map(([key,item])=>[key,key==="id"?BigInt(item as number):native(item)]));}return value;}
const source=():IfcSnapshot=>({...native(contract.snapshot),header:{fileDescription:[],fileName:[],fileSchema:[]}});
test("original domain metadata neutral names and pointer addresses agree with independent oracles",()=>{
 const metadata=JSON.parse(readFileSync(new URL("./🧪️tests/domain-metadata.json",import.meta.url),"utf8")),valid=new Ajv().compile({type:"string",maxLength:256,pattern:"^[\\x00-\\x7F]*$"});
 for(const row of metadata.names){const canonical=valid(row.source)?row.source.toUpperCase():null;expect(canonical).toBe(row.canonical);if(canonical)expect(ifc4x3Catalog.entities[canonical]).toBeDefined();}
 for(const row of metadata.membership)expect(ifc4x3Catalog.entities[row.expected]?ifcSubtype(ifc4x3Catalog,row.actual,row.expected):ifcSelectAccepts(ifc4x3Catalog,row.expected,row.actual)).toBe(row.admitted);
 for(const row of metadata.addresses){const input=source(),root=input.entities[row.position].complex![row.part].args[row.argument],relative=row.suffix?getValueByPointer(root,row.suffix):root;expect(exact(relative)).toBe(exact(getValueByPointer(input,row.path)));}
});
for(const row of contract.cases)test(`retained constituent value contract ${row.id}`,()=>{
 const input=source(),path=row.path.slice(1).split("/").map((field:string)=>/^\d+$/.test(field)?Number(field):field),position=path[1] as number;
 const value=typeof row.value==="number"?real(row.value):native(row.value);
 const kind=row.event??"set",from=row.from?.slice(1).split("/").map((field:string)=>/^\d+$/.test(field)?Number(field):field);
 const event=kind==="remove"?{kind:"remove" as const,path}:kind==="move"?{kind:"move" as const,path,from}:kind==="insert"?{kind:"insert" as const,path,value}:{kind:"set" as const,path,value};
 const run=()=>declaredValueMutation(input,ifc4x3Catalog,event);
 if(!row.admitted){expect(run).toThrow();return;}
 const mutation=run(),next=applyIfcMutation(input,mutation);
 const oracle=applyPatch(JSON.parse(exact(input)),[{op:kind==="set"?"replace":kind==="insert"?"add":kind,path:row.path,...(value===undefined?{}:{value:JSON.parse(exact(value))}),from:row.from}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[position].args)).toBe(exact(input.entities[position].args));
 expect(exact(next.entities[position].complex![0].args[path[5] as number])).toBe(exact(native(row.expected)));
 let restored=next;for(const inverse of inverseIfcMutation(input,mutation))restored=applyIfcMutation(restored,inverse);
 expect(exact(restored)).toBe(exact(input));
});
