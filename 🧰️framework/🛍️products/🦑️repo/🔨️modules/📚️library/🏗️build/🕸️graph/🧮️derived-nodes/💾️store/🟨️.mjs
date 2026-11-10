import{brotliCompressSync as brotliCompress,brotliDecompressSync,constants as zlibConstants}from"node:zlib";
import{mkdir,readFile,readdir,rename,rm,stat,writeFile}from"node:fs/promises";
import{join}from"node:path";
import{sha256V1,DERIVED_NODES_POLICY_V1}from"../🟨️.mjs";

/** 📁 The node file port of the durable stores. */
export const NODE_FILE_PORT_V1=Object.freeze({read:path=>readFile(path).catch(()=>undefined),write:async(path,bytes)=>{const temp=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";await writeFile(temp,bytes);await rename(temp,path);},list:async directory=>{const names=await readdir(directory).catch(()=>[]);return Promise.all(names.filter(name=>name.endsWith(".br")).map(async name=>({name,path:join(directory,name),mtimeMs:(await stat(join(directory,name)).catch(()=>({mtimeMs:0}))).mtimeMs})));},remove:path=>rm(path,{force:true}),mkdir:directory=>mkdir(directory,{recursive:true})});

/**
 * 💾️ The durable node store (a record above `maximumRecordBytes` is kept as references: the value and the raw reads, with glob members, are content addressed brotli chunks of `chunkEntries` entries in `blobs`, a host CAS `{put(bytes)->{hash}, read(hash), pin?(holder,hashes,ttl)}`; a missing chunk makes the record absent, so a damaged store only costs a recompute): one content addressed brotli record per node (`<root>/n/<sha256 of the node id>.json.br`: key, value, node edges, raw reads, params), written atomically, never trusted without revalidation. Bounded: records beyond `maximumRecords` or older than `retainMilliseconds` are removed by `prune`, the oldest first.
 * @see ../🧬️schema/🔣️.json
 */
export function createNodeStoreV1({root,fs=NODE_FILE_PORT_V1,blobs,limits=DERIVED_NODES_POLICY_V1.store}){
 const brotliCompressSync=bytes=>brotliCompress(bytes,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:limits.compressionQuality}});
 const directory=join(root,limits.directory),path=id=>join(directory,sha256V1(id)+".json.br"),writes=new Set();
 return Object.freeze({
  async get(id){
   try{
    const bytes=await fs.read(path(id));if(!bytes)return undefined;
    const record=JSON.parse(brotliDecompressSync(bytes,{maxOutputLength:limits.maximumRecordBytes}).toString("utf8"));
    if(record?.version===1)return Array.isArray(record.deps)&&Array.isArray(record.rawReads)?record:undefined;
    if(record?.version!==2||!blobs)return undefined;
    const chunk=async hash=>{const stored=await blobs.read(hash);if(!stored)throw Error("chunk");return JSON.parse(brotliDecompressSync(stored,{maxOutputLength:limits.maximumChunkBytes}).toString("utf8"));};
    const rawReads=[],globs=[];
    for(const hash of record.readsRefs)for(const entry of await chunk(hash)){
     if(entry.kind==="members"){const ref=globs[entry.glob];for(let at=0;at<entry.paths.length;at++){ref.paths.push(entry.paths[at]);ref.digests.push(entry.digests[at]);}}
     else{rawReads.push(entry);if(entry.kind==="glob"){entry.paths=[];entry.digests=[];globs.push(entry);}}
    }
    const{valueRef,readsRefs,...rest}=record;
    return{...rest,version:1,value:valueRef===undefined?undefined:await chunk(valueRef),rawReads};
   }catch{return undefined;}
  },
  put(node,params){
   const work=(async()=>{
    const text=JSON.stringify({version:1,...node,params}),identity=node.key.kind+"\0"+node.key.subject+"\0"+node.key.params;
    await fs.mkdir(directory);
    if(Buffer.byteLength(text)<=limits.maximumRecordBytes){await fs.write(path(identity),brotliCompressSync(Buffer.from(text,"utf8")));return;}
    if(!blobs)return;
    const hashes=[],readsRefs=[],keep=async value=>{const stored=await blobs.put(brotliCompressSync(Buffer.from(JSON.stringify(value),"utf8")));hashes.push(stored.hash);return stored.hash;};
    let batch=[],ordinal=0;
    const flush=async()=>{if(batch.length){readsRefs.push(await keep(batch));batch=[];}};
    for(const ref of node.rawReads){
     if(ref.kind!=="glob"){batch.push(ref);if(batch.length>=limits.chunkEntries)await flush();continue;}
     const{paths,digests,...meta}=ref,at=ordinal++;
     batch.push(meta);
     for(let from=0;from<paths.length;from+=limits.chunkEntries){batch.push({kind:"members",glob:at,paths:paths.slice(from,from+limits.chunkEntries),digests:digests.slice(from,from+limits.chunkEntries)});await flush();}
    }
    await flush();
    const valueRef=node.value===undefined?undefined:await keep(node.value),{value,rawReads,...rest}=node;
    await fs.write(path(identity),brotliCompressSync(Buffer.from(JSON.stringify({version:2,...rest,params,valueRef,readsRefs}),"utf8")));
    await blobs.pin?.("derived-nodes."+sha256V1(identity),hashes,limits.retainMilliseconds);
   })().finally(()=>writes.delete(work));
   writes.add(work);return work;
  },
  async flush(){await Promise.allSettled([...writes]);},
  async prune({now=Date.now()}={}){
   const files=(await fs.list(directory)).sort((a,b)=>a.mtimeMs-b.mtimeMs);
   const expired=files.filter(file=>now-file.mtimeMs>limits.retainMilliseconds),kept=files.filter(file=>!expired.includes(file)),over=kept.slice(0,Math.max(0,kept.length-limits.maximumRecords));
   for(const file of[...expired,...over])await fs.remove(file.path);
   return{removed:expired.length+over.length,kept:files.length-expired.length-over.length};
  },
 });
}
