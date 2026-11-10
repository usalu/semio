/** 🔌 The two seams of the graph generation host (membership proof and fact checks) answered from the watcher journal: the dirty set since the cursor at which the generation was last verified decides what is stat'ed again, and a journal that cannot vouch is announced as one verify epoch (the unchanged scan of the host, then trusted again). No service has to answer: the journal is read directly. */
import {GRAPH_HOOKS_POLICY_V1,advanceFrontierByDirtySetV1,classifyChangesV1,derivedDirtySetV1,dirtyIndexV1,dirtySetFromRowsV1} from "../🟨️.mjs";

const TRUSTED=new Set(GRAPH_HOOKS_POLICY_V1.trustedKinds),cursorKey=generation=>"g"+generation;
const slashed=path=>String(path).replaceAll("\\","/");

/** 📐 Maps the path of a fact row to a workspace-relative path; undefined where the row lies outside the workspace (the journal cannot see it). */
export function relativeFactPathV1(root,path){
 const base=slashed(root).replace(/\/+$/u,""),text=slashed(path);
 if(text===base)return "";
 if(text.startsWith(base+"/"))return text.slice(base.length+1);
 return undefined;
}

const parentOfPath=path=>{const cut=path.lastIndexOf("/");return cut<0?"":path.slice(0,cut);};

/** 🧭 Whether a `resolve` row `[importer, specifier] => resolved` may have moved: an absolute workspace specifier resolves by probing its own path, its directory and the resolved file, so the journal vouches when none of them is dirty; a bare specifier walks `node_modules` directories the journal never reports and is always doubtful. */
export function resolveRowDoubtfulV1(root,row,index,excluded,installed=false){
 let key;
 try{key=JSON.parse(row.path);}catch{return true;}
 if(!Array.isArray(key)||key.length!==2||typeof key[1]!=="string")return true;
 if(!key[1].startsWith("/")&&!/^[A-Za-z]:[\\/]/u.test(key[1])&&!key[1].startsWith("."))return bareResolveDoubtfulV1(root,key,row,index,installed);
 const specifier=relativeFactPathV1(root,key[1]);
 if(specifier===undefined)return true;
 const paths=[specifier,parentOfPath(specifier)];
 if(typeof row.fact==="string"){const resolved=relativeFactPathV1(root,row.fact);if(resolved===undefined)return true;paths.push(resolved,parentOfPath(resolved));}
 else if(row.fact!==false&&row.fact!==null)return true;
 return paths.some(path=>index.has(path)||excluded(path));
}

const withInstall=(cursor,fingerprint)=>fingerprint===undefined?cursor:{...cursor,install:fingerprint};
const unionDirty=(set,carry)=>carry?{...set,exact:[...set.exact,...carry.exact],subtrees:[...set.subtrees,...carry.subtrees]}:set;

/** 📌 The membership changes of a scan as a dirty carry: the paths a frontier advance must stat again because the stored frontier predates them. */
function carryOfChanges(changes){
 const exact=new Set(),subtrees=new Set();
 for(const change of changes){exact.add(change.path);const cut=change.path.lastIndexOf("/");exact.add(cut<0?"":change.path.slice(0,cut));if(change.directory&&change.kind!=="changed")subtrees.add(change.path);}
 return{exact,subtrees};
}

const CONFIGURATION_NAMES=["package.json","tsconfig.json","jsconfig.json"];

/** 📦 Whether a workspace-relative path lies where only a dependency install changes anything: a `node_modules` directory (the journal never reports them) or the install store below `.cache`. */
export function installCoveredV1(relative){
 return relative==="node_modules"||relative.startsWith("node_modules/")||relative.includes("/node_modules/")||relative.startsWith(GRAPH_HOOKS_POLICY_V1.installStore);
}

/** 🧾 A bare specifier resolves through `node_modules` directories that change only through `repo dependencies sync/lock`: with the install fingerprint (install root and lockfile digest) unchanged since the cursor, the row holds unless a configuration file on the way (`package.json`, `tsconfig.json`, `jsconfig.json` of an ancestor of the importer or of the result) or a workspace result moved. */
function bareResolveDoubtfulV1(root,key,row,index,installed){
 if(!installed||typeof row.fact!=="string")return true;
 const importer=relativeFactPathV1(root,key[0]),resolved=relativeFactPathV1(root,row.fact);
 if(importer===undefined||resolved===undefined)return true;
 const ancestors=new Set();
 for(const start of[importer,resolved])for(let at=parentOfPath(start);;at=parentOfPath(at)){for(const name of CONFIGURATION_NAMES)ancestors.add(at?at+"/"+name:name);if(at==="")break;}
 if([...ancestors].some(path=>index.has(path)))return true;
 return installCoveredV1(resolved)?false:index.has(resolved)||index.has(parentOfPath(resolved));
}

/** 🔌 Wraps `membership` and `holds` of the host. `journal` reads the watcher journal, `store` keeps the cursor (and the paths still owed a check) at which each generation was verified, `ports` are the frontier ports of the host, `membership`/`holds` the unchanged scan used by a verify epoch, `adopt(files)` receives the member list like the scan stores it and `notify` announces every verify epoch. A verify epoch of a full load commits its cursor as soon as the fact verification ends (`factsVerified`), with the changes and stale rows found so far as a carry that stays doubtful, so a scan that dies later resumes instead of repeating. */
export function journalGraphSeamsV1({root,journal,store,ports,digest,membership,holds,classify=async changes=>changes,adopt=()=>undefined,notify=()=>undefined,install=async()=>undefined,now=()=>Date.now(),maximumCarry=GRAPH_HOOKS_POLICY_V1.maximumCarry}){
 const stats={advanced:0,verified:0,checked:0,trusted:0,doubtful:0,resumed:0};
 let pending,full;
 const track=async(rows,signal)=>{
  const ok=await holds(rows,signal);
  const held=pending;
  if(!ok&&held&&held.stale&&!held.overflow){
   for(const row of rows){const path=TRUSTED.has(row.kind)?relativeFactPathV1(root,row.path):undefined;if(path!==undefined)held.stale.add(path);}
   if(held.stale.size>maximumCarry)held.overflow=true;
  }
  return ok;
 };
 const lookup=manifest=>{
  const own=store.get(cursorKey(manifest.generation));
  if(own)return own;
  const base=manifest.built?.base;
  return typeof base==="number"?store.get(cursorKey(base)):undefined;
 };
 const epoch=async(previous,notice,signal)=>{
  notify(notice);stats.verified++;
  const cursor=await journal.cursor(signal),fingerprint=await install(),result=await membership(previous,signal);
  const owed=carryOfChanges(result.changes??[]);
  pending={digest:result.digest,cursor,index:undefined,excluded:()=>false,generation:previous?.manifest?.generation,owed,stale:new Set(),overflow:owed.exact.size>maximumCarry,entry:undefined,install:fingerprint,installed:false};
  return result;
 };
 const doubtfulRow=(row,held)=>{
  if(row.kind===GRAPH_HOOKS_POLICY_V1.resolveKind)return resolveRowDoubtfulV1(root,row,held.index,held.excluded,held.installed);
  if(!TRUSTED.has(row.kind))return true;
  const path=relativeFactPathV1(root,row.path);
  if(path===undefined)return true;
  if(held.installed&&installCoveredV1(path))return false;
  return held.index.has(path)||held.excluded(path);
 };
 return Object.freeze({
  stats,
  scope(isFull){full=isFull;},
  async membership(previous,signal){
   if(!previous?.frontier){
    const cursor=await journal.cursor(signal),result=await membership(previous,signal);
    pending={digest:result.digest,cursor,index:undefined,excluded:()=>false,generation:undefined,owed:undefined,stale:undefined,overflow:true,entry:undefined,install:undefined,installed:false};
    return result;
   }
   const generation=previous.manifest.generation,entry=lookup(previous.manifest);
   if(!entry)return epoch(previous,{reason:"unrecorded",generation},signal);
   const answer=await journal.since({instance:entry.instance,seq:entry.seq},signal);
   if(!answer.complete)return epoch(previous,{...answer,generation},signal);
   const set=unionDirty(dirtySetFromRowsV1(answer.rows,answer.cursor),entry.carry),advanced=await advanceFrontierByDirtySetV1({frontier:previous.frontier,set,...ports,signal,now:now()});
   if(!advanced)return epoch(previous,{reason:"outside",generation},signal);
   stats.advanced++;stats.checked+=advanced.checked;if(entry.carry)stats.resumed++;
   const fingerprint=await install();
   pending={digest:previous.manifest.membership,cursor:answer.cursor,index:dirtyIndexV1(set),excluded:journal.excluded(),generation,owed:undefined,stale:undefined,overflow:true,entry,install:fingerprint,installed:fingerprint!==undefined&&entry.install===fingerprint};
   const dirty={...derivedDirtySetV1(set),directories:set.complete?[...set.subtrees].filter(path=>path!==""):[],excluded:journal.excluded()};
   if(advanced.unchanged){adopt(previous.frontier.members.map(row=>row[0]));return{digest:previous.manifest.membership,frontier:previous.frontier,changes:[],dirty};}
   const frontier={version:1,capturedAt:advanced.capturedAt,directories:advanced.directories,members:advanced.members},changes=await classify(classifyChangesV1(advanced.changes));
   pending.digest=changes.some(change=>!change.directory)?digest(frontier.members):previous.manifest.membership;
   adopt(frontier.members.map(row=>row[0]));
   return{digest:pending.digest,frontier,changes,dirty};
  },
  /** 🔢️ The doubt of each row as the journal sees it (1 where the row must be checked), or undefined when no dirty set vouches (an epoch): rows the journal does not name are vouched for without any call. */
  doubtful(rows){
   const held=pending;
   return held?.index?Uint8Array.from(rows,row=>doubtfulRow(row,held)?1:0):undefined;
  },
  async holds(rows,signal){
   const held=pending;
   if(!held?.index)return track(rows,signal);
   const doubtful=rows.filter(row=>doubtfulRow(row,held));
   stats.trusted+=rows.length-doubtful.length;stats.doubtful+=doubtful.length;
   return doubtful.length===0?true:holds(doubtful,signal);
  },
  factsVerified(){
   const held=pending;
   if(full!==true||!held?.cursor||held.overflow||held.generation===undefined||held.index)return;
   store.set(cursorKey(held.generation),withInstall(held.cursor,held.install),{exact:[...new Set([...held.owed.exact,...held.stale])].sort(),subtrees:[...held.owed.subtrees].sort()});
  },
  commit(record){
   const held=pending;
   if(!held)return;
   const complete=full!==false&&(record.covered===undefined||record.covered.size>=record.owners.length);
   if(complete&&held.cursor)store.set(cursorKey(record.generation),withInstall(held.cursor,held.install));
   else if(held.entry)store.set(cursorKey(record.generation),withInstall({instance:held.entry.instance,seq:held.entry.seq},held.entry.install),held.entry.carry);
  },
 });
}
