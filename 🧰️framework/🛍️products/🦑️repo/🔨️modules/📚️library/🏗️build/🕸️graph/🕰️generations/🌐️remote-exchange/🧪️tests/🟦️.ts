import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1,readGenerationManifestV1,readGenerationPointerV1} from "../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {generationDocumentPathsV1,generationExchangeKeyV1,publishGenerationV1,remoteGenerationExchangeV1,restoreGenerationV1} from "../🟦️.ts";
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],identity=sha("registry"),workspace="/srv/semio",key=generationExchangeKeyV1(identity,workspace);
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-exchange-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
function put(root:string,directory:string,text:string,extension:string):string{const bytes=Buffer.from(text),name=sha(bytes);mkdirSync(join(root,directory),{recursive:true});writeFileSync(join(root,directory,name+extension),bytes);return name;}
function generation(root:string,owners:number,number:number,known=identity):string{
 const rows:Record<string,unknown>={};for(let at=0;at<owners;at++){const fragment=put(root,"o","fragment "+at+" of "+owners,".json.br");rows["p"+at]={root:"p"+at,kind:"project",fragment,bytes:10,edgesEnv:sha("env"),targets:["build"]};}
 const manifest={version:1,generation:number,parent:null,identity:known,implementation:sha("implementation"),env:sha("env"),membership:sha("membership"),frontier:put(root,"f","frontier",".json.br"),factIndex:put(root,"x","index "+owners,".facts.br"),global:put(root,"o","global",".json.br"),owners:rows,built:{kind:"cold",base:null,dirty:[]}},bytes=Buffer.from(JSON.stringify(manifest)),digest=sha(bytes);
 mkdirSync(join(root,"m"),{recursive:true});writeFileSync(join(root,"m",digest+".json"),bytes);writeFileSync(join(root,"current.json"),JSON.stringify({version:1,generation:number,manifest:digest,parent:null,verified:{atMs:1,count:owners,owners:"AA=="}}));return digest;
}
const location=(server:RemoteCacheServerV1,root:string)=>({content:httpRemoteExecutionContentV1({endpoint:server.url}),instance:"main",key,root,files});

test("a generation uploaded by one machine restores byte for byte into an empty store of another, pointer last",async()=>{
 const server=await serve(),source=scratch(),target=scratch(),digest=generation(source,5,7);
 const uploaded=await publishGenerationV1(location(server,source));
 expect(uploaded.status).toBe("uploaded");expect(uploaded.documents).toBe(5+4);expect(uploaded.owners).toBe(5);
 expect((await publishGenerationV1(location(server,source))).status).toBe("current");
 const restored=await restoreGenerationV1({...location(server,target),identity});
 expect(restored).toMatchObject({status:"restored",generation:7,owners:5,documents:9});
 const pointer=readGenerationPointerV1(readFileSync(join(target,"current.json"))),manifest=readGenerationManifestV1(readFileSync(join(target,"m",digest+".json")));
 expect(pointer).toMatchObject({generation:7,manifest:digest,parent:null});
 for(const path of generationDocumentPathsV1(digest,manifest))expect(sha(readFileSync(join(target,path)))).toBe(sha(readFileSync(join(source,path))));
 expect((await restoreGenerationV1({...location(server,target),identity})).status).toBe("local");
});

test("restore refuses a foreign registry and finds nothing under an unknown key; a smaller or offered generation never replaces a larger one",async()=>{
 const server=await serve(),big=scratch(),small=scratch(),empty=scratch();generation(big,6,3);generation(small,2,9);
 expect((await publishGenerationV1(location(server,big))).status).toBe("uploaded");
 expect((await restoreGenerationV1({...location(server,empty),identity:sha("other")})).status).toBe("foreign");
 expect((await restoreGenerationV1({...location(server,empty),key:sha("unknown"),identity})).status).toBe("absent");
 expect((await publishGenerationV1(location(server,small))).status).toBe("remote-newer");
 const bigger=scratch();generation(bigger,8,1);
 expect((await publishGenerationV1({...location(server,bigger),ifAbsent:true})).status).toBe("remote-newer");
 expect((await publishGenerationV1(location(server,bigger))).status).toBe("uploaded");
 expect((await restoreGenerationV1({...location(server,empty),identity})).owners).toBe(8);
});

test("the exchange is off without an endpoint, read-only without write access and stays down after the first failure",async()=>{
 const base={workspace,identity:async()=>identity};
 expect(remoteGenerationExchangeV1({...base,environment:{}})).toBeUndefined();
 const server=await serve(),source=scratch(),target=scratch();generation(source,3,2);
 const readOnly=remoteGenerationExchangeV1({...base,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url}})!,writer=remoteGenerationExchangeV1({...base,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"}})!;
 expect(readOnly.upload).toBe(false);expect(writer.upload).toBe(true);
 expect((await writer.publish({root:source,fs:files})).status).toBe("uploaded");
 expect((await readOnly.restore({root:target,fs:files})).status).toBe("restored");
 const dead=remoteGenerationExchangeV1({...base,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_RETRIES:"0"}})!,first=await dead.restore({root:scratch(),fs:files}),again=await dead.restore({root:scratch(),fs:files});
 expect(first.status).toBe("failed");expect(again.reason).toContain("earlier in this process");
});

test("a machine with an empty store restores the generation of a peer through the real load and infers again only the projects whose inputs differ",async()=>{
 const server=await serve(),workspace=scratch(),environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"};
 const load=async()=>{const mock=createMockHost(workspace);(mock.host as any).exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity()});const result=await loadGenerationGraphV1(mock.host);return{result,said:mock.said.map(row=>row.text)};};
 await seed(workspace,ringFiles(40));
 const warm=await load();expect(warm.result.stats.ownersReinferred).toBe(40);expect(warm.said.some(text=>text.includes("uploaded to the remote cache"))).toBe(true);
 rmSync(join(workspace,".gen"),{recursive:true,force:true});
 await applyOps(workspace,[{op:"write",path:"pkg/p3/src/index.ts",content:"export const edited=1;\n"},{op:"write",path:"pkg/p9/src/index.ts",content:"export const edited=2;\n"}]);
 const fresh=await load();
 expect(fresh.said.some(text=>/restored from the remote cache, \d+ documents/u.test(text))).toBe(true);
 expect(fresh.result.stats.ownersReinferred).toBe(2);expect(Object.keys((fresh.result.graph as any).projects).length).toBe(40);
 rmSync(join(workspace,".gen"),{recursive:true,force:true});
 const again=await load();expect(again.result.stats.ownersReinferred).toBe(0);
});
