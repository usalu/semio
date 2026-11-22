import {afterAll,expect,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createCasStoreV1} from "../../../../🗄️cas/🗃️store/🟦️.ts";
import {CAS_POLICY_V1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const hex=():string=>randomBytes(32).toString("hex");

/** ⏱️ Fills a store directory with fake entries (files and slot sidecars) without the store, so that a server of fifty thousand entries starts in seconds. */
function fill(directory:string,entries:number):string[]{
 const keys:string[]=[],made=new Set<string>(),slots=Array.from({length:Math.floor(entries/4)},hex),now=Date.now();
 const put=(path:string,data:string|Uint8Array)=>{const folder=path.slice(0,path.lastIndexOf("/"));if(!made.has(folder)){mkdirSync(folder,{recursive:true});made.add(folder);}writeFileSync(path,data);};
 for(let index=0;index<entries;index++){
  const key=hex(),relative=`main/ac/${key.slice(0,2)}/${key}`;keys.push(key);
  put(join(directory,"v1",...relative.split("/")),randomBytes(200));
  put(join(directory,"v1",".meta",...relative.split("/"))+".json",JSON.stringify({version:1,slot:slots[index%slots.length],storedAt:now-(entries-index)*50,refs:[]}));
 }
 return keys;
}

test("fifty thousand entries under concurrent requests, a lapse burst of claims and the maintenance and host-store ticks: the event loop never stalls and no request waits long",async()=>{
 const base=mkdtempSync(join(tmpdir(),"rcache-lag-"));scratch.push(base);
 const entries=Number(process.env.SEMIO_LAG_TEST_ENTRIES??50000),directory=join(base,"dir"),keys=fill(directory,entries);
 const host=createCasStoreV1({root:join(base,"cas"),policy:{...CAS_POLICY_V1,capBytes:1<<20,minimumCapBytes:1,enforceTickMilliseconds:500,minAgeMs:1}});
 const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"t"},validation:"off",hostStore:host,maintenanceSeconds:1,startupScan:"background"});
 const auth={authorization:"Bearer t"},gaps:number[]=[],latencies:number[]=[];let last=performance.now(),stop=false;
 const probe=setInterval(()=>{const at=performance.now();gaps.push(at-last-5);last=at;},5);
 try{
  for(let index=0;index<8000;index++)server.claims.claim("main","ac",hex(),{owner:"m1:1:aa",ttlMilliseconds:1000},false);
  const until=Date.now()+12000,worker=async(id:number)=>{
   while(Date.now()<until&&!stop){
    const started=performance.now(),pick=Math.random();let response:Response;
    if(pick<0.45)response=await fetch(`${server.url}/main/ac/${keys[Math.floor(Math.random()*keys.length)]}`,{headers:auth});
    else if(pick<0.7){const body=randomBytes(2000+id),digest=createHash("sha256").update(body).digest("hex");response=await fetch(`${server.url}/main/cas/${digest}`,{method:"PUT",headers:auth,body});}
    else if(pick<0.9)response=await fetch(`${server.url}/main/claim/ac/${hex()}`,{method:"POST",headers:auth,body:JSON.stringify({owner:`w${id}`,ttlMilliseconds:1000})});
    else response=await fetch(`${server.url}/healthz`);
    await response.arrayBuffer();latencies.push(performance.now()-started);
   }
  };
  await Promise.all(Array.from({length:16},(_,id)=>worker(id)));
 }finally{stop=true;clearInterval(probe);await server.stop();}
 gaps.sort((a,b)=>a-b);latencies.sort((a,b)=>a-b);
 const lag={max:gaps.at(-1)!,p99:gaps[Math.floor(gaps.length*0.99)]!},wait={p99:latencies[Math.floor(latencies.length*0.99)]!,max:latencies.at(-1)!};
 expect(latencies.length,"the load really ran").toBeGreaterThan(2000);
 expect(lag.max,`the longest stall of the event loop (${JSON.stringify({lag,wait})})`).toBeLessThan(500);
 expect(wait.p99,"p99 request latency").toBeLessThan(500);
},240000);
