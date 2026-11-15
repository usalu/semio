import{brotliDecompressSync}from"node:zlib";
import{readFile,readdir,rm,stat}from"node:fs/promises";
import{join}from"node:path";

const EXTENSION=".json.br",MAXIMUM_RECORD_BYTES=268435456,BATCH=64,READ_BATCH=8;

/** 🪢️ The slot of a closure recipe: the identity of what it answers (function, project, command and working directory, role, name, view, port, entry files) without the settings and inputs it was derived from, so a newer entry of a slot supersedes the older ones. @see ../💾️entries/🟨️.mjs */
export function closureSlotOfV1(recipe){
 const options=recipe?.target?.options;
 return JSON.stringify([recipe?.fn??null,recipe?.projectRoot??null,options?.command??null,options?.cwd??null,recipe?.role??null,recipe?.name??null,recipe?.view??null,recipe?.port??null,recipe?.entries??null]);
}

/** 🧮️ The pure retention rule: per slot the newest entry (ties: the larger name) is current; an older entry is superseded and goes once the current one is at least `grace` old; a damaged entry goes once it is `grace` old; a layer or value that no kept entry names goes once it was neither written nor reused for `grace`. */
export function planClosureRetentionV1({entries,layers,values,now,grace}){
 const slots=new Map(),drop={entries:[],layers:[],values:[]},kept=entries.filter(row=>row.record);
 for(const row of entries.filter(row=>!row.record))if(now-row.mtimeMs>=grace)drop.entries.push(row.name);
 for(const row of kept){const slot=closureSlotOfV1(row.record.recipe),best=slots.get(slot);if(!best||row.mtimeMs>best.mtimeMs||row.mtimeMs===best.mtimeMs&&row.name>best.name)slots.set(slot,row);}
 const current=new Set([...slots.values()].map(row=>row.name)),live=[];
 for(const row of kept){
  if(!current.has(row.name)){const winner=slots.get(closureSlotOfV1(row.record.recipe));if(now-winner.mtimeMs>=grace&&row.mtimeMs<=winner.mtimeMs){drop.entries.push(row.name);continue;}}
  live.push(row);
 }
 const named=new Set(live.flatMap(row=>row.record.layers)),valued=new Set(live.map(row=>row.record.valueDigest));
 for(const row of layers)if(!named.has(row.name)&&now-row.mtimeMs>=grace)drop.layers.push(row.name);
 for(const row of values)if(!valued.has(row.name)&&now-row.mtimeMs>=grace)drop.values.push(row.name);
 return drop;
}

const listing=async(directory)=>{
 const names=(await readdir(directory).catch(()=>[])),rows=[];
 for(let at=0;at<names.length;at+=BATCH)rows.push(...await Promise.all(names.slice(at,at+BATCH).map(async name=>{const info=await stat(join(directory,name)).catch(()=>undefined);return info?{name,mtimeMs:info.mtimeMs,bytes:info.size}:undefined;})));
 return rows.filter(Boolean);
};
const share=(pool,text)=>{const known=pool.get(text);if(known!==undefined)return known;pool.set(text,text);return text;};
/** 🪶️ The part of a stored entry the retention rule reads: the identity fields of its recipe (what `closureSlotOfV1` reads), the layers and the value it names, strings shared between entries; its observation rows (the bulk of an entry: 3.8 GB on the 41 429 entries of the server) are dropped at once, so a collection holds a few hundred megabytes, not the whole store. */
const readRecord=async(path,pool)=>{try{const record=JSON.parse(brotliDecompressSync(await readFile(path),{maxOutputLength:MAXIMUM_RECORD_BYTES}).toString("utf8"));if(!(record?.version===3&&Array.isArray(record.layers)&&typeof record.valueDigest==="string"))return undefined;const recipe=record.recipe,options=recipe?.target?.options;return{version:3,recipe:{fn:recipe?.fn,projectRoot:recipe?.projectRoot,target:{options:{command:options?.command,cwd:options?.cwd}},role:recipe?.role,name:recipe?.name,view:recipe?.view,port:recipe?.port,entries:recipe?.entries},layers:record.layers.map(layer=>share(pool,layer)),valueDigest:share(pool,record.valueDigest)};}catch{return undefined;}};

/** 🧹️ Collects one closure store directory (`e`, `l`, `v`): reads every entry, applies `planClosureRetentionV1`, removes what it names (unless `dryRun`) and answers the counts and bytes per kind. Leftover temporary files go with the layers and values they sit beside. Safe beside running writers: a writer refreshes the time of every layer and value it reuses, and nothing younger than the grace is removed. */
export async function collectClosureStoreV1({directory,graceMilliseconds,now=Date.now(),dryRun=false,cancelled=()=>false}){
 const entries=await listing(join(directory,"e")),layers=await listing(join(directory,"l")),values=await listing(join(directory,"v"));
 const pool=new Map();
 for(let at=0;at<entries.length;at+=READ_BATCH){if(cancelled())return{cancelled:true};await Promise.all(entries.slice(at,at+READ_BATCH).map(async row=>{row.record=row.name.endsWith(EXTENSION)?await readRecord(join(directory,"e",row.name),pool):undefined;}));await new Promise(resolve=>setImmediate(resolve));}
 const tag=name=>name.endsWith(EXTENSION)?name.slice(0,-EXTENSION.length):undefined,stems=rows=>rows.map(row=>({...row,file:row.name,name:tag(row.name)??"\0"+row.name}));
 const drop=planClosureRetentionV1({entries:entries.map(row=>({...row,record:row.record})),layers:stems(layers),values:stems(values),now,grace:graceMilliseconds}),report={cancelled:false,dryRun};
 const total=(rows,names,suffix)=>{const set=new Set(names);let bytes=0,count=0;const files=[];for(const row of rows){const key=suffix?(tag(row.name)??"\0"+row.name):row.name;if(set.has(key)){bytes+=row.bytes;count++;files.push(row.name);}}return{bytes,count,files};};
 const gone={entries:total(entries,drop.entries,false),layers:total(layers,drop.layers,true),values:total(values,drop.values,true)};
 if(!dryRun)for(const[kind,folder]of[["entries","e"],["layers","l"],["values","v"]])for(let at=0;at<gone[kind].files.length;at+=BATCH){if(cancelled())return{...report,cancelled:true};await Promise.all(gone[kind].files.slice(at,at+BATCH).map(file=>rm(join(directory,folder,file),{force:true})));}
 return{...report,kept:{entries:entries.length-gone.entries.count,layers:layers.length-gone.layers.count,values:values.length-gone.values.count},removed:{entries:gone.entries.count,layers:gone.layers.count,values:gone.values.count},bytes:gone.entries.bytes+gone.layers.bytes+gone.values.bytes};
}
