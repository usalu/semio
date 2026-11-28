import {statfs} from "node:fs/promises";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🛑️ The disk rule of the apply: the bytes and the share of the volume that must stay free after a write, and how long one reading of the volume is reused. */
export const APPLY_GUARD_POLICY_V1=schema.definitions.Policy.const;
/** 📐️ The volume as `statfs` reports it: the bytes available to this user and the size. */
export type ApplySpaceV1=Readonly<{free:number;total:number}>;
/** 🩺️ One verdict: `short` when the write must wait, with the reading and the reserve that judged it, and the words in both languages. */
export type ApplyGuardVerdictV1=Readonly<{short:boolean;free:number;total:number;reserve:number;message:string}>;

const gib=(bytes:number):string=>(bytes/2**30).toFixed(1);

/** 🧮️ The reserve that must stay free after a write on a volume of `total` bytes: `reserveBytes` plus `reserveBasisPoints` of the volume. */
export function applyReserveV1(total:number):number{return APPLY_GUARD_POLICY_V1.reserveBytes+Math.ceil(total*APPLY_GUARD_POLICY_V1.reserveBasisPoints/10000);}

/** 🚦️ Judges a write of `incomingBytes`: it waits when the volume would keep less than the reserve free. A volume that cannot be measured (`undefined`, or the row of zeros that some runtimes answer on Windows) never holds a write back, because the write error of the file system is then the only truth.
 * @see ./🧪️tests/🟦️.ts */
export function judgeApplyV1(space:ApplySpaceV1|undefined,incomingBytes:number=0):ApplyGuardVerdictV1{
 if(space===undefined||!(space.total>0)||!Number.isFinite(space.free))return Object.freeze({short:false,free:space?.free??0,total:space?.total??0,reserve:0,message:""});
 const reserve=applyReserveV1(space.total),short=space.free-Math.max(0,incomingBytes)<reserve;
 return Object.freeze({short,free:space.free,total:space.total,reserve,message:short?`Disk short: ${gib(space.free)} GiB free, ${gib(reserve)} GiB must stay free; files from the store wait, deletions go on. / Platte knapp: ${gib(space.free)} GiB frei, ${gib(reserve)} GiB müssen frei bleiben; Dateien aus dem Store warten, Löschungen laufen weiter.`:""});
}

/** 🔌️ The reading of the volume of a directory, `undefined` when it cannot be read. */
export async function readApplySpaceV1(directory:string):Promise<ApplySpaceV1|undefined>{
 try{const info=await statfs(directory);return {free:Number(info.bavail)*Number(info.bsize),total:Number(info.blocks)*Number(info.bsize)};}catch{return undefined;}
}

/** 🛡️ The guard of one replica: asks the volume of `directory` at most once per `cacheMilliseconds` and judges every write of the turn by that reading. `read` and `now` are injectable.
 * @see ./🧪️tests/🟦️.ts */
export function createApplyGuardV1(input:Readonly<{directory:string;now:()=>number;read?:(directory:string)=>Promise<ApplySpaceV1|undefined>}>):Readonly<{check(incomingBytes?:number):Promise<ApplyGuardVerdictV1>}>{
 const read=input.read??readApplySpaceV1;
 let at=-Infinity,space:ApplySpaceV1|undefined,pending:Promise<void>|undefined;
 return Object.freeze({async check(incomingBytes:number=0):Promise<ApplyGuardVerdictV1>{
  if(input.now()-at>=APPLY_GUARD_POLICY_V1.cacheMilliseconds){
   pending??=read(input.directory).then(value=>{space=value;at=input.now();}).finally(()=>{pending=undefined;});
   await pending;
  }
  return judgeApplyV1(space,incomingBytes);
 }});
}
