import {isAbsolute,join,sep} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const limits=schema.$defs.limits.const;

/** 🏷️ The stamp of a directory as a stat reports it: modification time, change time, inode and size, with the later of the two times a caller needs to judge whether the stamp is calm. */
export function directoryStampV1(status){return Object.freeze({text:status.mtimeMs+":"+status.ctimeMs+":"+status.ino+":"+status.size,touchedMs:Math.max(status.mtimeMs,status.ctimeMs)});}

/** 📂️ The memo of directory facts by the stamps of the directories that produced them: `record` keeps a fact with the stamp of every directory its computation listed, `lookup` returns it again while each of those directories still has that stamp (one stat each, supplied by `current`). A fact whose computation listed a directory that failed (`null` row) or whose stamp is younger than `calmMilliseconds` is not remembered, because a change inside the same clock tick would keep its stamp. Every dependency is a listing, so a name added, removed or renamed in any of those directories moves its stamp and drops the fact.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🟦️.ts */
export function createDirectoryFactMemoV1(options={}){
 const capacity=options.entries??limits.entries,calm=options.calmMilliseconds??2000,rows=new Map(),counts={hits:0,misses:0,recorded:0,refused:0};
 const relativeOf=(directory,path)=>path===directory?"":path.startsWith(directory+sep)?path.slice(directory.length+1):path;
 return {
  counts:()=>({...counts,entries:rows.size}),
  entries:()=>rows,
  clear:()=>rows.clear(),
  lookup(directory,current){
   const entry=rows.get(directory);
   if(entry===undefined){counts.misses++;return undefined;}
   for(const [relative,text] of entry.stamps)if(current(relative===""?directory:isAbsolute(relative)?relative:join(directory,relative))!==text){rows.delete(directory);counts.misses++;return undefined;}
   counts.hits++;return entry.fact;
  },
  record(directory,consulted,fact,now=Date.now()){
   const stamps=new Map();
   for(const row of consulted){
    if(row===null||now-row[1].touchedMs<=calm){counts.refused++;return false;}
    stamps.set(relativeOf(directory,row[0]),row[1].text);
   }
   if(stamps.size===0){counts.refused++;return false;}
   if(rows.size>=capacity&&!rows.has(directory)){const oldest=rows.keys().next().value;rows.delete(oldest);}
   rows.delete(directory);rows.set(directory,{fact,stamps:[...stamps]});counts.recorded++;return true;
  },
 };
}
