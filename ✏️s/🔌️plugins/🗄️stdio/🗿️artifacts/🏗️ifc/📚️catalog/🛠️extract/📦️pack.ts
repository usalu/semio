import {deflateRawSync,inflateRawSync} from "node:zlib";
import type {IfcCatalog,IfcTypeExpression,IfcAttributeDeclaration,IfcEntityDeclaration,IfcTypeDeclaration} from "../🟦️.ts";
const MAGIC=[0x49,0x46,0x43,0x50,1];
type Retained=Omit<IfcCatalog,"validation">;
class Writer{readonly bytes:number[]=[];varint(value:number):void{if(!Number.isSafeInteger(value)||value<0)throw Error("ifc catalog pack: unsigned integer expected");for(;value>=128;value=Math.floor(value/128))this.bytes.push(value%128|128);this.bytes.push(value);}byte(value:number):void{this.bytes.push(value);}}
class Reader{at=0;constructor(readonly bytes:Uint8Array){}byte():number{if(this.at>=this.bytes.length)throw Error("ifc catalog pack: truncated");return this.bytes[this.at++]!;}varint():number{let value=0,scale=1;for(;;){const b=this.byte();value+=(b&127)*scale;if(b<128)return value;scale*=128;if(scale>2**49)throw Error("ifc catalog pack: oversized integer");}}take(length:number):Uint8Array{if(this.at+length>this.bytes.length)throw Error("ifc catalog pack: truncated");const slice=this.bytes.subarray(this.at,this.at+length);this.at+=length;return slice;}}
const zigzag=(value:number):number=>value<0?-2*value-1:2*value,unzigzag=(value:number):number=>value%2?-(value+1)/2:value/2;
/** 🥡️ Encode the retained declarations into the language-neutral IFCP container: interned strings, deduplicated type expressions and attributes, LEB128 integers, first-use order. */
export function encodeCatalogPayload(catalog:Retained):Uint8Array{
 const strings=new Map<string,number>(),expressions=new Map<string,number>(),attributes=new Map<string,number>(),expressionRows:number[][]=[],attributeRows:number[][]=[];
 const str=(value:string):number=>{let id=strings.get(value);if(id===undefined){id=strings.size;strings.set(value,id);}return id;};
 const optString=(value:string|null|undefined):number=>value==null?0:str(value)+1,optInt=(value:number|null|undefined):number=>value==null?0:zigzag(value)+1;
 const expression=(value:IfcTypeExpression):number=>{const row=value.kind==="named"?[0,optString(value.name),optInt(value.width),Number(Boolean(value.fixed))]:[1,optString(value.aggregate),optInt(value.lower),optInt(value.upper),optString(value.lowerExpression),optString(value.upperExpression),Number(Boolean(value.unique))|Number(Boolean(value.optional))<<1,expression(value.item!)],key=row.join(",");let id=expressions.get(key);if(id===undefined){id=expressionRows.length;expressions.set(key,id);expressionRows.push(row);}return id;};
 const attribute=(value:IfcAttributeDeclaration):number=>{const row=[str(value.name),expression(value.typeExpr),Number(value.optional)|Number(value.derived)<<1,str(value.declaredBy)],key=row.join(",");let id=attributes.get(key);if(id===undefined){id=attributeRows.length;attributes.set(key,id);attributeRows.push(row);}return id;};
 const body=new Writer(),list=(values:readonly number[]):void=>{body.varint(values.length);values.forEach(v=>body.varint(v));};
 [catalog.schema,catalog.version,catalog.source.url,catalog.source.mirror,catalog.source.sha256].forEach(v=>body.varint(str(v)));
 const entities=Object.entries(catalog.entities);body.varint(entities.length);
 for(const[key,e]of entities){body.varint(str(key));body.varint(str(e.name));body.byte(Number(e.abstractEntity));body.varint(optString(e.supertype));body.varint(str(e.supertypeExpression));list(e.attributes.map(attribute));list(e.allAttributes.map(attribute));body.varint(e.derived.length);for(const d of e.derived){body.varint(str(d.name));body.varint(expression(d.typeExpr));body.varint(optString(d.redeclares));body.varint(str(d.expression));}list(e.inverse.map(str));list(e.whereRules.map(str));list(e.uniqueRules.map(str));}
 const types=Object.entries(catalog.types);body.varint(types.length);
 for(const[key,t]of types){body.varint(str(key));body.varint(str(t.name));body.varint(str(t.kind));list(t.members.map(str));body.varint(t.typeExpr?expression(t.typeExpr)+1:0);list(t.whereRules.map(str));}
 for(const table of [catalog.functions,catalog.rules]){const rows=Object.entries(table);body.varint(rows.length);for(const[key,text]of rows){body.varint(str(key));body.varint(str(text));}}
 const head=new Writer(),encoder=new TextEncoder();MAGIC.forEach(b=>head.byte(b));head.varint(strings.size);for(const value of strings.keys()){const utf8=encoder.encode(value);head.varint(utf8.length);utf8.forEach(b=>head.byte(b));}
 head.varint(expressionRows.length);for(const row of expressionRows){head.byte(row[0]!);row.slice(1).forEach(v=>head.varint(v));}
 head.varint(attributeRows.length);for(const row of attributeRows)row.forEach(v=>head.varint(v));
 return Uint8Array.from([...head.bytes,...body.bytes]);
}
/** 🗜️ Compress the IFCP payload as one raw DEFLATE stream (RFC 1951) at maximum effort; the committed resource is exactly this output. */
export const encodeCatalogPack=(catalog:Retained):Uint8Array=>deflateRawSync(encodeCatalogPayload(catalog),{level:9});
/** 🥠️ Independent TypeScript reader of the IFCP payload, the oracle for the Rust reader. */
export function decodeCatalogPayload(payload:Uint8Array):Retained{
 const input=new Reader(payload),decoder=new TextDecoder("utf-8",{fatal:true});
 for(const b of MAGIC)if(input.byte()!==b)throw Error("ifc catalog pack: bad magic");
 const strings=Array.from({length:input.varint()},()=>decoder.decode(input.take(input.varint()))),str=(id:number):string=>strings[id]??(()=>{throw Error("ifc catalog pack: string index");})(),opt=(id:number):string|undefined=>id?str(id-1):undefined,optInt=(id:number):number|null=>id?unzigzag(id-1):null;
 const expressions:IfcTypeExpression[]=[];for(let n=input.varint();n--;){const tag=input.byte();if(tag===0){const name=opt(input.varint()),width=optInt(input.varint()),fixed=input.varint()===1;expressions.push({kind:"named",name,width,fixed});}else{const aggregate=opt(input.varint()) as IfcTypeExpression["aggregate"],lower=optInt(input.varint()),upper=optInt(input.varint()),lowerExpression=opt(input.varint()),upperExpression=opt(input.varint()),flags=input.varint(),item=expressions[input.varint()];if(!item)throw Error("ifc catalog pack: expression index");expressions.push({kind:"aggregate",aggregate,lower,upper,lowerExpression,upperExpression,unique:Boolean(flags&1),optional:Boolean(flags&2),item});}}
 const attributes:IfcAttributeDeclaration[]=[];for(let n=input.varint();n--;){const name=str(input.varint()),typeExpr=expressions[input.varint()]!,flags=input.varint();attributes.push({name,typeExpr,optional:Boolean(flags&1),derived:Boolean(flags&2),declaredBy:str(input.varint())});}
 const strs=():string[]=>Array.from({length:input.varint()},()=>str(input.varint())),attrs=():IfcAttributeDeclaration[]=>Array.from({length:input.varint()},()=>attributes[input.varint()]!);
 const schema=str(input.varint()),version=str(input.varint()),url=str(input.varint()),mirror=str(input.varint()),sha256=str(input.varint());
 const entities:Record<string,IfcEntityDeclaration>={};for(let n=input.varint();n--;){const key=str(input.varint()),name=str(input.varint()),abstractEntity=input.byte()===1,supertype=opt(input.varint())??null,supertypeExpression=str(input.varint()),own=attrs(),all=attrs(),derived=Array.from({length:input.varint()},()=>({name:str(input.varint()),typeExpr:expressions[input.varint()]!,redeclares:opt(input.varint())??null,expression:str(input.varint())}));entities[key]={name,abstractEntity,supertype,attributes:own,allAttributes:all,derived,inverse:strs(),whereRules:strs(),uniqueRules:strs(),supertypeExpression};}
 const types:Record<string,IfcTypeDeclaration>={};for(let n=input.varint();n--;){const key=str(input.varint()),name=str(input.varint()),kind=str(input.varint()) as IfcTypeDeclaration["kind"],members=strs(),at=input.varint(),whereRules=strs();types[key]=at?{name,kind,members,typeExpr:expressions[at-1]!,whereRules}:{name,kind,members,whereRules};}
 const tables=[0,1].map(()=>Object.fromEntries(Array.from({length:input.varint()},()=>[str(input.varint()),str(input.varint())] as const)));
 if(input.at!==payload.length)throw Error("ifc catalog pack: trailing bytes");
 return {schema,version,source:{url,mirror,sha256},entities,types,functions:tables[0]!,rules:tables[1]!};
}
/** 📤️ Inflate and decode a committed catalog resource. */
export const decodeCatalogPack=(pack:Uint8Array):Retained=>decodeCatalogPayload(inflateRawSync(pack));
