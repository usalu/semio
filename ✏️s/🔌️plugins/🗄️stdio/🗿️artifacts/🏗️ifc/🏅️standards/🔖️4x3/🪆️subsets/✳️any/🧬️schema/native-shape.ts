/** 🛂️ The artifact's JSON Schemas define native required fields and container shapes. Decimal wire leaves admit native bigint values here. */
import snapshotSchema from "./🔣️.json";
import mutationSchema from "./🧬️mutations/🔣️.json";
import diffSchema from "./🔺️diff/🔣️.json";
import applicationSchema from './🏷️templates/✍️apply/🔣️.json';
import type {IfcApplyOptions,IfcEditProgress} from "./🔺️diff/🟦️.ts";
interface Shape {$ref?:string;$id?:string;$defs?:Record<string,Shape>;oneOf?:Shape[];type?:string;const?:unknown;enum?:unknown[];required?:string[];properties?:Record<string,Shape>;additionalProperties?:boolean;items?:Shape;minimum?:number;maximum?:number}
const documents={snapshot:snapshotSchema as unknown as Shape,mutation:mutationSchema as unknown as Shape,diff:diffSchema as unknown as Shape,application:applicationSchema as unknown as Shape};
const sources=new Map(Object.values(documents).map(document=>[document.$id!,document]));
interface Job {value:unknown;schema:Shape;document:Shape;depth:number}
/** 🧭️ Validates owned native data with one lazy cursor per container and no external validator dependency. */
export function* nativeShapeSteps(value:unknown,kind:keyof typeof documents,options:IfcApplyOptions,refuse:(message:string,code?:"invalidValue"|"invalidIndex")=>never):Generator<IfcEditProgress,void>{
 const stack:Iterator<Job>[]=[[{value,schema:documents[kind],document:documents[kind],depth:0}][Symbol.iterator]()];let checked=0;const maximum=(options.maxDepth??512)*4+64;
 while(stack.length){const cursor=stack.at(-1)!,next=cursor.next();if(next.done){stack.pop();continue;}let{schema,document,value,depth}=next.value;if(depth>maximum)refuse("native shape nesting exceeds admission");let nativeInteger:string|undefined;
  while(schema.$ref){const[uri,pointer]=schema.$ref.split("#");if(uri){const target=sources.get(uri);if(!target)refuse("native shape references an unknown schema");document=target!;}if(document===documents.snapshot&&["/$defs/EntityId","/$defs/Unsigned64","/$defs/Signed64"].includes(pointer??"")){nativeInteger=pointer!.split("/").at(-1);break;}schema=(pointer?pointer.slice(1).split("/").reduce<unknown>((item,key)=>(item as Record<string,unknown>)[key]!,document):document)as Shape;}
  if(nativeInteger){if(typeof value!=="bigint"||(nativeInteger==="Signed64"?value<-(1n<<63n)||value>=(1n<<63n):value<(nativeInteger==="EntityId"?1n:0n)||value>=(1n<<64n)))refuse("native decimal leaves require their declared bigint domain");}
  else{
   if(schema.oneOf){if(value===null||typeof value!=="object"||Array.isArray(value))refuse("native discriminated values require objects");const record=value as Record<string,unknown>,tag=Object.hasOwn(record,"mutation")?"mutation":"kind",selected=schema.oneOf.find(branch=>{const rule=branch.properties?.[tag];return rule&&(Object.hasOwn(rule,"const")?Object.is(rule.const,record[tag]):rule.enum?.some(item=>Object.is(item,record[tag])));});if(!selected)refuse("native data requires its declared discriminant");schema=selected!;}
   if(Object.hasOwn(schema,"const")&&!Object.is(schema.const,value)||schema.enum&&!schema.enum.some(item=>Object.is(item,value)))refuse("native literal differs from its declared value");
   if(schema.type==="object"){
    if(value===null||typeof value!=="object"||Array.isArray(value))refuse("native object container is required");const record=value as Record<string,unknown>;for(const key of schema.required??[])if(!Object.hasOwn(record,key)||record[key]===undefined)refuse(`native required field ${key} is missing`);
    const properties=schema.properties??{},closed=schema.additionalProperties===false;const children=function*():Generator<Job>{for(const key in record)if(Object.hasOwn(record,key)){const child=properties[key];if(!child){if(closed)refuse(`native field ${key} is not declared`);continue;}if(record[key]!==undefined)yield{value:record[key],schema:child,document,depth:depth+1};}};stack.push(children());
   }else if(schema.type==="array"){
    if(!Array.isArray(value))refuse("native array container is required");const array=value as unknown[],length=array.length,item=schema.items;if(!item)refuse("native array items require a declared shape");const children=function*():Generator<Job>{for(let index=0;index<length;index++){if(!Object.hasOwn(array,index)||array[index]===undefined)refuse("native arrays cannot contain holes or undefined items");yield{value:array[index],schema:item!,document,depth:depth+1};}};stack.push(children());
   }else if(schema.type==="string"){if(typeof value!=="string")refuse("native string field is required");}
   else if(schema.type==="integer"){if(typeof value!=="number"||!Number.isFinite(value)||!Number.isInteger(value))refuse("native integer coordinate is required","invalidIndex");}
   else if(schema.type==="number"){if(typeof value!=="number"||!Number.isFinite(value))refuse("native finite number is required");}
   if(typeof value==="number"&&(schema.minimum!==undefined&&value<schema.minimum||schema.maximum!==undefined&&value>schema.maximum))refuse("native coordinate exceeds its declared range","invalidIndex");
  }
  yield{phase:"admitInput",completed:++checked,total:checked};
 }
}
