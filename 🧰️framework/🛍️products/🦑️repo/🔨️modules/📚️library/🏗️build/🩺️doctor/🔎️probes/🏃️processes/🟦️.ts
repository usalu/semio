import {commandRelevantV1,redactCommandV1,type ProcessFactsV1} from "../../🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";

/** 🏃️ The processes that matter to a stalled command: those matching the repo tool patterns, those that hold or wait for a lease or grant and all their descendants. */
export const processesProbeV1:DoctorProbeV1=Object.freeze({
 area:"processes",phase:2,
 async read(ports,context){
  const table=(await ports.processes()).map(row=>({...row,command:redactCommandV1(row.command)})),patterns=context.policy.processes.patterns.map(pattern=>new RegExp(pattern,"u")),excluded=context.policy.processes.exclude.map(pattern=>new RegExp(pattern,"u"));
  const children=new Map<number,ProcessFactsV1[]>();
  for(const row of table)children.set(row.parent,[...children.get(row.parent)??[],row]);
  const keep=new Map<number,ProcessFactsV1>();
  const add=(row:ProcessFactsV1):void=>{
   const stack=[row];
   while(stack.length){const current=stack.pop()!;if(keep.has(current.pid))continue;keep.set(current.pid,current);stack.push(...children.get(current.pid)??[]);}
  };
  for(const row of table)if(row.pid!==process.pid&&(context.known.has(row.pid)||commandRelevantV1(row.command,patterns,excluded)))add(row);
  return {sections:{processes:[...keep.values()].sort((left,right)=>left.pid-right.pid)}};
 },
});
