import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const variable=schema.definitions.Environment.const.directory,files=schema.definitions.Files.const,offering=schema.definitions.Offered.const;

/** 🧭️ Where the steering files of a running graph read live: the directory the graph server named in `SEMIO_GRAPH_STEER_DIRECTORY`, or nothing when the read is not served by one. */
export function steeringPathsV1(environment=process.env){
 const directory=environment[variable];
 return typeof directory==="string"&&directory!==""?{directory,priority:join(directory,files.priority),pending:join(directory,files.pending)}:undefined;
}

const names=value=>Array.isArray(value)&&value.every(name=>typeof name==="string"&&name!=="");
const readJson=path=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};

/** 📥️ The priority the server wrote: its time and the project names, or `undefined` when there is none or the file is damaged. */
export function readPriorityV1(environment=process.env){
 const paths=steeringPathsV1(environment),row=paths&&readJson(paths.priority);
 return row&&row.version===1&&Number.isSafeInteger(row.atMs)&&names(row.projects)?{atMs:row.atMs,projects:row.projects}:undefined;
}

/** 📤️ Replaces the priority file atomically (the server's side). */
export function writePriorityV1(directory,projects,atMs=Date.now()){
 mkdirSync(directory,{recursive:true});
 const path=join(directory,files.priority),temporary=path+"."+process.pid+".tmp";
 writeFileSync(temporary,JSON.stringify({version:1,atMs,projects:[...new Set(projects)]}));renameSync(temporary,path);
}

/** 🧹️ Removes both files, so a new read starts without the answers of an earlier one. */
export function clearSteeringV1(directory){for(const name of Object.values(files))rmSync(join(directory,name),{force:true});}

/** 🔀️ The dirty projects of a wave in the order the wave takes them: the priority projects first, in the order the server listed them, then the others in their given order. Without a priority the order stays. */
export function orderOwnersV1(dirty,priority){
 if(priority===undefined||priority.projects.length===0)return dirty;
 const present=new Set(dirty),first=[],seen=new Set();
 for(const name of priority.projects)if(present.has(name)&&!seen.has(name)){seen.add(name);first.push(name);}
 return first.length===0?dirty:first.concat(dirty.filter(name=>!seen.has(name)));
}

/** 📝️ Tells the server which priority projects are still dirty after a plan (`dirty` is every dirty project of the read). Never throws: a read does not depend on its server. */
export function writePendingV1(environment,generation,dirty,priority){
 const paths=steeringPathsV1(environment);if(paths===undefined||priority===undefined)return;
 try{
  const wanted=new Set(priority.projects),temporary=paths.pending+"."+process.pid+".tmp";
  writeFileSync(temporary,JSON.stringify({version:1,generation,atMs:Date.now(),priorityAtMs:priority.atMs,projects:dirty.filter(name=>wanted.has(name))}));renameSync(temporary,paths.pending);
 }catch{}
}

/** 🆕️ At most `limit` priority projects (one wave unless told otherwise) that are fresh in this tree, not offered yet and accepted by `known`: the server's plan (pending file, written against the current priority) does not list them as dirty, so their records exist locally and only have to reach the fleet cache. Empty while the pending file does not answer the current priority. */
export function freshPriorityV1(environment,offered,limit=offering.maximumPerWave,known=()=>true){
 const paths=steeringPathsV1(environment),priority=readPriorityV1(environment),pending=paths&&readPendingV1(paths.directory);
 if(priority===undefined||pending===undefined||pending.priorityAtMs<priority.atMs)return[];
 const dirty=new Set(pending.projects),out=[];
 for(const name of new Set(priority.projects)){if(out.length>=limit)break;if(!dirty.has(name)&&!offered.has(name)&&known(name))out.push(name);}
 return out;
}

/** 📚️ The names an earlier call or read offered to the fleet cache and that have not outlived the time to live; nothing when no directory is named or the file is missing or damaged. */
export function readOfferedV1(environment=process.env,now=Date.now()){
 const paths=steeringPathsV1(environment),row=paths&&readJson(join(paths.directory,offering.file)),out=new Set();
 if(!row||row.version!==1||row.offered===null||typeof row.offered!=="object"||Array.isArray(row.offered))return out;
 for(const[name,at]of Object.entries(row.offered))if(Number.isSafeInteger(at)&&at>=0&&now-at<offering.ttlMilliseconds)out.add(name);
 return out;
}

/** 📝️ Notes names as offered at `now`: merges them into the file, drops the expired and, above the maximum, the oldest names, and replaces the file atomically. Never throws: a read does not depend on its notes. */
export function writeOfferedV1(environment,names,now=Date.now()){
 const paths=steeringPathsV1(environment);if(paths===undefined)return;
 try{
  const row=readJson(join(paths.directory,offering.file)),kept=row&&row.version===1&&row.offered!==null&&typeof row.offered==="object"&&!Array.isArray(row.offered)?Object.entries(row.offered).filter(([,at])=>Number.isSafeInteger(at)&&at>=0&&now-at<offering.ttlMilliseconds):[];
  const merged=new Map(kept);for(const name of names)merged.set(name,now);
  const offered=Object.fromEntries([...merged].sort((a,b)=>b[1]-a[1]).slice(0,offering.maximumNames).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0)),path=join(paths.directory,offering.file),temporary=path+"."+process.pid+".tmp";
  mkdirSync(paths.directory,{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,offered}));renameSync(temporary,path);
 }catch{}
}

/** 🌡️ The pending file of a read that has no generation to publish yet (the cold state: all owners are inferred before the first generation): `done(name)` notes an owner whose record is complete, and the file lists the priority projects that are still dirty, at most once per `intervalMs` (a trailing write follows), so the server can tell a requester which of its projects are fresh while the read goes on and not only at the end of the generation. `begin()` writes the first file at once, `flush()` the last one. */
export function createColdPendingV1(environment=process.env,intervalMs=1000,now=Date.now){
 const done=new Set();let last=-Infinity,timer;
 const write=()=>{timer=undefined;last=now();const priority=readPriorityV1(environment);if(priority===undefined)return;writePendingV1(environment,-1,priority.projects.filter(name=>!done.has(name)),priority);};
 return Object.freeze({
  begin:write,
  done(name){done.add(name);if(timer!==undefined)return;const wait=Math.max(0,intervalMs-(now()-last));if(wait===0)write();else{timer=setTimeout(write,wait);timer.unref?.();}},
  flush(){if(timer!==undefined){clearTimeout(timer);timer=undefined;}write();},
 });
}

/** 👂️ The pending file of the read: undefined when there is none or it is damaged. */
export function readPendingV1(directory){
 const row=readJson(join(directory,files.pending));
 return row&&row.version===1&&Number.isSafeInteger(row.generation)&&Number.isSafeInteger(row.atMs)&&Number.isSafeInteger(row.priorityAtMs)&&names(row.projects)?row:undefined;
}
