import {Database} from "bun:sqlite";

export type StageOperation={kind:"begin";units:number}|{kind:"advance";units:number}|{kind:"charge";units:number}|{kind:"scope";operations:StageOperation[]}|{kind:"reject"};
export interface StageResult {progress:{completed:number;total:number;ownedBytes:number};errors:string[]}
export interface StageCase {id:string;maximumBytes:number;cancel:null|{total:number;at:number};operations:StageOperation[];expected:StageResult}

/** 🪆️ Independent workload stack and SQLite cumulative ownership oracle. */
export function stageOracle(c:StageCase):StageResult {
  const db=new Database(":memory:");db.run("CREATE TABLE charges(bytes INTEGER NOT NULL)");
  let completed=0,total=0,started=false,stage=0;const errors:string[]=[];
  const owned=()=>Number((db.query("SELECT COALESCE(SUM(bytes),0) AS n FROM charges").get() as {n:number}).n);
  const checkpoint=()=>{if(c.cancel&&total===c.cancel.total&&completed>=c.cancel.at)throw new Error("canceled");started=true;};
  const run=(operations:StageOperation[])=>{for(const op of operations){
    if(op.kind==="begin"){stage++;completed=0;total=op.units;started=false;checkpoint();}
    else if(op.kind==="advance"){if(!started)checkpoint();const previous=completed;completed+=op.units;if(total!==0&&completed>total)throw new Error("overrun");if(Math.floor(previous/256)!==Math.floor(completed/256)||total!==0&&completed===total)checkpoint();}
    else if(op.kind==="charge"){if(owned()+op.units>c.maximumBytes)throw new Error("limit");if(!started||op.units>65536)checkpoint();db.run("INSERT INTO charges VALUES(?)",[op.units]);}
    else if(op.kind==="reject")throw new Error("rejected");
    else {const parent=[completed,total,started,stage] as const;try{run(op.operations);}catch(error){errors.push((error as Error).message);}finally{if(stage!==parent[3]){[completed,total,started,stage]=parent;}}}
  }};
  try{try{run(c.operations);}catch(error){errors.push((error as Error).message);}checkpoint();return {progress:{completed,total,ownedBytes:owned()},errors};}finally{db.close();}
}
