import {afterEach,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtempSync,rmSync,statSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {generationExchangeKeyV1,generationImplementationKeyV1,generationKeepsLocalV1,publishGenerationV1,remoteGenerationExchangeV1} from "../../🟦️.ts";
import cases from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

/** 📥️ Contract: a worker takes a newer complete head of its own code and platform even while it holds owners and its own pointer was written after the state the head describes, once per publication of that head; the decision equals an independent SQL model on a table of cases, and implementations are compared under the platform key.
 * @see ../../🟦️.ts */
const platform="darwin-arm64",directories:string[]=[],servers:RemoteCacheServerV1[]=[];
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-newer-complete-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const writer=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
async function load(workspace:string,environment?:Record<string,string>){
 const mock=createMockHost(workspace);
 if(environment)(mock.host as any).exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity(),platform});
 const result=await loadGenerationGraphV1(mock.host);
 return{result,said:mock.said.map(row=>row.text)};
}
const restored=(said:string[])=>said.some(text=>/restored from the remote cache, \d+ documents/u.test(text)),uploaded=(said:string[])=>said.some(text=>text.includes("uploaded to the remote cache"));

test("the keep decision equals an independent SQL model on every case of the table",()=>{
 expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);
 using model=new Database(":memory:");
 const query=model.query(`SELECT CAST(($same OR ($state <= $local + $margin AND NOT ($stale IS NOT NULL AND $stale = 0 AND $published > $local AND ($reader IS NULL OR $head = $reader)))
  OR ($reader IS NOT NULL AND $head <> $reader AND $localCode = $reader)) AS INTEGER) AS keeps`);
 for(const row of cases){
  const sql=query.get({$same:row.sameSignatureFromOwnHead?1:0,$state:row.headStateMs,$published:row.headPublishedMs,$stale:row.headStale,$head:row.headImplementation,$local:row.localModifiedMs,$localCode:row.localImplementation,$reader:row.readerImplementation,$margin:row.marginMilliseconds}) as {keeps:number};
  const actual=generationKeepsLocalV1({sameSignatureFromOwnHead:row.sameSignatureFromOwnHead,headStateMs:row.headStateMs,headPublishedMs:row.headPublishedMs,headStale:row.headStale??undefined,headImplementation:row.headImplementation,localModifiedMs:row.localModifiedMs,localImplementation:row.localImplementation,readerImplementation:row.readerImplementation??undefined,marginMilliseconds:row.marginMilliseconds});
  expect(Boolean(sql.keeps),row.name+" (model)").toBe(row.keeps);
  expect(actual,row.name).toBe(row.keeps);
 }
});

test("implementations are compared under the platform key, never across platforms",()=>{
 const digest="d".repeat(64);
 expect(generationImplementationKeyV1("darwin-arm64",digest)).toMatch(/^[0-9a-f]{64}$/u);
 expect(generationImplementationKeyV1("darwin-arm64",digest)).toBe(generationImplementationKeyV1("darwin-arm64",digest));
 expect(generationImplementationKeyV1("darwin-arm64",digest)).not.toBe(generationImplementationKeyV1("linux-x64",digest));
 expect(generationImplementationKeyV1("darwin-arm64",digest)).not.toBe(generationImplementationKeyV1("darwin-arm64","e".repeat(64)));
});

const extraOwners=20,extra=Object.fromEntries(Object.entries(ringFiles(40)).filter(([path])=>!(path in ringFiles(20))));
const stage=async(producer:string,worker:string,environment:Record<string,string>,workerFirst:boolean)=>{
 await seed(producer,ringFiles(40));await seed(worker,ringFiles(20));
 await load(producer);
 const pointer=join(producer,".gen","current.json"),old=new Date(Date.now()-3_600_000);utimesSync(pointer,old,old);
 if(workerFirst){await Bun.sleep(30);await load(worker);await Bun.sleep(30);}
 const published=await load(producer,environment);expect(uploaded(published.said)).toBe(true);
 if(!workerFirst){await Bun.sleep(30);await load(worker);await Bun.sleep(30);}
 await seed(worker,extra);
};

test("a worker whose own pointer is newer than the state of a complete head still takes that head once it was published after the worker sealed, and keeps its own work afterwards",async()=>{
 const server=await serve(),environment=writer(server),producer=scratch(),worker=scratch();
 await stage(producer,worker,environment,true);
 const taken=await load(worker,environment);
 expect(restored(taken.said)).toBe(true);expect(taken.result.stats.ownersReinferred).toBeLessThan(extraOwners);expect(Object.keys((taken.result.graph as any).projects).length).toBe(40);
 const again=await load(worker,environment);expect(restored(again.said)).toBe(false);
});

test("a head published before the worker sealed its own state is not taken over it",async()=>{
 const server=await serve(),environment=writer(server),producer=scratch(),worker=scratch();
 await stage(producer,worker,environment,false);
 const kept=await load(worker,environment);
 expect(restored(kept.said)).toBe(false);expect(kept.result.stats.ownersReinferred).toBe(20);
});

test("a worker below the serving size publishes its complete head over a partial one however close, and a complete head is never shadowed by an equal one",async()=>{
 const server=await serve(),first=scratch(),second=scratch(),files=createNodeFilePortV1();
 await seed(first,ringFiles(12));await seed(second,ringFiles(14));
 const one=createMockHost(first),two=createMockHost(second);await loadGenerationGraphV1(one.host);await loadGenerationGraphV1(two.host);
 const identity=await one.host.identity(),keys=["g1-a","g1-b","g1-c"].map(origin=>generationExchangeKeyV1(identity,platform,origin)),content=httpRemoteExecutionContentV1({endpoint:server.url});
 const base=(workspace:string,at:number,origin:string)=>({content,instance:"main",key:keys[at]!,keys,root:join(workspace,".gen"),workspace,platform,files,origin});
 expect((await publishGenerationV1({...base(first,0,"g1-a"),stale:30,serving:true})).status).toBe("uploaded");
 expect((await publishGenerationV1({...base(second,1,"g1-b"),stale:0,serving:false})).status).toBe("uploaded");
 const third=scratch();await seed(third,ringFiles(16));const mock=createMockHost(third);await loadGenerationGraphV1(mock.host);
 expect((await publishGenerationV1({...base(third,2,"g1-c"),stale:0,serving:false})).status).toBe("remote-better");
 expect((await publishGenerationV1({...base(third,2,"g1-c"),stale:20,serving:false})).status).toBe("remote-better");
});
