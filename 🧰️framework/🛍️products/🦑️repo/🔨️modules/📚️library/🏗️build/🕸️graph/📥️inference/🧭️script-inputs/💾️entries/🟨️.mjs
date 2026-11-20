import{brotliCompressSync,brotliDecompressSync,constants}from"node:zlib";
import{createHash}from"node:crypto";
import{mkdir,readFile,readdir,rename,rm,stat,utimes,writeFile}from"node:fs/promises";
import{existsSync,readFileSync}from"node:fs";
import{join}from"node:path";
import{collectClosureStoreV1}from"../🧹️retention/🟨️.mjs";
import{createBoundedCacheV1}from"../../../../🧊️bounded/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";
import{observationRowJsonV1,shareObservationRowsV1}from"../../../../👁️observation/🟨️.mjs";
import{graphStringInternerV1,internJsonStringsV1}from"../../../../🧠️memory/🟨️.mjs";
import{createLayerIndexV1}from"../🧾️layer-index/🟨️.mjs";

const sha=text=>createHash("sha256").update(text).digest("hex"),EXTENSION=".json.br",MAXIMUM_RECORD_BYTES=268435456;

/**
 * 💾️ Creates the durable store of closure entries: one memo node per recipe (`e/<sha256 of the recipe id>`: recipe, digest of its value, observed rows, digest of the observed inputs) whose shared row layers (`l/<sha256 of the layer>`) and whose value (`v/<sha256 of the value>`, the file list of a closure) are content addressed nodes shared by every entry that names them: thousands of targets that run the same script name one value, which is stored and parsed once; layers are streamed from disk when needed, never cached parsed. A damaged, missing or foreign record answers `undefined`, so a damaged store only costs a derivation; whether a loaded entry still holds is decided by the caller with the row-hold check.
 * @see ../../🧬️schema/🔣️.json
 */
export function createClosureStoreV1({directory,limits=INFERENCE_LIMITS_V1,readOnly=false,write=writeFile}){
 const entries=join(directory,"e"),layers=join(directory,"l"),values=join(directory,"v"),entryPath=id=>join(entries,sha(id)+EXTENSION),layerPath=digest=>join(layers,digest+EXTENSION),valuePath=digest=>join(values,digest+EXTENSION);
 const listText=rows=>"["+rows.map(observationRowJsonV1).join(",")+"]",listDigest=rows=>{const hash=createHash("sha256");hash.update("[");for(let at=0;at<rows.length;at++){if(at>0)hash.update(",");hash.update(observationRowJsonV1(rows[at]));}hash.update("]");return hash.digest("hex");};
 const written=createBoundedCacheV1({maxEntries:65536,maxBytes:65536}),valueItems=value=>Array.isArray(value)?value.length:Object.values(value??{}).reduce((sum,part)=>sum+(Array.isArray(part)?part.length:1),1),parsedValues=createBoundedCacheV1({name:"inference:closure-values",maxEntries:limits.closureValueCacheEntries,maxBytes:limits.closureValueCacheBytes,sizeOf:value=>limits.closureEntryValueItemBytes*valueItems(value)}),valueImages=new WeakMap();
 const pack=text=>brotliCompressSync(Buffer.from(text,"utf8"),{params:{[constants.BROTLI_PARAM_QUALITY]:limits.closureStoreCompressionQuality}}),unpack=bytes=>brotliDecompressSync(bytes,{maxOutputLength:MAXIMUM_RECORD_BYTES}).toString("utf8");
 const attempt=async(path,text)=>{const temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";try{await write(temporary,pack(text));await rename(temporary,path);}catch(error){await rm(temporary,{force:true});throw error;}};
 let relief;const relieve=()=>relief??=collectClosureStoreV1({directory,graceMilliseconds:limits.closureStoreEmergencyGraceMilliseconds}).finally(()=>{relief=undefined;});
 const place=async(path,text)=>{try{await attempt(path,text);}catch(error){if(error?.code!=="ENOSPC")throw error;await relieve();await attempt(path,text);}};
 let pruned;const images=new WeakMap(),questions=new Map(),intern=text=>{const known=questions.get(text);if(known!==undefined)return known;questions.set(text,text);return text;};
 const sweep=async(directoryPath,maximum)=>{
  const names=(await readdir(directoryPath).catch(()=>[])).filter(name=>name.endsWith(EXTENSION));
  if(names.length<=maximum)return;
  const dated=await Promise.all(names.map(async name=>({path:join(directoryPath,name),time:(await stat(join(directoryPath,name)).catch(()=>({mtimeMs:0}))).mtimeMs})));
  const youngest=Date.now()-limits.closureStoreGraceMilliseconds;
  for(const file of dated.sort((a,b)=>a.time-b.time).slice(0,dated.length-maximum))if(file.time<youngest)await rm(file.path,{force:true});
 };
 const sharedFacts=createBoundedCacheV1({name:"inference:closure-sidecar-facts",maxEntries:limits.closureStoreMaximumLayers,maxBytes:limits.closureSidecarCacheBytes,sizeOf:()=>limits.closureSidecarFactBytes}),sharedMembers=createBoundedCacheV1({name:"inference:closure-sidecar-members",maxEntries:limits.closureStoreMaximumLayers,maxBytes:limits.closureSidecarCacheBytes,sizeOf:list=>limits.closureSidecarWalkBytes*(list.length+8)});
 const sameList=(left,right)=>left.length===right.length&&left.every((item,at)=>item===right[at]);
 const memberSets=new WeakMap(),unionMembers=(digest,mapped)=>{const held=sharedMembers.get(digest);if(held===undefined){sharedMembers.set(digest,mapped);return mapped;}if(held===mapped||sameList(held,mapped))return held;let have=memberSets.get(held);if(have===undefined||have.size!==held.length){have=new Set(held);memberSets.set(held,have);}const extra=mapped.filter(walk=>!have.has(walk));if(extra.length===0)return held;held.push(...extra);for(const walk of extra)have.add(walk);return held;};
 const packSidecar=sidecar=>{const walks=[],index=new Map(),at=walk=>{let found=index.get(walk);if(found===undefined){found=walks.length;walks.push(walk);index.set(walk,found);}return found;};return{...sidecar,members:sidecar.members?.map(list=>list&&list.map(at)),walks};};
 const unpackSidecar=({walks=[],...sidecar},digests)=>{
  const table=walks.map(intern);
  const facts=sidecar.facts?.map((fact,at)=>{if(!fact)return fact;const text=JSON.stringify(fact),known=sharedFacts.get(digests[at]);if(known!==undefined&&known.text===text)return known.fact;sharedFacts.set(digests[at],{text,fact});return fact;});
  const members=sidecar.members?.map((list,at)=>{if(!list)return list;return unionMembers(digests[at],list.map(index=>table[index]));});
  return{...sidecar,facts,members};
 };
 const layerIndex=createLayerIndexV1({directory,limits});
 const layerBytesSync=digest=>{
  try{const bytes=brotliDecompressSync(readFileSync(layerPath(digest)),{maxOutputLength:MAXIMUM_RECORD_BYTES});if(bytes.length<2||bytes[0]!==91||bytes[bytes.length-1]!==93||createHash("sha256").update(bytes).digest("hex")!==digest)return undefined;return bytes;}catch{return undefined;}
 };
 const valueSync=digest=>{
  const known=parsedValues.get(digest);if(known!==undefined)return known;
  try{const text=unpack(readFileSync(valuePath(digest)));if(sha(text)!==digest)return undefined;const value=internJsonStringsV1(JSON.parse(text),graphStringInternerV1);parsedValues.set(digest,value);if(value!==null&&typeof value==="object")valueImages.set(value,{digest});return value;}catch{return undefined;}
 };
 const trusted=(digest,path)=>{const at=written.get(digest);return typeof at==="number"&&Date.now()-at<limits.closureStoreGraceMilliseconds/4&&existsSync(path);};
 const reuse=async path=>{const info=await stat(path).catch(()=>undefined);if(info&&Date.now()-info.mtimeMs>limits.closureStoreGraceMilliseconds/4){const now=new Date();await utimes(path,now,now).catch(()=>undefined);}return info!==undefined;};
 const keep=async(digest,text)=>{
  if(trusted(digest,layerPath(digest)))return;
  if(!await reuse(layerPath(digest)))await place(layerPath(digest),text());
  written.set(digest,Date.now());
 };
 const collect=async()=>{
  const marker=join(directory,"collected"),info=await stat(marker).catch(()=>undefined);
  if(info&&Date.now()-info.mtimeMs<limits.closureStoreCollectIntervalMilliseconds)return;
  await writeFile(marker,"");
  await collectClosureStoreV1({directory,graceMilliseconds:limits.closureStoreGraceMilliseconds});
  layerIndex.sweepSync(layers);
 };
 return Object.freeze({
  /** 💾️ Stores the entry of `id`; answers the size of its own record in characters, without the shared layers (what the in-process cache charges for it). */
  async save(id,{recipe,value,rows,layers:shared,sidecar=null}){
   if(readOnly)return{bytes:JSON.stringify([value,rows]).length+8*shared.length,digests:undefined};
   await Promise.all([mkdir(entries,{recursive:true}),mkdir(layers,{recursive:true}),mkdir(values,{recursive:true})]);
   const digests=[];
   shared.forEach(layer=>{if(!images.has(layer))images.set(layer,{digest:listDigest(layer)});});
   const pending=[];
   shared.forEach(layer=>{const image=images.get(layer);digests.push(image.digest);if(!trusted(image.digest,layerPath(image.digest)))pending.push(keep(image.digest,()=>listText(layer)));});
   await Promise.all(pending);
   let image=value!==null&&typeof value==="object"?valueImages.get(value):undefined,valueText;
   if(!image){valueText=JSON.stringify(value);image={digest:sha(valueText)};if(value!==null&&typeof value==="object")valueImages.set(value,image);}
   if(!trusted(image.digest,valuePath(image.digest))){if(!await reuse(valuePath(image.digest)))await place(valuePath(image.digest),valueText??JSON.stringify(value));written.set(image.digest,Date.now());}
   const rowsText=listText(rows),text='{"version":3,"id":'+JSON.stringify(id)+',"digest":"'+sha(rowsText+digests.join(",")+image.digest)+'","recipe":'+JSON.stringify(recipe)+',"valueDigest":"'+image.digest+'","rows":'+rowsText+',"layers":'+JSON.stringify(digests)+',"sidecar":'+JSON.stringify(sidecar&&packSidecar(sidecar))+"}";
   await place(entryPath(id),text);
   pruned??=collect().catch(()=>undefined).then(()=>Promise.all([sweep(entries,limits.closureStoreMaximumEntries),sweep(layers,limits.closureStoreMaximumLayers),sweep(values,limits.closureStoreMaximumValues)]));
   return{bytes:text.length+8*digests.length,digests};
  },
  /** 📖️ The stored entry of `id` (recipe, value, rows, shared layers, size), or undefined while it is absent, damaged or of another format; a layer it names that is lost is found when the layer is read, which every verification of the entry does, so loading never probes the layers one by one. */
  async load(id){
   try{
    const bytes=await readFile(entryPath(id)).catch(()=>undefined);if(!bytes)return undefined;
    const text=unpack(bytes),record=JSON.parse(text);
    if(record?.version!==3||record.id!==id||typeof record.valueDigest!=="string"||!Array.isArray(record.rows)||!Array.isArray(record.layers))return undefined;
    if(sha(JSON.stringify(record.rows)+record.layers.join(",")+record.valueDigest)!==record.digest)return undefined;
    const value=valueSync(record.valueDigest);if(value===undefined)return undefined;
    const digests=record.layers.map(intern);return{recipe:record.recipe,digest:record.digest,value,rows:shareObservationRowsV1(record.rows),digests,sidecar:record.sidecar?unpackSidecar(record.sidecar,digests):null,bytes:text.length+8*record.layers.length};
   }catch{return undefined;}
  },
  /** 🌊️ The rows of the shared layer `digest` in batches of `size`, read, verified and parsed once per call by the native parser (a byte scan in JavaScript to cut batches costs several times more than parsing the whole layer) and held by nobody afterwards; throws while the layer is missing or damaged, so a caller never mistakes a lost layer for an empty one. */
  *layerBatchesSync(digest,size){
   const bytes=layerBytesSync(digest);if(bytes===undefined)throw Object.assign(Error("Intact closure layer required: "+digest),{code:"CLOSURE_LAYER_LOST"});
   const rows=JSON.parse(bytes.toString("utf8"));
   layerIndex.indexSync(digest,rows);
   for(let at=0;at<rows.length;at+=size)yield rows.slice(at,at+size);
  },
  /** 🪿️ The digests among `digests` whose layer file the store no longer has. */
  lostLayersSync(digests){return digests.filter(digest=>!existsSync(layerPath(digest)));},
  /** 🪪️ The dictionary ids of the rows of the stored layer `digest` in layer order (the layer is decided by the verdicts of its distinct rows, never parsed), or undefined while the layer is lost or no process has parsed it yet; a layer parsed by `layerBatchesSync` is listed for every later reader. */
  layerListingSync(digest){return existsSync(layerPath(digest))?layerIndex.listingSync(digest):undefined;},
  /** 📊️ How many layers this process listed and read and how many rows it read from the dictionary. */
  layerIndexStats(){return layerIndex.stats();},
  /** 🌊️ The layers named by `digests` one after the other, each read, verified and parsed when it is asked for and held by nobody afterwards, so an entry of thousands of layers travels one layer at a time; throws while a layer is missing or damaged. */
  *layersSync(digests){
   for(const digest of digests){const bytes=layerBytesSync(digest);if(bytes===undefined)throw Object.assign(Error("Intact closure layer required: "+digest),{code:"CLOSURE_LAYER_LOST"});yield shareObservationRowsV1(JSON.parse(bytes.toString("utf8")));}
  },
  /** 🔖️ The size and modification time of the stored entry of `id` (what changes when it is written again), or undefined while it is absent: the key under which a caller may remember the bundle of an unchanged entry. */
  async stamp(id){const info=await stat(entryPath(id)).catch(()=>undefined);return info===undefined?undefined:`${info.size}:${Math.floor(info.mtimeMs)}`;},
  /** 📦️ The entry of `id` as one portable object (recipe, value, rows, every shared layer inline, sidecar), or undefined while the entry or one of its layers is absent or damaged: what travels with a project record through the fleet. */
  async bundle(id){
   const stored=await this.load(id);if(stored===undefined)return undefined;
   const layers=[];
   for(const digest of stored.digests){const bytes=layerBytesSync(digest);if(bytes===undefined)return undefined;layers.push(shareObservationRowsV1(JSON.parse(bytes.toString("utf8"))));}
   return{version:1,recipe:stored.recipe,value:stored.value,rows:stored.rows,layers,sidecar:stored.sidecar};
  },
  /** 📥️ Stores a bundle of another machine as the entry of `id`; the digests are computed here from the rows, and every consumer checks the rows against its own tree before it uses the entry, so a wrong bundle costs a derivation and never a wrong graph. Refuses anything that is not a closed bundle. */
  async adopt(id,bundle){
   if(bundle===null||typeof bundle!=="object"||bundle.version!==1||bundle.recipe===null||typeof bundle.recipe!=="object"||!Array.isArray(bundle.rows)||!Array.isArray(bundle.layers)||!bundle.layers.every(Array.isArray))throw Error("Closed closure bundle required");
   return this.save(id,{recipe:bundle.recipe,value:bundle.value,rows:bundle.rows,layers:bundle.layers,sidecar:bundle.sidecar??null});
  },
  /** 🧷️ Remembers that the entry stored under the recipe key `key` of this store was taken from the fleet document of digest `source`, so a later record that names the same document does not download it again. */
  async remember(source,key){await mkdir(join(directory,"travel"),{recursive:true});await write(join(directory,"travel",source),key);},
  /** 🧷️ The recipe key of the entry that was taken from the fleet document of digest `source`, or undefined. */
  async travelled(source){try{return(await readFile(join(directory,"travel",source),"utf8")).trim();}catch{return undefined;}},
  /** 🧷️ Remembers that the entry stored under the recipe key `key` is the local form of the fleet document named `neutral` (the digest of its recipe with the root token), whatever bytes that document had when it was taken: the shared entries an entry names travel inside every entry that names them, each time with other bytes. */
  async rememberNeutral(neutral,key){await mkdir(join(directory,"travel"),{recursive:true});await write(join(directory,"travel","n-"+neutral),key);},
  /** 🧷️ The recipe key of the entry that was taken from the fleet document named `neutral` (or that the document was refused for, because it does not hold here) and how long ago, or undefined. */
  async travelledNeutral(neutral){try{const path=join(directory,"travel","n-"+neutral),[text,info]=await Promise.all([readFile(path,"utf8"),stat(path)]);return{key:text.trim(),ageMilliseconds:Date.now()-info.mtimeMs};}catch{return undefined;}},
  /** 🔒️ Claims the intake of the fleet document named `name` for the process that asks: the answer is the release of the claim, or undefined while another process holds a claim younger than `staleMilliseconds` (a claim of a process that died is taken over after that time); a store that cannot hold claims answers a release that does nothing, so the caller goes on and takes the document itself. */
  async claim(name,staleMilliseconds){
   const parent=join(directory,"travel"),path=join(parent,"c-"+name);
   try{await mkdir(parent,{recursive:true});}catch{return async()=>undefined;}
   for(let attempt=0;attempt<2;attempt++){
    try{await mkdir(path);return async()=>{await rm(path,{recursive:true,force:true});};}
    catch(error){
     if(error?.code!=="EEXIST")return async()=>undefined;
     const info=await stat(path).catch(()=>undefined);
     if(info!==undefined&&Date.now()-info.mtimeMs<staleMilliseconds)return undefined;
     await rm(path,{recursive:true,force:true});
    }
   }
   return undefined;
  },
  /** 🔒️ Whether a claim of the intake of the fleet document named `name` exists. */
  async claimed(name){return(await stat(join(directory,"travel","c-"+name)).catch(()=>undefined))!==undefined;},
  /** 🧹️ Waits for the removal of the oldest records beyond the declared maxima that the first save of this process started. */
  async settled(){await pruned;},
 });
}
