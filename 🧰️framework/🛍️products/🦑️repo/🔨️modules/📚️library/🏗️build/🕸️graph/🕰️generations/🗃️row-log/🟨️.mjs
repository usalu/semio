import{closeSync,existsSync,mkdirSync,openSync,readSync,readdirSync,rmSync,statSync,writeSync}from"node:fs";
import{dirname,join}from"node:path";
import{rowKeyV1}from"../🧮️fact-rows/🟨️.mjs";

/** 📏️ The declared limits of the row log: the bytes buffered before a write, the rows of one batch, the gap and the length of one coalesced read. */
export const ROW_LOG_POLICY_V1=Object.freeze({flushBytes:4194304,batchRows:2048,gapBytes:65536,spanBytes:4194304,staleSeconds:3600});
const P=ROW_LOG_POLICY_V1,EXTENSION=".rows",NEWLINE="\n";

/** 🌱️ A typed array grown to hold `length` entries (doubling, never shrinking). */
const grown=(array,length)=>{if(array.length>=length)return array;const next=new array.constructor(Math.max(length,array.length*2,1024));next.set(array);return next;};

/** 🧹️ Removes the logs of dead or long-gone processes next to `directory`, so a killed build leaves nothing behind for long. */
function sweepStaleLogsV1(directory){
 let names;try{names=readdirSync(directory);}catch{return;}
 const now=Date.now();
 for(const name of names){
  if(!name.endsWith(EXTENSION))continue;
  const path=join(directory,name);
  try{if(now-statSync(path).mtimeMs>P.staleSeconds*1000)rmSync(path,{force:true});}catch{}
 }
}

/**
 * 🗃️ An append-only log of observation rows on disk: every distinct row (by `rowKeyV1`) is written once as one JSON line when it is first observed, an owner keeps only the ids of its rows (4 bytes each) and its rows are read back in batches where they are consumed, so no list of rows ever exists in memory together. Rows are read after a JSON round trip, which is what every persisted record of the graph already is.
 * @see ../🌊️stream-patch/🟨️.mjs the consumer that reads the lists in batches
 */
export function createRowLogV1({path,policy=P}){
 mkdirSync(dirname(path),{recursive:true});sweepStaleLogsV1(dirname(path));
 const descriptor=openSync(path,"w+"),byKey=new Map(),exit=()=>{try{rmSync(path,{force:true});}catch{}};
 let api,keys=[],offsets=new Float64Array(1024),lengths=new Uint32Array(1024),pending=[],pendingBytes=0,written=0,size=0,closed=false,reads=0,readBytes=0;
 process.once("exit",exit);
 const flush=()=>{
  if(!pending.length)return;
  const buffer=Buffer.from(pending.join(""),"utf8");let at=0;
  while(at<buffer.length)at+=writeSync(descriptor,buffer,at,buffer.length-at,written+at);
  written+=buffer.length;pending=[];pendingBytes=0;
 };
 const require=()=>{if(closed)throw Error("Open row log required");};
 const intern=row=>{
  const key=rowKeyV1(row);let id=byKey.get(key);
  if(id!==undefined)return id;
  id=keys.length;byKey.set(key,id);keys.push(key);
  const line=JSON.stringify(row)+NEWLINE,bytes=Buffer.byteLength(line);
  offsets=grown(offsets,id+1);lengths=grown(lengths,id+1);offsets[id]=size;lengths[id]=bytes;size+=bytes;
  pending.push(line);pendingBytes+=bytes;if(pendingBytes>=policy.flushBytes)flush();
  return id;
 };
 const readBatch=(ids,from,to)=>{
  flush();
  const count=to-from,order=Array.from({length:count},(_unused,at)=>from+at).sort((a,b)=>offsets[ids[a]]-offsets[ids[b]]),rows=new Array(count);
  let at=0;
  while(at<count){
   const first=ids[order[at]],start=offsets[first];let end=start+lengths[first],last=at;
   while(last+1<count){const next=ids[order[last+1]],gap=offsets[next]-end;if(gap>policy.gapBytes||offsets[next]+lengths[next]-start>policy.spanBytes)break;end=Math.max(end,offsets[next]+lengths[next]);last++;}
   const buffer=Buffer.allocUnsafe(end-start);let got=0;
   while(got<buffer.length){const read=readSync(descriptor,buffer,got,buffer.length-got,start+got);if(read<=0)throw Error("Complete row log read required");got+=read;}
   reads++;readBytes+=buffer.length;
   for(let item=at;item<=last;item++){const id=ids[order[item]],begin=offsets[id]-start;rows[order[item]-from]=JSON.parse(buffer.toString("utf8",begin,begin+lengths[id]));}
   at=last+1;
  }
  return rows;
 };
 const list=(ids,tails)=>{
  const length=ids.length+tails.reduce((sum,tail)=>sum+tail.length,0);
  const self={
   length,ids,tails,get log(){return api;},
   keys:function*(){for(let at=0;at<ids.length;at++)yield keys[ids[at]];for(const tail of tails)for(const row of tail)yield rowKeyV1(row);},
   paths:function*(){for(const row of self.rows())yield row.path;},
   rows:function*(){for(const batch of self.batchesSync())yield*batch;},
   batchesSync:function*(){require();for(let at=0;at<ids.length;at+=policy.batchRows)yield readBatch(ids,at,Math.min(ids.length,at+policy.batchRows));for(const tail of tails)yield tail;},
   async*batches(){for(const batch of self.batchesSync()){yield batch;await new Promise(resolve=>setImmediate(resolve));}},
   concat(...more){return list(ids,[...tails,...more.filter(rows=>rows.length>0)]);},
   [Symbol.iterator](){return self.rows();},
  };
  return Object.freeze(self);
 };
 api=Object.freeze({
  path,
  /** 📖️ The rows of the given log ids, in the order of the ids, read in coalesced spans; ids are the numbers the lists of this log hold (`ids`), so a consumer can learn the row of a number once and never read it again. */
  read(ids){require();return readBatch(ids,0,ids.length);},
  /** 🔢️ How many distinct rows the log holds, the size of the number space of its lists. */
  size:()=>keys.length,
  /** 🧾️ Appends rows (written once per distinct row) and answers the list of them; further rows join the list of the same owner through `concat`. */
  append(rows){require();const ids=[];const seen=new Set();for(const row of rows){const id=intern(row);if(!seen.has(id)){seen.add(id);ids.push(id);}}return list(Uint32Array.from(ids),[]);},
  /** 🧱️ Opens a builder that keeps one owner's ids as they arrive, in batches, without a set of the rows. */
  builder(){
   require();let ids=new Uint32Array(1024),count=0,marks=new Uint8Array(Math.max(1,Math.ceil(keys.length/8)));
   const mark=id=>{const byte=id>>>3;if(byte>=marks.length)marks=grown(marks,byte+1);const bit=1<<(id&7);if(marks[byte]&bit)return false;marks[byte]|=bit;return true;};
   return Object.freeze({
    add(rows){for(const row of rows){const id=intern(row);if(mark(id)){ids=grown(ids,count+1);ids[count++]=id;}}},
    addList(other){if(other.ids===undefined)throw Error("Row log list required");for(const id of other.ids)if(mark(id)){ids=grown(ids,count+1);ids[count++]=id;}},
    finish(){const out=ids.slice(0,count);ids=new Uint32Array(0);marks=new Uint8Array(0);return list(out,[]);},
   });
  },
  list:ids=>list(Uint32Array.from(ids),[]),
  empty:list(new Uint32Array(0),[]),
  stats:()=>({rows:keys.length,bytes:size,reads,readBytes,pendingBytes}),
  close(){if(closed)return;closed=true;process.removeListener("exit",exit);try{closeSync(descriptor);}catch{}if(existsSync(path))rmSync(path,{force:true});byKey.clear();keys=[];},
 });
 return api;
}
