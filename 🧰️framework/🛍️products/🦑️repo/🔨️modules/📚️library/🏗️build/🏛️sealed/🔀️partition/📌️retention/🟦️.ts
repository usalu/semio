import {randomUUID} from "node:crypto";
import {mkdirSync} from "node:fs";
import {join,resolve} from "node:path";
import {SqliteDatabaseV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗃️sqlite/🟦️.ts";
import {readSealedLoaderV1,readSealedReceiptV1,readSealedSnapshotV1,sealedEntryOfV1,sealedHashV1,sealedPointerOfV1,sealedPolicyV1,type SealedPointerV1,type SealedPolicyV1} from "../../🟦️.ts";
import {sealedTreesCoherentV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

export type PartitionGroupV1=Readonly<{version:1;door:SealedPointerV1;kernels:readonly SealedPointerV1[]}>;
/** 🧬️ Constructs a closed group of distinct authenticated entry identities. */
export function partitionGroupV1(door:SealedPointerV1,kernels:readonly SealedPointerV1[]):PartitionGroupV1{
 if(!sealedPointerOfV1(door)||kernels.length<1||kernels.length>16||kernels.some(value=>!sealedPointerOfV1(value))||new Set([door.entry,...kernels.map(value=>value.entry)]).size!==kernels.length+1)throw Error("Distinct bounded sealed partition identities required");
 return Object.freeze({version:1,door,kernels:Object.freeze([...kernels].sort((a,b)=>a.entry<b.entry?-1:a.entry>b.entry?1:0))});
}
/** 🌱️ Lists the exact snapshot and loader roots retained by a group. */
export function partitionReferencesV1(group:PartitionGroupV1):readonly Readonly<{entry:string;digest:string;loader:string}>[]{return [group.door,...group.kernels].map(({entry,digest,loader})=>({entry,digest,loader}));}
const parse=(text:string):PartitionGroupV1|undefined=>{try{const value=JSON.parse(text);if(!value||Object.keys(value).sort().join(",")!=="door,kernels,version"||value.version!==1||!Array.isArray(value.kernels))return;return partitionGroupV1(value.door,value.kernels);}catch{return;}};
const groupKey=(door:string,kernels:readonly string[])=>sealedHashV1(JSON.stringify([door,[...kernels].sort()]));
function database(root:string,policy:SealedPolicyV1){const dir=join(resolve(root),...policy.cacheDirectory.split("/"));mkdirSync(dir,{recursive:true});const db=new SqliteDatabaseV1(join(dir,"partition.sqlite"));db.exec("PRAGMA busy_timeout=5000; CREATE TABLE IF NOT EXISTS groups (id TEXT PRIMARY KEY, current TEXT NOT NULL, previous TEXT NOT NULL); CREATE TABLE IF NOT EXISTS pins (token TEXT PRIMARY KEY, pid INTEGER NOT NULL, value TEXT NOT NULL)");return db;}
function livePins(db:SqliteDatabaseV1):PartitionGroupV1[]{const groups:PartitionGroupV1[]=[];for(const row of db.query<{token:string;pid:number;value:string},[]>("SELECT token,pid,value FROM pins").all()){if(!Number.isSafeInteger(row.pid)||row.pid<=0)throw Error("Corrupt sealed partition reader process");let alive=true;try{process.kill(row.pid,0);}catch(error){if((error as NodeJS.ErrnoException).code==="ESRCH")alive=false;}if(!alive){db.run("DELETE FROM pins WHERE token=?",row.token);continue;}const group=parse(row.value);if(!group)throw Error("Corrupt sealed partition reader");groups.push(group);}return groups;}
/** 🔒️ Serializes group publication, reader registration and collection under one crash-released store lock. */
export function withPartitionStoreV1<T>(root:string,policy:SealedPolicyV1,body:(db:SqliteDatabaseV1)=>T):T{const db=database(root,policy);try{db.exec("BEGIN IMMEDIATE");try{const result=body(db);db.exec("COMMIT");return result;}catch(error){db.exec("ROLLBACK");throw error;}}finally{db.close();}}
function usable(root:string,group:PartitionGroupV1,policy:SealedPolicyV1):boolean{
 const rows=[group.door,...group.kernels],trees=rows.map(pointer=>{const entry=sealedEntryOfV1(root,pointer.entry,policy),receipt=readSealedReceiptV1(entry,pointer.digest);return receipt?.loader===pointer.loader&&readSealedSnapshotV1(entry,pointer)!==undefined&&readSealedLoaderV1(entry,pointer)!==undefined?receipt.tree:undefined;});
 return trees.every(Boolean)&&trees.slice(1).every(tree=>sealedTreesCoherentV1(trees[0]!,tree!));
}
/** 🪪️ Atomically publishes a coherent group and pins it for the caller; a failed candidate retains the last usable group. */
export function acquirePartitionGroupV1(root:string,doorEntry:string,kernelEntries:readonly string[],candidate:PartitionGroupV1|undefined,policy:SealedPolicyV1=sealedPolicyV1):Readonly<{token:string;group:PartitionGroupV1;committed:boolean}>{
 const id=groupKey(doorEntry,kernelEntries),expected=[...kernelEntries].sort();
 return withPartitionStoreV1(root,policy,db=>{const row=db.query<{current:string;previous:string},[string]>("SELECT current,previous FROM groups WHERE id=?").get(id),old=row&&parse(row.current),matches=(group:PartitionGroupV1)=>group.door.entry===doorEntry&&JSON.stringify(group.kernels.map(value=>value.entry).sort())===JSON.stringify(expected),next=candidate&&matches(candidate)&&usable(root,candidate,policy)?candidate:undefined,group=next??(old&&matches(old)&&usable(root,old,policy)?old:undefined);if(!group)throw Error("No coherent authenticated partition group is available");
  if(next){const count=db.query<{count:number},[]>("SELECT COUNT(*) AS count FROM groups").get()?.count??0;if(!row&&count>=schema.$defs.limits.const.groups)throw Error("Sealed partition group capacity exceeded");const previous=old&&JSON.stringify(old)!==JSON.stringify(next)?[old,...(row?JSON.parse(row.previous):[])].slice(0,policy.keepPrevious):row?JSON.parse(row.previous):[];db.run("INSERT OR REPLACE INTO groups(id,current,previous) VALUES(?,?,?)",id,JSON.stringify(next),JSON.stringify(previous));}
  if(livePins(db).length>=schema.$defs.limits.const.readers)throw Error("Sealed partition reader capacity exceeded");const token=randomUUID();db.run("INSERT INTO pins(token,pid,value) VALUES(?,?,?)",token,process.pid,JSON.stringify(group));return Object.freeze({token,group,committed:next!==undefined});});
}
/** 📖️ Resolves a reader's exact pinned group, never an independently moving entry pointer. */
export function readPartitionPinV1(root:string,token:string,policy:SealedPolicyV1=sealedPolicyV1):PartitionGroupV1|undefined{return withPartitionStoreV1(root,policy,db=>{const row=db.query<{value:string},[string]>("SELECT value FROM pins WHERE token=?").get(token);return row?parse(row.value):undefined;});}
/** 🫴️ Registers the receiving process before the sender may release its own pin. */
export function adoptPartitionPinV1(root:string,token:string,policy:SealedPolicyV1=sealedPolicyV1):string{return withPartitionStoreV1(root,policy,db=>{const row=db.query<{value:string},[string]>("SELECT value FROM pins WHERE token=?").get(token),group=row&&parse(row.value);if(!group)throw Error("Original sealed partition reader is unavailable");if(livePins(db).length>=schema.$defs.limits.const.readers)throw Error("Sealed partition reader capacity exceeded");const adopted=randomUUID();db.run("INSERT INTO pins(token,pid,value) VALUES(?,?,?)",adopted,process.pid,JSON.stringify(group));return adopted;});}
/** 📤️ Releases the caller's reader pin after the sealed child exits. */
export function releasePartitionPinV1(root:string,token:string,policy:SealedPolicyV1=sealedPolicyV1):void{withPartitionStoreV1(root,policy,db=>{db.run("DELETE FROM pins WHERE token=?",token);});}
/** 🧹️ Holds publication and reader admission while the caller collects every protected root. */
export function withPartitionRootsV1<T>(root:string,policy:SealedPolicyV1,body:(roots:readonly Readonly<{entry:string;digest:string;loader:string}>[])=>T):T{return withPartitionStoreV1(root,policy,db=>{const groups:PartitionGroupV1[]=[];for(const row of db.query<{current:string;previous:string},[]>("SELECT current,previous FROM groups").all()){const current=parse(row.current);if(!current)throw Error("Corrupt sealed partition group");groups.push(current);const previous=JSON.parse(row.previous);if(!Array.isArray(previous)||previous.length>policy.keepPrevious)throw Error("Corrupt sealed partition history");for(const value of previous){const group=parse(JSON.stringify(value));if(!group)throw Error("Corrupt sealed partition history");groups.push(group);}}
 groups.push(...livePins(db));
 return body(groups.flatMap(partitionReferencesV1));});}
