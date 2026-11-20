import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {brotliCompressSync,brotliDecompressSync,constants} from "node:zlib";
import {GENERATION_POLICY_V1 as P,canonicalJsonV1,readGenerationManifestV1} from "../../🟨️.mjs";
import {transcodeFactIndexV1} from "../../🧮️fact-codec/🟨️.mjs";
const policy={rootToken:"\u27e6semio-root\u27e7"};

/** 🔤️ The roots of one machine in the three spellings a JSON or shell text may hold, each with its own token: as is, with forward slashes, with doubled backslashes. Spellings that coincide (every POSIX root) keep the first token. Longer spellings come first so a nested spelling is never cut. */
function spellings(root:string):readonly (readonly [string,string])[]{
 const open=policy.rootToken.slice(0,-1),close=policy.rootToken.slice(-1),raw=root.replace(/[\\/]+$/u,""),rows:[string,string][]=[[raw,open+close],[raw.replaceAll("\\","/"),open+"-slash"+close],[raw.replaceAll("\\","\\\\"),open+"-json"+close]];
 return rows.filter((row,index)=>rows.findIndex(other=>other[0]===row[0])===index).sort((a,b)=>b[0].length-a[0].length);
}
const escapeRegExp=(text:string):string=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
const boundary="(?=$|[^\\p{L}\\p{N}_.-])",lead="(?<![\\p{L}\\p{N}_.-])";

/** 🔁️ The text rewrite of one direction for one root, built once: `neutralize` replaces the root by the root token in every spelling, only where the root stands as a whole path prefix; `expand` replaces the tokens by the root of this machine. */
export function rootRewriterV1(root:string,direction:"neutralize"|"expand"):(text:string)=>string{
 if(root==="")return text=>text;
 const rows=spellings(root);
 if(direction==="expand")return text=>{for(const [spelling,token] of rows)if(text.includes(token))text=text.replaceAll(token,()=>spelling);return text;};
 const patterns=rows.map(([spelling,token])=>[spelling,token,new RegExp(lead+escapeRegExp(spelling)+boundary,"gu")] as const);
 return text=>{for(const [spelling,token,pattern] of patterns)if(text.includes(spelling))text=text.replace(pattern,()=>token);return text;};
}

/** 🧼️ Replaces the root of this machine by the root token in every spelling, only where the root stands as a whole path prefix, so a document computed at any root has the same bytes. */
export function neutralizeRootV1(bytes:Uint8Array,root:string):Uint8Array{
 if(root==="")return bytes;
 return Buffer.from(rootRewriterV1(root,"neutralize")(Buffer.from(bytes).toString("utf8")),"utf8");
}

/** 🏠️ The reverse of {@link neutralizeRootV1} for the root of this machine. */
export function expandRootV1(bytes:Uint8Array,root:string):Uint8Array{
 if(root==="")return bytes;
 return Buffer.from(rootRewriterV1(root,"expand")(Buffer.from(bytes).toString("utf8")),"utf8");
}

/** 🔤️ The same rewrite for one text of a fact index, in the domain the old JSON path worked in: the text is escaped as JSON, rewritten, and read back, so every spelling of a root (also the doubled backslash one) is moved exactly as it was inside the JSON document. */
export function indexTextRewriterV1(root:string,direction:"neutralize"|"expand"):(text:string)=>string{
 const rewrite=rootRewriterV1(root,direction);
 return text=>{const escaped=JSON.stringify(text).slice(1,-1),moved=rewrite(escaped);return moved===escaped?text:JSON.parse('"'+moved+'"') as string;};
}


const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex"),maximum=536870912;
/** 📄️ One document of a generation: its relative path (`m/<sha>.json`, `o/…json.br`, `x/….facts.br`, `f/…json.br`) and its bytes. */
export type GenerationDocumentV1=Readonly<{path:string;bytes:Uint8Array}>;
type Direction="neutralize"|"expand";

const pack=(raw:Uint8Array):Uint8Array=>brotliCompressSync(raw,{params:{[constants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[constants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[constants.BROTLI_PARAM_SIZE_HINT]:raw.length}});
const unpack=(bytes:Uint8Array):Uint8Array=>brotliDecompressSync(bytes,{maxOutputLength:maximum});
const rewrite=(bytes:Uint8Array,root:string,direction:Direction):Uint8Array=>direction==="neutralize"?neutralizeRootV1(bytes,root):expandRootV1(bytes,root);

/** 🔁️ Moves the root of one machine into or out of the bytes of one stored document and repacks it: JSON documents as text, the binary fact index through the text rewrite of its codec (its rows hold the absolute paths; no row is ever held as an object). */
export function transformGenerationDocumentV1(path:string,bytes:Uint8Array,root:string,direction:Direction):Uint8Array{
 if(path.startsWith(P.indexes+"/")){
  return pack(transcodeFactIndexV1(unpack(bytes),{mapText:indexTextRewriterV1(root,direction),maximumBytes:maximum}));
 }
 return pack(rewrite(unpack(bytes),root,direction));
}

const nameOf=(path:string):string=>path.slice(path.lastIndexOf("/")+1).split(".")[0]!,extensionOf=(path:string):string=>path.slice(path.lastIndexOf("/")+1).slice(64);
const withName=(path:string,name:string):string=>path.slice(0,path.lastIndexOf("/")+1)+name+extensionOf(path);

/** 🧾️ The name of the stored document a path names: the digest between the last slash and the first dot. */
export const documentNameV1=nameOf;

/** 🏷️ Transforms one stored document and names it by the hash of its rewritten bytes. */
export function transformNamedGenerationDocumentV1(document:GenerationDocumentV1,root:string,direction:Direction):GenerationDocumentV1{
 const bytes=transformGenerationDocumentV1(document.path,document.bytes,root,direction);
 return {path:withName(document.path,sha(bytes)),bytes};
}

/** 📜️ Rewrites the manifest of a generation for the other side: the root moves, every document is named by `renamed`, and the manifest is named after its own new bytes. `generation` renumbers it. */
export function transformGenerationManifestV1(manifestBytes:Uint8Array,renamed:ReadonlyMap<string,string>,root:string,direction:Direction,generation?:number):Readonly<{document:GenerationDocumentV1;sourceGeneration:number}>{
 const manifest=JSON.parse(Buffer.from(rewrite(manifestBytes,root,direction)).toString("utf8")) as ReturnType<typeof readGenerationManifestV1>&Record<string,unknown>;
 const map=(name:string|null):string|null=>name===null?null:renamed.get(name)??(()=>{throw Error("Generation document missing: "+name);})();
 const moved={...manifest,...(generation===undefined?{}:{generation}),factIndex:map(manifest.factIndex),frontier:map(manifest.frontier),global:map(manifest.global),owners:Object.fromEntries(Object.entries(manifest.owners).map(([name,row])=>[name,{...row,fragment:map(row.fragment)}]))};
 const bytes=Buffer.from(canonicalJsonV1(moved),"utf8"),path=`${P.manifests}/${sha(bytes)}.json`;
 readGenerationManifestV1(bytes);
 return{document:{path,bytes},sourceGeneration:manifest.generation};
}

/** 🧳️ Rewrites a whole generation between the form of one machine and the neutral form: every document is moved, renamed after the hash of its new bytes, and the manifest is rewritten with the new names and named after its own new bytes. The input holds the manifest and every document it needs (see `generationDocumentPathsV1`). `generation` renumbers the manifest (a restore keeps the numbers of the local store growing). Returns the documents of the new form and the path of its manifest. */
export function transformGenerationV1(documents:readonly GenerationDocumentV1[],root:string,direction:Direction,generation?:number):Readonly<{documents:readonly GenerationDocumentV1[];manifestPath:string;sourceGeneration:number}>{
 const manifestDocument=documents.find(document=>document.path.startsWith(P.manifests+"/"));
 if(!manifestDocument)throw Error("Generation manifest required");
 const renamed=new Map<string,string>(),out:GenerationDocumentV1[]=[];
 for(const document of documents){
  if(document===manifestDocument)continue;
  const moved=transformNamedGenerationDocumentV1(document,root,direction);
  renamed.set(nameOf(document.path),nameOf(moved.path));out.push(moved);
 }
 const{document:manifest,sourceGeneration}=transformGenerationManifestV1(manifestDocument.bytes,renamed,root,direction,generation);
 return Object.freeze({documents:[...out,manifest],manifestPath:manifest.path,sourceGeneration});
}

/** 🧠️ What an earlier neutralization of one document produced: the name (the hash of the neutral bytes) and the size. */
export type NeutralKnownV1=ReadonlyMap<string,Readonly<{name:string;size:number}>>;
/** 📦️ One document of the neutral form: path, hash, size, and the bytes where they were produced now (a remembered document carries none and names its `source`, the path of the stored document it was made from). */
export type NeutralRowV1=Readonly<{path:string;hash:string;size:number;bytes?:Uint8Array;source?:string}>;

/** 🔑️ The key a remembered neutralization stays valid under: the root it replaced, the compression it used, the runtime and the code of this module and of the fact codec; undefined where the code cannot be read, so nothing is remembered. */
export function neutralKeyV1(root:string):string|undefined{
 const hash=createHash("sha256");
 try{for(const file of["./🟦️.ts","../../🧮️fact-codec/🟨️.mjs"])hash.update(readFileSync(fileURLToPath(new URL(file,import.meta.url))));}catch{return undefined;}
 return createHash("sha256").update(JSON.stringify(["semio.graph-generation.neutral.v1",root,P.brotliQuality,P.brotliWindow,process.versions.bun??process.version,hash.digest("hex")])).digest("hex");
}

/** 🧳️ The neutral form of one generation from the documents that are new and the memory of the others: a document whose name is in `known` is not read or rewritten (`bytes` holds only the others and the manifest), the rest are moved as {@link transformGenerationV1} moves them. The rows equal the documents of the full transform; `known` of the result names every document of this generation. */
export function neutralizeGenerationFromMemoryV1(input:Readonly<{paths:readonly string[];bytes:ReadonlyMap<string,Uint8Array>;root:string;known:NeutralKnownV1}>):Readonly<{rows:readonly NeutralRowV1[];manifestPath:string;known:ReadonlyMap<string,Readonly<{name:string;size:number}>>}>{
 const manifestPath=input.paths.find(path=>path.startsWith(P.manifests+"/"));
 if(manifestPath===undefined)throw Error("Generation manifest required");
 const renamed=new Map<string,string>(),rows:NeutralRowV1[]=[],known=new Map<string,Readonly<{name:string;size:number}>>();
 for(const path of input.paths){
  if(path===manifestPath)continue;
  const original=nameOf(path),remembered=input.known.get(original);
  if(remembered!==undefined){renamed.set(original,remembered.name);known.set(original,remembered);rows.push({path:withName(path,remembered.name),hash:remembered.name,size:remembered.size,source:path});continue;}
  const source=input.bytes.get(path);if(source===undefined)throw Error("Generation document unavailable: "+path);
  const bytes=transformGenerationDocumentV1(path,source,input.root,"neutralize"),name=sha(bytes);
  renamed.set(original,name);known.set(original,{name,size:bytes.length});rows.push({path:withName(path,name),hash:name,size:bytes.length,bytes});
 }
 const source=input.bytes.get(manifestPath);if(source===undefined)throw Error("Generation manifest unavailable");
 const{document:manifest}=transformGenerationManifestV1(source,renamed,input.root,"neutralize");
 rows.push({path:manifest.path,hash:sha(manifest.bytes),size:manifest.bytes.length,bytes:manifest.bytes});
 return Object.freeze({rows,manifestPath:manifest.path,known});
}

/** 🏷️ The root independent signature of the state a manifest describes: identity, implementation, environment, membership and the edge environment of every owner. Two machines at the same state have the same signature although every stamp in their documents differs. */
export function generationSignatureV1(manifest:ReturnType<typeof readGenerationManifestV1>):string{
 return sha(Buffer.from(JSON.stringify(["semio.graph-generation.signature.v1",manifest.identity,manifest.implementation,manifest.env,manifest.membership,Object.entries(manifest.owners).map(([name,row])=>[name,row.edgesEnv,row.kind]).sort((a,b)=>a[0]!<b[0]!?-1:1)])));
}
