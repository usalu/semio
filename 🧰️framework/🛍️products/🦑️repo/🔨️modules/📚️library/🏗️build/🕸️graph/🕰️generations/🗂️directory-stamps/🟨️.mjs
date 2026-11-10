import{readdir,readFile,rename,stat,writeFile}from"node:fs/promises";

/**
 * 🗂️ The directory stamp table of the fact verification: for every directory that structural observation rows (`readdir`, `exists`, `lstat`, `fileExists`, `directoryExists`) look into, the modification time, inode, device and the sorted names of its symbolic-link children it had when every such row below it was last verified to hold. A directory whose stamp is unchanged and was not racy (older than `racyMilliseconds` when stamped) cannot have gained, lost or renamed an entry, so its rows hold without any further call; a changed, new or racy directory sends its rows to the precise check. One stat per distinct directory replaces one call per row.
 * @see ../../../👁️observation/🟨️.mjs `observationsHold` the precise check a doubtful row still gets
 */
export function createDirectoryStampsV1({file,racyMilliseconds=2000,concurrency=128,now=Date.now,statDirectory=path=>stat(path).catch(error=>error?.code==="ENOENT"||error?.code==="ENOTDIR"?null:undefined),listLinks=path=>readdir(path,{withFileTypes:true}).then(entries=>entries.filter(entry=>entry.isSymbolicLink()).map(entry=>entry.name).sort(),()=>undefined),maximumDirectories=262144}){
 let table;
 const load=async()=>{
  if(table)return table;
  table=new Map();
  try{const value=JSON.parse(await readFile(file,"utf8"));if(value?.version===2&&Array.isArray(value.directories))for(const row of value.directories)if(Array.isArray(row)&&typeof row[0]==="string"&&Number.isFinite(row[1])&&Number.isFinite(row[2])&&Number.isFinite(row[3])&&Array.isArray(row[4]))table.set(row[0],[row[1],row[2],row[3],row[4]]);}catch{}
  return table;
 };
 return Object.freeze({
  /** 🔎️ Stats each distinct directory once and answers which hold their stamp (`observedAt` moves the racy window to the moment the facts were observed: a directory touched since then is never measured); `measured` carries the stamps of this pass for `save`. Cancellable, reports `{phase:"directories",done,total}`. */
  async classify(directories,{signal,progress,observedAt}={}){
   const known=await load(),list=[...new Set(directories)],unchanged=new Set(),measured=new Map(),links=new Map(),missing=new Set();
   let next=0,done=0;const at=observedAt??now();
   await progress?.({phase:"directories",done:0,total:list.length});
   await Promise.all(Array.from({length:Math.min(concurrency,list.length)},async()=>{
    while(next<list.length){
     signal?.throwIfAborted();
     const path=list[next++],found=await statDirectory(path);
     if(found===null)missing.add(path);
     if(found?.isDirectory?.()&&at-found.mtimeMs>=racyMilliseconds){
      const stamp=known.get(path),same=stamp&&stamp[0]===found.mtimeMs&&stamp[1]===Number(found.ino)&&stamp[2]===Number(found.dev);
      if(same){unchanged.add(path);measured.set(path,stamp);links.set(path,stamp[3]);}
      else{const names=await listLinks(path),again=await statDirectory(path);if(names!==undefined&&again?.mtimeMs===found.mtimeMs&&Number(again.ino)===Number(found.ino)){measured.set(path,[found.mtimeMs,Number(found.ino),Number(found.dev),names]);links.set(path,names);}}
     }
     done++;if(done%1024===0||done===list.length)await progress?.({phase:"directories",done,total:list.length});
    }
   }));
   return{unchanged,measured,links,missing};
  },
  /** 💾️ Keeps the stamps of the directories this pass measured, except those named in `exclude` (directories of rows found stale), and replaces the file atomically; losing the write only costs the next pass a precise check. */
  async save(measured,exclude=new Set()){
   const known=await load();
   for(const[path,row]of measured)if(!exclude.has(path))known.set(path,row);else known.delete(path);
   while(known.size>maximumDirectories)known.delete(known.keys().next().value);
   const temporary=file+"."+process.pid+".tmp";
   try{await writeFile(temporary,JSON.stringify({version:2,directories:[...known].map(([path,row])=>[path,row[0],row[1],row[2],row[3]])}));await rename(temporary,file);return true;}catch{return false;}
  },
  size:()=>table?.size??0,
 });
}
