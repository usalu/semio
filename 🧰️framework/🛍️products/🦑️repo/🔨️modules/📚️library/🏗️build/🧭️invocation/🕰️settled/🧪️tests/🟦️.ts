/** ⌛️ Contract: while the files of the host service are edited in a burst, every client of the checkout asks for the implementation its service already runs, so the burst makes no new service generation; once the tree was quiet (or has differed for the maximum) the next client asks for the wanted implementation once and every later one agrees. The rule matches an independent SQLite oracle on the shared case table.
 * @see ../🟦️.ts
 * @see ../../🛰️plan/🟦️.ts */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import table from "./🔣️.json";
import {settleHostImplementationV1} from "../🟦️.ts";

const policy={quietMilliseconds:table.quietMilliseconds,maximumDeferralMilliseconds:table.maximumDeferralMilliseconds};

test("every case of the table is settled as the table says and as the SQLite oracle says",()=>{
 const db=new Database(":memory:");
 try{
  const oracle=db.query<{wanted:number},[number,number,number,number,number,number]>("SELECT (?3-?4>=?1 OR ?3-?2>=?5) AND ?6 AS wanted");
  for(const row of table.cases){
   const record=(row as {from?:{settled:string;firstAt?:number}}).from;
   const found=settleHostImplementationV1({wanted:row.wanted,record,now:row.nowMs,editedAt:row.editedAtMs,answers:row.answers,...policy});
   expect(found.implementation,row.name).toBe(row.implementation);
   expect(found.record,row.name).toEqual(row.to as never);
   if(record!==undefined&&record.settled!==row.wanted&&row.answers){
    const due=Boolean(oracle.get(policy.quietMilliseconds,record.firstAt??row.nowMs,row.nowMs,row.editedAtMs,policy.maximumDeferralMilliseconds,1)!.wanted);
    expect(found.implementation===row.wanted,row.name).toBe(due);
   }
  }
 }finally{db.close();}
});

test("a burst of edits makes no generation while it lasts and exactly one after it",()=>{
 for(const burst of table.bursts){
  let record:{settled:string;firstAt?:number}|undefined={settled:burst.settled},now=1_000_000,editedAt=now;const asked=new Set<string>();
  for(let edit=1;edit<=burst.edits;edit++){
   now+=burst.everyMilliseconds;editedAt=now;
   const found=settleHostImplementationV1({wanted:"h"+edit,record,now,editedAt,answers:true,...policy});record=found.record;asked.add(found.implementation);
  }
  expect([...asked],burst.name).toEqual([burst.settled]);
  now+=burst.quietMilliseconds;
  const after=new Set<string>();
  for(let client=0;client<5;client++){const found=settleHostImplementationV1({wanted:"h"+burst.edits,record,now:now+client,editedAt,answers:true,...policy});record=found.record;after.add(found.implementation);}
  expect([...after],burst.name).toEqual(["h"+burst.edits]);
 }
});

test("a successor starts beside the settled service only while fewer than the maximum generations live; a burst of edits over hours makes at most the maximum (SQL oracle)",()=>{
 const generations=(live:string[],maximumMilliseconds=1_800_000)=>({live,maximum:2,maximumMilliseconds}),db=new Database(":memory:");
 try{
  const oracle=db.query<{ride:number},[number,number,number,number,number,number,number]>("SELECT (?1>=?2 AND ?3=0 AND ?4-?5<?6 AND (?4-?7>=?8 OR ?4-?5>=?9)) AS ride");
  for(const live of[[],["s"],["s","old"],["s","wanted"],["a","b","c"]])for(const sinceFirst of[0,600_000,1_799_999,1_800_000,2_400_000]){
   const now=10_000_000,record={settled:"s",firstAt:now-sinceFirst},editedAt=now-95_000;
   const found=settleHostImplementationV1({wanted:"wanted",record,now,editedAt,answers:true,...policy,generations:generations(live)});
   const due=now-editedAt>=policy.quietMilliseconds||sinceFirst>=policy.maximumDeferralMilliseconds;
   const ride=Boolean(oracle.get(live.length,2,live.includes("wanted")?1:0,now,record.firstAt,1_800_000,editedAt,policy.quietMilliseconds,policy.maximumDeferralMilliseconds)!.ride)&&due;
   expect(found.implementation===record.settled,`${live.join(",")}/${sinceFirst}`).toBe(ride);
   if(!due)expect(found.implementation).toBe("s");
   if(!ride)expect(found.implementation).toBe("wanted");
  }
 }finally{db.close();}
 let record:{settled:string;firstAt?:number}|undefined={settled:"g0"},now=1_000_000;const live=new Set(["g0"]),started:string[]=[];
 for(let edit=1;edit<=40;edit++){
  now+=300_000;
  const found=settleHostImplementationV1({wanted:"g"+edit,record,now,editedAt:now-1000,answers:true,...policy,generations:generations([...live])});record=found.record;
  if(found.implementation==="g"+edit){started.push(found.implementation);live.add(found.implementation);}
  while(live.size>2)live.delete([...live][0]!);
 }
 expect(started.length).toBeLessThanOrEqual(Math.ceil(40*300_000/policy.maximumDeferralMilliseconds));
});
