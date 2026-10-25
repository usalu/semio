import {createHash} from "node:crypto";

/** 🔢️ One content address of Bazel's remote execution API: the lowercase sha256 and the byte size of a blob. */
export type RemoteCacheDigestV1=Readonly<{hash:string;size:number}>;

/** 📦️ What an action result points at: blobs it needs in the content store (outputs and streams that are not inline) and the output trees whose files it needs as well. */
export type RemoteCacheActionResultV1=Readonly<{references:readonly RemoteCacheDigestV1[];trees:readonly RemoteCacheDigestV1[]}>;

/** 🚦️ How far an uploaded action result is checked: not at all, for a well-formed message, or additionally for every blob it names. */
export type RemoteCacheValidationV1="off"|"structure"|"complete";

/** 🗃️ The content store as the validator sees it: a cheap presence test and a read of one small blob. */
export interface RemoteCacheBlobsV1{has(hash:string):boolean;read(hash:string):Promise<Uint8Array|undefined>}

/** ⚖️ The verdict on one action result. */
export type RemoteCacheActionVerdictV1=Readonly<{valid:true}>|Readonly<{valid:false;kind:"malformed"|"missing";reason:string}>;

type Field=Readonly<{number:number;wire:number;value:number;big:boolean;data:Uint8Array}>;

const HASH=/^[0-9a-f]{64}$/u,EMPTY=createHash("sha256").digest("hex"),NONE=new Uint8Array(0),utf8=new TextDecoder("utf-8",{fatal:true}),fail=(reason:string):never=>{throw Error(reason);};

function varint(bytes:Uint8Array,offset:number):readonly [number,boolean,number]{
 let result=0;
 for(let index=0;index<10;index++){
  const byte=bytes[offset+index];
  if(byte===undefined)return fail("truncated varint");
  if(index===9&&(byte&0x7e)!==0)return fail("varint exceeds 64 bits");
  result+=(byte&0x7f)*2**(7*index);
  if((byte&0x80)===0)return [result,result>Number.MAX_SAFE_INTEGER,offset+index+1];
 }
 return fail("varint longer than ten bytes");
}

function* fields(bytes:Uint8Array):Generator<Field>{
 let offset=0;
 while(offset<bytes.length){
  const [tag,,afterTag]=varint(bytes,offset),number=Math.floor(tag/8),wire=tag%8;
  if(number<1||number>536870911)fail("invalid field number");
  offset=afterTag;
  if(wire===0){const [value,big,next]=varint(bytes,offset);offset=next;yield {number,wire,value,big,data:NONE};}
  else if(wire===1||wire===5){const length=wire===1?8:4;if(offset+length>bytes.length)fail("truncated fixed field");yield {number,wire,value:0,big:false,data:bytes.subarray(offset,offset+length)};offset+=length;}
  else if(wire===2){const [length,big,next]=varint(bytes,offset);if(big||length>bytes.length-next)fail("length-delimited field exceeds the message");yield {number,wire,value:length,big:false,data:bytes.subarray(next,next+length)};offset=next+length;}
  else fail(`wire type ${wire} is not allowed`);
 }
}

const expect=(field:Field,wire:number,what:string):void=>{if(field.wire!==wire)fail(`${what} has wire type ${field.wire}, expected ${wire}`);};
function text(field:Field,what:string):string{expect(field,2,what);try{return utf8.decode(field.data);}catch{return fail(`${what} is not UTF-8`);}}
const required=(value:string,what:string):string=>value===""?fail(`${what} must not be empty`):value;

function digest(field:Field,what:string):RemoteCacheDigestV1{
 expect(field,2,what);let hash:string|undefined,size=0;
 for(const part of fields(field.data)){
  if(part.number===1)hash=text(part,`${what}.hash`);
  else if(part.number===2){expect(part,0,`${what}.size_bytes`);if(part.big)fail(`${what}.size_bytes is out of range`);size=part.value;}
 }
 if(hash===undefined||!HASH.test(hash))return fail(`${what} needs a lowercase sha256 hash`);
 if((size===0)!==(hash===EMPTY))fail(`${what} is not consistent with the empty blob`);
 return Object.freeze({hash,size});
}

/** 🔬️ Decodes Bazel's `ActionResult` message strictly: every field of a known number must have its wire type, paths must not be empty, digests must be lowercase sha256 with a consistent size, inline contents must match their digest. Unknown fields are skipped, so newer protocol revisions stay accepted. Throws with the first problem. */
export function parseRemoteCacheActionResultV1(bytes:Uint8Array):RemoteCacheActionResultV1{
 const references:RemoteCacheDigestV1[]=[],trees:RemoteCacheDigestV1[]=[],reference=(value:RemoteCacheDigestV1):void=>{if(value.size>0)references.push(value);};
 for(const field of fields(bytes)){
  if(field.number===2){
   expect(field,2,"output_files");let path="",entry:RemoteCacheDigestV1|undefined,contents:Uint8Array|undefined;
   for(const part of fields(field.data)){
    if(part.number===1)path=required(text(part,"output_files.path"),"output_files.path");
    else if(part.number===2)entry=digest(part,"output_files.digest");
    else if(part.number===4)expect(part,0,"output_files.is_executable");
    else if(part.number===5){expect(part,2,"output_files.contents");contents=part.data;}
   }
   if(path==="")fail("output_files.path is required");
   if(entry===undefined)fail("output_files.digest is required");
   else if(contents!==undefined){if(contents.length!==entry.size||createHash("sha256").update(contents).digest("hex")!==entry.hash)fail("output_files.contents does not match its digest");}
   else reference(entry);
  }else if(field.number===3){
   expect(field,2,"output_directories");let path="",tree:RemoteCacheDigestV1|undefined;
   for(const part of fields(field.data)){
    if(part.number===1)path=required(text(part,"output_directories.path"),"output_directories.path");
    else if(part.number===3)tree=digest(part,"output_directories.tree_digest");
    else if(part.number===5)digest(part,"output_directories.root_directory_digest");
   }
   if(path==="")fail("output_directories.path is required");
   if(tree===undefined)fail("output_directories.tree_digest is required");
   else if(tree.size>0)trees.push(tree);
  }else if(field.number===4)expect(field,0,"exit_code");
  else if(field.number===5||field.number===7)expect(field,2,field.number===5?"stdout_raw":"stderr_raw");
  else if(field.number===6||field.number===8)reference(digest(field,field.number===6?"stdout_digest":"stderr_digest"));
  else if(field.number===9){expect(field,2,"execution_metadata");for(const part of fields(field.data))void part;}
  else if(field.number===10||field.number===11||field.number===12){
   expect(field,2,"output_symlinks");let path="";
   for(const part of fields(field.data)){if(part.number===1)path=required(text(part,"output_symlinks.path"),"output_symlinks.path");else if(part.number===2)text(part,"output_symlinks.target");}
   if(path==="")fail("output_symlinks.path is required");
  }
 }
 return Object.freeze({references:Object.freeze(references),trees:Object.freeze(trees)});
}

/** 🌳️ Decodes a `Tree` blob (a root directory and its children) and returns the digests of every file inside; directory nodes are checked for shape only because their content travels inside the tree. Throws with the first problem. */
export function parseRemoteCacheTreeV1(bytes:Uint8Array):readonly RemoteCacheDigestV1[]{
 const files:RemoteCacheDigestV1[]=[];
 for(const field of fields(bytes)){
  if(field.number!==1&&field.number!==2)continue;
  expect(field,2,"tree.directory");
  for(const part of fields(field.data)){
   if(part.number===1){
    expect(part,2,"directory.files");let name="",entry:RemoteCacheDigestV1|undefined;
    for(const node of fields(part.data)){if(node.number===1)name=required(text(node,"file.name"),"file.name");else if(node.number===2)entry=digest(node,"file.digest");}
    if(name===""||entry===undefined)fail("file nodes need a name and a digest");
    else if(entry.size>0)files.push(entry);
   }else if(part.number===2){
    expect(part,2,"directory.directories");let name="",entry:RemoteCacheDigestV1|undefined;
    for(const node of fields(part.data)){if(node.number===1)name=required(text(node,"directory.name"),"directory.name");else if(node.number===2)entry=digest(node,"directory.digest");}
    if(name===""||entry===undefined)fail("directory nodes need a name and a digest");
   }else if(part.number===3){
    expect(part,2,"directory.symlinks");let name="";
    for(const node of fields(part.data)){if(node.number===1)name=required(text(node,"symlink.name"),"symlink.name");else if(node.number===2)text(node,"symlink.target");}
    if(name==="")fail("symlink nodes need a name");
   }
  }
 }
 return Object.freeze(files);
}

/** 🎬️ Judges an uploaded action result at the requested level: malformed messages are refused at `structure`; at `complete` every blob the result names, and every file of its output trees, must already be in the content store, because Bazel uploads blobs before the result that points at them. */
export async function validateRemoteCacheActionResultV1(bytes:Uint8Array,level:RemoteCacheValidationV1,blobs:RemoteCacheBlobsV1):Promise<RemoteCacheActionVerdictV1>{
 if(level==="off")return {valid:true};
 let parsed:RemoteCacheActionResultV1;
 try{parsed=parseRemoteCacheActionResultV1(bytes);}catch(error){return {valid:false,kind:"malformed",reason:(error as Error).message};}
 if(level==="structure")return {valid:true};
 for(const value of parsed.references)if(!blobs.has(value.hash))return {valid:false,kind:"missing",reason:`blob ${value.hash} is not in the content store`};
 for(const value of parsed.trees){
  const data=blobs.has(value.hash)?await blobs.read(value.hash):undefined;
  if(data===undefined)return {valid:false,kind:"missing",reason:`tree ${value.hash} is not in the content store`};
  let files:readonly RemoteCacheDigestV1[];
  try{files=parseRemoteCacheTreeV1(data);}catch(error){return {valid:false,kind:"malformed",reason:`tree ${value.hash}: ${(error as Error).message}`};}
  for(const file of files)if(!blobs.has(file.hash))return {valid:false,kind:"missing",reason:`blob ${file.hash} of tree ${value.hash} is not in the content store`};
 }
 return {valid:true};
}
