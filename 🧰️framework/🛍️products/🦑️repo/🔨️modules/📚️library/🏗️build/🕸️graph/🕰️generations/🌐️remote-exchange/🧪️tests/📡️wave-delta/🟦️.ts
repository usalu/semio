import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,readFileSync,rmSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1,readGenerationManifestV1,readGenerationPointerV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1,renderGenerationMessageV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {generationDocumentPathsV1,generationExchangeKeyV1,publishGenerationV1,remoteGenerationExchangeV1} from "../../🟦️.ts";
import {createGenerationUploaderV1,localGenerationUploadSourceV1} from "../../../📡️wave-upload/🟦️.ts";
import {generationSignatureV1} from "../../🧳️portable/🟦️.ts";

/** 📡️ Contract (one real cache server on a free port with a temporary store and test origins, never a production identity, port or fleet fact): a server that publishes three waves after its first generation uploads three deltas, a reader started after each wave restores that wave's generation with no stale owner, a head uploaded later by a machine that holds an older state is not taken, a machine under the serving size does not shadow the server by a hair, and an interrupted upload resumes from what arrived. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64",all=["g1-test-server","g1-test-worker","g1-test-reader"];
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-wave-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const writer=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
const edit=(at:number,round:number)=>({op:"write" as const,path:`pkg/p${at}/src/index.ts`,content:`import "@syn/p1";\nexport const wave=${round};\n`});
const origins={self:"g1-test-server",all};
const mount=(server:RemoteCacheServerV1,workspace:string,self:string,extra:object={})=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment:writer(server),workspace,identity:()=>mock.host.identity(),platform,origins:{self,all},...extra});return mock;};
const state=(workspace:string)=>{const store=join(workspace,".gen"),pointer=readGenerationPointerV1(readFileSync(join(store,"current.json"))),manifest=readGenerationManifestV1(readFileSync(join(store,"m",pointer.manifest+".json")));return{pointer,manifest,paths:generationDocumentPathsV1(pointer.manifest!,manifest)};};
const restored=(said:string[])=>said.some(text=>/restored from the remote cache, \d+ documents/u.test(text));
const headOf=async(server:RemoteCacheServerV1,identity:string,origin:string)=>{const bytes=await httpRemoteExecutionContentV1({endpoint:server.url}).readActionResult("main",generationExchangeKeyV1(identity,platform,origin));return bytes===undefined?undefined:sha(bytes);};

test("a server that publishes three waves after its first generation uploads three deltas and a reader started after each wave restores that wave with no stale owner",async()=>{
 const server=await serve(),mother=scratch(),lines:string[]=[];let clock=1_800_000_000_000;
 await seed(mother,ringFiles(40));
 const uploader=createGenerationUploaderV1({source:localGenerationUploadSourceV1({workspace:mother,root:join(mother,".gen"),environment:writer(server),platform,origins}),language:"en",say:line=>lines.push(line),now:()=>clock});
 const treeEdits=[[edit(3,1),edit(5,1)],[edit(7,2)],[edit(9,3),edit(11,3),edit(13,3)]],applied:ReturnType<typeof edit>[]=[],sent:number[]=[],totals:number[]=[];
 let previous=new Set<string>();
 for(const[wave,ops]of[[0,[]],...treeEdits.map((ops,at)=>[at+1,ops] as const)] as [number,ReturnType<typeof edit>[]][]){
  if(ops.length){await Bun.sleep(30);await applyOps(mother,ops);applied.push(...ops);}
  const producer=createMockHost(mother);await loadGenerationGraphV1(producer.host);
  const now=state(mother);uploader.wave({generation:now.pointer.generation,done:ops.length,total:ops.length});
  clock+=4*60000;uploader.tick();await uploader.idle();
  const line=lines.at(-1)!,match=/(\d+) of (\d+) documents/u.exec(line);expect(line).toContain("uploaded to the fleet cache as a delta");expect(match).not.toBeNull();
  sent.push(Number(match![1]));totals.push(Number(match![2]));
  const added=now.paths.filter(path=>!previous.has(path));previous=new Set(now.paths);
  expect(sent.at(-1)).toBe(added.length);
  const reader=scratch();await seed(reader,ringFiles(40));if(applied.length)await applyOps(reader,applied);
  const taken=mount(server,reader,"g1-test-reader"),result=await loadGenerationGraphV1(taken.host);
  expect(restored(taken.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBe(0);
  const publishedHead=lines.at(-2)!,restoredHead=taken.said.find(row=>row.key==="restoredHead")?.text;
  expect(publishedHead).toMatch(/^graph: published fleet head origin=g1-test-server producerGeneration=\d+ scope=all manifest=[0-9a-f]{64}$/u);
  expect(restoredHead).toBe(publishedHead.replace("published fleet head","restored fleet head"));
  const restoredLine=taken.said.map(row=>row.text).find(text=>text.includes("restored from the remote cache"))!;
  expect(restoredLine).toMatch(/restored from the remote cache, \d+ documents \(\d+ MB\) downloaded in \d+ s, from a state \d+ min old with 0 projects stale$/u);
  expect(renderGenerationMessageV1("restored",{generation:7,owners:40,documents:5,megabytes:1,seconds:2,age:3,stale:9},"de")).toBe("Graph: Generation 7 (40 Projekte) aus dem Remote-Cache wiederhergestellt, 5 Dokumente (1 MB) in 2 s geladen, von einem Stand von vor 3 Min. mit 9 veralteten Projekten");
  expect(generationSignatureV1(state(reader).manifest)).toBe(generationSignatureV1(now.manifest));
  const behind=scratch();await seed(behind,ringFiles(40));await applyOps(behind,[...applied,edit(21,wave+9)]);
  const nearly=mount(server,behind,"g1-test-reader"),fewer=await loadGenerationGraphV1(nearly.host);
  expect(fewer.stats.ownersReinferred).toBeLessThanOrEqual(2);
 }
 expect(sent[0]).toBe(totals[0]);for(const at of[1,2,3])expect(sent[at]!).toBeLessThan(totals[at]!/2);
 expect(lines).toHaveLength(8);
},30000);

test("a head uploaded later by a machine that holds an older state is not taken, and a refused upload leaves no head", async()=>{
 const server=await serve(),serverRoot=scratch(),workerRoot=scratch(),readerRoot=scratch(),late=3_600_000;
 for(const root of[serverRoot,workerRoot,readerRoot])await seed(root,ringFiles(24));
 const oldState=createMockHost(workerRoot);await loadGenerationGraphV1(oldState.host);await Bun.sleep(30);
 const identity=await oldState.host.identity(),content=httpRemoteExecutionContentV1({endpoint:server.url}),base=(root:string,self:string)=>({content,instance:"main",key:generationExchangeKeyV1(identity,platform,self),keys:all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:self,root:join(root,".gen"),workspace:root,platform,files});
 const workerHead=await publishGenerationV1({...base(workerRoot,"g1-test-worker"),stale:300,now:()=>Date.now()+late});expect(workerHead.status).toBe("uploaded");
 const treeEdits=[edit(3,1),edit(8,1)];await applyOps(serverRoot,treeEdits);await applyOps(readerRoot,treeEdits);
 const built=createMockHost(serverRoot);await loadGenerationGraphV1(built.host);
 const serverHead=await publishGenerationV1({...base(serverRoot,"g1-test-server"),stale:5});expect(serverHead.status).toBe("uploaded");
 const heads=await Promise.all(["g1-test-server","g1-test-worker"].map(origin=>httpRemoteExecutionContentV1({endpoint:server.url}).readActionResult("main",generationExchangeKeyV1(identity,platform,origin))));
 expect(heads.every(head=>head!==undefined)).toBe(true);
 const refused=await publishGenerationV1({...base(workerRoot,"g1-test-worker"),stale:undefined,ifAbsent:false,now:()=>Date.now()+2*late});
 expect(["remote-better","current"]).toContain(refused.status);
 const taken=mount(server,readerRoot,"g1-test-reader"),result=await loadGenerationGraphV1(taken.host);
 expect(restored(taken.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBe(0);
});

test("a machine under the serving size must beat the head of the server by the shadow margin, a serving machine by any amount",async()=>{
 const server=await serve(),serverRoot=scratch(),workerRoot=scratch();
 for(const root of[serverRoot,workerRoot])await seed(root,ringFiles(200));
 await loadGenerationGraphV1(createMockHost(serverRoot).host);await applyOps(workerRoot,[edit(2,1)]);await loadGenerationGraphV1(createMockHost(workerRoot).host);
 const probe=createMockHost(serverRoot),identity=await probe.host.identity(),content=httpRemoteExecutionContentV1({endpoint:server.url}),base=(root:string,self:string)=>({content,instance:"main",key:generationExchangeKeyV1(identity,platform,self),keys:all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:self,root:join(root,".gen"),workspace:root,platform,files});
 expect((await publishGenerationV1({...base(serverRoot,"g1-test-server"),stale:150,serving:true})).status).toBe("uploaded");
 expect((await publishGenerationV1({...base(workerRoot,"g1-test-worker"),stale:120,serving:false})).status).toBe("remote-better");
 expect(await headOf(server,identity,"g1-test-worker")).toBeUndefined();
 expect((await publishGenerationV1({...base(workerRoot,"g1-test-worker"),stale:90,serving:false})).status).toBe("uploaded");
 expect((await publishGenerationV1({...base(workerRoot,"g1-test-worker"),stale:0,serving:true})).status).toMatch(/current|uploaded|remote-better/u);
 const hairs=await serve();
 expect((await publishGenerationV1({...base(serverRoot,"g1-test-server"),content:httpRemoteExecutionContentV1({endpoint:hairs.url}),stale:150,serving:true})).status).toBe("uploaded");
 expect((await publishGenerationV1({...base(workerRoot,"g1-test-worker"),content:httpRemoteExecutionContentV1({endpoint:hairs.url}),stale:149,serving:true})).status).toBe("uploaded");
});

test("an interrupted upload resumes: the documents that arrived are not sent again and the head appears only with the second try",async()=>{
 const server=await serve(),root=scratch();await seed(root,ringFiles(30));
 const mock=createMockHost(root);await loadGenerationGraphV1(mock.host);const identity=await mock.host.identity(),real=httpRemoteExecutionContentV1({endpoint:server.url});
 let written=0,failAt=12;
 const flaky={...real,write:async(...args:Parameters<typeof real.write>)=>{if(written>=failAt)throw new Error("connection lost");written++;return real.write(...args);},missing:real.missing.bind(real),read:real.read.bind(real),readActionResult:real.readActionResult.bind(real),writeActionResult:real.writeActionResult.bind(real)} as typeof real;
 const base={instance:"main",key:generationExchangeKeyV1(identity,platform,"g1-test-server"),keys:all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:"g1-test-server",root:join(root,".gen"),workspace:root,platform,files,stale:0};
 await expect(publishGenerationV1({...base,content:flaky})).rejects.toThrow("connection lost");
 expect(await headOf(server,identity,"g1-test-server")).toBeUndefined();
 await Bun.sleep(300);const arrived=written;expect(arrived).toBeGreaterThanOrEqual(failAt);
 const second=await publishGenerationV1({...base,content:real});
 expect(second.status).toBe("uploaded");expect(second.sent).toBeGreaterThan(0);expect(Math.abs(second.sent!+arrived-second.documents!)).toBeLessThanOrEqual(2);expect(second.sent!).toBeLessThan(second.documents!);
 expect(await headOf(server,identity,"g1-test-server")).toBeDefined();
 const third=await publishGenerationV1({...base,content:real});expect(third.status).toBe("current");
 const fresh=scratch();await seed(fresh,ringFiles(30));const taken=mount(server,fresh,"g1-test-reader"),result=await loadGenerationGraphV1(taken.host);
 expect(restored(taken.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBe(0);
});

test("the load names its code to the cache and calls every upload of a complete read fresh, so a worker restore ranks by it",async()=>{
 const server=await serve(),root=scratch();await seed(root,ringFiles(10));
 const mock=createMockHost(root),inner=remoteGenerationExchangeV1({environment:writer(server),workspace:root,identity:()=>mock.host.identity(),platform,origins})!,calls:{restore?:string;publish?:number}[]=[];
 (mock.host as any).exchange=Object.freeze({upload:inner.upload,restore:(options:Parameters<typeof inner.restore>[0])=>{calls.push({restore:options.implementation});return inner.restore(options);},ahead:(options:Parameters<typeof inner.ahead>[0])=>{calls.push({restore:options.implementation});return inner.ahead(options);},publish:(options:Parameters<typeof inner.publish>[0])=>{calls.push({publish:options.stale});return inner.publish(options);}});
 await loadGenerationGraphV1(mock.host);
 expect(calls.some(call=>call.publish===0)).toBe(true);
 expect(calls.filter(call=>"restore" in call).every(call=>/^[0-9a-f]{64}$/u.test(call.restore??""))).toBe(true);
});

test("when the reader can measure the distance between the tree of a head and its own, the nearest head wins over the newest",async()=>{
 const server=await serve(),a=scratch(),b=scratch(),reader=scratch(),treeA=[edit(3,1)],treeB=[edit(9,2)],near="aaaaaaaaaaaaaaaa",far="bbbbbbbbbbbbbbbb";
 for(const root of[a,b,reader])await seed(root,ringFiles(24));
 await applyOps(a,treeA);await applyOps(reader,treeA);await applyOps(b,treeB);
 await loadGenerationGraphV1(createMockHost(a).host);await Bun.sleep(30);await loadGenerationGraphV1(createMockHost(b).host);
 const identity=await createMockHost(a).host.identity(),content=httpRemoteExecutionContentV1({endpoint:server.url}),base=(root:string,self:string,instance:string)=>({content,instance:"main",key:generationExchangeKeyV1(identity,platform,self),keys:all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:self,root:join(root,".gen"),workspace:root,platform,files,cursors:{snapshotFor:async()=>({instance,bytes:Buffer.from("snapshot of "+instance)}),install:async()=>undefined}});
 expect((await publishGenerationV1({...base(a,"g1-test-server",near),stale:10})).status).toBe("uploaded");
 expect((await publishGenerationV1({...base(b,"g1-test-worker",far),stale:5})).status).toBe("uploaded");
 const asked:string[]=[],distances=new Map([[near,2],[far,5000]]);
 const sample=scratch();await seed(sample,ringFiles(24));await applyOps(sample,treeA);
 const without=createMockHost(sample);(without.host as any).exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url},workspace:sample,identity:()=>without.host.identity(),platform,origins:{self:"g1-test-reader",all}});const newest=await loadGenerationGraphV1(without.host);
 expect(newest.stats.ownersReinferred).toBe(2);
 const measured=mount(server,reader,"g1-test-reader",{cursors:{snapshotFor:async()=>undefined,install:async()=>undefined,distance:async(snapshot:{instance:string})=>{asked.push(snapshot.instance);return distances.get(snapshot.instance);}}}),result=await loadGenerationGraphV1(measured.host);
 expect(restored(measured.said.map(row=>row.text))).toBe(true);expect([...new Set(asked)].sort()).toEqual([near,far]);expect(result.stats.ownersReinferred).toBe(0);
});

test("a store that holds a generation of its own code keeps it against a newer head built by other code, and takes the head when it holds nothing of its own",async()=>{
 const server=await serve(),producer=scratch(),reader=scratch(),empty=scratch();
 for(const root of[producer,reader,empty])await seed(root,ringFiles(20));
 await seed(producer,{"impl.txt":"other code\n"});
 await loadGenerationGraphV1(createMockHost(reader).host);utimesSync(join(reader,".gen","current.json"),1,1);
 const built=createMockHost(producer,{implementationFiles:["impl.txt"]});(built.host as any).exchange=remoteGenerationExchangeV1({environment:writer(server),workspace:producer,identity:()=>built.host.identity(),platform,origins:{self:"g1-test-server",all}});
 await loadGenerationGraphV1(built.host);
 const keeper=mount(server,reader,"g1-test-reader"),kept=await loadGenerationGraphV1(keeper.host);
 expect(restored(keeper.said.map(row=>row.text))).toBe(false);expect(kept.stats.ownersReinferred).toBe(0);
 const fresh=mount(server,empty,"g1-test-worker"),taken=await loadGenerationGraphV1(fresh.host);
 expect(restored(fresh.said.map(row=>row.text))).toBe(true);expect(taken.stats.ownersReinferred).toBe(20);
});
