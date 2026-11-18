/** 🦶️ Proves a farm child is recycled for the memory it physically holds (its footprint), never for freed pages its resident set still counts: a macOS child that held 300 MB of footprint read 1.5 to 2.1 GB resident and was replaced after one to three owners, each replacement paying the ten second module boot again. */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{createFarmV1,GRAPH_FARM_POLICY_V1}from"../../🏊️pool/🟨️.mjs";

class Child extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;pid=4242;
 runs=0;constructor(readonly rss:number[]){super();queueMicrotask(()=>this.emit("message",{type:"ready",pid:4242,identity:"law"}));}
 send(message:{type:string;id?:number}){
  if(message.type==="run")queueMicrotask(()=>this.emit("message",{type:"result",id:message.id,reply:7,rssMegabytes:this.rss[Math.min(this.runs++,this.rss.length-1)]}));
  if(message.type==="shutdown")queueMicrotask(()=>this.finish());
  return true;
 }
 kill(){return true;}
 finish(){this.exitCode=0;this.connected=false;this.emit("exit",0,null);}
}

const served=async(rss:number[],footprint:(number|undefined)[],jobs:number)=>{let reads=0;
 const children:Child[]=[],reasons:string[]=[],pool=createFarmV1({entry:"law",service:"file:///law",session:{},workers:1,megabytes:1000,policy:{...GRAPH_FARM_POLICY_V1,recycleAfterProjects:100},accept:identity=>identity==="law",spawnProcess:(()=>{const child=new Child(rss);children.push(child);return child;})as never,footprintOf:async()=>{const value=footprint[Math.min(reads++,footprint.length-1)];return value===undefined?undefined:value*1048576;},report:(kind:string,fields:{reason:string})=>{if(kind==="recycled")reasons.push(fields.reason);}});
 try{for(let i=0;i<jobs;i++)expect((await pool.run({value:7}) as {reply:number}).reply).toBe(7);await Bun.sleep(20);return{started:children.length,reasons};}
 finally{for(const child of children)if(child.exitCode===null)child.finish();await pool.close();}
};

test("a child with a large resident set and a small footprint keeps serving",async()=>{
 expect(await served([200,2000],[200,300],4)).toEqual({started:1,reasons:[]});
});

test("a child whose footprint passed its budget is replaced after the owner that grew it",async()=>{
 const row=await served([300],[200,2000,200],3);
 expect(row.reasons).toEqual(["memory"]);
 expect(row.started).toBe(2);
});

test("a child whose footprint cannot be read is judged by its resident set",async()=>{
 const row=await served([200,2000],[undefined],2);
 expect(row.reasons).toEqual(["memory"]);
});
