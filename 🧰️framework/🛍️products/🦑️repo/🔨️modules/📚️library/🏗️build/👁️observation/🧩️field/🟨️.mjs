import{createHash}from"node:crypto";

const sha256=text=>createHash("sha256").update(text).digest("hex");
const byCodePoint=(a,b)=>{const left=[...a],right=[...b];for(let at=0;at<Math.min(left.length,right.length);at++)if(left[at]!==right[at])return left[at].codePointAt(0)-right[at].codePointAt(0);return left.length-right.length;};
const canonical=value=>JSON.stringify((function walk(item){if(item===null||typeof item!=="object")return item;if(Array.isArray(item))return item.map(walk);const out={};for(const key of Object.keys(item).sort(byCodePoint))Object.defineProperty(out,key,{value:walk(item[key]),enumerable:true});return out;})(value));

/** 🧩️ The digest of one extracted value: the sha256 of its canonical JSON (keys in code point order, no spaces). The Python oracle in the tests derives the same text. */
export const fieldDigestV1=value=>sha256(canonical(value));

/** 📍️ The value at an RFC 6901 JSON pointer, `undefined` when a step is absent. */
export function jsonPointerValueV1(document,pointer){
 if(pointer==="")return document;
 if(!pointer.startsWith("/"))throw Error("JSON pointer required: "+pointer);
 let value=document;
 for(const raw of pointer.slice(1).split("/")){
  const key=raw.replaceAll("~1","/").replaceAll("~0","~");
  if(value===null||typeof value!=="object"||!Object.hasOwn(value,key))return undefined;
  value=value[key];
 }
 return value;
}

const extractors=new Map([["json",{extract:(text,pointer)=>jsonPointerValueV1(JSON.parse(text),pointer),text:true}]]);
const STORE_PATTERN=/[\\/](?:node_modules[\\/]\.bun[\\/][^\\/]+[\\/]node_modules|\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/][^\\/]+[\\/]node_modules)[\\/]/u;

/** 📦️ Whether a path lies in an installed package store whose directories are immutable per package name and version (the versioned Bun store and the content-addressed install store). */
export const installedPackageFileV1=path=>STORE_PATTERN.test(path);

/** 🔌️ Registers one more extraction format (a TOML key path, a validated designation): `extract(text, pointer, path, readOther)` returns the JSON value the observer depends on, or `undefined` when it is absent; every other file it reads goes through `readOther` and is named in the `paths` of the row. */
export function registerFieldExtractorV1(format,extract,{text=true}={}){
 if(typeof format!=="string"||!format||typeof extract!=="function")throw Error("Closed field extractor required");
 const known=extractors.get(format);if(known!==undefined&&known.extract!==extract)throw Error("Field extractor already registered: "+format);
 extractors.set(format,{extract,text});
}

registerFieldExtractorV1("package-identity",(_text,_pointer,path,readOther)=>{
 const match=STORE_PATTERN.exec(path),stop=match===null?0:match.index+match[0].length;
 for(let directory=path.slice(0,Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\")));directory.length>=stop&&directory.length>0;directory=directory.slice(0,Math.max(directory.lastIndexOf("/"),directory.lastIndexOf("\\")))){
  let manifest;try{manifest=JSON.parse(readOther(directory+"/package.json"));}catch{continue;}
  if(typeof manifest?.name==="string"&&typeof manifest?.version==="string")return{name:manifest.name,version:manifest.version};
 }
 return undefined;
},{text:false});

/** 🧮️ The fact of one field: its format, pointer and the digest of the value extracted from `read()` (the file text, or `undefined` when the file is absent); `!absent-file`, `!unparsable` and `!missing` digests keep every failure distinct from every value. */
export function fieldFactV1(path,format,pointer,read,readOther=()=>{throw Error("No other reads");},paths=[]){
 const registered=extractors.get(format);
 if(registered===undefined)return{format,pointer,digest:"!unregistered"};
 const extract=registered.extract;
 let text;try{text=registered.text?read():undefined;}catch{return paths.length?{format,pointer,digest:"!absent-file",paths}:{format,pointer,digest:"!absent-file"};}
 try{const value=extract(text,pointer,path,readOther),digest=value===undefined?"!missing":fieldDigestV1(value);return paths.length?{format,pointer,digest,paths}:{format,pointer,digest};}
 catch{return paths.length?{format,pointer,digest:"!unparsable",paths}:{format,pointer,digest:"!unparsable"};}
}

/** 🔎️ The value a field observer reads, with the same failures as the fact: throws when the file is absent, unparsable or the pointer is missing. */
export function fieldValueV1(path,format,pointer,read,readOther=()=>{throw Error("No other reads");}){
 const registered=extractors.get(format);if(registered===undefined)throw Error("Registered field extractor required: "+format);
 const value=registered.extract(registered.text?read():undefined,pointer,path,readOther);
 if(value===undefined)throw Error("Field absent: "+pointer);
 return value;
}
