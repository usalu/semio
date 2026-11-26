/** 🧬️ A unit never measured under its own key is predicted from its measured siblings, not from the cold size model. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createConnection} from "node:net";
import {admissionSiblingHistoryV1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type UnitObservationV1} from "../../🟦️.ts";
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1;
const row=(key:string,lines:number,peakMiB:number):UnitObservationV1=>({v:1,key,peakRssBytes:peakMiB*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines});
const history=()=>{const map=new Map<string,UnitObservationV1[]>();for(const item of fixture.rows)map.set(item.key,[...map.get(item.key)??[],row(item.key,item.lines,item.peakMiB)]);return map;};

test("siblings are the rows of the same owner, crate, target and crate types at a comparable size, as an independent pattern oracle sees them",()=>{
 const oracle=new Database(":memory:");
 try{
  for(const item of fixture.cases){
   const sibling=admissionSiblingHistoryV1(item.key,item.lines,history()).map(entry=>entry.peakRssBytes/MiB).sort((left,right)=>left-right);
   expect(sibling,item.name).toEqual([...item.peaksMiB].sort((left,right)=>left-right));
   const shape=(key:string)=>{const parts=/^([^|]+)\|([^|]+)\|[^|]+\|([^|]+)/u.exec(key)!;return `${parts[1]}|${parts[2]}|${parts[3]}`;};
   const near=(left:number,right:number)=>oracle.query("select ?1*100<=?2*125 and ?2*100<=?1*125 as near").get(left,right) as {near:number};
   const expected=fixture.rows.filter(candidate=>shape(candidate.key)===shape(item.key)&&near(candidate.lines,item.lines).near===1).map(candidate=>candidate.peakMiB).sort((left,right)=>left-right);
   expect(sibling,item.name).toEqual(expected);
  }
 }finally{oracle.close();}
});

test("a unit with a measured sibling is admitted by that measurement and refused only when the measurement exceeds the parent grant",async()=>{
 for(const item of fixture.broker){
  const crate="sibling_unit",unit:AdmitRequestV1={...base,pass:"check-00",crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1,source:"/nonexistent/sibling.rs",manifestDir:"/nonexistent"};
  const sibling=unitKeyV1(unit,"release"),broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:new Map([[sibling,[row(sibling,0,item.siblingMiB)]]]),parentMemoryAllowanceBytes:item.parentMiB*MiB,available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
  broker.registerPass({id:"check-00",profile:"check",target:"host"});
  const socket=createConnection(broker.endpoint);
  try{
   const reply=await new Promise<Record<string,unknown>>((accept,reject)=>{socket.once("error",reject);socket.once("data",chunk=>accept(JSON.parse(String(chunk).split("\n")[0]!)));socket.write(JSON.stringify(unit)+"\n");});
   expect(reply.op==="refused"?"refused":"admitted",item.name).toBe(item.expected);
  }finally{socket.destroy();await broker.close();}
 }
});
