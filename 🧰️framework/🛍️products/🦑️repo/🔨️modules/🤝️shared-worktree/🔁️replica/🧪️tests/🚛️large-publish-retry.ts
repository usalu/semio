import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {posix} from "node:path";
import ts from "typescript";
import {evaluateExtractedFunctionV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🚛️large-publish-retry.json" with {type:"json"};

test("an unstable large publication cools while another large file can use the slot",async()=>{
 const {unstable,independent,bytes,unstableAt,secondTurn,eligibleAt}=fixture;
 const source=ts.createSourceFile("replica.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
 const declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="largePublishV1");
 if(!declaration)throw Error("Native large publication required");
 const ready=new Map<string,ReturnType<typeof Promise.withResolvers<void>>>(),attempts:string[]=[];
 const next=(path:string)=>{
  const result=Promise.withResolvers<void>();
  ready.set(path,result);
  return result.promise;
 };
 let now=unstableAt;
 const inode={dev:1n,ino:2n,mode:33188n,nlink:1n,size:BigInt(bytes),mtimeNs:1n,ctimeNs:1n,isFile:()=>true};
 const controller=new AbortController(),oracle=new Database(":memory:");
 const job=(path:string)=>({entry:{path,bytes,kind:"file",executable:false},current:{operation:"put",hlc:"held"}});
 const body=ts.transpileModule(declaration.getText(source),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText;
 const background=evaluateExtractedFunctionV1(body,"largePublishV1",{
  schema,largeBlobBytesV1:bytes,absolute:posix.join,lstat:async()=>inode,sharedV1:()=>true,ancestryOf:async()=>"ok",
  prepare:async(_replica:unknown,item:ReturnType<typeof job>)=>{
   attempts.push(item.entry.path);
   return attempts.length===1?undefined:{job:item,touch:true};
  },
 });
 const replica={settings:{worktree:"/fixture",now:()=>now},queue:{has:()=>false}};
 const publisher=background(replica,controller.signal,(path:string)=>ready.get(path)?.resolve());
 const eligible=oracle.query<{path:string},[string,string,number,number]>(
  "select ?1 as path where ?3>=?4 union all select ?2");
 try{
  expect(eligibleAt).toBe(unstableAt+schema.definitions.LargePublish.const.retryMilliseconds);
  const failed=next(unstable);
  await publisher.prepare(job(unstable));await failed;
  expect(await publisher.prepare(job(unstable))).toBeUndefined();
  now=secondTurn;
  await publisher.prepare(job(unstable));
  const other=next(independent);
  await publisher.prepare(job(independent));
  await new Promise<void>(resolve=>setImmediate(resolve));
  expect(attempts.slice(1)).toEqual(eligible.all(unstable,independent,now,eligibleAt).map(row=>row.path));
  await other;
  expect(await publisher.prepare(job(independent))).toMatchObject({touch:true});
  now=eligibleAt;
  const retried=next(unstable);
  await publisher.prepare(job(unstable));await retried;
  expect(await publisher.prepare(job(unstable))).toMatchObject({touch:true});
  expect(attempts).toEqual([unstable,independent,unstable]);
 }finally{controller.abort();publisher.reset();oracle.close();}
});
