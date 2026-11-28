import {readdirSync,statfsSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const fleet=schema.definitions.Policy.const;

/** 📮️ The directory of the upload spool of a cache root. */
export const spoolDirectoryV1=(root:string)=>join(root,fleet.layout.state,fleet.layout.spool);

const bounds=new Map<string,{at:number;entries:number}>();

/** 📏️ How many units the spool of this cache root may hold: `spool.freeSharePercent` of the free room of its disk, one entry per `spool.entryBytes`, between `spool.minimumEntries` and `spool.maximumEntries` (a machine with a small disk still holds a build's worth, a big one never fills its disk with queue files). Re-measured every `spool.boundTtlMilliseconds`. This module reads only the policy and the file system, so a status reader never loads the uploader (and with it the revision and cache modules). */
export function spoolBoundV1(root:string):number{
 const policy=fleet.spool,known=bounds.get(root),now=Date.now();
 if(known!==undefined&&now-known.at<policy.boundTtlMilliseconds)return known.entries;
 let entries=policy.minimumEntries;
 try{const stats=statfsSync(root);entries=Math.floor(Number(stats.bavail)*Number(stats.bsize)*policy.freeSharePercent/100/policy.entryBytes);}catch{}
 entries=Math.min(policy.maximumEntries,Math.max(policy.minimumEntries,entries));
 bounds.set(root,{at:now,entries});return entries;
}

/** 🔢️ How many units wait in the spool. */
export function spoolLengthV1(root:string):number{try{return readdirSync(spoolDirectoryV1(root)).filter(name=>name.endsWith(".json")).length;}catch{return 0;}}
