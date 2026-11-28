/** 🗄️ Contract: the upload worker reads the objects of a unit from the content store its OWN environment names (`SEMIO_CAS_ROOT`), not from the one of the process it happens to run in.
 * @see ../../📤️uploader/🟦️.ts (`uploadUnitV1`, `drainUploadsV1`) */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {putCasObjectSyncV1} from "../../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {uploadUnitV1,type SpoolJobV1} from "../../📤️uploader/🟦️.ts";

const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"casroot-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("an upload reads the objects from the content store of the environment it is given, and reports the unit gone when that store lacks them",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const settings=readFleetSettingsV1(directory("state"),{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!,client=createFleetClientV1(settings);
 const custom=directory("cas"),elsewhere=directory("other-cas"),root=directory("cache"),bytes=Buffer.from("object bytes of the unit"),sha256=putCasObjectSyncV1(custom,bytes);
 expect(sha256).toBe(createHash("sha256").update(bytes).digest("hex"));
 const key="a".repeat(64),shard=join(root,"e",key.slice(0,2),key);mkdirSync(shard,{recursive:true});
 writeFileSync(join(shard,"000000000000001-aaaaaa.json"),JSON.stringify({version:1,key,crate:"pa",dependencies:[],environment:[],outputs:[{name:"libpa-1.rlib",sha256,bytes:bytes.length,executable:false}],stderr:null,stdout:null,createdMs:Date.now()}));
 const platform={version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""} as const;
 const job:SpoolJobV1={version:1,key:"b".repeat(64),localKey:key,slot:"2".repeat(64),crate:"pa",cwd:"/w",workspace:null,laneRoot:null,owner:"t:1:aa",claimed:false,costMs:1,platform,queuedMs:Date.now()};
 expect(process.env.SEMIO_CAS_ROOT===custom,"the process itself does not name the store").toBe(false);
 expect(await uploadUnitV1(client,root,job,{SEMIO_CAS_ROOT:elsewhere}),"the store of the environment lacks the object").toBe("gone");
 expect(await uploadUnitV1(client,root,job,{SEMIO_CAS_ROOT:custom})).toBe(true);
},60000);
