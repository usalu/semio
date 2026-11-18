import {afterEach,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,rmSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {remoteGenerationExchangeV1,type ClosurePortV1} from "../../🟦️.ts";

/** 🧹️ Contract (real cache servers on free ports with temporary stores): a cache that lost the leaves an earlier upload sent, because it was replaced or evicted, refuses the head with the reason once; the upload then forgets what it believed was sent and the next upload sends the leaves again, so the head is accepted. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64",NAMESPACE="build-input-closures" as const;
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-lost-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const environment=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
const port=(base:string):ClosurePortV1=>{const directory=(namespace:string)=>join(base,namespace);return{directory,size:(namespace,key)=>{try{const row=statSync(join(directory(namespace),key+".json"));return row.isFile()?row.size:undefined;}catch{return undefined;}},read:async(namespace,key)=>{try{return readFileSync(join(directory(namespace),key+".json"));}catch{return undefined;}},put:async(namespace,key,bytes)=>{const{mkdirSync,writeFileSync}=await import("node:fs");mkdirSync(directory(namespace),{recursive:true});writeFileSync(join(directory(namespace),key+".json"),bytes);}} as ClosurePortV1;};

test("a replaced cache refuses the head naming the missing leaf, the upload forgets its memo, and the next upload is accepted",async()=>{
 const first=await serve(),bytes=Buffer.from(JSON.stringify({files:["a.ts"]})),key=sha(bytes),mother=scratch(),leaves=port(scratch());
 await leaves.put(NAMESPACE,key,bytes);
 await seed(mother,{...ringFiles(6),"pkg/q0/project.json":JSON.stringify({name:"q0",package:"SEMIO_SOURCE_CLOSURE_"+key,uses:[]})});
 const mock=createMockHost(mother);
 (mock.host as any).exchange=remoteGenerationExchangeV1({environment:environment(first),workspace:mother,identity:()=>mock.host.identity(),platform,closures:leaves});
 await loadGenerationGraphV1(mock.host);
 const root=join(mother,".gen"),sentFile=root+".closures-sent.json";
 expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([key]);
 const second=await serve(),fresh=()=>remoteGenerationExchangeV1({environment:environment(second),workspace:mother,identity:()=>mock.host.identity(),platform,closures:leaves})!;
 const refused=await fresh().publish({root,fs:files});
 expect(refused.status).not.toBe("uploaded");expect(JSON.stringify(refused)).toContain("is not");
 expect(existsSync(sentFile),"the memo of what was sent is forgotten").toBe(false);
 const accepted=await fresh().publish({root,fs:files});
 expect(accepted.status).toBe("uploaded");
 expect(await httpRemoteExecutionContentV1({endpoint:second.url}).has("main",key)).toBe(true);
 expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([key]);
});
