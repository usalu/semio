import{brotliCompressSync,brotliDecompressSync,constants}from"node:zlib";
import{createHash}from"node:crypto";
import{mkdir,readFile,readdir,rename,rm,stat,writeFile}from"node:fs/promises";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{createBoundedCacheV1}from"../../../../🧊️bounded/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";
import{shareObservationRowsV1}from"../../../../👁️observation/🟨️.mjs";

const sha=text=>createHash("sha256").update(text).digest("hex"),EXTENSION=".json.br",MAXIMUM_RECORD_BYTES=268435456;

/**
 * 💾️ Creates the durable store of closure entries: one memo node per recipe (`e/<sha256 of the recipe id>`: recipe, value, observed rows, digest of the observed inputs) whose shared row layers are content addressed nodes (`l/<sha256 of the layer>`) loaded once and shared by every entry that names them. A damaged, missing or foreign record answers `undefined`, so a damaged store only costs a derivation; whether a loaded entry still holds is decided by the caller with the row-hold check.
 * @see ../../🧬️schema/🔣️.json
 */
export function createClosureStoreV1({directory,limits=INFERENCE_LIMITS_V1,readOnly=false}){
 const entries=join(directory,"e"),layers=join(directory,"l"),entryPath=id=>join(entries,sha(id)+EXTENSION),layerPath=digest=>join(layers,digest+EXTENSION);
 const loaded=createBoundedCacheV1({name:"inference:closure-layers:"+directory,maxEntries:65536,maxBytes:limits.closureLayerCacheBytes,sizeOf:layer=>64+limits.closureLayerRowBytes*layer.rows.length}),written=createBoundedCacheV1({maxEntries:65536,maxBytes:65536});
 const pack=text=>brotliCompressSync(Buffer.from(text,"utf8"),{params:{[constants.BROTLI_PARAM_QUALITY]:limits.closureStoreCompressionQuality}}),unpack=bytes=>brotliDecompressSync(bytes,{maxOutputLength:MAXIMUM_RECORD_BYTES}).toString("utf8");
 const place=async(path,text)=>{const temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";await writeFile(temporary,pack(text));await rename(temporary,path);};
 let pruned;const images=new WeakMap(),questions=new Map(),intern=text=>{const known=questions.get(text);if(known!==undefined)return known;questions.set(text,text);return text;};
 const sweep=async(directoryPath,maximum)=>{
  const names=(await readdir(directoryPath).catch(()=>[])).filter(name=>name.endsWith(EXTENSION));
  if(names.length<=maximum)return;
  const dated=await Promise.all(names.map(async name=>({path:join(directoryPath,name),time:(await stat(join(directoryPath,name)).catch(()=>({mtimeMs:0}))).mtimeMs})));
  for(const file of dated.sort((a,b)=>a.time-b.time).slice(0,dated.length-maximum))await rm(file.path,{force:true});
 };
 const layerSync=digest=>{
  const known=loaded.get(digest);if(known)return known.rows;
  try{const text=unpack(readFileSync(layerPath(digest))),rows=JSON.parse(text);if(!Array.isArray(rows)||sha(text)!==digest)return undefined;shareObservationRowsV1(rows);loaded.set(digest,{rows,bytes:text.length});return rows;}catch{return undefined;}
 };
 const layerPresent=async digest=>written.has(digest)||loaded.has(digest)||await stat(layerPath(digest)).then(()=>true,()=>false);
 const keep=async(digest,text)=>{
  if(written.has(digest))return;
  if(!await stat(layerPath(digest)).then(()=>true,()=>false))await place(layerPath(digest),text);
  written.set(digest,true);
 };
 return Object.freeze({
  /** 💾️ Stores the entry of `id`; answers the size of its own record in characters, without the shared layers the layer cache charges (what the in-process cache charges for it). */
  async save(id,{recipe,value,rows,layers:shared,sidecar=null}){
   if(readOnly)return{bytes:JSON.stringify([value,rows]).length+8*shared.length,digests:undefined};
   await Promise.all([mkdir(entries,{recursive:true}),mkdir(layers,{recursive:true})]);
   const digests=[];
   const known=shared.map(layer=>images.get(layer)),texts=shared.map((layer,at)=>known[at]?undefined:JSON.stringify(layer));
   shared.forEach((layer,at)=>{if(!known[at])images.set(layer,{digest:sha(texts[at]),bytes:texts[at].length});});
   const pending=[];
   shared.forEach((layer,at)=>{const image=known[at]??images.get(layer);digests.push(image.digest);if(!written.has(image.digest))pending.push(keep(image.digest,texts[at]??JSON.stringify(layer)));});
   await Promise.all(pending);
   const rowsText=JSON.stringify(rows),text=JSON.stringify({version:1,id,digest:sha(rowsText+digests.join(",")),recipe,value,rows,layers:digests,sidecar});
   await place(entryPath(id),text);
   pruned??=Promise.all([sweep(entries,limits.closureStoreMaximumEntries),sweep(layers,limits.closureStoreMaximumLayers)]);
   return{bytes:text.length+8*digests.length,digests};
  },
  /** 📖️ The stored entry of `id` (recipe, value, rows, shared layers, size), or undefined while it is absent, damaged or names a missing layer. */
  async load(id){
   try{
    const bytes=await readFile(entryPath(id)).catch(()=>undefined);if(!bytes)return undefined;
    const text=unpack(bytes),record=JSON.parse(text);
    if(record?.version!==1||record.id!==id||!Array.isArray(record.rows)||!Array.isArray(record.layers))return undefined;
    const found=await Promise.all(record.layers.map(layerPresent));
    if(found.includes(false)||sha(JSON.stringify(record.rows)+record.layers.join(","))!==record.digest)return undefined;
    return{recipe:record.recipe,value:record.value,rows:shareObservationRowsV1(record.rows),digests:record.layers,sidecar:record.sidecar?{...record.sidecar,members:record.sidecar.members?.map(walks=>walks&&walks.map(intern))}:null,bytes:text.length+8*record.layers.length};
   }catch{return undefined;}
  },
  /** 🧱️ The shared row layers named by `digests`, read through the bounded layer cache and held by nobody else; undefined while one is missing or damaged. */
  layers(digests){const found=[];for(const digest of digests){const rows=layerSync(digest);if(!rows)return undefined;found.push(rows);}return found;},
  /** 🧹️ Waits for the removal of the oldest records beyond the declared maxima that the first save of this process started. */
  async settled(){await pruned;},
 });
}
