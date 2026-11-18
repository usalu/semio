import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {generationExchangeKeyV1,remoteGenerationExchangeV1} from "../../🟦️.ts";

/** 🔭️ A slice generation is keyed by its scope: machine A seals and publishes the generation of a slice, machine B of the same scope restores it and infers nothing (same tree) or exactly the edited owner (one edit), and a reader of another scope or of the whole workspace never adopts it. */
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),platform="darwin-arm64",directories:string[]=[],servers:RemoteCacheServerV1[]=[];
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-scope-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const origins=["g1-test-a","g1-test-b"],slice=sha("slice:a"),other=sha("slice:b");
const mount=(server:RemoteCacheServerV1,workspace:string,self:string,scope?:string)=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"},workspace,identity:()=>mock.host.identity(),platform,origins:{self,all:origins},...scope===undefined?{}:{scope}} as never);return mock;};
const restored=(mock:ReturnType<typeof createMockHost>)=>mock.said.some(row=>/restored from the remote cache, \d+ documents/u.test(row.text)),uploaded=(mock:ReturnType<typeof createMockHost>)=>mock.said.some(row=>row.text.includes("uploaded to the remote cache")),computed=(mock:ReturnType<typeof createMockHost>)=>mock.log.flatMap(row=>row.full).sort();

test("the key of the whole workspace is the key without a scope, every slice has its own key",()=>{
 const identity=sha("registry");
 expect(generationExchangeKeyV1(identity,platform,"g1-m01","all")).toBe(generationExchangeKeyV1(identity,platform,"g1-m01"));
 expect(generationExchangeKeyV1(identity,platform,"g1-m01",slice)).not.toBe(generationExchangeKeyV1(identity,platform,"g1-m01"));
 expect(generationExchangeKeyV1(identity,platform,"g1-m01",slice)).not.toBe(generationExchangeKeyV1(identity,platform,"g1-m01",other));
});

test("machine B of a slice scope restores the generation of machine A without inferring, and with one edited owner infers that owner only",async()=>{
 const server=await serve(),a=scratch(),b=scratch();
 await seed(a,ringFiles(12));await seed(b,ringFiles(12));
 const sealed=mount(server,a,"g1-test-a",slice),built=await loadGenerationGraphV1(sealed.host);
 expect(built.stats.ownersReinferred).toBe(12);expect(uploaded(sealed)).toBe(true);
 const same=mount(server,b,"g1-test-b",slice),taken=await loadGenerationGraphV1(same.host);
 expect(restored(same)).toBe(true);expect(taken.stats.ownersReinferred).toBe(0);expect(computed(same)).toEqual([]);expect(Object.keys((taken.graph as any).projects).length).toBe(12);
 await Bun.sleep(40);
 await applyOps(b,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"}]);
 const edited=mount(server,b,"g1-test-b",slice),again=await loadGenerationGraphV1(edited.host);
 expect(again.stats.ownersReinferred).toBe(1);expect(computed(edited)).toEqual(["p3"]);expect(edited.log.every(row=>!row.cold)).toBe(true);
});

test("a reader of the whole workspace and a reader of another slice never adopt the generation of a slice",async()=>{
 const server=await serve(),a=scratch(),whole=scratch(),second=scratch();
 await seed(a,ringFiles(12));await seed(whole,ringFiles(40));await seed(second,ringFiles(12));
 await loadGenerationGraphV1(mount(server,a,"g1-test-a",slice).host);
 const full=mount(server,whole,"g1-test-b"),all=await loadGenerationGraphV1(full.host);
 expect(restored(full)).toBe(false);expect(all.stats.ownersReinferred).toBe(40);
 const wrong=mount(server,second,"g1-test-b",other),cold=await loadGenerationGraphV1(wrong.host);
 expect(restored(wrong)).toBe(false);expect(cold.stats.ownersReinferred).toBe(12);
});

test("the slice reader finds the slice head after the whole-workspace head was written beside it",async()=>{
 const server=await serve(),a=scratch(),whole=scratch(),reader=scratch();
 await seed(a,ringFiles(12));await seed(whole,ringFiles(40));await seed(reader,ringFiles(12));
 await loadGenerationGraphV1(mount(server,a,"g1-test-a",slice).host);await loadGenerationGraphV1(mount(server,whole,"g1-test-a").host);
 const mine=mount(server,reader,"g1-test-b",slice),result=await loadGenerationGraphV1(mine.host);
 expect(restored(mine)).toBe(true);expect(result.stats.ownersReinferred).toBe(0);expect(Object.keys((result.graph as any).projects).length).toBe(12);
});
