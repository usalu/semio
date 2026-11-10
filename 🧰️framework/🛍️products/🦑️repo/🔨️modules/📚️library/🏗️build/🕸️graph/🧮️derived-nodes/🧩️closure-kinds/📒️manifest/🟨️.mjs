import{rowKeyV1}from"../../../🕰️generations/🧮️fact-rows/🟨️.mjs";

/** 📒️ Creates the settle manifest: for every closure its answer digest, canonical answer, support length and the set of its parts as a bitset over part numbers; the inverted index of the parts (by path, directory, bare ancestor, configuration, volatility and outside rows) that tells which parts a dirty set can move; and the marks of the index the closures were published in. It is read once per process and answers a round without reading a record of a closure or a part. */
export function createManifestV1(){return{partNames:[],partNo:new Map(),indexed:new Set(),entries:new Map(),marks:new Map(),paths:new Map(),dirs:new Map(),bare:new Map(),volatile:new Set(),config:new Set(),bareAny:new Set(),outside:new Map()};}

const partNumberV1=(book,id)=>{let number=book.partNo.get(id);if(number===undefined){number=book.partNames.length;book.partNames.push(id);book.partNo.set(id,number);}return number;};
const bitsOfV1=numbers=>{let top=0;for(const number of numbers)if(number>top)top=number;const words=new Uint32Array((top>>5)+1);for(const number of numbers)words[number>>5]|=1<<(number&31);return words;};
const addTo=(map,key,number)=>{const list=map.get(key);if(list===undefined)map.set(key,[number]);else list.push(number);};

/** 🧭️ Whether two part sets share a part. */
export function intersectsV1(left,right){const length=Math.min(left.length,right.length);for(let at=0;at<length;at++)if((left[at]&right[at])!==0)return true;return false;}

/** 🧭️ Registers what can move one part (its full summary) in the inverted index, once. */
export function indexPartV1(book,id,meta){
 if(book.indexed.has(id))return;book.indexed.add(id);const number=partNumberV1(book,id);
 for(const path of meta.paths)addTo(book.paths,path,number);
 for(const dir of meta.dirs)addTo(book.dirs,dir,number);
 for(const dir of meta.bare)addTo(book.bare,dir,number);
 if(meta.bare.length)book.bareAny.add(number);
 if(meta.volatile)book.volatile.add(number);
 if(meta.config)book.config.add(number);
 for(const row of meta.outside){const key=rowKeyV1(row),known=book.outside.get(key);if(known===undefined)book.outside.set(key,{row,parts:[number]});else known.parts.push(number);}
}

/** ✍️ Records the answer of one closure. */
export function recordEntryV1(book,name,{digest,fact,length,ids}){book.entries.set(name,{digest,fact,length,bits:bitsOfV1(ids.map(id=>partNumberV1(book,id)))});}

/** 📚️ The sorted part ids of one closure entry. */
export function partIdsOfV1(book,entry){const out=[];for(let at=0;at<entry.bits.length;at++){let word=entry.bits[at];while(word!==0){const bit=31-Math.clz32(word&-word);out.push(book.partNames[at*32+bit]);word&=word-1;}}return out.sort();}

/**
 * 🎯️ The parts a dirty set can move, as a bitset: exactly the parts `rowTouchedV1` would call touched (volatile, configuration when the set names one, named files, named directories, bare ancestors, anything behind an `excluded` predicate), plus the parts of every outside row that no longer holds (`verifyOutside(rows)` returns the rows that failed; the journal never watches them, so they are checked every round, once each).
 * @param {ReturnType<typeof createManifestV1>} book
 * @param {{paths:ReadonlySet<string>;directories:ReadonlySet<string>;configs:boolean;modules:boolean;excluded?:(path:string)=>boolean}} summary
 * @param {(rows:unknown[])=>Promise<unknown[]>} verifyOutside
 */
export async function affectedPartsV1(book,summary,verifyOutside){
 const hit=new Set(book.volatile),add=list=>{if(list!==undefined)for(const number of list)hit.add(number);};
 if(summary.configs)add(book.config);
 for(const path of summary.paths)add(book.paths.get(path));
 for(const dir of summary.directories){add(book.dirs.get(dir));add(book.bare.get(dir));}
 if(summary.modules)add(book.bareAny);
 if(summary.excluded!==undefined){for(const[path,list]of book.paths)if(summary.excluded(path)===true)add(list);for(const[dir,list]of book.dirs)if(summary.excluded(dir)===true)add(list);}
 if(book.outside.size){for(const row of await verifyOutside([...book.outside.values()].map(entry=>entry.row)))add(book.outside.get(rowKeyV1(row))?.parts);}
 return bitsOfV1(hit);
}

const pairs=map=>[...map];
/** 💾️ The persisted form of the manifest (bitsets as base64 words of one common width). */
export function writeManifestV1(book){
 const width=((book.partNames.length-1)>>5)+1||1,names=[...book.entries.keys()],all=new Uint32Array(names.length*width);
 names.forEach((name,at)=>all.set(book.entries.get(name).bits,at*width));
 return{version:1,parts:book.partNames,indexed:[...book.indexed],names,digests:names.map(name=>book.entries.get(name).digest),facts:names.map(name=>book.entries.get(name).fact),lengths:names.map(name=>book.entries.get(name).length),width,bits:Buffer.from(all.buffer).toString("base64"),marks:pairs(book.marks),paths:pairs(book.paths),dirs:pairs(book.dirs),bare:pairs(book.bare),bareAny:[...book.bareAny],volatile:[...book.volatile],config:[...book.config],outside:[...book.outside.values()].map(entry=>[entry.row,entry.parts])};
}

/** 💾️ The manifest of a persisted form (an unknown version is an empty manifest). */
export function readManifestV1(value){
 const book=createManifestV1();if(value?.version!==1)return book;
 value.parts.forEach((id,number)=>{book.partNames.push(id);book.partNo.set(id,number);});
 for(const id of value.indexed)book.indexed.add(id);
 const bytes=Buffer.from(value.bits,"base64"),all=new Uint32Array(bytes.buffer,bytes.byteOffset,bytes.byteLength>>2);
 value.names.forEach((name,at)=>book.entries.set(name,{digest:value.digests[at],fact:value.facts[at],length:value.lengths[at],bits:all.slice(at*value.width,(at+1)*value.width)}));
 for(const[name,mark]of value.marks)book.marks.set(name,mark);
 for(const[key,list]of value.paths)book.paths.set(key,list);for(const[key,list]of value.dirs)book.dirs.set(key,list);for(const[key,list]of value.bare)book.bare.set(key,list);
 for(const number of value.bareAny)book.bareAny.add(number);for(const number of value.volatile)book.volatile.add(number);for(const number of value.config)book.config.add(number);
 for(const[row,parts]of value.outside)book.outside.set(rowKeyV1(row),{row,parts});
 return book;
}
