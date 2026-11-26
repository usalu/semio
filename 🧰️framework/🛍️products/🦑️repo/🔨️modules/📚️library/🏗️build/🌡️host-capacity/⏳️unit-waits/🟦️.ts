import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import {alive,hostCapacityDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import {formatWaitV1,waitValuesV1,type UnitWaitV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import admission from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json" with {type:"json"};

const WAITS_V1=admission.definitions.Policy.const.waits,FOLDER_V1="unit-waits";

/** ⏳️ What one broker (a cargo invocation) publishes of its waiting compile units: the rows, when and by which process. */
export type UnitWaitRecordV1=Readonly<{v:1;pid:number;startMs?:number;atMs:number;rows:readonly UnitWaitV1[]}>;

/** ✍️ Publishes the waiting units of this process atomically below the gate directory (`unit-waits/<pid>.json`); no rows removes the file. */
export function writeUnitWaitsV1(directory:string,rows:readonly UnitWaitV1[],nowMs:number=Date.now(),pid:number=process.pid):void{
 const folder=join(directory,FOLDER_V1),path=join(folder,`${pid}.json`);
 if(rows.length===0){rmSync(path,{force:true});return;}
 mkdirSync(folder,{recursive:true});
 const record:UnitWaitRecordV1={v:1,pid,...pid===process.pid?{startMs:ownProcessIdentityV1().startMs}:{},atMs:nowMs,rows},temporary=`${path}.${process.pid}.tmp`;
 writeFileSync(temporary,JSON.stringify(record));renameSync(temporary,path);
}

/** 📖️ The live publications: a record of a dead process or older than three publish periods is dropped (and its file removed). */
export function readUnitWaitsV1(directory:string,nowMs:number=Date.now(),isAlive:(pid:number,startMs?:number)=>boolean=alive):readonly UnitWaitRecordV1[]{
 const folder=join(directory,FOLDER_V1),records:UnitWaitRecordV1[]=[];
 let names:string[];try{names=readdirSync(folder).filter(name=>name.endsWith(".json"));}catch{return records;}
 for(const name of names){
  const path=join(folder,name);let record:UnitWaitRecordV1;
  try{record=JSON.parse(readFileSync(path,"utf8")) as UnitWaitRecordV1;}catch{continue;}
  if(record.v!==1||!Array.isArray(record.rows)||nowMs-record.atMs>3*WAITS_V1.publishMilliseconds||!isAlive(record.pid,record.startMs)){try{rmSync(path,{force:true});}catch{}continue;}
  records.push(record);
 }
 return records.sort((left,right)=>left.pid-right.pid);
}

/** 📣️ The `repo capacity status` lines of the waiting units, one per unit, in the given language. */
export function unitWaitLinesV1(language:"en"|"de",records:readonly UnitWaitRecordV1[],nowMs:number=Date.now()):string[]{
 const lines:string[]=[];
 for(const record of records)for(const row of record.rows){
  const seconds=row.seconds+Math.max(0,Math.round((nowMs-record.atMs)/1000)),values={...waitValuesV1(row),seconds},name=row.reason==="growth"?"growth":row.reason==="idle"?"idle":"queued";
  lines.push(formatWaitV1(language,"row",{pid:record.pid,text:formatWaitV1(language,name,values)}));
 }
 return lines;
}

/** 📤️ The broker port that publishes into the gate directory of the environment; a failing write never disturbs a build. */
export function createUnitWaitPublisherV1(environment:NodeJS.ProcessEnv=process.env):(rows:readonly UnitWaitV1[])=>void{
 return rows=>{try{writeUnitWaitsV1(hostCapacityDirectoryV1(environment),rows);}catch{return;}};
}
