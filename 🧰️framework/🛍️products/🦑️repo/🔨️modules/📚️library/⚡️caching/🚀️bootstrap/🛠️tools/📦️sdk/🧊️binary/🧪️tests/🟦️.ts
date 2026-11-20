import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {posix,win32} from "node:path";
import ts from "typescript";
import fixture from "./🔣️.json";
import {nxSdkBinaryPathV1,nxSdkBinaryReceiptV1} from "../🟦️.ts";
import {publishNxSdkBinaryV1} from "../📥️publication/🟦️.ts";
test("SDK immutable binary intent and seal agree with independent SQLite and SHA256",()=>{const db=new Database(":memory:");try{db.exec("create table paths(path text,shared integer);create table objects(path text,sha256 text,bytes integer)");for(const row of fixture.paths){db.query("insert into paths values(?,?)").run(row.path,row.shared?1:0);const oracle=db.query("select case when path like 'node_modules/%' and path not like '%/../%' and path not like '%/./%' and (path like '%.wasm' or path like '%.node') then 1 else 0 end shared from paths where path=?").get(row.path)as{shared:number};expect(Boolean(oracle.shared)).toBe(row.shared);expect(nxSdkBinaryPathV1(row.path)).toBe(row.shared);}for(const row of fixture.objects)db.query("insert into objects values(?,?,?)").run(row.path,row.sha256,row.bytes);const objects=db.query<{path:string;sha256:string;bytes:number},[]>("select path,sha256,bytes from objects order by path collate binary").all(),receipt=nxSdkBinaryReceiptV1(fixture.objects),digest=new Bun.CryptoHasher("sha256").update(JSON.stringify({version:1,objects})+"\n").digest("hex");expect(receipt.objects).toEqual(objects);expect(receipt.digest).toBe(digest);expect(nxSdkBinaryReceiptV1([...fixture.objects].reverse())).toEqual(receipt);expect(()=>nxSdkBinaryReceiptV1([...fixture.objects,fixture.objects[0]!])).toThrow();}finally{db.close();}});

test("SDK binary publication matches whole portable member components under both native path conventions",async()=>{
 expect(typeof publishNxSdkBinaryV1).toBe("function");
 const text=readFileSync(new URL("../📥️publication/🟦️.ts",import.meta.url),"utf8");
 const source=ts.createSourceFile("publication.ts",text,ts.ScriptTarget.Latest,true);
 const owner=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="publishNxSdkBinaryV1");
 if(!owner)throw Error("Actual SDK publication owner required");
 const code=ts.transpileModule(owner.getText(source).replace("export async function","async function"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const db=new Database(":memory:"),admitted=Error("reached native storage verification");
 try{
  for(const row of fixture.bindingPaths){
   const paths=row.platform==="win32"?win32:posix;
   const oracle=db.query("select substr(case when ?1 = 'win32' then replace(?2, char(92), '/') else ?2 end, -length(?3)-1) = '/' || ?3 as accepted").get(row.platform,row.destination,row.member) as {accepted:number};
   expect(Boolean(oracle.accepted)).toBe(row.accepted);
   const check=()=>undefined,ports={join:paths.join,sep:paths.sep,nxSdkBinaryPathV1,requireNativeStorageReservationV1:check,requireIssuedResourceControlInvocationV1:check,requireIssuedResourceControlRefinementV1:check,checkResourceLeaseControl:check,requireNativeStoragePathV1:()=>{throw admitted;}};
   const publish=new Function(...Object.keys(ports),code+";return publishNxSdkBinaryV1;")(...Object.values(ports));
   const store={root:row.platform==="win32"?"C:\\store":"/store",image:{recheck:async()=>undefined}};
   const result=await publish(row.destination,row.destination,row.member,store,"replace",{}).catch((error:unknown)=>error);
   expect(result===admitted,row.destination).toBe(row.accepted);
   if(!row.accepted)expect(result).toMatchObject({message:"Exact SDK immutable binary publication required"});
  }
 }finally{db.close();}
});

test("SDK ownership receipt flushes staged bytes and the parent through the native storage port",async()=>{
 const text=readFileSync(new URL("../📥️publication/🟦️.ts",import.meta.url),"utf8");
 const source=ts.createSourceFile("publication.ts",text,ts.ScriptTarget.Latest,true);
 const owner=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="atomic");
 if(!owner)throw Error("Actual SDK receipt publication required");
 const code=ts.transpileModule(owner.getText(source),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 for(const row of fixture.receiptPublications){
  const path=row.platform==="win32"?win32:posix,events:string[]=[],stage=row.path+"~01234567";
  const ports={
   process:{platform:row.platform},dirname:path.dirname,randomBytes:()=>Buffer.from("01234567","hex"),
   checkResourceLeaseControl:()=>undefined,requireNativeStoragePathV1:(root:string,name:string)=>{expect(root).toBe(row.root);return name;},
   open:async(name:string,flags:string,mode:number)=>{
    expect([name,flags,mode]).toEqual([stage,"wx",row.mode]);events.push("open");
    return {writeFile:async(data:string)=>{expect(JSON.parse(data)).toEqual({phase:"sealed"});events.push("write");},sync:async()=>{events.push("file-flush");},close:async()=>{events.push("close");}};
   },
   rename:async(from:string,to:string)=>{expect([from,to]).toEqual([stage,row.path]);events.push("publish");},
   nativeDirectorySystemPortV1:()=>({flush:(parent:string)=>{expect(parent).toBe(path.dirname(row.path));events.push("directory-flush");}}),
   rm:async(name:string)=>{expect(name).toBe(stage);events.push("remove-stage");},
  };
  const atomic=new Function(...Object.keys(ports),code+";return atomic;")(...Object.values(ports));
  await atomic(row.path,{phase:"sealed"},{root:row.root,recheck:async()=>{events.push("recheck");}},{});
  expect(events).toEqual(["open","write","file-flush","recheck","publish","directory-flush","close","remove-stage"]);
 }
});

test("shared snapshot clones retain the original store and narrowed caller control",async()=>{
 const fs=await import("node:fs/promises"),path=await import("node:path"),{tmpdir}=await import("node:os"),{createHash}=await import("node:crypto");
 const routing=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{checkResourceLeaseControl}=await import("../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts");
 const text=readFileSync(new URL("../../📥️snapshot/🟦️.ts",import.meta.url),"utf8"),source=ts.createSourceFile("snapshot.ts",text,ts.ScriptTarget.Latest,true);
 const owner=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="cloneNxSdkSnapshotV1");
 if(!owner)throw Error("Actual SDK snapshot clone required");
 const code=ts.transpileModule(owner.getText(source).replace("export async function","async function"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const root=await fs.mkdtemp(path.join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"sdk-clone-")),origin=path.join(root,"src"),destination=path.join(root,"dst"),member=fixture.objects[0]!.path,from=path.join(origin,member),to=path.join(destination,member),bytes=Buffer.from(fixture.moduleBytes),digest=createHash("sha256").update(bytes).digest("hex");
 try{
  await fs.mkdir(path.dirname(from),{recursive:true});await fs.writeFile(from,bytes);await fs.mkdir(destination);
  await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"test:sdk-clone",maximumElapsedMilliseconds:15000},{},Date.now()),async original=>{
   const control=routing.createIssuedResourceControlV1(original,"test:sdk-clone",0),caller=routing.createRequestedIssuedResourceControlV1(control,10000,new AbortController().signal),store={control,invocation:original};
   const stamp=await fs.lstat(from,{bigint:true}),known=new Map([[member,{device:stamp.dev,inode:stamp.ino}]]),unchanged=(a:{ino:bigint},b:{ino:bigint})=>a.ino===b.ino;
   const ports={resolve:path.resolve,join:path.join,dirname:path.dirname,isAbsolute:path.isAbsolute,lstat:fs.lstat,readdir:fs.readdir,mkdir:fs.mkdir,readlink:fs.readlink,realpath:fs.realpath,symlink:fs.symlink,requireIssuedResourceControlRefinementV1:routing.requireIssuedResourceControlRefinementV1,checkResourceLeaseControl,nxSdkBinaryPathV1,nxSdkBinaryReceiptV1,bounded:()=>undefined,contained:()=>false,ancestry:async()=>[],stable:unchanged,stableImmutable:unchanged,policy:{maximumEntries:128,concurrency:2},
    verifyNxSdkBinaryLinksV1:async()=>known,verifyNxSdkSharedSourcesV1:async()=>new Map(),publishNxSdkBinaryV1:async(a:string,b:string,label:string,issued:typeof store,_kind:string,phase:typeof caller)=>{
     routing.requireIssuedResourceControlRefinementV1(issued.control,phase);expect(issued).toBe(store);expect(phase).toBe(caller);expect(phase.signal).not.toBe(issued.control.signal);
     await fs.link(a,b);return {path:label,sha256:digest,bytes:bytes.length};
    }};
   const clone=new Function(...Object.keys(ports),code+";return cloneNxSdkSnapshotV1;")(...Object.values(ports));
   const result=await clone(origin,destination,caller,{version:1,kind:"shared",store,binaryDigest:digest});
   expect(result.binaries.objects).toEqual([{path:member,sha256:digest,bytes:bytes.length}]);
   expect((await fs.lstat(to)).ino).toBe((await fs.lstat(from)).ino);
  });
 }finally{await fs.rm(root,{recursive:true,force:true});}
});
