import {createHash} from "node:crypto";
import {isRevisionDigestV1,isSafeRevisionPathV1,revisionErrorV1,type DigestV1,type InputManifestV1,type ManifestEntryV1,type RevisionBodyV1,type RevisionV1,type TreeEntryV1,type TreeRefV1} from "../🧬️schema/🟦️.ts";

const encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});

/** #️⃣ The sha256 hex of bytes or text. */
export const revisionDigestV1=(data:string|Uint8Array):DigestV1=>createHash("sha256").update(data).digest("hex");

/** 🔤️ Orders strings by Unicode code point (equal to UTF-8 byte order and to Python `str` order, unlike UTF-16 order for astral characters). */
export function compareCodePointsV1(left:string,right:string):number{
 let i=0,j=0;
 while(i<left.length&&j<right.length){
  const a=left.codePointAt(i)!,b=right.codePointAt(j)!;
  if(a!==b)return a<b?-1:1;
  i+=a>0xffff?2:1;j+=b>0xffff?2:1;
 }
 return i<left.length?1:j<right.length?-1:0;
}

/** 🧾️ The canonical JSON bytes that are hashed: sorted keys, no whitespace, no ASCII escaping. */
export function canonicalJsonV1(value:unknown):string{
 if(value===null||typeof value!=="object")return JSON.stringify(value);
 if(Array.isArray(value))return "["+value.map(canonicalJsonV1).join(",")+"]";
 const record=value as Record<string,unknown>;
 return "{"+Object.keys(record).filter(key=>record[key]!==undefined).sort(compareCodePointsV1).map(key=>JSON.stringify(key)+":"+canonicalJsonV1(record[key])).join(",")+"}";
}
export const canonicalBytesV1=(value:unknown):Uint8Array=>encoder.encode(canonicalJsonV1(value));

type Node={entries:Map<string,Node|ManifestEntryV1>};
export type BuiltTreeV1=Readonly<{root:DigestV1;size:number;objects:ReadonlyMap<DigestV1,Uint8Array>}>;

/** 🌳️ Builds the Merkle tree of a manifest: every directory is a tree object, equal subtrees share one digest. Entries need not be sorted. */
export function buildTreeV1(entries:readonly ManifestEntryV1[]):BuiltTreeV1{
 const root:Node={entries:new Map()};
 for(const entry of entries){
  if(!isSafeRevisionPathV1(entry.path))throw revisionErrorV1("unsafe-path",{path:entry.path});
  const parts=entry.path.split("/");let node=root;
  for(const part of parts.slice(0,-1)){
   let next=node.entries.get(part);
   if(next===undefined){next={entries:new Map()};node.entries.set(part,next);}
   if(!("entries" in next))throw revisionErrorV1("unsafe-path",{path:entry.path});
   node=next;
  }
  const name=parts.at(-1)!;
  if(node.entries.has(name))throw revisionErrorV1("unsafe-path",{path:entry.path});
  node.entries.set(name,entry);
 }
 const objects=new Map<DigestV1,Uint8Array>();
 const seal=(node:Node):{digest:DigestV1;size:number}=>{
  const rows:TreeEntryV1[]=[];let total=0;
  for(const name of [...node.entries.keys()].sort(compareCodePointsV1)){
   const child=node.entries.get(name)!;
   if("entries" in child){const sealed=seal(child);rows.push({digest:sealed.digest,kind:"tree",name,size:sealed.size});total+=sealed.size;}
   else{rows.push({digest:child.digest,kind:child.mode,name,size:child.size});total+=child.size;}
  }
  const bytes=canonicalBytesV1({entries:rows}),digest=revisionDigestV1(bytes);
  objects.set(digest,bytes);return{digest,size:total};
 };
 const sealed=seal(root);
 return Object.freeze({root:sealed.digest,size:sealed.size,objects});
}

/** 📋️ Flattens a tree to sorted manifest entries; `read` returns the bytes of a tree object. */
export async function flattenTreeV1(root:DigestV1,read:(digest:DigestV1)=>Promise<Uint8Array|undefined>,signal?:AbortSignal):Promise<ManifestEntryV1[]>{
 const out:ManifestEntryV1[]=[];
 const walk=async(digest:DigestV1,prefix:string):Promise<void>=>{
  signal?.throwIfAborted();
  const bytes=await read(digest);if(bytes===undefined)throw revisionErrorV1("unknown-revision",{revision:digest});
  const tree=JSON.parse(decoder.decode(bytes)) as {entries:TreeEntryV1[]};
  for(const row of tree.entries){
   const path=prefix===""?row.name:prefix+"/"+row.name;
   if(row.kind==="tree")await walk(row.digest,path);else out.push({path,mode:row.kind,digest:row.digest,size:row.size});
  }
 };
 await walk(root,"");return out;
}

/** 🧊️ Makes the revision of a body: its id is the digest of its canonical bytes. */
export function sealRevisionV1(body:RevisionBodyV1):{revision:RevisionV1;bytes:Uint8Array}{
 const bytes=canonicalBytesV1(body);return{revision:Object.freeze({...body,id:revisionDigestV1(bytes)}),bytes};
}

/** 🧊️ Parses revision bytes read from the store at `id`; the bytes must hash to `id`. */
export function parseRevisionV1(id:DigestV1,bytes:Uint8Array):RevisionV1{
 if(revisionDigestV1(bytes)!==id)throw revisionErrorV1("invalid-revision",{revision:id});
 let body:RevisionBodyV1;try{body=JSON.parse(decoder.decode(bytes)) as RevisionBodyV1;}catch{throw revisionErrorV1("invalid-revision",{revision:id});}
 if(typeof body.journalCursor!=="string"||typeof body.workspace!=="string"||!Array.isArray(body.roots)||body.roots.length===0||body.roots.some((root:TreeRefV1)=>!isRevisionDigestV1(root.tree)))throw revisionErrorV1("invalid-revision",{revision:id});
 return Object.freeze({...body,id});
}

/** 📋️ Makes the input manifest of a revision from its sorted entries (single root at the workspace root). */
export const manifestOfV1=(revision:RevisionV1,entries:readonly ManifestEntryV1[]):InputManifestV1=>Object.freeze({revision:revision.id,digest:revision.roots[0]!.tree,entries});

/** 🔑️ The key of an action: tool, command, environment and input manifest digest; no workspace, path or time, so equal inputs share one result everywhere. */
export function actionKeyV1(input:{tool:string;command:readonly string[];environment?:Readonly<Record<string,string>>;manifest:DigestV1}):DigestV1{
 return revisionDigestV1(canonicalBytesV1({command:[...input.command],environment:input.environment??{},manifest:input.manifest,tool:input.tool}));
}
