import{readdirSync,readFileSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The authored numbers of the fleet view: how fresh a status must be to count as alive and how long each rank waits for the larger machine before it claims a heavy owner.
 * @see ./🧬️schema/🔣️.json */
export const OWNER_POOL_FLEET_POLICY_V1=schema.definitions.Policy.const;
export type OwnerPoolMachineV1={origin:string;totalMegabytes:number;grantMegabytes:number;state:string;rssMegabytes:number;updatedMs:number};
const TEXTS=schema.definitions.Texts.const;
const fill=(text:string,fields:Readonly<Record<string,string|number>>)=>text.replace(/\{(\w+)\}/gu,(_all,name)=>String(fields[name]??""));

/** 🖥️ The machine a pool status document describes; `undefined` for a document that is not a puller status with its size (older writers, other roles). */
export function ownerPoolMachineOfV1(doc:unknown):OwnerPoolMachineV1|undefined{
 const row=doc as Record<string,unknown>|null;
 if(row===null||typeof row!=="object"||row.version!==1||row.role!=="puller"||typeof row.origin!=="string"||typeof row.state!=="string")return undefined;
 const numbers=[row.totalMegabytes,row.grantMegabytes,row.updatedMs];
 if(!numbers.every(value=>typeof value==="number"&&Number.isFinite(value)&&value>=0))return undefined;
 return{origin:row.origin,totalMegabytes:Math.round(row.totalMegabytes as number),grantMegabytes:Math.round(row.grantMegabytes as number),state:row.state,rssMegabytes:Math.round(Number(row.rssMegabytes??0)),updatedMs:Math.round(row.updatedMs as number)};
}

/** 🌐️ Every machine of the fleet that reports a pool in the shared store (`<store>/v1/origins/<origin>/pool.json`); a missing or broken file is a machine that says nothing, never a fault. */
export function ownerPoolReadFleetV1(store:string|undefined):readonly OwnerPoolMachineV1[]{
 if(store===undefined||store==="")return[];
 try{
  const base=join(store,"v1","origins"),found:OwnerPoolMachineV1[]=[];
  for(const origin of readdirSync(base)){try{const machine=ownerPoolMachineOfV1(JSON.parse(readFileSync(join(base,origin,"pool.json"),"utf8")));if(machine!==undefined)found.push(machine);}catch{}}
  return found;
 }catch{return[];}
}

/** 🏭️ The machines that can produce an owner needing `need` MB: alive (a status younger than `aliveMilliseconds`, a pool that is not off or too small) and with a child grant that holds it, the one with the most free grant first (grant less what its children hold), the origin name breaking ties. */
export function ownerPoolProducersV1(machines:readonly OwnerPoolMachineV1[],need:number,now:number,policy:typeof OWNER_POOL_FLEET_POLICY_V1=OWNER_POOL_FLEET_POLICY_V1):readonly OwnerPoolMachineV1[]{
 const free=(machine:OwnerPoolMachineV1)=>machine.grantMegabytes-Math.min(machine.rssMegabytes,machine.grantMegabytes);
 return machines.filter(machine=>now-machine.updatedMs<policy.aliveMilliseconds&&machine.state!=="off"&&machine.state!=="too-small"&&machine.grantMegabytes>=need).slice().sort((a,b)=>free(b)-free(a)||(a.origin<b.origin?-1:a.origin>b.origin?1:0));
}

/** ⏱️ How long the machine `origin` waits before it claims an owner needing `need` MB: its rank among the producers times `turnMilliseconds`, so the largest free machine claims at once and the next one only when the owner still lies there after its turn; `undefined` when this machine is no producer for the owner. */
export function ownerPoolTurnMillisecondsV1(origin:string,machines:readonly OwnerPoolMachineV1[],need:number,now:number,policy:typeof OWNER_POOL_FLEET_POLICY_V1=OWNER_POOL_FLEET_POLICY_V1):number|undefined{
 const rank=ownerPoolProducersV1(machines,need,now,policy).findIndex(machine=>machine.origin===origin);
 return rank<0?undefined:rank*policy.turnMilliseconds;
}

/** 📜️ What the fleet looks like when no producer can hold an owner: the alive machines grouped by size, both languages in one line. */
export function ownerPoolNoProducerTextV1(machines:readonly OwnerPoolMachineV1[],need:number,now:number,policy:typeof OWNER_POOL_FLEET_POLICY_V1=OWNER_POOL_FLEET_POLICY_V1):string{
 const alive=machines.filter(machine=>now-machine.updatedMs<policy.aliveMilliseconds&&machine.state!=="off"&&machine.state!=="too-small"),groups=new Map<string,{count:number;gigabytes:number;grant:number}>();
 for(const machine of alive){const gigabytes=Math.round(machine.totalMegabytes/1024),key=gigabytes+"/"+machine.grantMegabytes,row=groups.get(key);groups.set(key,{count:(row?.count??0)+1,gigabytes,grant:machine.grantMegabytes});}
 const rows=[...groups.values()].sort((a,b)=>b.gigabytes-a.gigabytes);
 return(["en","de"] as const).map(language=>fill(TEXTS[language].noProducer,{need,sizes:rows.length===0?TEXTS[language].none:rows.map(row=>fill(TEXTS[language].sizes,row)).join(", ")})).join(" / ");
}
