/** 🪢️ Shares the equal strings of the owner facts one closure walk retains: the heap census of a whole-closure read (`slice-heap`, 2.4 GB heap, 7.7 M nodes) held 3.7 M strings (999 MB, 75% of the heap) behind 4.6 M array slots, while the files of all owners of the closure are 40 k distinct paths; every owner fact is read from its page or its memo as fresh strings, so the same path was held once per owner that lists it.
 * @see ../🟦️.ts
 * @see ../../../🧠️memory/📸️heap-snapshot/🟨️.mjs */
export type StringSharingV1={share<T>(value:T):T;readonly size:number};

/** 🪢️ One table of shared strings: `share` answers a copy of a plain JSON value whose every string (values and keys) is the one instance held by the table, so what a caller keeps costs the distinct strings once. The table is the lifetime of the walk that made it. */
export function createStringSharingV1():StringSharingV1{
 const table=new Map<string,string>();
 const one=(text:string):string=>{const known=table.get(text);if(known!==undefined)return known;table.set(text,text);return text;};
 const copy=(value:unknown):unknown=>{
  if(typeof value==="string")return one(value);
  if(Array.isArray(value)){const out=new Array(value.length);for(let at=0;at<value.length;at++)out[at]=copy(value[at]);return out;}
  if(value!==null&&typeof value==="object"){const out:Record<string,unknown>={};for(const key of Object.keys(value))out[one(key)]=copy((value as Record<string,unknown>)[key]);return out;}
  return value;
 };
 return{share:<T>(value:T)=>copy(value) as T,get size(){return table.size;}};
}
