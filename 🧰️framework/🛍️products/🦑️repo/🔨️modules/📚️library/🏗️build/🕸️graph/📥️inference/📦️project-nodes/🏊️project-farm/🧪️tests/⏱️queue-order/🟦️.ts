/** ⏱️ Contract: the queue of a farm serves the shortest expected job first (a look before a verification before a warm settling before an inference, an inference by the megabytes the cost table knows for its owner), with aging so that no long owner waits longer than about its own cost, a job sent back stays in front, and equal scores keep their arrival order; the order is the same in an independent implementation (python) and in the pump of a real farm with scripted children.
 * @see ../../🏊️pool/🟨️.mjs `farmQueueOrderV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{EventEmitter}from"node:events";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import fixture from"./🔣️.json";
import{GRAPH_FARM_POLICY_V1,createFarmV1,farmJobExpectedMillisecondsV1,farmQueueOrderV1}from"../../🏊️pool/🟨️.mjs";

const policy={...GRAPH_FARM_POLICY_V1,queue:fixture.queue};
const order=(row:typeof fixture.cases[number])=>farmQueueOrderV1(row.jobs.map(job=>({id:job.id,expected:farmJobExpectedMillisecondsV1(job.payload,policy),queued:job.queued,front:(job as{front?:boolean}).front===true})),row.now,policy).map(job=>job.id);

test("the order of the fixture cases, and the live policy carries the declared queue keys",()=>{
 for(const row of fixture.cases)expect(order(row),row.name).toEqual(row.expected);
 expect(GRAPH_FARM_POLICY_V1.queue).toEqual(fixture.queue);
});

test("an independent python implementation orders every case the same",()=>{
 const ran=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(ran.status,ran.stderr).toBe(0);
 const said=JSON.parse(ran.stdout);
 for(const row of fixture.cases){expect(said[row.name],row.name).toEqual(row.expected);expect(order(row)).toEqual(said[row.name]);}
});

class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly ran:{id:number;job:any}[]){super();queueMicrotask(()=>this.emit("message",{type:"ready",pid:7,identity:"law"}));}
 send(message:{type:string;id?:number;jobFile?:string}){
  if(message.type==="run")this.ran.push({id:message.id!,job:JSON.parse(readFileSync(message.jobFile!,"utf8"))});
  if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit",0,null));}
  return true;
 }
 answer(id:number){this.emit("message",{type:"result",id,reply:1,rssMegabytes:1});}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit",0,null));return true;}
}

test("a real farm with one child serves a look, a verification and a warm job before a long owner that queued first",async()=>{
 const ran:{id:number;job:any}[]=[];let child!:Scripted;
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers:1,megabytes:64,environment:{LANG:"en_US.UTF-8"},accept:(identity:unknown)=>identity==="law",spawnProcess:(()=>{child=new Scripted(ran);return child;})as never} as never);
 const blocker=farm.run({tag:"blocker",observe:true});await Bun.sleep(20);
 expect(ran.map(row=>row.job.tag)).toEqual(["blocker"]);
 const pending=[farm.run({tag:"rust",costMegabytes:2500}),farm.run({tag:"warm",warm:true}),farm.run({tag:"verify",observe:true}),farm.run({tag:"look",look:true,owners:[1]})];
 await Bun.sleep(5);
 const served:string[]=[];
 for(let step=0;step<5;step++){const last=ran[ran.length-1]!;if(last.job.tag!=="blocker"&&served[served.length-1]!==last.job.tag)served.push(last.job.tag);child.answer(last.id);await Bun.sleep(20);}
 await Promise.all([blocker,...pending]);
 expect(ran.map(row=>row.job.tag)).toEqual(["blocker","look","verify","warm","rust"]);
 await farm.close();
},20000);
