import{createHash}from"node:crypto";
import{gzip,gunzip}from"node:zlib";
import{promisify}from"node:util";

const gzipAsync=promisify(gzip),gunzipAsync=promisify(gunzip),sha256=bytes=>createHash("sha256").update(bytes).digest("hex"),HEX=/^[a-f0-9]{64}$/u;
const yieldTurn=()=>new Promise(resolve=>setImmediate(resolve));
const fail=message=>Error(message);

/** 🧾️ The closed manifest of a pinned build graph: the graph without its three large members, and the digests of the content-addressed records that hold them. */
const MANIFEST_KEYS=["version","graph","projects","dependencies","externalNodes"];

/** 📇️ Encodes a build graph as content-addressed gzip records (one per project, one per chunk of dependencies and of external nodes) plus a small manifest; no step ever holds the graph as one string. `bytes` counts every compressed byte as it is produced and refuses a graph beyond `maximumBytes`. */
export async function encodeGraphRecordsV1(graph,{policy,maximumBytes}){
 if(!graph||typeof graph!=="object"||Array.isArray(graph)||!graph.projects||typeof graph.projects!=="object"||!Array.isArray(graph.dependencies))throw fail("Closed build graph required");
 const records=new Map();let bytes=0,work=0;
 const store=async text=>{
  const raw=Buffer.from(text);if(raw.length>policy.maximumRecordBytes)throw fail("Graph record exceeds its finite size");
  const packed=await gzipAsync(raw,{level:policy.compressionLevel}),digest=sha256(packed);
  if(!records.has(digest)){bytes+=packed.length;if(bytes>maximumBytes)throw fail("Build graph exceeds finite snapshot byte limit");records.set(digest,packed);}
  if(++work%policy.projectsPerYield===0)await yieldTurn();
  return digest;
 };
 const{projects,dependencies,externalNodes,...rest}=graph,names=Object.keys(projects),listed=[];
 for(const name of names)listed.push([name,await store(JSON.stringify(projects[name]))]);
 const edges=[];for(let at=0;at<dependencies.length;at+=policy.edgesPerRecord)edges.push(await store(JSON.stringify(dependencies.slice(at,at+policy.edgesPerRecord))));
 let external;
 if(externalNodes!==undefined){const keys=Object.keys(externalNodes);external=[];for(let at=0;at<keys.length;at+=policy.externalNodesPerRecord)external.push(await store(JSON.stringify(Object.fromEntries(keys.slice(at,at+policy.externalNodesPerRecord).map(key=>[key,externalNodes[key]])))));}
 const manifest={version:policy.version,graph:rest,projects:listed,dependencies:edges,...(external?{externalNodes:external}:{})},text=JSON.stringify(manifest),manifestBytes=Buffer.from(text);
 return Object.freeze({manifestBytes,digest:sha256(manifestBytes),records,recordBytes:bytes,bytes:bytes+manifestBytes.length});
}

/** 🧷️ Collapses long identical string arrays (the expanded input lists many projects share) to one retained array, so a decoded graph costs what the encoded one cost in memory. */
export function createArrayInternerV1(minimumItems){
 const buckets=new Map();
 const same=(left,right)=>{for(let index=0;index<left.length;index++)if(left[index]!==right[index])return false;return true;};
 const visit=value=>{
  if(value===null||typeof value!=="object")return value;
  if(Array.isArray(value)){
   if(value.length>=minimumItems&&value.every(item=>typeof item==="string")){
    const key=value.length+"|"+value[0]+"|"+value[value.length-1]+"|"+value[value.length>>1],list=buckets.get(key)??[];
    for(const known of list)if(same(known,value))return known;
    list.push(value);buckets.set(key,list);return value;
   }
   for(let index=0;index<value.length;index++)value[index]=visit(value[index]);return value;
  }
  for(const key of Object.keys(value))value[key]=visit(value[key]);return value;
 };
 return visit;
}

/** 📖️ Decodes a pinned graph from its manifest through `readRecord(digest)` (compressed bytes); every record is verified against its name before it is parsed and interned. */
export async function decodeGraphRecordsV1(manifestBytes,readRecord,{policy}){
 const manifest=JSON.parse(Buffer.from(manifestBytes).toString("utf8"));
 if(!manifest||typeof manifest!=="object"||Array.isArray(manifest)||Object.keys(manifest).some(key=>!MANIFEST_KEYS.includes(key))||manifest.version!==policy.version||!Array.isArray(manifest.projects)||!Array.isArray(manifest.dependencies)||(manifest.externalNodes!==undefined&&!Array.isArray(manifest.externalNodes))||!manifest.graph||typeof manifest.graph!=="object")throw fail("Closed graph record manifest required");
 const intern=createArrayInternerV1(policy.internMinimumItems);let work=0;
 const load=async digest=>{
  if(typeof digest!=="string"||!HEX.test(digest))throw fail("Graph record digest required");
  const packed=await readRecord(digest);if(!packed||sha256(packed)!==digest)throw fail("Graph record content differs from its name");
  const text=(await gunzipAsync(packed,{maxOutputLength:policy.maximumRecordBytes})).toString("utf8");
  if(++work%policy.projectsPerYield===0)await yieldTurn();
  return JSON.parse(text);
 };
 const projects={};for(const row of manifest.projects){if(!Array.isArray(row)||row.length!==2||typeof row[0]!=="string")throw fail("Graph record project row required");projects[row[0]]=intern(await load(row[1]));}
 const dependencies=[];for(const digest of manifest.dependencies)for(const edge of await load(digest))dependencies.push(edge);
 let externalNodes;if(manifest.externalNodes!==undefined){externalNodes={};for(const digest of manifest.externalNodes)Object.assign(externalNodes,await load(digest));}
 return{...manifest.graph,projects,dependencies,...(externalNodes?{externalNodes}:{})};
}

/** 🔗️ Names the record digests one manifest holds. */
export function graphRecordDigestsV1(manifestBytes){
 const manifest=JSON.parse(Buffer.from(manifestBytes).toString("utf8"));
 return new Set([...manifest.projects.map(row=>row[1]),...manifest.dependencies,...(manifest.externalNodes??[])]);
}
