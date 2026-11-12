import{brotliCompressSync,brotliDecompressSync,constants}from"node:zlib";
import{createHash}from"node:crypto";
import{mkdir,readFile,readdir,rename,rm,stat,writeFile}from"node:fs/promises";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{createBoundedCacheV1}from"../../../../🧊️bounded/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";
import{observationRowJsonV1,shareObservationRowsV1}from"../../../../👁️observation/🟨️.mjs";
import{graphStringInternerV1,internJsonStringsV1}from"../../../../🧠️memory/🟨️.mjs";

const sha=text=>createHash("sha256").update(text).digest("hex"),EXTENSION=".json.br",MAXIMUM_RECORD_BYTES=268435456;

/**
 * 💾️ Creates the durable store of closure entries: one memo node per recipe (`e/<sha256 of the recipe id>`: recipe, digest of its value, observed rows, digest of the observed inputs) whose shared row layers (`l/<sha256 of the layer>`) and whose value (`v/<sha256 of the value>`, the file list of a closure) are content addressed nodes shared by every entry that names them: thousands of targets that run the same script name one value, which is stored and parsed once; layers are streamed from disk when needed, never cached parsed. A damaged, missing or foreign record answers `undefined`, so a damaged store only costs a derivation; whether a loaded entry still holds is decided by the caller with the row-hold check.
 * @see ../../🧬️schema/🔣️.json
 */
export function createClosureStoreV1({directory,limits=INFERENCE_LIMITS_V1,readOnly=false}){
 const entries=join(directory,"e"),layers=join(directory,"l"),values=join(directory,"v"),entryPath=id=>join(entries,sha(id)+EXTENSION),layerPath=digest=>join(layers,digest+EXTENSION),valuePath=digest=>join(values,digest+EXTENSION);
 const listText=rows=>"["+rows.map(observationRowJsonV1).join(",")+"]",listDigest=rows=>{const hash=createHash("sha256");hash.update("[");for(let at=0;at<rows.length;at++){if(at>0)hash.update(",");hash.update(observationRowJsonV1(rows[at]));}hash.update("]");return hash.digest("hex");};
 const written=createBoundedCacheV1({maxEntries:65536,maxBytes:65536}),valueItems=value=>Array.isArray(value)?value.length:Object.values(value??{}).reduce((sum,part)=>sum+(Array.isArray(part)?part.length:1),1),parsedValues=createBoundedCacheV1({name:"inference:closure-values",maxEntries:limits.closureValueCacheEntries,maxBytes:limits.closureValueCacheBytes,sizeOf:value=>limits.closureEntryValueItemBytes*valueItems(value)}),valueImages=new WeakMap();
 const pack=text=>brotliCompressSync(Buffer.from(text,"utf8"),{params:{[constants.BROTLI_PARAM_QUALITY]:limits.closureStoreCompressionQuality}}),unpack=bytes=>brotliDecompressSync(bytes,{maxOutputLength:MAXIMUM_RECORD_BYTES}).toString("utf8");
 const place=async(path,text)=>{const temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";await writeFile(temporary,pack(text));await rename(temporary,path);};
 let pruned;const images=new WeakMap(),questions=new Map(),intern=text=>{const known=questions.get(text);if(known!==undefined)return known;questions.set(text,text);return text;};
 const sweep=async(directoryPath,maximum)=>{
  const names=(await readdir(directoryPath).catch(()=>[])).filter(name=>name.endsWith(EXTENSION));
  if(names.length<=maximum)return;
  const dated=await Promise.all(names.map(async name=>({path:join(directoryPath,name),time:(await stat(join(directoryPath,name)).catch(()=>({mtimeMs:0}))).mtimeMs})));
  for(const file of dated.sort((a,b)=>a.time-b.time).slice(0,dated.length-maximum))await rm(file.path,{force:true});
 };
 const layerBytesSync=digest=>{
  try{const bytes=brotliDecompressSync(readFileSync(layerPath(digest)),{maxOutputLength:MAXIMUM_RECORD_BYTES});if(bytes.length<2||bytes[0]!==91||bytes[bytes.length-1]!==93||createHash("sha256").update(bytes).digest("hex")!==digest)return undefined;return bytes;}catch{return undefined;}
 };
 const valueSync=digest=>{
  const known=parsedValues.get(digest);if(known!==undefined)return known;
  try{const text=unpack(readFileSync(valuePath(digest)));if(sha(text)!==digest)return undefined;const value=internJsonStringsV1(JSON.parse(text),graphStringInternerV1);parsedValues.set(digest,value);if(value!==null&&typeof value==="object")valueImages.set(value,{digest});return value;}catch{return undefined;}
 };
 const valuePresent=async digest=>parsedValues.has(digest)||await stat(valuePath(digest)).then(()=>true,()=>false);
 const layerPresent=async digest=>written.has(digest)||await stat(layerPath(digest)).then(()=>true,()=>false);
 const keep=async(digest,text)=>{
  if(written.has(digest))return;
  if(!await stat(layerPath(digest)).then(()=>true,()=>false))await place(layerPath(digest),text());
  written.set(digest,true);
 };
 return Object.freeze({
  /** 💾️ Stores the entry of `id`; answers the size of its own record in characters, without the shared layers (what the in-process cache charges for it). */
  async save(id,{recipe,value,rows,layers:shared,sidecar=null}){
   if(readOnly)return{bytes:JSON.stringify([value,rows]).length+8*shared.length,digests:undefined};
   await Promise.all([mkdir(entries,{recursive:true}),mkdir(layers,{recursive:true}),mkdir(values,{recursive:true})]);
   const digests=[];
   shared.forEach(layer=>{if(!images.has(layer))images.set(layer,{digest:listDigest(layer)});});
   const pending=[];
   shared.forEach(layer=>{const image=images.get(layer);digests.push(image.digest);if(!written.has(image.digest))pending.push(keep(image.digest,()=>listText(layer)));});
   await Promise.all(pending);
   let image=value!==null&&typeof value==="object"?valueImages.get(value):undefined,valueText;
   if(!image){valueText=JSON.stringify(value);image={digest:sha(valueText)};if(value!==null&&typeof value==="object")valueImages.set(value,image);}
   if(!written.has(image.digest)){if(!await stat(valuePath(image.digest)).then(()=>true,()=>false))await place(valuePath(image.digest),valueText??JSON.stringify(value));written.set(image.digest,true);}
   const rowsText=listText(rows),text='{"version":2,"id":'+JSON.stringify(id)+',"digest":"'+sha(rowsText+digests.join(",")+image.digest)+'","recipe":'+JSON.stringify(recipe)+',"valueDigest":"'+image.digest+'","rows":'+rowsText+',"layers":'+JSON.stringify(digests)+',"sidecar":'+JSON.stringify(sidecar)+"}";
   await place(entryPath(id),text);
   pruned??=Promise.all([sweep(entries,limits.closureStoreMaximumEntries),sweep(layers,limits.closureStoreMaximumLayers),sweep(values,limits.closureStoreMaximumValues)]);
   return{bytes:text.length+8*digests.length,digests};
  },
  /** 📖️ The stored entry of `id` (recipe, value, rows, shared layers, size), or undefined while it is absent, damaged or names a missing layer. */
  async load(id){
   try{
    const bytes=await readFile(entryPath(id)).catch(()=>undefined);if(!bytes)return undefined;
    const text=unpack(bytes),record=JSON.parse(text);
    if(record?.version!==2||record.id!==id||typeof record.valueDigest!=="string"||!Array.isArray(record.rows)||!Array.isArray(record.layers))return undefined;
    const found=await Promise.all([...record.layers.map(layerPresent),valuePresent(record.valueDigest)]);
    if(found.includes(false)||sha(JSON.stringify(record.rows)+record.layers.join(",")+record.valueDigest)!==record.digest)return undefined;
    const value=valueSync(record.valueDigest);if(value===undefined)return undefined;
    return{recipe:record.recipe,value,rows:shareObservationRowsV1(record.rows),digests:record.layers,sidecar:record.sidecar?{...record.sidecar,members:record.sidecar.members?.map(walks=>walks&&walks.map(intern))}:null,bytes:text.length+8*record.layers.length};
   }catch{return undefined;}
  },
  /** 🌊️ The rows of the shared layer `digest` in batches of `size`, read, verified and parsed when asked for and held by nobody afterwards (the layer is held as its verified bytes only, one batch of rows is parsed at a time); throws while the layer is missing or damaged, so a caller never mistakes a lost layer for an empty one. */
  *layerBatchesSync(digest,size){
   const bytes=layerBytesSync(digest);if(bytes===undefined)throw Object.assign(Error("Intact closure layer required: "+digest),{code:"CLOSURE_LAYER_LOST"});
   const last=bytes.length-1;let depth=0,quoted=false,escaped=false,from=-1,count=0;
   const batch=to=>{const rows=JSON.parse("["+bytes.toString("utf8",from,to)+"]");from=-1;count=0;return rows;};
   for(let at=1;at<last;at++){
    const byte=bytes[at];
    if(quoted){if(escaped)escaped=false;else if(byte===92)escaped=true;else if(byte===34)quoted=false;continue;}
    if(byte===34){quoted=true;if(from<0)from=at;continue;}
    if(byte===123||byte===91){if(from<0)from=at;depth++;continue;}
    if(byte===125||byte===93){depth--;continue;}
    if(byte===44&&depth===0){if(++count===size)yield batch(at);continue;}
    if(from<0&&byte>32)from=at;
   }
   if(from>=0)yield batch(last);
  },
  /** 🧹️ Waits for the removal of the oldest records beyond the declared maxima that the first save of this process started. */
  async settled(){await pruned;},
 });
}
