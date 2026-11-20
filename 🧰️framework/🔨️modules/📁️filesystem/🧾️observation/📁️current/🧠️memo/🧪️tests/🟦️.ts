/** 🧠️ Contract: a digest is answered from the memo only while the file's identity is the remembered one, a file edited inside the racy window is never remembered, the memo survives a restart, and a retried owner hashes only what changed. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {DigestMemoV1,type FileIdentityV1} from "../🟦️.ts";
import {CurrentPhysicalFilesV1,CurrentPhysicalOwnerV1} from "../../🟦️.ts";
import type {FileObservationControlV1} from "../../../🟦️.ts";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url));
const oracle=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","🧬️schema","🔣️.json")]).stdout.toString()) as {racy:Record<string,boolean>;digests:Record<string,string>};
const control=():FileObservationControlV1=>({maxBytes:64*1024*1024,maxWork:1_000_000,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>60_000,onProgress:()=>undefined});
const identity=(mtime:string,ctime:string):FileIdentityV1=>[1n,2n,3n,BigInt(mtime),BigInt(ctime)];
let root:string;
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"digest-memo-")));});
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const quiet=():Promise<void>=>Bun.sleep(schema.definitions.Protocol.const.racyMilliseconds+200);
const digest={sha256:"a".repeat(64),byteLength:3};

test("the racy window decision equals the independent oracle",()=>{
 const memo=new DigestMemoV1(join(root,"racy"));
 for(const row of fixture.racy){expect(memo.remember("/x/"+row.id,identity(row.mtimeNs,row.ctimeNs),digest,row.startedMs)).toBe(oracle.racy[row.id]!);expect(oracle.racy[row.id]).toBe(row.remembered);}
});

test("a digest is answered only for the exact remembered identity",()=>{
 const memo=new DigestMemoV1(join(root,"exact")),known=identity("1000000000","1000000000");
 expect(memo.remember("/x/file",known,digest,60_000)).toBe(true);
 expect(memo.lookup("/x/file",known)).toEqual(digest);
 for(let index=0;index<5;index++){const moved=[...known] as bigint[];moved[index]=moved[index]!+1n;expect(memo.lookup("/x/file",moved as unknown as FileIdentityV1)).toBeUndefined();}
 expect(memo.lookup("/x/other",known)).toBeUndefined();
});

test("the memo survives a restart and refuses a damaged file",()=>{
 const directory=join(root,"restart"),first=new DigestMemoV1(directory),known=identity("1000000000","1000000000");
 first.remember("/x/file",known,digest,60_000);first.flush();
 expect(new DigestMemoV1(directory).lookup("/x/file",known)).toEqual(digest);
 writeFileSync(join(directory,schema.definitions.Protocol.const.fileName),"{damaged");
 expect(new DigestMemoV1(directory).size).toBe(0);
});

test("a retried owner hashes only the files a peer edited and its digests equal the oracle",async()=>{
 const directory=join(root,"retry"),files=join(root,"retry-files");mkdirSync(files,{recursive:true});
 const paths=fixture.files.map(row=>{const path=join(files,row.name);writeFileSync(path,row.text);return path;});
 await quiet();
 const memo=new DigestMemoV1(directory),first=new CurrentPhysicalOwnerV1(root,control(),new CurrentPhysicalFilesV1(memo));
 for(const [index,path] of paths.entries()){const row=fixture.files[index]!;expect(await first.digest(path)).toBe(oracle.digests[row.name]!);expect(createHash("sha256").update(row.text).digest("hex")).toBe(oracle.digests[row.name]!);}
 const firstReads=first.observedReads().files.size;expect(firstReads).toBe(paths.length);
 const peer=join(files,"wide.rs");writeFileSync(peer,"pub fn key_frame() -> u8 { 8 }\n");
 const second=new CurrentPhysicalOwnerV1(root,control(),new CurrentPhysicalFilesV1(memo));
 for(const path of paths)await second.digest(path);
 expect([...second.observedReads().files.keys()]).toEqual([peer]);
 await second.recheck();expect(second.observedReads().files.get(peer)!.reads).toBe(2);
 expect([...second.observedReads().files.keys()]).toEqual([peer]);
});

test("a recheck of an unchanged remembered file reads nothing and a changed file still fails",async()=>{
 const files=join(root,"plain");mkdirSync(files,{recursive:true});const path=join(files,"a.bin");writeFileSync(path,"one");await quiet();
 const memo=new DigestMemoV1(join(root,"plain-memo")),owner=new CurrentPhysicalOwnerV1(root,control(),new CurrentPhysicalFilesV1(memo));
 await owner.digest(path);await owner.recheck();expect(owner.observedReads().files.get(path)!.reads).toBe(1);
 writeFileSync(path,"two");await expect(owner.recheck()).rejects.toThrow();
});
