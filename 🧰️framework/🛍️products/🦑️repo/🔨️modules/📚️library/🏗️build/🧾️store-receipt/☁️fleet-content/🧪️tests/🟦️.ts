import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {startRemoteCacheServerV1} from "../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {startNxFlightV1} from "../../../🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {encodeStoreReceiptV1,observeStoreContentV1,storeTreeDigestV1,verifyStoreReceiptV1} from "../../🟨️.mjs";
import {FLEET_CONTENT_POLICY_V1,fleetContentFromEnvironmentV1,fleetContentTextV1,parseStoreTreeManifestV1,publishStoreReceiptContentV1,restoreStoreReceiptContentV1,storeReceiptLinkPlanV1,storeTreeManifestV1} from "../🟦️.ts";

const scratch:string[]=[],stops:Array<()=>Promise<void>>=[],TOKEN="fleet-token";
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-content-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stops)await stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");

async function serve(){
 const server=await startRemoteCacheServerV1({directory:directory("store"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:TOKEN}});
 stops.push(()=>server.stop());
 return {server,store:{content:httpRemoteExecutionContentV1({endpoint:server.url,token:TOKEN,timeoutSeconds:20,retries:1}),instance:"main"}};
}

const payload:Record<string,string|Buffer>={
 "pkg/main.wasm":Buffer.from(Array.from({length:70000},(_,at)=>(at*31+7)%256)),
 "pkg/main.js":"export const answer = 42;\n",
 "pkg/types/main.d.ts":"export declare const answer: number;\n",
 "pkg/types/ünï cödé.txt":"unicode name\n",
 "pkg/empty.bin":"",
 "pkg/same-as-main-js.js":"export const answer = 42;\n",
};

async function produce(label:string){
 const root=directory(label),base=".cache/b/p/m/browser/prototype/wasm-packages/abc";
 for(const [path,content] of Object.entries(payload)){const target=join(root,base,path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,content);}
 mkdirSync(join(root,base,"pkg/empty-dir"),{recursive:true});
 const single=".cache/b/p/m/models/index.json";mkdirSync(dirname(join(root,single)),{recursive:true});writeFileSync(join(root,single),'{"models":[1,2,3]}\n');
 const directoryEntry=await observeStoreContentV1(root,join(root,base)),fileEntry=await observeStoreContentV1(root,join(root,single));
 const link=".cache/b/p/m/current";symlinkSync("browser/prototype/wasm-packages/abc",join(root,link));
 const receipt={version:1 as const,project:"@p/plugin",target:"wasm",entries:[directoryEntry,fileEntry],links:[{path:link,target:base}]};
 const file=join(root,"proj",".🧾️store","wasm.json");mkdirSync(dirname(file),{recursive:true});writeFileSync(file,encodeStoreReceiptV1(receipt));
 return {root,base,single,receipt,file,link};
}
const treeFiles=(root:string)=>{const out:string[]=[];const walk=(at:string)=>{for(const name of readdirSync(join(root,at)).sort()){const path=at?`${at}/${name}`:name,facts=lstatSync(join(root,path));if(facts.isDirectory())walk(path);else out.push(path);}};walk("");return out;};

test("the manifest text is the digest basis of a receipt entry: the repository's own digest, our manifest and Python's hashlib agree",async()=>{
 const made=await produce("manifest"),dir=join(made.root,made.base),manifest=await storeTreeManifestV1(dir),own=await storeTreeDigestV1(dir);
 expect(sha(manifest.text)).toBe(own.sha256);expect(manifest.rows).toHaveLength(own.files);
 const python=Bun.spawnSync(["python3","-I","-c","import hashlib,os,sys\nroot=sys.argv[1]\nrows=[]\nfor d,_,fs in os.walk(root):\n  for f in fs:\n    p=os.path.join(d,f);rel=os.path.relpath(p,root).replace(os.sep,'/');b=open(p,'rb').read();rows.append((rel.encode('utf8'),len(b),hashlib.sha256(b).hexdigest()))\nrows.sort()\nh=hashlib.sha256()\nfor r,n,x in rows:h.update(r+b'\\n'+str(n).encode()+b'\\n'+x.encode()+b'\\n')\nprint(h.hexdigest())",dir],{stdout:"pipe"}).stdout.toString().trim();
 expect(python).toBe(own.sha256);
 expect(parseStoreTreeManifestV1(manifest.text)).toEqual(manifest.rows.map(({path,bytes,hash})=>({path,bytes,hash})));
});

test("hostile or malformed manifests are refused",()=>{
 const good=`a/b\n3\n${"a".repeat(64)}\n`;
 expect(parseStoreTreeManifestV1(good)).toHaveLength(1);expect(parseStoreTreeManifestV1("")).toEqual([]);
 for(const bad of["../x\n3\n"+"a".repeat(64)+"\n","/abs\n3\n"+"a".repeat(64)+"\n","a//b\n3\n"+"a".repeat(64)+"\n","a\\b\n3\n"+"a".repeat(64)+"\n","a\n03\n"+"a".repeat(64)+"\n","a\n3\n"+"A".repeat(64)+"\n","a\n3\n"+"a".repeat(64),"a\n3\n","./a\n3\n"+"a".repeat(64)+"\n",good+good])expect(()=>parseStoreTreeManifestV1(bad),JSON.stringify(bad)).toThrow();
});

test("a machine that produced the payload publishes it once; a second machine at a different path restores it byte for byte from the fleet store",async()=>{
 const {store,server}=await serve(),made=await produce("producer");
 const first=await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store});
 expect(first.state).toBe("published");expect(first.uploaded).toBe(first.blobs-1);expect(first.uploaded).toBeGreaterThan(0);
 const again=await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store});
 expect(again).toMatchObject({state:"published",uploaded:0});
 const consumer=directory("consumer");expect(consumer).not.toBe(made.root);
 mkdirSync(join(consumer,".cache/b/p/m"),{recursive:true});
 const before=await verifyStoreReceiptV1(consumer,made.receipt);expect(before.state).toBe("invalid");
 const restored=await restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store});
 expect(restored).toMatchObject({state:"restored",entries:2});
 expect((await verifyStoreReceiptV1(consumer,made.receipt,"digest")).state).toBe("valid");
 expect(treeFiles(join(consumer,made.base))).toEqual(treeFiles(join(made.root,made.base)));
 for(const path of treeFiles(join(made.root,made.base)))expect(sha(readFileSync(join(consumer,made.base,path))),path).toBe(sha(readFileSync(join(made.root,made.base,path))));
 expect(readlinkSync(join(consumer,made.link))).toBe("browser/prototype/wasm-packages/abc");
 expect(existsSync(join(consumer,made.base,"pkg/empty-dir"))).toBe(false);
 expect(readdirSync(join(consumer,dirname(made.base))).filter(name=>name.includes(FLEET_CONTENT_POLICY_V1.restoreDirectorySuffix))).toEqual([]);
 const noop=await restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store});expect(noop).toMatchObject({state:"nothing",entries:0});
 expect(server.store.usage().entries).toBeGreaterThan(0);
});

test("two builds restoring the same receipt into one workspace at once both end with a valid store",async()=>{
 const {store}=await serve(),made=await produce("concurrent-producer");await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store});
 const consumer=directory("concurrent-consumer"),both=await Promise.all([1,2,3].map(()=>restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store})));
 expect(both.map(row=>row.state+(row.reason??"")).join()).toMatch(/^((restored|nothing),?)+$/u);
 expect((await verifyStoreReceiptV1(consumer,made.receipt,"digest")).state).toBe("valid");
 expect(readdirSync(join(consumer,dirname(made.base))).filter(name=>name.includes(FLEET_CONTENT_POLICY_V1.restoreDirectorySuffix))).toEqual([]);
});

test("content that changed after its receipt was written is not published",async()=>{
 const {store}=await serve(),made=await produce("changed");
 writeFileSync(join(made.root,made.base,"pkg/main.js"),"export const answer = 43;\n");
 expect(await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store})).toMatchObject({state:"skipped",reason:"entry changed"});
 expect(await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:join(made.root,"absent.json"),store})).toMatchObject({state:"skipped"});
});

function lacking(inner:RemoteExecutionContentV1,rule:(hash:string)=>"absent"|"corrupt"|"ok"):RemoteExecutionContentV1{
 return {...inner,
  async stream(instance,digest,signal){const mode=rule(digest.hash);if(mode==="absent")return undefined;const real=await inner.stream(instance,digest,signal);if(mode==="ok"||!real)return real;return (async function*(){for await(const chunk of real){yield new Uint8Array(chunk.map(byte=>byte^1));}})();},
  async read(instance,digest,signal){const mode=rule(digest.hash);if(mode==="absent")return undefined;const real=await inner.read(instance,digest,signal);if(mode==="ok"||!real)return real;return real.map(byte=>byte^1);},
 };
}

test("a restore that cannot complete (blob absent or corrupt) leaves nothing half-written and says why",async()=>{
 const {store}=await serve(),made=await produce("partial-producer");await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store});
 const jsHash=sha(payload["pkg/main.js"] as string),wasmHash=sha(payload["pkg/main.wasm"] as Buffer);
 for(const [label,rule] of[["absent",(hash:string)=>hash===jsHash?"absent" as const:"ok" as const],["corrupt",(hash:string)=>hash===wasmHash?"corrupt" as const:"ok" as const]]){
  const consumer=directory("partial-"+label),result=await restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store:{...store,content:lacking(store.content,rule)}});
  expect(["partial","unavailable"]).toContain(result.state);expect(result.reason).toBeDefined();
  expect(existsSync(join(consumer,made.base))).toBe(false);
  const parent=join(consumer,dirname(made.base));expect(existsSync(parent)?readdirSync(parent):[]).toEqual([]);
  expect((await verifyStoreReceiptV1(consumer,made.receipt)).state).toBe("invalid");
 }
});

test("an unreachable fleet store is unavailable, not an error",async()=>{
 const {server}=await serve();const dead=httpRemoteExecutionContentV1({endpoint:server.url,token:TOKEN,timeoutSeconds:2,retries:0});await server.stop();
 const made=await produce("dead"),consumer=directory("dead-consumer");
 expect((await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store:{content:dead,instance:"main"}})).state).toBe("failed");
 expect(await restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store:{content:dead,instance:"main"}})).toMatchObject({state:"unavailable"});
});

test("through the Nx single-flight front the executor and the backend reach the same content store with no real token in their hands",async()=>{
 const {server}=await serve(),flight=await startNxFlightV1({upstream:server.url,instance:"main",token:TOKEN});stops.push(()=>flight.stop());
 const environment=flight.environment(),store=fleetContentFromEnvironmentV1(environment)!;
 expect(store.instance).toBe("main");expect(JSON.stringify(environment)).not.toContain(TOKEN);
 const made=await produce("front-producer");
 expect((await publishStoreReceiptContentV1({workspaceRoot:made.root,receipt:made.file,store})).state).toBe("published");
 const consumer=directory("front-consumer");
 expect((await restoreStoreReceiptContentV1({workspaceRoot:consumer,receipt:made.receipt,store})).state).toBe("restored");
 expect((await verifyStoreReceiptV1(consumer,made.receipt)).state).toBe("valid");
 expect(fleetContentFromEnvironmentV1({})).toBeUndefined();expect(fleetContentFromEnvironmentV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:"http://x/a/b"})).toBeUndefined();
});

test("the user-visible texts exist in English and German",()=>{
 for(const language of["en","de"] as const)for(const key of["restored","unavailable"] as const)expect(fleetContentTextV1(language,key,{files:3,bytes:9,task:"p:wasm",reason:"x"})).toContain("p:wasm");
});

test("a receipt link is a relative POSIX link on POSIX and a directory junction to the absolute target on Windows",()=>{
 expect(storeReceiptLinkPlanV1("/w/a/b/link","/w/a/store/x",true,"linux")).toEqual({text:"../store/x",type:"dir"});
 expect(storeReceiptLinkPlanV1("/w/a/b/link","/w/a/store/f.txt",false,"darwin")).toEqual({text:"../store/f.txt",type:"file"});
 expect(storeReceiptLinkPlanV1("C:\\w\\a\\link","C:\\w\\store\\x",true,"win32")).toEqual({text:"C:\\w\\store\\x",type:"junction"});
});
