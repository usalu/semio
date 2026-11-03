import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

const P=schema.definitions.Policy.const,aborted=signal=>signal?.throwIfAborted();

/** 🗺️ Captures the membership frontier: every directory with its entry stamp (`[path, mtimeMs, ino]`) and every member with its mode (`[path, mode]`), so unchanged membership is proven by stat calls instead of a walk. */
export function createFrontierV1({directories,members,capturedAt}){
 return{version:1,capturedAt,directories:[...directories].sort((a,b)=>compareCodePointsV1(a[0],b[0])),members:[...members].sort((a,b)=>compareCodePointsV1(a[0],b[0]))};
}

/** 0️⃣ Whether a recorded frontier fact (an inode or a member mode) agrees with a present one: `0` is the native scan's word for a fact the platform does not give it without a handle (Windows has neither POSIX inodes nor modes in `std`), and an unknown fact never contradicts. https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle */
export function frontierFactHoldsV1(present,recorded){return recorded===0||present===0||present===recorded;}

/** 🚪️ True when no directory entry stamp and no member mode moved since the capture; a directory touched within the racy window of the capture never counts as proven. */
export async function frontierHoldsV1(frontier,stat,{signal,concurrency=P.readConcurrency*2,racyMilliseconds=P.racyMilliseconds}={}){
 const rows=[...frontier.directories.map(row=>({row,directory:true})),...frontier.members.map(row=>({row,directory:false}))];
 let next=0,ok=true;
 await Promise.all(Array.from({length:Math.min(concurrency,rows.length)},async()=>{
  while(ok&&next<rows.length){
   aborted(signal);const{row,directory}=rows[next++];let status;
   try{status=await stat(row[0]);}catch{ok=false;return;}
   if(!status){ok=false;return;}
   if(directory){if(!status.directory||status.mtimeMs!==row[1]||!frontierFactHoldsV1(status.ino,row[2])||frontier.capturedAt-row[1]<racyMilliseconds)ok=false;}
   else if(!status.file||!frontierFactHoldsV1(status.mode,row[1]))ok=false;
  }
 }));
 return ok;
}

/** 📁️ The directories whose entry sets are proven unchanged since `before` was captured: same stamp and inode in `after`, and not touched within the racy window of the earlier capture (a change in the same clock tick would be invisible). A recorded listing, type or absence of an entry of such a directory cannot have moved, because an entry appears, vanishes or changes type only by changing its parent. */
export function stableDirectoriesV1(before,after,{racyMilliseconds=P.racyMilliseconds}={}){
 const old=new Map(before.directories.map(row=>[row[0],row])),stable=new Set();
 for(const row of after.directories){const was=old.get(row[0]);if(was&&was[1]===row[1]&&frontierFactHoldsV1(was[2],row[2])&&before.capturedAt-was[1]>=racyMilliseconds)stable.add(row[0]);}
 return stable;
}

/** ➗️ Names the paths that appeared, vanished or changed between two frontiers (directories whose stamp moved are `changed`, members whose mode moved are `changed`). */
export function diffFrontiersV1(before,after){
 const changes=[];
 for(const directory of[true,false]){
  const old=new Map((directory?before.directories:before.members).map(row=>[row[0],row])),next=new Map((directory?after.directories:after.members).map(row=>[row[0],row]));
  for(const[path,row]of next){const was=old.get(path);if(!was)changes.push({path,kind:"added",directory});else if(directory?was[1]!==row[1]||!frontierFactHoldsV1(was[2],row[2]):!frontierFactHoldsV1(was[1],row[1]))changes.push({path,kind:"changed",directory});}
  for(const path of old.keys())if(!next.has(path))changes.push({path,kind:"removed",directory});
 }
 return changes.sort((a,b)=>compareCodePointsV1(a.path,b.path));
}
