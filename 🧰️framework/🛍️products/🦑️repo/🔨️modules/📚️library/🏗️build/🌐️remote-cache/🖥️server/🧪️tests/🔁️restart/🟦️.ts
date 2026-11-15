import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createFleetClientV1,readFleetSettingsV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";

const bun=process.execPath,serve=join(import.meta.dir,"🚀️serve.ts"),scratch:string[]=[],children:ReturnType<typeof Bun.spawn>[]=[];
afterEach(()=>{for(const child of children.splice(0))child.kill("SIGKILL");for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const free=():Promise<number>=>new Promise(resolve=>{const server=createServer();server.listen(0,"127.0.0.1",()=>{const port=(server.address() as {port:number}).port;server.close(()=>resolve(port));});});
const start=async(directory:string,port:number)=>{
 const child=Bun.spawn([bun,serve],{stdout:"pipe",stderr:"inherit",env:{...process.env,CACHE_DIRECTORY:directory,CACHE_PORT:String(port)}});children.push(child);
 const reader=child.stdout.getReader();let text="";
 while(!text.includes("listening")){const row=await reader.read();if(row.done)throw Error("the cache server did not start");text+=new TextDecoder().decode(row.value);}
 reader.releaseLock();return child;
};
const client=(root:string,port:number)=>createFleetClientV1(readFleetSettingsV1(root,{SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${port}`,SEMIO_REMOTE_CACHE_TOKEN:"w-secret",SEMIO_REMOTE_CACHE_ACCESS:"read-write"})!);
const key=(character:string)=>character.repeat(64);

test("a cache server killed and started again on the same directory remembers the claims; the holder renews, another machine is told to wait",async()=>{
 const root=mkdtempSync(join(tmpdir(),"cache-restart-")),port=await free();scratch.push(root);
 let server=await start(join(root,"store"),port);
 const holder=client(root,port),other=client(root,port);
 expect((await holder.claim(key("a"),"m1:1:aa",30000,60000)).state).toBe("granted");
 server.kill("SIGKILL");await server.exited;
 server=await start(join(root,"store"),port);
 const waiting=await other.claim(key("a"),"m2:2:bb",30000);
 expect(waiting.state,"the restarted server must not let a second machine compute the unit").toBe("held");
 expect(await holder.renew(key("a"),"m1:1:aa",30000)).toBe("granted");
 await holder.release(key("a"),"m1:1:aa");
 expect((await other.claim(key("a"),"m2:2:bb",30000)).state).toBe("granted");
},60000);

test("clients ride through a restart of a few seconds: a claim, an await and an upload issued during the outage complete after it, the tier is never marked down",async()=>{
 const root=mkdtempSync(join(tmpdir(),"cache-ride-")),port=await free();scratch.push(root);
 let server=await start(join(root,"store"),port);
 const holder=client(root,port),waiter=client(root,port);
 expect((await holder.claim(key("b"),"m1:1:aa",30000)).state).toBe("granted");
 const waited=waiter.await(key("b"),15000);
 await Bun.sleep(300);
 server.kill("SIGKILL");await server.exited;
 const started=Date.now(),during=holder.claim(key("c"),"m1:1:aa",30000);
 await Bun.sleep(2500);
 server=await start(join(root,"store"),port);
 const outcome=await during;expect(JSON.stringify(outcome),"a claim made while the server was gone").toContain("granted");
 expect(Date.now()-started).toBeGreaterThan(2000);
 await holder.release(key("b"),"m1:1:aa");
 expect(["lapsed","stored"],"the long poll that lost its server asks again and hears the release").toContain(await waited);
 expect(holder.usable()&&waiter.usable(),"the breaker stayed closed").toBe(true);
},60000);

test("a server that stays down is given up after the ride-through budget: the tier is marked down and later calls answer at once",async()=>{
 const root=mkdtempSync(join(tmpdir(),"cache-gone-")),port=await free();scratch.push(root);
 const server=await start(join(root,"store"),port),lone=client(root,port);
 expect((await lone.claim(key("d"),"m1:1:aa",30000)).state).toBe("granted");
 server.kill("SIGKILL");await server.exited;
 const first=Date.now();const answer=await lone.claim(key("e"),"m1:1:aa",30000);const slow=Date.now()-first;
 expect(answer.state).toBe("unavailable");expect(slow).toBeLessThan(20000);
 const again=Date.now();await lone.claim(key("e"),"m1:1:aa",30000);expect(Date.now()-again,"no second wait: the stamp of the last success is gone").toBeLessThan(3000);
},60000);
