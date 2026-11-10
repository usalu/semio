/** 🕰 Journal-driven invalidation of the graph generation load: the dirty set of the watcher journal decides which frontier entries are stat'ed again and which fact rows are doubtful, so an edit costs its own paths and never a pass over the tree. Pure functions over journal rows and the ports of the host. */
import {frontierFactHoldsV1} from "../../🕸️graph/🕰️generations/🧭️frontier/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪜 The declared limits and texts of the journal hooks; the schema file is the single source. */
export const GRAPH_HOOKS_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

const ROOT=".",byCodePoint=(a,b)=>a<b?-1:a>b?1:0;
const parentOf=path=>{const cut=path.lastIndexOf("/");return cut<0?"":path.slice(0,cut);};
const frontierParent=path=>{const cut=path.lastIndexOf("/");return cut<0?ROOT:path.slice(0,cut);};
const childOf=(directory,name)=>directory===ROOT?name:directory+"/"+name;

/** 🧭 The dirty set of journal rows `[kind, isDirectory, path]` read after `cursor`: every changed path, the directory that holds it, and the directories added, removed or renamed (their whole subtree). `complete:false` with `overflow` names an answer too large to use. */
export function dirtySetFromRowsV1(rows,cursor,{overflow=false}={}){
 const exact=new Set(),subtrees=new Set();
 if(!overflow)for(const[kind,directory,path]of rows){exact.add(path);exact.add(parentOf(path));if(directory===1&&kind!=="m")subtrees.add(path);}
 return{cursor,complete:!overflow,exact:[...exact].sort(byCodePoint),subtrees:[...subtrees].sort(byCodePoint),overflow};
}

/** 🔎 Membership test over a dirty set: a path is doubtful when it is named or lies below a changed directory. */
export function dirtyIndexV1(set){
 const exact=new Set(set.exact),subtrees=new Set(set.subtrees);
 return{size:exact.size+subtrees.size,has(path){
  if(exact.has(path))return true;
  if(subtrees.size===0)return false;
  for(let at=path;;){if(subtrees.has(at))return true;const cut=at.lastIndexOf("/");if(cut<0)return false;at=at.slice(0,cut);}
 }};
}

/** 🚶 The directories and members below `start` (a directory already known to exist), walked breadth first like {@link walkFrontierV1}; a directory that vanishes meanwhile is skipped, because the journal reports its removal. */
async function walkBelow(start,{list,stat,descend,member,signal,concurrency},into,counter){
 let pending=[start];
 while(pending.length){
  const batch=pending;pending=[];const listed=new Array(batch.length);let next=0;
  await Promise.all(Array.from({length:Math.min(concurrency,batch.length)},async()=>{
   while(next<batch.length){signal?.throwIfAborted();const at=next++,directory=batch[at];counter.checked++;const status=await stat(directory);const entries=status?.directory?await list(directory):undefined;listed[at]=entries&&{directory,status,entries};}
  }));
  for(const held of listed){
   if(!held)continue;
   into.directories.set(held.directory,[held.directory,held.status.mtimeMs,held.status.ino]);
   const files=[];
   for(const entry of held.entries){
    const path=childOf(held.directory,entry.name);
    if(entry.kind==="directory"){if(descend(entry.name,path,held.directory))pending.push(path);}
    else if(entry.kind==="file"&&member(path))files.push(path);
   }
   let at=0;
   await Promise.all(Array.from({length:Math.min(concurrency,files.length)},async()=>{while(at<files.length){signal?.throwIfAborted();const path=files[at++];counter.checked++;const info=await stat(path);if(info?.file)into.members.set(path,info.mode);}}));
  }
 }
}

const mergeSorted=(old,touched,fresh)=>{
 const kept=old.filter(row=>!touched.has(row[0])),out=[];
 let a=0,b=0;
 while(a<kept.length&&b<fresh.length)out.push(byCodePoint(kept[a][0],fresh[b][0])<0?kept[a++]:fresh[b++]);
 while(a<kept.length)out.push(kept[a++]);
 while(b<fresh.length)out.push(fresh[b++]);
 return out;
};

/**
 * ⏩ Advances a frontier `{capturedAt,directories,members}` to the current tree from a dirty set alone: only the named paths are stat'ed (a plain edit costs its file and the directory that holds it), the subtrees of added, removed or renamed directories are dropped and walked again, and no untouched entry is read. `list`, `stat`, `descend` and `member` are the ports of {@link walkFrontierV1}. Answers `undefined` when the dirty set names the whole tree (the caller verifies once), `{unchanged:true,checked}` when nothing moved, otherwise `{directories,members,changes,checked}` with `changes` equal to the difference of the two frontiers and `checked` the number of stat calls.
 */
export async function advanceFrontierByDirtySetV1({frontier,set,list,stat,descend,member,signal,now=Date.now(),concurrency=32}){
 if(!set.complete||set.subtrees.includes(""))return undefined;
 const dirs=new Map(frontier.directories.map(row=>[row[0],row])),modes=new Map(frontier.members.map(row=>[row[0],row[1]])),wasDirs=new Map(),wasModes=new Map(),counter={checked:0};
 const setDirectory=(path,row)=>{if(!wasDirs.has(path))wasDirs.set(path,dirs.get(path));dirs.set(path,row);};
 const dropDirectory=path=>{if(!wasDirs.has(path))wasDirs.set(path,dirs.get(path));dirs.delete(path);};
 const setMode=(path,mode)=>{if(!wasModes.has(path))wasModes.set(path,modes.get(path));modes.set(path,mode);};
 const dropMode=path=>{if(!wasModes.has(path))wasModes.set(path,modes.get(path));modes.delete(path);};
 const stats=async path=>{signal?.throwIfAborted();counter.checked++;return stat(path);};
 const roots=[];
 for(const path of set.subtrees)if(!roots.length||!(path===roots.at(-1)||path.startsWith(roots.at(-1)+"/")))roots.push(path);
 const rootSet=new Set(roots),underRoot=path=>{for(let at=path;;){if(rootSet.has(at))return true;const cut=at.lastIndexOf("/");if(cut<0)return false;at=at.slice(0,cut);}};
 if(roots.length){
  for(const path of[...dirs.keys()])if(underRoot(path))dropDirectory(path);
  for(const path of[...modes.keys()])if(underRoot(path))dropMode(path);
 }
 const known=path=>dirs.has(frontierParent(path));
 const admit=async path=>{
  const status=await stats(path);
  if(!status)return;
  if(status.directory){
   if(!known(path)||!descend(path.slice(path.lastIndexOf("/")+1),path,frontierParent(path)))return;
   const into={directories:new Map(),members:new Map()};
   await walkBelow(path,{list,stat,descend,member,signal,concurrency},into,counter);
   for(const[key,row]of into.directories)setDirectory(key,row);
   for(const[key,mode]of into.members)setMode(key,mode);
  }else if(status.file&&known(path)&&member(path))setMode(path,status.mode);
 };
 for(const path of roots)await admit(path);
 for(const named of set.exact){
  if(underRoot(named))continue;
  const path=named===""?ROOT:named,status=await stats(path);
  if(!status){
   if(dirs.has(path)&&path!==ROOT){for(const key of[...dirs.keys()])if(key===path||key.startsWith(path+"/"))dropDirectory(key);for(const key of[...modes.keys()])if(key.startsWith(path+"/"))dropMode(key);}
   if(modes.has(path))dropMode(path);
   continue;
  }
  if(status.directory){
   if(modes.has(path))dropMode(path);
   if(dirs.has(path)){const was=dirs.get(path);if(was[1]!==status.mtimeMs||was[2]!==status.ino)setDirectory(path,[path,status.mtimeMs,status.ino]);}
   else if(path!==ROOT&&known(path)&&descend(path.slice(path.lastIndexOf("/")+1),path,frontierParent(path))){
    const into={directories:new Map(),members:new Map()};
    await walkBelow(path,{list,stat,descend,member,signal,concurrency},into,counter);
    for(const[key,row]of into.directories)setDirectory(key,row);
    for(const[key,mode]of into.members)setMode(key,mode);
   }
   continue;
  }
  if(dirs.has(path)&&path!==ROOT){for(const key of[...dirs.keys()])if(key===path||key.startsWith(path+"/"))dropDirectory(key);for(const key of[...modes.keys()])if(key.startsWith(path+"/"))dropMode(key);}
  if(status.file&&known(path)&&member(path)){if(modes.get(path)!==status.mode)setMode(path,status.mode);}
  else if(modes.has(path))dropMode(path);
 }
 const changes=[];
 for(const[path,was]of wasDirs){const row=dirs.get(path);if(!was&&row)changes.push({path,kind:"added",directory:true});else if(was&&!row)changes.push({path,kind:"removed",directory:true});else if(was&&row&&(was[1]!==row[1]||!frontierFactHoldsV1(was[2],row[2])))changes.push({path,kind:"changed",directory:true});}
 for(const[path,was]of wasModes){const mode=modes.get(path);if(was===undefined&&mode!==undefined)changes.push({path,kind:"added",directory:false});else if(was!==undefined&&mode===undefined)changes.push({path,kind:"removed",directory:false});else if(was!==undefined&&mode!==undefined&&!frontierFactHoldsV1(mode,was))changes.push({path,kind:"changed",directory:false});}
 if(changes.length===0)return{unchanged:true,checked:counter.checked};
 changes.sort((a,b)=>byCodePoint(a.path,b.path));
 const touchedDirectories=new Set(wasDirs.keys()),touchedMembers=new Set(wasModes.keys());
 const freshDirectories=[...touchedDirectories].filter(path=>dirs.has(path)).sort(byCodePoint).map(path=>dirs.get(path)),freshMembers=[...touchedMembers].filter(path=>modes.has(path)).sort(byCodePoint).map(path=>[path,modes.get(path)]);
 return{directories:mergeSorted(frontier.directories,touchedDirectories,freshDirectories),members:mergeSorted(frontier.members,touchedMembers,freshMembers),changes,checked:counter.checked,capturedAt:now};
}

/** 🏷 Marks the changes of a membership advance like the stat path does: crate or project manifests carry `config:true`. */
export function classifyChangesV1(changes,isConfig=path=>["📋️project.json","Cargo.toml"].includes(path.slice(path.lastIndexOf("/")+1))){
 return changes.map(change=>!change.directory&&isConfig(change.path)?{...change,config:true}:change);
}

/** 🌐 Renders the notice of a verify epoch in one declared language; change journal refusals are rendered by `renderNoticeV1` of the direct reader. */
export function renderGraphHooksNoticeV1(notice,language,fields={}){
 const table=GRAPH_HOOKS_POLICY_V1.texts[language];
 if(!table||typeof table[notice]!=="string")throw Error("Declared graph hooks text required: "+language+"."+notice);
 return table[notice].replace(/\{([a-z]+)\}/gu,(_,name)=>String(fields[name]??"?"));
}

/** 🧮 The dirty set in the closed shape of the derived DAG (`DirtySet`: revision, paths, overflow): named paths, the directories that hold them and the changed subtree roots, so a node that read a path, a listing or a glob roster below any of them is seeded. An incomplete or overflowing answer is `overflow:true` and the DAG rereads everything. */
export function derivedDirtySetV1(set,revision){
 return{...(revision===undefined?{}:{revision}),paths:set.complete?[...new Set([...set.exact,...set.subtrees])].filter(path=>path!=="").sort(byCodePoint):[],overflow:!set.complete||set.exact.includes("")&&set.subtrees.includes("")};
}
