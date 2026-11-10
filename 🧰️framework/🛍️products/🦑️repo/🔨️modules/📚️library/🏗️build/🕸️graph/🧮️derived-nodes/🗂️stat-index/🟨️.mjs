import{createHash}from"node:crypto";
import{brotliCompressSync,brotliDecompressSync}from"node:zlib";
import{join}from"node:path";
import{DERIVED_NODES_POLICY_V1,sha256V1}from"../🟨️.mjs";
import{NODE_FILE_PORT_V1}from"../💾️store/🟨️.mjs";

const ABSENT="absent",MAGIC=0x32584953,cut=path=>path.lastIndexOf("/");
const shardOf=dir=>{const at=dir.indexOf("/");return at<0?dir:dir.slice(0,at);};

function writerOf(){
 let buffer=Buffer.allocUnsafe(1<<16),length=0;
 const room=count=>{if(length+count>buffer.length){const grown=Buffer.allocUnsafe(Math.max(buffer.length*2,length+count));buffer.copy(grown,0,0,length);buffer=grown;}};
 return{
  varint(value){room(10);while(value>=128){buffer[length++]=value%128|128;value=Math.floor(value/128);}buffer[length++]=value;},
  byte(value){room(1);buffer[length++]=value;},
  double(value){room(8);buffer.writeDoubleLE(value,length);length+=8;},
  bytes(source){room(source.length);source.copy?source.copy(buffer,length):buffer.set(source,length);length+=source.length;},
  done:()=>buffer.subarray(0,length),
 };
}

function readerOf(buffer,limit){
 let at=0;
 const need=count=>{if(at+count>limit)throw Error("short");};
 return{
  varint(){let value=0,scale=1;for(;;){need(1);const byte=buffer[at++];value+=(byte&127)*scale;if(byte<128)return value;scale*=128;}},
  byte(){need(1);return buffer[at++];},
  double(){need(8);const value=buffer.readDoubleLE(at);at+=8;return value;},
  bytes(count){need(count);const out=buffer.subarray(at,at+count);at+=count;return out;},
 };
}

/** 🗜️ A shard in the compact binary form: interned path segments and names, directory listing records and file stat rows, one integrity digest; brotli compressed. */
export function encodeShardV1(shard){
 const strings=new Map(),table=[],intern=text=>{let id=strings.get(text);if(id===undefined){id=table.length;strings.set(text,id);table.push(text);}return id;};
 const dirNames=[...new Set([...shard.files.keys(),...shard.dirs.keys()])].sort(),dirIndex=new Map(dirNames.map((name,index)=>[name,index]));
 const body=writerOf();
 body.varint(dirNames.length);
 for(const name of dirNames){
  const parts=name===""?[]:name.split("/");body.varint(parts.length);for(const part of parts)body.varint(intern(part));
  const record=shard.dirs.get(name);
  if(!record)body.byte(0);
  else{body.byte(1);body.double(record.mtimeMs);body.varint(record.at);body.varint(record.entries.length);for(const[entry,kind]of record.entries){body.varint(intern(entry));body.byte(kind);}}
  const files=shard.files.get(name);
  body.varint(files?files.size:0);
  if(files)for(const[file,row]of files){body.varint(intern(file));body.varint(row[0]);body.double(row[1]);body.varint(row[2]);body.bytes(Buffer.from(row[3],"hex"));body.varint(row[4]);}
 }
 void dirIndex;
 const head=writerOf();
 head.varint(MAGIC);head.varint(table.length);
 for(const text of table){const bytes=Buffer.from(text,"utf8");head.varint(bytes.length);head.bytes(bytes);}
 const payload=Buffer.concat([head.done(),body.done()]);
 return brotliCompressSync(Buffer.concat([payload,createHash("sha256").update(payload).digest()]));
}

/** 🗜️ The inverse of `encodeShardV1`; undefined for anything damaged. */
export function decodeShardV1(bytes,maximumBytes){
 try{
  const raw=brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),limit=raw.length-32;
  if(limit<0||!createHash("sha256").update(raw.subarray(0,limit)).digest().equals(raw.subarray(limit)))return undefined;
  const read=readerOf(raw,limit);
  if(read.varint()!==MAGIC)return undefined;
  const table=[],count=read.varint();
  for(let at=0;at<count;at++)table.push(read.bytes(read.varint()).toString("utf8"));
  const shard={files:new Map(),dirs:new Map()};
  for(let directories=read.varint();directories>0;directories--){
   const parts=[];for(let n=read.varint();n>0;n--)parts.push(table[read.varint()]);
   const name=parts.join("/");
   if(read.byte()===1){const mtimeMs=read.double(),at=read.varint(),entries=[];for(let n=read.varint();n>0;n--)entries.push([table[read.varint()],read.byte()]);shard.dirs.set(name,{mtimeMs,at,entries});}
   const files=read.varint();
   if(files){const map=new Map();for(let n=files;n>0;n--){const file=table[read.varint()],size=read.varint(),mtime=read.double(),ino=read.varint(),digest=read.bytes(32).toString("hex"),at=read.varint();map.set(file,[size,mtime,ino,digest,at]);}shard.files.set(name,map);}
  }
  return shard;
 }catch{return undefined;}
}

/**
 * 🗂️ The durable stat index in one compact record per top-level subtree (`<root>/d/<sha256 of the subtree>.br`, see `encodeShardV1`): for every file digested its size, mtime, inode, digest and the time it was digested, and for every directory walked its mtime and entry names. A file whose size, mtime and inode are unchanged and that was already `racyMilliseconds` older than the moment it was digested is not read again; a directory with unchanged mtime is not listed again. Memory is bounded (`maximumEntries`, whole subtrees evicted least recently used), the disk is pruned by record count and age.
 * @see ../🧬️schema/🔣️.json
 */
export function createStatIndexV1({root,fs=NODE_FILE_PORT_V1,limits=DERIVED_NODES_POLICY_V1.statIndex,now=Date.now}){
 const directory=join(root,limits.directory),shards=new Map(),ready=new Map(),dirty=new Set(),path=key=>join(directory,sha256V1(key)+".br"),sizes=new Map();
 let total=0;
 const load=async key=>{
  let shard;
  try{const bytes=await fs.read(path(key));if(bytes)shard=decodeShardV1(bytes,limits.maximumRecordBytes);}catch{}
  shard??={files:new Map(),dirs:new Map()};
  let count=0;for(const files of shard.files.values())count+=files.size;for(const record of shard.dirs.values())count+=record.entries.length;
  sizes.set(key,count);total+=count;return shard;
 };
 const slot=async dir=>{
  const key=shardOf(dir);let known=shards.get(key);
  if(known)shards.delete(key);else known=load(key);
  shards.set(key,known);
  const shard=await known;
  ready.set(key,shard);
  for(const[other,pending]of shards){if(total<=limits.maximumEntries)break;if(other!==key&&!dirty.has(other)){shards.delete(other);ready.delete(other);await pending;total-=sizes.get(other)??0;sizes.delete(other);}}
  return shard;
 };
 return Object.freeze({
  async digest(file,world,signal){
   signal?.throwIfAborted();
   const stat=await world.stat(file);
   if(!stat)return{digest:ABSENT,read:false,absent:true};
   const at=cut(file),dir=at<0?"":file.slice(0,at),name=file.slice(at+1),shard=await slot(dir);
   let files=shard.files.get(dir);const known=files?.get(name);
   if(known&&known[0]===stat.size&&known[1]===stat.mtimeMs&&known[2]===stat.ino&&known[4]-stat.mtimeMs>=limits.racyMilliseconds)return{digest:known[3],read:false};
   const bytes=await world.read(file);
   if(bytes===undefined)return{digest:ABSENT,read:true};
   const digest=sha256V1(bytes);
   files=shard.files.get(dir);if(!files)shard.files.set(dir,files=new Map());
   if(!known){total++;sizes.set(shardOf(dir),(sizes.get(shardOf(dir))??0)+1);}
   files.set(name,[stat.size,stat.mtimeMs,stat.ino,digest,now()]);dirty.add(shardOf(dir));
   return{digest,read:true,bytes};
  },
  async directory(dir){return(await slot(dir)).dirs.get(dir);},
  async remember(dir,record){const shard=await slot(dir);shard.dirs.set(dir,record);dirty.add(shardOf(dir));sizes.set(shardOf(dir),(sizes.get(shardOf(dir))??0)+record.entries.length);total+=record.entries.length;},
  directoryNow:dir=>ready.get(shardOf(dir))?.dirs.get(dir),
  async flush(){
   await fs.mkdir(directory);
   for(const key of[...dirty]){dirty.delete(key);await fs.write(path(key),encodeShardV1(await shards.get(key)));}
  },
  async prune({at=now()}={}){
   const files=(await fs.list(directory)).sort((a,b)=>a.mtimeMs-b.mtimeMs),expired=files.filter(file=>at-file.mtimeMs>limits.retainMilliseconds),kept=files.filter(file=>!expired.includes(file)),over=kept.slice(0,Math.max(0,kept.length-limits.maximumRecords));
   for(const file of[...expired,...over])await fs.remove(file.path);
   return{removed:expired.length+over.length,kept:files.length-expired.length-over.length};
  },
  stats:()=>({directories:shards.size,entries:total,dirty:dirty.size}),
 });
}
