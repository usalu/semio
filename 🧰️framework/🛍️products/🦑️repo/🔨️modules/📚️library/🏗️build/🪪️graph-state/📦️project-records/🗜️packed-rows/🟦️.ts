import schema from "./🧬️schema/🔣️.json"with{type:"json"};

/** 🗜️ The authored numbers of the packed row form: its version, the length from which a repeated string is pooled and the key of a pool reference.
 * @see ./🧬️schema/🔣️.json */
export const packedRowsPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type PackedRowsPolicyV1=typeof packedRowsPolicyV1;
export type PackedRowV1=Readonly<{kind:string;path:string;fact?:unknown}>;
export type PackedTableRowV1=readonly [string,number,string]|readonly [string,number,string,unknown];
export type PackedRowsV1=Readonly<{table:readonly PackedTableRowV1[];pool:readonly string[];lists:readonly (readonly number[])[];extras:readonly unknown[]}>;
export type RowPackerV1=Readonly<{add(rows:readonly PackedRowV1[]):void;size():Readonly<{rows:number;references:number}>;finish(extras?:readonly unknown[]):PackedRowsV1}>;

const isObject=(value:unknown):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const rowKey=(row:PackedRowV1)=>row.kind+"\0"+row.path+"\0"+(row.fact===undefined?"":"1"+JSON.stringify(row.fact));
const high=(code:number)=>code>=0xd800&&code<=0xdbff;

/** ✂️ The number of leading UTF-16 code units two paths share, never ending between the two halves of a surrogate pair. */
export function sharedPrefixV1(left:string,right:string):number{
 const limit=Math.min(left.length,right.length);let at=0;
 while(at<limit&&left.charCodeAt(at)===right.charCodeAt(at))at++;
 return at>0&&high(left.charCodeAt(at-1))?at-1:at;
}

const hasPoolKey=(value:unknown,key:string):boolean=>Array.isArray(value)?value.some(item=>hasPoolKey(item,key)):isObject(value)?key in value||Object.values(value).some(item=>hasPoolKey(item,key)):false;
const countStrings=(value:unknown,minimum:number,seen:Map<string,number>):void=>{
 if(typeof value==="string"){if(value.length>=minimum)seen.set(value,(seen.get(value)??0)+1);}
 else if(Array.isArray(value))for(const item of value)countStrings(item,minimum,seen);
 else if(isObject(value))for(const item of Object.values(value))countStrings(item,minimum,seen);
};
const pooled=(value:unknown,index:ReadonlyMap<string,number>,key:string):unknown=>typeof value==="string"?index.has(value)?{[key]:index.get(value)!}:value:Array.isArray(value)?value.map(item=>pooled(item,index,key)):isObject(value)?Object.fromEntries(Object.entries(value).map(([name,item])=>[name,pooled(item,index,key)])):value;

/** 📚️ A packer that takes the lists of rows one after the other and keeps nothing but the distinct rows and the numbers of each list, so a closure entry of thousands of layers is packed one layer at a time. `finish` writes the table, the pool and the lists (see {@link PackedRowsV1}). */
export function createRowPackerV1(policy:PackedRowsPolicyV1=packedRowsPolicyV1):RowPackerV1{
 const objects=new WeakMap<object,number>(),keys=new Map<string,number>(),rows:PackedRowV1[]=[],lists:number[][]=[];
 let references=0;
 const numberOf=(row:PackedRowV1)=>{
  let id=objects.get(row);
  if(id!==undefined)return id;
  const key=rowKey(row);id=keys.get(key);
  if(id===undefined){id=rows.length;keys.set(key,id);rows.push(row);}
  objects.set(row,id);return id;
 };
 return Object.freeze({
  add(list){const numbers=new Array<number>(list.length);let previous=-1;for(let at=0;at<list.length;at++){const id=numberOf(list[at]!);numbers[at]=id-previous-1;previous=id;}references+=list.length;lists.push(numbers);},
  size:()=>({rows:rows.length,references}),
  finish(extras=[]){
   const seen=new Map<string,number>(),clean=![...extras,...rows.map(row=>row.fact)].some(value=>hasPoolKey(value,policy.poolKey));
   if(clean){for(const row of rows)countStrings(row.fact,policy.minimumPoolLength,seen);for(const extra of extras)countStrings(extra,policy.minimumPoolLength,seen);}
   const pool=[...seen].filter(([,count])=>count>1).map(([text])=>text),index=new Map(pool.map((text,at)=>[text,at] as const));
   const use=(value:unknown)=>pool.length===0?value:pooled(value,index,policy.poolKey);
   let previous="";
   const table=rows.map((row):PackedTableRowV1=>{
    const prefix=sharedPrefixV1(previous,row.path),suffix=row.path.slice(prefix);previous=row.path;
    return row.fact===undefined?[row.kind,prefix,suffix]:[row.kind,prefix,suffix,use(row.fact)];
   });
   return{table,pool,lists,extras:extras.map(use)};
  },
 });
}

/** 📦️ The rows of several lists packed in one call (see {@link createRowPackerV1}). */
export function packRowListsV1(lists:readonly (readonly PackedRowV1[])[],extras:readonly unknown[]=[],policy:PackedRowsPolicyV1=packedRowsPolicyV1):PackedRowsV1{
 const packer=createRowPackerV1(policy);for(const list of lists)packer.add(list);return packer.finish(extras);
}

const refused=(reason:string):never=>{throw new Error("Packed rows refused: "+reason);};
const expand=(value:unknown,pool:readonly string[],key:string):unknown=>{
 if(Array.isArray(value))return value.map(item=>expand(item,pool,key));
 if(!isObject(value))return value;
 const names=Object.keys(value);
 if(names.length===1&&names[0]===key&&Number.isInteger(value[key])){const text=pool[value[key] as number];return text===undefined?refused("pool index"):text;}
 return Object.fromEntries(names.map(name=>[name,expand(value[name],pool,key)]));
};

/** 📖️ The lists of rows and the extra values of a packed form: every list holds the row objects of the table (one object per distinct row, shared by the lists that name it). A form that does not hold together (a prefix longer than the path before it, a number outside the table, a pool index outside the pool) is refused with an error. */
export function unpackRowListsV1(packed:PackedRowsV1,policy:PackedRowsPolicyV1=packedRowsPolicyV1):Readonly<{lists:PackedRowV1[][];extras:unknown[]}>{
 if(!isObject(packed)||!Array.isArray(packed.table)||!Array.isArray(packed.pool)||!Array.isArray(packed.lists)||!Array.isArray(packed.extras))return refused("shape");
 const use=(value:unknown)=>packed.pool.length===0?value:expand(value,packed.pool,policy.poolKey);
 let previous="";
 const rows=packed.table.map((entry):PackedRowV1=>{
  if(!Array.isArray(entry)||(entry.length!==3&&entry.length!==4)||typeof entry[0]!=="string"||!Number.isInteger(entry[1])||typeof entry[2]!=="string")return refused("table row");
  const prefix=entry[1] as number;if(prefix<0||prefix>previous.length)return refused("prefix");
  const path=previous.slice(0,prefix)+entry[2];previous=path;
  return entry.length===3?{kind:entry[0],path}:{kind:entry[0],path,fact:use(entry[3])};
 });
 const lists=packed.lists.map(numbers=>{
  if(!Array.isArray(numbers))return refused("list");
  const out=new Array<PackedRowV1>(numbers.length);let last=-1;
  for(let at=0;at<numbers.length;at++){const delta=numbers[at];if(!Number.isInteger(delta))return refused("number");last=last+(delta as number)+1;const row=rows[last];if(row===undefined)return refused("row number");out[at]=row;}
  return out;
 });
 return{lists,extras:packed.extras.map(use)};
}
