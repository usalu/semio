import{createHash}from"node:crypto";
import{readFile,readdir,stat}from"node:fs/promises";

const SKIPPED=new Set([".git","node_modules","target","__pycache__",".venv",".cache"]),WALKERS=16;

/** 🌍️ The world port over a directory: bytes and stat of one file and the workspace relative files below a root, listed by up to 16 concurrent directory walkers, cancellable. With `remember` a directory whose mtime is unchanged (and was already `racyMilliseconds` old when listed) is not read again: a relisting costs one stat per directory (at most `maximumDirectories` remembered, least recently used first). */
export function createDiskWorldV1(workspaceRoot,{skip=[],descends,remember=false,index,racyMilliseconds=2000,maximumDirectories=262144,now=Date.now}={}){
 const skipped=new Set([...SKIPPED,...skip]),memory=new Map(),stamps=new Map(),enters=descends??((name)=>!skipped.has(name));
 const table=index??Object.freeze({
  directory:async dir=>memory.get(dir),
  directoryNow:dir=>memory.get(dir),
  remember:async(dir,record)=>{memory.delete(dir);memory.set(dir,record);while(memory.size>maximumDirectories)memory.delete(memory.keys().next().value);},
 });
 const track=remember||index!==undefined;
 const absolute=relative=>relative===""?workspaceRoot:workspaceRoot+"/"+relative;
 const read=async relative=>(await readdir(absolute(relative),{withFileTypes:true}).catch(()=>[])).map(entry=>[entry.name,entry.isDirectory()?1:entry.isFile()?0:2]);
 const listed=async relative=>{
  if(!track)return read(relative);
  const found=await stat(absolute(relative)).catch(()=>undefined);
  if(!found)return[];
  const known=await table.directory(relative);
  if(known&&known.mtimeMs===found.mtimeMs&&known.at-found.mtimeMs>=racyMilliseconds)return known.entries;
  const at=now(),entries=await read(relative);
  stamps.clear();await table.remember(relative,{mtimeMs:found.mtimeMs,at,entries});
  return entries;
 };
 const merkle=new Map();
 const nodeOf=relative=>{
  if(merkle.has(relative))return merkle.get(relative);
  const record=table.directoryNow(relative);
  let stamp;
  if(!record||record.at-record.mtimeMs<racyMilliseconds)stamp=undefined;
  else{
   const children=[];let stable=true;
   for(const[name,kind]of record.entries){
    const path=relative===""?name:relative+"/"+name;
    if(kind!==1||!enters(name,path,relative))continue;
    const child=nodeOf(path);
    if(child===undefined){stable=false;break;}
    children.push(name+":"+child);
   }
   stamp=stable?createHash("sha256").update(record.mtimeMs+"\n"+children.sort().join("\n")).digest("hex"):undefined;
  }
  merkle.set(relative,stamp);return stamp;
 };
 const stampOf=root=>{
  if(stamps.has(root))return stamps.get(root);
  merkle.clear();const stamp=nodeOf(root);stamps.set(root,stamp);return stamp;
 };
 const traverse=async(root,signal,visit)=>{
  let active=0;const waiting=[];
  const gate=async work=>{
   while(active>=WALKERS)await new Promise(resume=>waiting.push(resume));
   active++;
   try{signal?.throwIfAborted();return await work();}finally{active--;waiting.shift()?.();}
  };
  const walk=async relative=>{
   const subdirectories=[];
   for(const[name,kind]of await gate(()=>listed(relative))){
    visit(relative,name,kind);
    if(kind===1&&enters(name,relative===""?name:relative+"/"+name,relative))subdirectories.push(walk(relative===""?name:relative+"/"+name));
   }
   await Promise.all(subdirectories);
  };
  await walk(root);
 };
 return Object.freeze({
  stat:path=>stat(workspaceRoot+"/"+path).then(found=>found.isFile()?{size:found.size,mtimeMs:found.mtimeMs,ino:found.ino}:undefined,()=>undefined),
  read:path=>readFile(workspaceRoot+"/"+path).catch(()=>undefined),
  async files(root,signal){
   const out=[];
   await traverse(root,signal,(relative,name,kind)=>{if(kind===0)out.push(relative===""?name:relative+"/"+name);});
   return out;
  },
  async verify(root,signal){
   if(!track)return undefined;
   await traverse(root,signal,()=>{});
   return stampOf(root);
  },
  stamp:root=>track?stampOf(root):undefined,
 });
}
