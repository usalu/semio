/** 🧹 Which spilled plan files of the host service to remove: a pure decision over file ages, sizes and live references, so retention is bounded by age, bytes and count and never takes a file a running invocation can still read. */

/** 🛡️ The names to remove; a file is kept when it is live (named in `live`) or younger than `retainMilliseconds`, since every process the invocation starts reads the same content-addressed file at its own start; beyond that, files older than `maximumAgeMilliseconds` go, then the oldest unprotected files go until the total is within `maximumBytes` and `maximumEntries`. Leftover `.tmp` files of failed writes go once past the retain window. Names that are neither `.json` nor `.tmp` are never touched. */
export function planRetentionV1({files,live=[],policy}){
 const kept=new Set(live),young=file=>file.ageMilliseconds<policy.retainMilliseconds,remove=[];
 const plans=files.filter(file=>file.name.endsWith(".json")),candidates=plans.filter(file=>!kept.has(file.name)&&!young(file)).sort((a,b)=>b.ageMilliseconds-a.ageMilliseconds||(a.name<b.name?-1:1));
 for(const file of files)if(file.name.endsWith(".tmp")&&!young(file))remove.push(file.name);
 let bytes=plans.reduce((sum,file)=>sum+file.bytes,0),count=plans.length;
 for(const file of candidates){
  if(file.ageMilliseconds>policy.maximumAgeMilliseconds||bytes>policy.maximumBytes||count>policy.maximumEntries){remove.push(file.name);bytes-=file.bytes;count--;}
 }
 return remove;
}
