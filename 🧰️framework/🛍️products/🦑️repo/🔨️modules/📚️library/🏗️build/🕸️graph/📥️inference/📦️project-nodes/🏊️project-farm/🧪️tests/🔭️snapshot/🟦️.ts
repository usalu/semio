/** 🔭️ Contract: a farm answers the live count of its slots (`snapshot`: busy, idle, starting, not started, queued jobs, whether a job runs alone, restarts), so the line of a parked owner names the children that really exist instead of the fixed words "no children" it printed for every pool, also while nine children ran; a read that waits on its children can tell a farm without children from a farm whose children are busy.
 * @see ../../🏊️pool/🟨️.mjs @see ../../🟨️.mjs `describe` */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{createFarmV1}from"../../🏊️pool/🟨️.mjs";

class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly pid:number){super();}
 send(message:{type:string}){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("a farm that started three children counts them in its snapshot and every slot is accounted for",async()=>{
 let pid=900000;
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers:3,megabytes:64,environment:{LANG:"en_US.UTF-8"},spawnProcess:(()=>new Scripted(pid++)) as never} as never);
 await new Promise(resolve=>setTimeout(resolve,50));
 const snapshot=(farm as unknown as {snapshot:Record<string,unknown>}).snapshot;
 expect(snapshot).toBeDefined();
 for(const key of ["busy","idle","starting","empty","queued","restarts"])expect(typeof snapshot[key]).toBe("number");
 expect(typeof snapshot.alone).toBe("boolean");
 expect((snapshot.busy as number)+(snapshot.idle as number)+(snapshot.starting as number)+(snapshot.empty as number)).toBe(3);
 expect(snapshot.starting).toBeGreaterThan(0);
 await farm.close();
},20000);

test("the line of a parked owner reads the snapshot the farm answers",()=>{
 const source=require("node:fs").readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8") as string;
 expect(source).toContain("pool.farm.snapshot");
});
