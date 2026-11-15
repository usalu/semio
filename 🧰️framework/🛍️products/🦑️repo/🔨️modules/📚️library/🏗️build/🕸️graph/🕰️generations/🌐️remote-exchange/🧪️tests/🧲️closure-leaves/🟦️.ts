import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {remoteGenerationExchangeV1,type ClosurePortV1} from "../../🟦️.ts";

/** 🧲️ Contract (one real cache server on a free port with a temporary store, never a production identity): the build input closure leaves a generation names travel with it. An upload sends each named leaf the cache misses once, a restore fetches the named leaves this machine lacks and verifies them against their names, and a leaf nobody holds is reported with the owners that name it (they are stale) without failing the restore. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64";
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-closures-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const environment=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
const NAMESPACE="build-input-closures" as const;
function port(base:string):ClosurePortV1{
 const directory=(namespace:string)=>join(base,namespace);
 return{directory,size:(namespace,key)=>{try{const row=statSync(join(directory(namespace),key+".json"));return row.isFile()?row.size:undefined;}catch{return undefined;}},read:async(namespace,key)=>{try{return readFileSync(join(directory(namespace),key+".json"));}catch{return undefined;}},put:async(namespace,key,bytes)=>{mkdirSync(directory(namespace),{recursive:true});writeFileSync(join(directory(namespace),key+".json"),bytes);}};
}
const leaf=(text:string)=>{const bytes=Buffer.from(JSON.stringify({files:[text]}));return{bytes,key:sha(bytes)};};
const tokens=(keys:Record<string,string>)=>Object.fromEntries(Object.entries(keys).map(([name,key])=>[`pkg/${name}/project.json`,JSON.stringify({name,package:"SEMIO_SOURCE_CLOSURE_"+key,uses:[]})]));
async function produce(server:RemoteCacheServerV1,workspace:string,closures:ClosurePortV1,named:Record<string,string>){
 await seed(workspace,{...ringFiles(6),...tokens(named)});
 const mock=createMockHost(workspace);
 (mock.host as any).exchange=remoteGenerationExchangeV1({environment:environment(server),workspace,identity:()=>mock.host.identity(),platform,closures});
 await loadGenerationGraphV1(mock.host);
 return mock;
}
const consumer=(server:RemoteCacheServerV1,workspace:string,closures:ClosurePortV1,identity:string)=>remoteGenerationExchangeV1({environment:environment(server),workspace,identity:async()=>identity,platform,closures})!;

test("an upload sends each named leaf once and a restore fetches the leaves this machine lacks, verified by name",async()=>{
 const server=await serve(),one=leaf("a.ts"),two=leaf("b.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);await motherLeaves.put(NAMESPACE,two.key,two.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:one.key,q1:two.key,q2:one.key}),content=httpRemoteExecutionContentV1({endpoint:server.url});
 expect(await content.has("main",one.key)).toBe(true);expect(await content.has("main",two.key)).toBe(true);
 const child=scratch(),childLeaves=port(scratch()),outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");expect(outcome.closures).toEqual({named:2,sent:0,fetched:2,missing:[]});
 expect(existsSync(join(childLeaves.directory(NAMESPACE),one.key+".json"))).toBe(true);expect(sha(readFileSync(join(childLeaves.directory(NAMESPACE),two.key+".json")))).toBe(two.key);
 expect(outcome.stale).toBe(0);
});

test("a leaf that is on this machine is not fetched and a leaf the fleet lacks is reported with the owners that name it, the restore still completes with those owners stale",async()=>{
 const server=await serve(),held=leaf("held.ts"),lost=leaf("lost.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,held.key,held.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:held.key,q1:lost.key,q3:lost.key});
 const content=httpRemoteExecutionContentV1({endpoint:server.url});expect(await content.has("main",lost.key)).toBe(false);
 const child=scratch(),childLeaves=port(scratch());await childLeaves.put(NAMESPACE,held.key,held.bytes);
 const outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");
 expect(outcome.closures).toEqual({named:2,sent:0,fetched:0,missing:[{key:lost.key,namespace:NAMESPACE,projects:["q1","q3"]}]});
 expect(outcome.stale).toBe(2);expect(existsSync(join(childLeaves.directory(NAMESPACE),lost.key+".json"))).toBe(false);
});

test("the leaf is taken from the fleet cache by the exchange and an absent leaf is nothing, not an error",async()=>{
 const server=await serve(),good=leaf("good.ts"),content=httpRemoteExecutionContentV1({endpoint:server.url}),exchange=remoteGenerationExchangeV1({environment:environment(server),workspace:scratch(),identity:async()=>sha("x"),platform,closures:port(scratch())})!;
 await content.write("main",{hash:good.key,size:good.bytes.length},good.bytes);
 expect(Buffer.from((await exchange.leaf({namespace:NAMESPACE,key:good.key}))!).equals(good.bytes)).toBe(true);
 expect(await exchange.leaf({namespace:NAMESPACE,key:sha("absent")})).toBeUndefined();
});

test("a later upload of the same generation lineage asks the cache nothing about the leaves it already sent and still lists them in its head, so a reader after the second upload gets them",async()=>{
 const server=await serve(),one=leaf("a.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:one.key});
 const sentFile=join(mother,".gen")+".closures-sent.json";expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([one.key]);
 await Bun.sleep(30);await applyOps(mother,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"}]);
 const again=createMockHost(mother);(again.host as any).exchange=remoteGenerationExchangeV1({environment:environment(server),workspace:mother,identity:()=>again.host.identity(),platform,closures:motherLeaves});
 await loadGenerationGraphV1(again.host);
 expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([one.key]);
 const child=scratch(),childLeaves=port(scratch()),outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");expect(outcome.closures).toEqual({named:1,sent:0,fetched:1,missing:[]});
});
