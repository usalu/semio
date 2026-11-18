/** 🪢️ Contract: a farm worker that differs from its parent AFTER the pool already accepted another one (a respawn that read a moved tree file at its own load: `🔣️taxonomy.json` edited at 07:01 UTC ended the farm of a 8-hour read at 2530 of 4775 owners and left the parent alone with the rest) is retired alone; the pool and its other workers go on and the slot is not started again. A pool whose every worker differs is still disabled, as is one whose first worker differs.
 * @see ../../🏊️pool/🟨️.mjs @see ../🟦️.ts (the first-worker law) */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{createFarmV1,GRAPH_FARM_MESSAGES_V1}from"../../🏊️pool/🟨️.mjs";
import farmSource from"../../🟨️.mjs"with{type:"text"};

class Child extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly pid:number,readonly identity:string,readonly log:number[]){super();setImmediate(()=>this.emit("message",{type:"ready",pid,identity}));}
 send(message:{type:string;id?:number}){
  if(message.type==="shutdown")this.end();
  if(message.type==="run"){this.log.push(this.pid);setTimeout(()=>this.emit("message",{type:"result",id:message.id,reply:{pid:this.pid,weight:1},rssMegabytes:10}),30);}
  return true;
 }
 end(){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
 kill(){this.end();return true;}
}

const open=(workers:number)=>{
 const state={spawned:0,moved:false,children:[] as Child[],ran:[] as number[],events:[] as string[]};
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers,megabytes:64,environment:{LANG:"en_US.UTF-8"},footprintOf:()=>undefined,
  spawnProcess:(()=>{const child=new Child(910000+state.spawned++,state.moved?"moved":"law",state.ran);state.children.push(child);return child;}) as never,
  accept:(identity:unknown)=>identity==="law",report:(event:string,fields:{worker?:number})=>{state.events.push(event+(fields.worker===undefined?"":":"+fields.worker));}} as never);
 return{farm,state};
};
const settle=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("a respawned worker that differs is retired alone: the pool goes on with the others and the slot is not started again",async()=>{
 const{farm,state}=open(2);
 expect(await farm.ready).toBe(true);await settle(60);
 expect(state.spawned).toBe(2);
 state.moved=true;state.children[0]!.end();await settle(60);
 const outcomes=await Promise.all([1,2,3,4].map(n=>farm.run({n})));
 for(const outcome of outcomes)expect((outcome.reply as {pid:number}).pid).toBe(state.children[1]!.pid);
 expect(state.events).toContain("refused:0");
 expect(state.events).not.toContain("disabled");
 expect(farm.usable).toBe(true);
 expect(state.spawned).toBe(3);
 const again=await farm.run({n:5});
 expect((again.reply as {pid:number}).pid).toBe(state.children[1]!.pid);expect(state.spawned).toBe(3);
 expect((farm as unknown as {snapshot:{refused:number}}).snapshot.refused).toBe(1);
 await farm.close();
},30000);

test("a pool whose every worker differs after one was accepted is disabled and its jobs are lost, never run",async()=>{
 const{farm,state}=open(1);
 expect(await farm.ready).toBe(true);await settle(40);
 state.moved=true;state.children[0]!.end();await settle(60);
 const outcome=await farm.run({n:1});
 expect(outcome.lost).toBe("disabled: identity differs");
 expect(farm.usable).toBe(false);expect(state.events).toContain("refused:0");expect(state.events).toContain("disabled");expect(state.ran).toEqual([]);
 await farm.close();
},30000);

test("the refusal of one worker is told in both languages and the line of a parked owner names a farm that is off",()=>{
 expect(GRAPH_FARM_MESSAGES_V1.en.workerRefused).toContain("{worker}");expect(GRAPH_FARM_MESSAGES_V1.de.workerRefused).toContain("{worker}");
 expect(farmSource).toContain('"the farm is off"');expect(farmSource).toContain('sayHub("workerRefused", fields)');
});
