import {expect,spyOn,test} from "bun:test";
import * as fs from "node:fs/promises";
import {join} from "node:path";
import {casPinDirectoryV1,holdCasPinsV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

test("a delayed older rename cannot displace newer held digests",async()=>{
 const documents=new Map<string,string>(),publications:string[][]=[];
 const olderEntered=Promise.withResolvers<void>(),finishOlder=Promise.withResolvers<void>();
 let renewal:()=>void=()=>undefined,failNext=false;
 const timer=spyOn(globalThis,"setInterval").mockImplementation(callback=>{
  renewal=()=>callback();
  return {unref:()=>undefined} as ReturnType<typeof setInterval>;
 });
 const mkdir=spyOn(fs,"mkdir").mockResolvedValue(undefined);
 const write=spyOn(fs,"writeFile").mockImplementation(async(path,data)=>{documents.set(String(path),String(data));});
 const rename=spyOn(fs,"rename").mockImplementation(async(from,to)=>{
  if(rename.mock.calls.length===1){olderEntered.resolve();await finishOlder.promise;}
  if(failNext){failNext=false;throw Error("late write refused");}
  const content=documents.get(String(from))!;
  documents.set(String(to),content);
  publications.push((JSON.parse(content).digests as string[]).map(digest=>digest[0]!));
 });
 const root="cas-pin-order",holder="test-held",policy={leaseMaxMs:3_600_000};
 const hold=(names:string[])=>holdCasPinsV1(root,policy,holder,names.map(name=>name.repeat(64)));
 try{
  const older=hold(fixture.requests[0]!);
  await olderEntered.promise;
  const newer=fixture.requests.slice(1).map(hold);
  renewal();
  expect(timer.mock.calls[0]?.[1]).toBe(policy.leaseMaxMs/3);
  await new Promise<void>(done=>setImmediate(done));
  finishOlder.resolve();
  await Promise.all([older,...newer]);
  const current=()=>(JSON.parse(documents.get(join(casPinDirectoryV1(root),holder+".json"))!).digests as string[]).map(digest=>digest[0]!);
  expect(current()).toEqual(fixture.final);
  expect(publications).toEqual(fixture.publications);
  failNext=true;
  await hold(fixture.failedWrite.request);
  expect(current()).toEqual(fixture.final);
  await hold(fixture.failedWrite.following);
  expect(current()).toEqual(fixture.failedWrite.expectedFinal);
 }finally{
  finishOlder.resolve();
  rename.mockRestore();write.mockRestore();mkdir.mockRestore();timer.mockRestore();
 }
});
