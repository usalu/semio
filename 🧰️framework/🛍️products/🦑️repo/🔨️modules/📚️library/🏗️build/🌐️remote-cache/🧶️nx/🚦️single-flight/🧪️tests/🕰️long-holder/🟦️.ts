/** 🕰️ Contract: a task that runs for many minutes on one machine is awaited and fetched, never computed twice, by machines that ask minutes after it started; a holder that dies is replaced by exactly one waiter. Time is warped (one real second is `WARP` simulated seconds) so a 15 minute task takes seconds; the claim table, the claim routes and the front are the real ones, only the transport and the clock are modelled.
 * @see ../../../../🎟️claims/🟦️.ts */
import {expect,test} from "bun:test";
import {answerClaimRequestV1,createClaimTableV1} from "../../../../🎟️claims/🟦️.ts";
import {startNxFlightV1,type NxFlightV1} from "../../🟦️.ts";

const WARP=200,TASK_SIMULATED_MILLISECONDS=900_000,sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
const key="15698478355623763392";

function fleet(){
 const origin=Date.now(),table=createClaimTableV1({now:()=>origin+(Date.now()-origin)*WARP}),store=new Map<string,string>(),down=new Set<string>(),puts:string[]=[];
 const handler=async(request:Request):Promise<Response>=>{
  const url=new URL(request.url),parts=url.pathname.split("/").slice(1);
  const claimed=await answerClaimRequestV1({request,url,table,defaultInstance:"main",authorize:()=>undefined,readOnly:false,has:async(_namespace,_kind,id)=>store.has(id),readBody:async()=>new Uint8Array(await request.arrayBuffer())});
  if(claimed)return claimed;
  if(parts[1]==="v1"&&parts[2]==="cache"){
   const id=parts[3]!;
   if(request.method==="GET"){const value=store.get(id);return value===undefined?new Response("Not found\n",{status:404}):new Response(value,{status:200});}
   if(request.method==="PUT"){store.set(id,await request.text());table.stored("main","nx",id);return new Response("Stored",{status:200});}
  }
  return new Response("Not found\n",{status:404});
 };
 const transport=(machine:string):typeof fetch=>(async(input:Request|string|URL,init?:RequestInit)=>{
  if(down.has(machine))throw new Error("network down");
  return handler(new Request(input as string,init));
 }) as typeof fetch;
 return {table,store,down,puts,transport};
}

async function machine(world:ReturnType<typeof fleet>,name:string):Promise<NxFlightV1>{
 return startNxFlightV1({upstream:"http://fleet.test",instance:"main",token:"t",owner:`${name}:1:aa`,fetch:world.transport(name),timing:{claimTtlMilliseconds:30_000,heartbeatMilliseconds:30,awaitSliceMilliseconds:200,waitBudgetMilliseconds:25_000,claimTimeoutMilliseconds:1000,upstreamTimeoutMilliseconds:5000}});
}

const ask=(flight:NxFlightV1)=>fetch(`${flight.url}/v1/cache/${key}`,{headers:{authorization:`Bearer ${flight.token}`}});
const upload=(flight:NxFlightV1,body:string)=>fetch(`${flight.url}/v1/cache/${key}`,{method:"PUT",headers:{authorization:`Bearer ${flight.token}`,"content-type":"application/octet-stream"},body});

async function build(world:ReturnType<typeof fleet>,flight:NxFlightV1,name:string,computed:string[],body:string,stopAt?:()=>boolean):Promise<string>{
 const answer=await ask(flight);
 if(answer.status===200)return await answer.text();
 await answer.text();
 computed.push(name);
 const until=Date.now()+TASK_SIMULATED_MILLISECONDS/WARP;
 while(Date.now()<until){if(stopAt?.())return "died";await sleep(20);}
 await upload(flight,body);
 return body;
}

test("a 15 minute task started by one machine is awaited by the machines that ask at minute 3 and minute 9 and computed once",async()=>{
 const world=fleet(),flights:NxFlightV1[]=[];
 try{
  const computed:string[]=[];
  for(const name of["m08","m07","m03"])flights.push(await machine(world,name));
  const [first,second,third]=flights as [NxFlightV1,NxFlightV1,NxFlightV1];
  const one=build(world,first,"m08",computed,"result of m08");
  await sleep(3*60_000/WARP);
  const two=build(world,second,"m07",computed,"result of m07");
  await sleep(6*60_000/WARP);
  const three=build(world,third,"m03",computed,"result of m03");
  const answers=await Promise.all([one,two,three]);
  expect(computed).toEqual(["m08"]);
  expect(answers).toEqual(["result of m08","result of m08","result of m08"]);
  expect(second.counters()).toMatchObject({waited:1,served:1,duplicated:0,claimed:0});
  expect(third.counters()).toMatchObject({waited:1,served:1,duplicated:0,claimed:0});
  expect(world.table.summary().active).toBe(0);
 }finally{for(const flight of flights)await flight.stop();}
},60_000);

test("a holder that dies mid-task is replaced by exactly one waiter and the last machine receives that result",async()=>{
 const world=fleet(),flights:NxFlightV1[]=[];
 try{
  const computed:string[]=[];
  for(const name of["m08","m07","m03"])flights.push(await machine(world,name));
  const [first,second,third]=flights as [NxFlightV1,NxFlightV1,NxFlightV1];
  const one=build(world,first,"m08",computed,"result of m08",()=>world.down.has("m08"));
  await sleep(60_000/WARP);
  const two=build(world,second,"m07",computed,"result of m07");
  await sleep(60_000/WARP);
  const three=build(world,third,"m03",computed,"result of m03");
  await sleep(3*60_000/WARP);
  world.down.add("m08");
  const answers=await Promise.all([one,two,three]);
  expect(answers[0]).toBe("died");
  expect(answers[1]).toBe(answers[2]);
  expect(["result of m07","result of m03"]).toContain(answers[1]);
  expect(computed.filter(name=>name!=="m08")).toHaveLength(1);
  expect(computed).toHaveLength(2);
  expect(second.counters().taken+third.counters().taken).toBe(1);
  expect(second.counters().duplicated+third.counters().duplicated).toBe(0);
 }finally{for(const flight of flights)await flight.stop();}
},120_000);
