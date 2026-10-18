/** 📋️ Identity: module exporting `compareRustUnitTextV1`, `canonicalRustUnitJsonV1`, `normalizeRustUnitV1`, `rustUnitPlanDigestV1` and more. */
import{createHash}from"node:crypto";
import type{RustUnitV1,RustUnitPlanV1}from"../../🟦️.ts";
/** 🔡️ Orders UTF8 bytes without temporary encoding and preserves replacement for unpaired surrogates. */
export function compareRustUnitTextV1(a:string,b:string):number{if(a===b)return 0;let left=0,right=0;while(left<a.length&&right<b.length){const x=a.codePointAt(left)!,y=b.codePointAt(right)!,first=x>=0xd800&&x<=0xdfff?0xfffd:x,second=y>=0xd800&&y<=0xdfff?0xfffd:y;if(first!==second)return first<second?-1:1;left+=x>0xffff?2:1;right+=y>0xffff?2:1;}return left<a.length?1:right<b.length?-1:0;}
function canonical(value:unknown):unknown{if(Array.isArray(value))return value.map(canonical);if(value&&typeof value==="object")return Object.fromEntries(Object.entries(value).sort(([a],[b])=>compareRustUnitTextV1(a,b)).map(([key,item])=>[key,canonical(item)]));return value;}
/** 🧬️ Serializes only already validated ordinary DATA with canonical UTF8 key ordering. */
export function canonicalRustUnitJsonV1(value:unknown):string{return JSON.stringify(canonical(value));}
/** 🧮️ Normalizes only already validated ordinary unit data before deriving identities. */
export function normalizeRustUnitV1(unit:RustUnitV1):RustUnitV1{return{...unit,features:[...unit.features].sort(compareRustUnitTextV1),cfg:[...unit.cfg].sort(compareRustUnitTextV1),cfgSnapshot:[...unit.cfgSnapshot].sort(compareRustUnitTextV1),inputs:[...unit.inputs].sort((a,b)=>compareRustUnitTextV1(a.path,b.path)),dependencies:[...unit.dependencies].sort((a,b)=>compareRustUnitTextV1(a.unit,b.unit)),outputs:[...unit.outputs].sort(compareRustUnitTextV1)};}
/** 🔏️ Includes declared semantic edges while separately bound artifact receipts remain capture facts. */
export function rustUnitPlanDigestV1(unit:RustUnitV1,dependencyDigests:RustUnitPlanV1["dependencyDigests"],context:RustUnitPlanV1["context"]):string{const edges=new Map(unit.dependencies.map(edge=>[edge.unit,edge])),semantic=dependencyDigests.map(row=>{const edge=edges.get(row.unit);if(!edge)throw Error("Exact declared plan dependency required");return{...row,artifactDigest:edge.artifactDigest};});return createHash("sha256").update(JSON.stringify(canonical({unit,dependencyDigests:semantic,context}))).digest("hex");}
/** 📦️ Derives the mode-local output slot solely from normalized unit identity facts. */
export function rustUnitPlanSlotV1(unit:RustUnitV1):string{return createHash("sha256").update(JSON.stringify(canonical({id:unit.id,target:unit.target,profile:unit.profile,features:unit.features,cfg:unit.cfg}))).digest("hex").slice(0,20);}
