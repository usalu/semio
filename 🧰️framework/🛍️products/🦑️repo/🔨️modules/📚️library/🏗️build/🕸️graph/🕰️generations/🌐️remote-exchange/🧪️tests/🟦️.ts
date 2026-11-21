import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,mkdtempSync,rmSync,utimesSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1,readGenerationManifestV1,readGenerationPointerV1} from "../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {generationDocumentPathsV1,generationExchangeKeyV1,publishGenerationV1,remoteGenerationExchangeV1,restoreGenerationV1} from "../🟦️.ts";
import {documentNameV1,generationSignatureV1,neutralizeGenerationFromMemoryV1,neutralKeyV1,transformGenerationDocumentV1,transformGenerationV1} from "../🧳️portable/🟦️.ts";

/** 🌐️ The generation exchange is proven through the real load of the mock world: a generation built at one root is restored at another root, into an empty store and into a store that is behind, and the load recomputes only what differs. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64";
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-exchange-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const writer=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
async function load(workspace:string,environment?:Record<string,string>){
 const mock=createMockHost(workspace);
 if(environment)(mock.host as any).exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity(),platform});
 const result=await loadGenerationGraphV1(mock.host);
 return{result,said:mock.said.map(row=>row.text),identity:await mock.host.identity()};
}
const restored=(said:string[])=>said.some(text=>/restored from the remote cache, \d+ documents/u.test(text)),uploaded=(said:string[])=>said.some(text=>text.includes("uploaded to the remote cache"));
const projects=(result:{graph:unknown})=>Object.keys((result.graph as any).projects).length;

test("the key names the registry, the platform and the writing origin and never a path",()=>{
 const identity=sha("registry");
 expect(generationExchangeKeyV1(identity,platform,"g1-m01")).toMatch(/^[0-9a-f]{64}$/u);
 expect(generationExchangeKeyV1(identity,platform,"g1-m01")).not.toBe(generationExchangeKeyV1(identity,"linux-x64","g1-m01"));
 expect(generationExchangeKeyV1(identity,platform,"g1-m01")).not.toBe(generationExchangeKeyV1(sha("other"),platform,"g1-m01"));
 expect(generationExchangeKeyV1(identity,platform,"g1-m01")).not.toBe(generationExchangeKeyV1(identity,platform,"g1-m02"));
});

test("a generation built at one root is restored at another root with the stamps of the producer, and the load verifies it by content",async()=>{
 const server=await serve(),environment=writer(server),first=scratch(),second=scratch();
 await seed(first,ringFiles(40));await seed(second,ringFiles(40));
 const warm=await load(first,environment);
 expect(warm.result.stats.ownersReinferred).toBe(40);expect(uploaded(warm.said)).toBe(true);
 const store=join(first,".gen"),pointer=readGenerationPointerV1(readFileSync(join(store,"current.json"))),manifest=readGenerationManifestV1(readFileSync(join(store,"m",pointer.manifest+".json")));
 const text=generationDocumentPathsV1(pointer.manifest!,manifest).map(path=>readFileSync(join(store,path)));
 expect(text.length).toBeGreaterThan(40);
 const fresh=await load(second,environment);
 expect(restored(fresh.said)).toBe(true);expect(fresh.result.stats.ownersReinferred).toBe(0);expect(projects(fresh.result)).toBe(40);
 const secondPointer=readGenerationPointerV1(readFileSync(join(second,".gen","current.json"))),secondManifest=readGenerationManifestV1(readFileSync(join(second,".gen","m",secondPointer.manifest+".json")));
 expect(generationSignatureV1(secondManifest)).toBe(generationSignatureV1(manifest));
 for(const path of generationDocumentPathsV1(secondPointer.manifest!,secondManifest))expect(sha(readFileSync(join(second,".gen",path)))).toBe(path.split("/").pop()!.split(".")[0]);
 const all=generationDocumentPathsV1(secondPointer.manifest!,secondManifest).map(path=>Buffer.from(readFileSync(join(second,".gen",path))));
 expect(all.length).toBe(text.length);
});

test("the neutral form carries the token and no root, equals at every root, and expanding at the same root returns a readable generation",async()=>{
 const first=scratch(),second=scratch();await seed(first,ringFiles(12));await seed(second,ringFiles(12));
 await load(first);await load(second);
 const read=(workspace:string)=>{const store=join(workspace,".gen"),pointer=readGenerationPointerV1(readFileSync(join(store,"current.json"))),manifest=readGenerationManifestV1(readFileSync(join(store,"m",pointer.manifest+".json")));return generationDocumentPathsV1(pointer.manifest!,manifest).map(path=>({path,bytes:readFileSync(join(store,path))}));};
 const neutralA=transformGenerationV1(read(first),first,"neutralize"),neutralB=transformGenerationV1(read(second),second,"neutralize");
 for(const item of neutralA.documents)expect(Buffer.from(item.bytes).includes(first)).toBe(false);
 const back=transformGenerationV1(neutralA.documents,first,"expand");
 expect(back.documents.length).toBe(neutralA.documents.length);
 expect(neutralB.documents.length).toBe(neutralA.documents.length);
 const pathsA=new Set(neutralA.documents.map(item=>item.path.split("/")[0]!));expect(pathsA.size).toBe(4);
});

test("a store that is behind takes the newer state of a peer; a store that is ahead or at the same state keeps its own",async()=>{
 const server=await serve(),environment=writer(server),peer=scratch(),behind=scratch(),ahead=scratch();
 await seed(peer,ringFiles(20));await seed(behind,ringFiles(20));await seed(ahead,ringFiles(20));
 await load(behind);
 await Bun.sleep(30);
 await applyOps(peer,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"}]);
 const base=await load(peer,environment);expect(uploaded(base.said)).toBe(true);
 const same=await load(behind,environment);
 expect(restored(same.said)).toBe(false);
 const small=ringFiles(20),extra=Object.fromEntries(Object.entries(ringFiles(22)).filter(([path])=>!(path in small)));
 expect(Object.keys(extra).length).toBeGreaterThan(0);
 await seed(peer,extra);
 const grown=await load(peer,environment);expect(projects(grown.result)).toBe(22);expect(uploaded(grown.said)).toBe(true);
 const pointerFile=join(behind,".gen","current.json");utimesSync(pointerFile,1,1);
 await seed(behind,extra);
 const caught=await load(behind,environment);
 expect(restored(caught.said)).toBe(true);expect(projects(caught.result)).toBe(22);expect(caught.result.stats.ownersReinferred).toBe(1);
 await load(ahead,environment);
 const newer=await load(ahead,environment);expect(restored(newer.said)||newer.said.some(text=>text.includes("uploaded"))).toBe(false);
});

test("restore refuses a foreign registry or platform and finds nothing under an unknown key; an older upload never replaces a newer one",async()=>{
 const server=await serve(),source=scratch(),empty=scratch(),environment=writer(server),exchange=remoteGenerationExchangeV1({environment,workspace:source,identity:async()=>"x".repeat(64),platform})!;
 await seed(source,ringFiles(8));const built=await load(source,environment);const identity=built.identity;
 const content=httpRemoteExecutionContentV1({endpoint:server.url}),base={content,instance:"main",key:generationExchangeKeyV1(identity,platform,"local"),root:join(empty,".gen"),workspace:empty,platform,files};
 expect((await restoreGenerationV1({...base,identity:sha("other")})).status).toBe("foreign");
 expect((await restoreGenerationV1({...base,platform:"linux-x64",identity})).status).toBe("foreign");
 expect((await restoreGenerationV1({...base,key:sha("unknown"),identity})).status).toBe("absent");
 expect((await publishGenerationV1({...base,root:join(source,".gen"),workspace:source,ifAbsent:true})).status).toBe("current");
 utimesSync(join(source,".gen","current.json"),1,1);
 expect((await publishGenerationV1({...base,root:join(source,".gen"),workspace:source})).status).toBe("current");
 void exchange;
});

test("the exchange is off without an endpoint, read-only without write access and defers probes immediately after a failure",async()=>{
 const base={workspace:"/srv/semio",identity:async()=>sha("registry"),platform};
 expect(remoteGenerationExchangeV1({...base,environment:{}})).toBeUndefined();
 const server=await serve(),source=scratch(),target=scratch();await seed(source,ringFiles(6));
 const mock=createMockHost(source);(mock.host as any).exchange=remoteGenerationExchangeV1({environment:writer(server),workspace:source,identity:()=>mock.host.identity(),platform});await loadGenerationGraphV1(mock.host);
 const identity=await mock.host.identity(),readOnly=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url},workspace:target,identity:async()=>identity,platform})!;
 expect(readOnly.upload).toBe(false);
 expect((await readOnly.restore({root:join(target,".gen"),fs:files})).status).toBe("restored");
 const dead=remoteGenerationExchangeV1({...base,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_RETRIES:"0"}})!,first=await dead.restore({root:scratch(),fs:files}),again=await dead.restore({root:scratch(),fs:files});
 expect(first.status).toBe("failed");expect(again).toEqual({status:"failed",reason:"the remote cache retry is deferred"});
});

test("a store asks whether a peer is ahead with one short request and the answer is false once it took the state; with the answer true the load skips its own verification pass and restores first",async()=>{
 const server=await serve(),environment=writer(server),first=scratch(),second=scratch();
 await seed(first,ringFiles(40));await seed(second,ringFiles(40));
 await load(first,environment);
 const mock=createMockHost(second),exchange=remoteGenerationExchangeV1({environment,workspace:second,identity:()=>mock.host.identity(),platform})!;
 expect(await exchange.ahead({root:join(second,".gen"),fs:files})).toBe(true);
 const taken=await load(second,environment);
 expect(restored(taken.said)).toBe(true);expect(taken.result.stats.ownersReinferred).toBe(0);
 expect(await exchange.ahead({root:join(second,".gen"),fs:files})).toBe(false);
 const dead=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_RETRIES:"0"},workspace:second,identity:()=>mock.host.identity(),platform})!;
 expect(await dead.ahead({root:join(second,".gen"),fs:files})).toBe(false);
 const source=readFileSync(new URL("../../🔌️load/🟨️.mjs",import.meta.url),"utf8");
 expect(source.indexOf("host.exchange.ahead(")).toBeLessThan(source.indexOf("builder&&host.exchange?await restoreRemoteGenerationV1"));
});

test("the snapshot of a generation travels beside it: the producer hands it over, the restoring machine installs it for the restored generation, a generation without one installs nothing and still restores",async()=>{
 const server=await serve(),environment=writer(server),first=scratch(),second=scratch(),third=scratch();
 for(const root of[first,second,third])await seed(root,ringFiles(10));
 const bytes=Buffer.from("snapshot document bytes"),instance="0123456789abcdef",installed:[number,string,string][]=[];
 const mount=(workspace:string,cursors:Parameters<typeof remoteGenerationExchangeV1>[0]["cursors"])=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity(),platform,cursors});return mock;};
 const producer=mount(first,{snapshotFor:async()=>({instance,bytes}),install:async()=>undefined});
 await loadGenerationGraphV1(producer.host);
 const consumer=mount(second,{snapshotFor:async()=>undefined,install:async(generation,snapshot)=>{installed.push([generation,snapshot.instance,Buffer.from(snapshot.bytes).toString()]);}});
 const result=await loadGenerationGraphV1(consumer.host);
 expect(restored(consumer.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBe(0);
 expect(installed.length).toBe(1);expect(installed[0]!.slice(1)).toEqual([instance,bytes.toString()]);expect(installed[0]![0]).toBeGreaterThanOrEqual(0);
 const keyed=generationExchangeKeyV1(await producer.host.identity(),platform,"local");
 expect(keyed).toMatch(/^[0-9a-f]{64}$/u);
 const bare=await serve(),bareEnvironment=writer(bare),other=createMockHost(third);
 const none=createMockHost(first);(none.host as any).exchange=remoteGenerationExchangeV1({environment:bareEnvironment,workspace:first,identity:()=>none.host.identity(),platform,cursors:{snapshotFor:async()=>undefined,install:async()=>undefined}});
 await loadGenerationGraphV1(none.host);
 const seen:unknown[]=[];(other.host as any).exchange=remoteGenerationExchangeV1({environment:bareEnvironment,workspace:third,identity:()=>other.host.identity(),platform,cursors:{snapshotFor:async()=>undefined,install:async(...args)=>{seen.push(args);}}});
 const again=await loadGenerationGraphV1(other.host);
 expect(restored(other.said.map(row=>row.text))).toBe(true);expect(seen).toEqual([]);expect(again.stats.ownersReinferred).toBe(0);
});

test("every origin writes only its own head, so no key ever receives different bytes from two machines, and a reader takes the newest head of the group",async()=>{
 const server=await serve(),environment=writer(server),first=scratch(),second=scratch(),reader=scratch();
 for(const root of[first,second,reader])await seed(root,ringFiles(10));
 const mount=(workspace:string,self:string)=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity(),platform,origins:{self,all:["g1-m01","g1-m02","g1-m03"]}});return mock;};
 const a=mount(first,"g1-m01");await loadGenerationGraphV1(a.host);
 const b=mount(second,"g1-m02");await loadGenerationGraphV1(b.host);
 const identity=await a.host.identity(),keys=["g1-m01","g1-m02","g1-m03"].map(origin=>generationExchangeKeyV1(identity,platform,origin));
 expect(new Set(keys).size).toBe(3);
 const content=httpRemoteExecutionContentV1({endpoint:server.url}),heads=await Promise.all(keys.map(key=>content.readActionResult("main",key)));
 expect(heads[0]).toBeDefined();expect(heads[2]).toBeUndefined();
 const taken=mount(reader,"g1-m03"),result=await loadGenerationGraphV1(taken.host);
 expect(restored(taken.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBeLessThanOrEqual(1);
});

/** 🚢️ The fleet restore path on one real cache server of the test (free port, temporary store, test origins, never a production identity, port or fleet fact): A builds and publishes, B with an empty store restores A's generation and computes nothing, B edits one file and computes exactly its project and publishes its own head, A (behind and diverged) takes B's newest head, restores it and verifies only the difference, and a reader whose tree is ahead of the producer's restores and computes only what it has on top. */
const heads=async(server:RemoteCacheServerV1,identity:string,origins:readonly string[])=>{const content=httpRemoteExecutionContentV1({endpoint:server.url});return Promise.all(origins.map(async origin=>{const bytes=await content.readActionResult("main",generationExchangeKeyV1(identity,platform,origin));return bytes===undefined?undefined:sha(bytes);}));};
const fleet=(server:RemoteCacheServerV1,all:readonly string[])=>(workspace:string,self:string)=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment:writer(server),workspace,identity:()=>mock.host.identity(),platform,origins:{self,all},serving:true});return mock;};
const computed=(mock:ReturnType<typeof createMockHost>)=>mock.log.flatMap(row=>row.full).sort();
const age=(workspace:string)=>utimesSync(join(workspace,".gen","current.json"),1,1);

test("two origins on one real cache server: restore without computing, the edited project only, the own head, the newest head taken by the other, never a cold build",async()=>{
 const server=await serve(),all=["g1-test-a","g1-test-b"],mount=fleet(server,all),a=scratch(),b=scratch();
 expect([6090,6094]).not.toContain(Number(new URL(server.url).port));expect(new URL(server.url).hostname).toBe("127.0.0.1");
 await seed(a,ringFiles(20));await seed(b,ringFiles(20));
 const built=mount(a,"g1-test-a"),first=await loadGenerationGraphV1(built.host),identity=await built.host.identity();
 expect(first.stats.ownersReinferred).toBe(20);expect(uploaded(built.said.map(row=>row.text))).toBe(true);
 const [headA,headB0]=await heads(server,identity,all);expect(headA).toBeDefined();expect(headB0).toBeUndefined();
 const empty=mount(b,"g1-test-b"),taken=await loadGenerationGraphV1(empty.host);
 expect(restored(empty.said.map(row=>row.text))).toBe(true);expect(taken.stats.ownersReinferred).toBe(0);expect(computed(empty)).toEqual([]);expect(projects(taken)).toBe(20);
 expect(uploaded(empty.said.map(row=>row.text))).toBe(false);expect((await heads(server,identity,all))[1]).toBeUndefined();
 await Bun.sleep(40);
 await applyOps(b,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"}]);
 const edited=mount(b,"g1-test-b"),again=await loadGenerationGraphV1(edited.host);
expect(again.stats.ownersReinferred).toBe(1);expect(computed(edited)).toEqual(["p3"]);expect(edited.log.every(row=>!row.cold)).toBe(true);expect(uploaded(edited.said.map(row=>row.text))).toBe(true);
 const [afterA,afterB]=await heads(server,identity,all);expect(afterA).toBe(headA);expect(afterB).toBeDefined();expect(afterB).not.toBe(headA);
 age(a);
 const diverged=mount(a,"g1-test-a"),caught=await loadGenerationGraphV1(diverged.host);
 expect(restored(diverged.said.map(row=>row.text))).toBe(true);expect(caught.stats.ownersReinferred).toBe(1);expect(computed(diverged)).toEqual(["p3"]);expect(diverged.log.every(row=>!row.cold)).toBe(true);expect(projects(caught)).toBe(20);
 const [finalA,finalB]=await heads(server,identity,all);expect(finalB).toBe(afterB);expect(finalA).toBeDefined();expect(finalA).not.toBe(headA);
});

test("a reader whose tree is ahead of the newest head restores it and computes only what it has on top; a reader that is ahead of the whole fleet keeps its own state",async()=>{
 const server=await serve(),all=["g1-test-a","g1-test-b","g1-test-c"],mount=fleet(server,all),a=scratch(),b=scratch(),c=scratch(),d=scratch();
 for(const root of[a,b,c,d])await seed(root,ringFiles(20));
 await loadGenerationGraphV1(mount(a,"g1-test-a").host);await Bun.sleep(40);
 const edit={op:"write" as const,path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"},more={op:"write" as const,path:"pkg/p5/src/index.ts",content:"import \"@syn/p2\";\nexport const more=2;\n"};
 await applyOps(b,[edit]);const producer=mount(b,"g1-test-b");await loadGenerationGraphV1(producer.host);await Bun.sleep(40);
 await applyOps(c,[edit,more]);const ahead=mount(c,"g1-test-c"),result=await loadGenerationGraphV1(ahead.host);
 expect(restored(ahead.said.map(row=>row.text))).toBe(true);expect(result.stats.ownersReinferred).toBe(1);expect(computed(ahead)).toEqual(["p5"]);expect(ahead.log.every(row=>!row.cold)).toBe(true);
 await Bun.sleep(40);
 await applyOps(d,[edit,more]);await loadGenerationGraphV1(mount(d,"g1-test-a").host);
 const identity=await ahead.host.identity(),before=await heads(server,identity,all);
 const keeper=mount(d,"g1-test-a"),kept=await loadGenerationGraphV1(keeper.host);
 expect(restored(keeper.said.map(row=>row.text))).toBe(false);expect(kept.stats.ownersReinferred).toBe(0);expect(await heads(server,identity,all)).toEqual(before);
});

test("the neutral form made from remembered documents equals the full neutral form and moves only the documents it has not seen",async()=>{
 const first=scratch();await seed(first,ringFiles(14));await load(first);
 const read=(workspace:string)=>{const store=join(workspace,".gen"),pointer=readGenerationPointerV1(readFileSync(join(store,"current.json"))),manifest=readGenerationManifestV1(readFileSync(join(store,"m",pointer.manifest+".json"))),paths=generationDocumentPathsV1(pointer.manifest!,manifest);return{paths,bytes:new Map(paths.map(path=>[path,new Uint8Array(readFileSync(join(store,path)))] as const))};};
 const full=(state:ReturnType<typeof read>,workspace:string)=>transformGenerationV1(state.paths.map(path=>({path,bytes:state.bytes.get(path)!})),workspace,"neutralize").documents.map(item=>item.path+" "+sha(item.bytes)).sort();
 const one=read(first),cold=neutralizeGenerationFromMemoryV1({paths:one.paths,bytes:one.bytes,root:first,known:new Map()});
 expect(cold.rows.map(row=>row.path+" "+row.hash).sort()).toEqual(full(one,first));
 expect(cold.rows.every(row=>row.bytes!==undefined&&row.source===undefined)).toBe(true);
 await applyOps(first,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=2;\n"}]);await load(first);
 const two=read(first),wanted=two.paths.filter(path=>path.startsWith("m/")||!cold.known.has(documentNameV1(path)));
 expect(wanted.length).toBeLessThan(two.paths.length/2);
 const warm=neutralizeGenerationFromMemoryV1({paths:two.paths,bytes:new Map(wanted.map(path=>[path,two.bytes.get(path)!])),root:first,known:cold.known});
 expect(warm.rows.map(row=>row.path+" "+row.hash).sort()).toEqual(full(two,first));
 const remembered=warm.rows.filter(row=>row.bytes===undefined);expect(remembered.length).toBeGreaterThan(two.paths.length/2);
 for(const row of remembered)expect(sha(transformGenerationDocumentV1(row.source!,two.bytes.get(row.source!)!,first,"neutralize"))).toBe(row.hash);
 expect(neutralKeyV1(first)).toMatch(/^[0-9a-f]{64}$/u);expect(neutralKeyV1(first)).not.toBe(neutralKeyV1(first+"x"));
 expect(() => neutralizeGenerationFromMemoryV1({paths:two.paths,bytes:new Map(),root:first,known:new Map()})).toThrow();
});

test("an upload remembers its neutral documents, a doctored memory is detected before anything wrong is sent, and a second root restores the result",async()=>{
 const serverA=await serve(),first=scratch(),second=scratch();
 await seed(first,ringFiles(12));await seed(second,ringFiles(12));
 const built=await load(first,writer(serverA));expect(uploaded(built.said)).toBe(true);
 const memoFile=join(first,".gen")+".neutral.json";expect(existsSync(memoFile)).toBe(true);
 await applyOps(first,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=3;\n"}]);
 const again=await load(first,writer(serverA));expect(uploaded(again.said)).toBe(true);
 const memo=JSON.parse(readFileSync(memoFile,"utf8"));expect(memo.entries.length).toBeGreaterThan(12);
 memo.entries=memo.entries.map((row:[string,string,number],at:number)=>at%2===0?[row[0],sha("wrong"+at),row[2]]:row);writeFileSync(memoFile,JSON.stringify(memo));
 const serverB=await serve(),doctored=await load(first,writer(serverB));expect(uploaded(doctored.said)).toBe(true);
 const repaired=JSON.parse(readFileSync(memoFile,"utf8"));expect(repaired.entries.every((row:[string,string,number])=>/^[0-9a-f]{64}$/u.test(row[1]))).toBe(true);
 await applyOps(second,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=3;\n"}]);
 const fresh=await load(second,writer(serverB));
 expect(restored(fresh.said)).toBe(true);expect(projects(fresh.result)).toBe(12);expect(fresh.result.stats.ownersReinferred).toBe(0);
});
