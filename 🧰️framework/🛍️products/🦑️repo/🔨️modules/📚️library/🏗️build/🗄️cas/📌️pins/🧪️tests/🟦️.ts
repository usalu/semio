import {expect,spyOn,test} from "bun:test";
import * as fs from "node:fs/promises";
import {join} from "node:path";
import {mkdtempSync,rmSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {casPinDirectoryV1,holdCasPinsV1,pinCasV1,readCasPinsV1} from "../🟦️.ts";
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

test("a pin names its holder by pid and creation time: a reused pid frees it, a pin from before the start time was recorded is judged by its pid alone",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-pin-id-")),directory=casPinDirectoryV1(root),digest="a".repeat(64);
 try{
  await pinCasV1(root,{leaseMaxMs:3_600_000},"mine",[digest],60_000,Date.now());
  expect(JSON.parse(await fs.readFile(join(directory,"mine.json"),"utf8")).startMs).toBeGreaterThan(0);
  expect([...(await readCasPinsV1(root,Date.now())).keys()]).toEqual([digest]);
  writeFileSync(join(directory,"reused.json"),JSON.stringify({version:1,holder:"reused",pid:process.pid,startMs:1_000,expiresAtMs:Date.now()+60_000,digests:["b".repeat(64)]}));
  expect([...(await readCasPinsV1(root,Date.now())).keys()]).toEqual([digest]);
  expect(existsSync(join(directory,"reused.json"))).toBe(false);
  writeFileSync(join(directory,"legacy.json"),JSON.stringify({version:1,holder:"legacy",pid:process.pid,expiresAtMs:Date.now()+60_000,digests:["c".repeat(64)]}));
  expect([...(await readCasPinsV1(root,Date.now())).keys()].sort()).toEqual([digest,"c".repeat(64)].sort());
  writeFileSync(join(directory,"legacy.json"),JSON.stringify({version:1,holder:"legacy",pid:2_000_000_000,expiresAtMs:Date.now()+60_000,digests:["c".repeat(64)]}));
  expect([...(await readCasPinsV1(root,Date.now())).keys()]).toEqual([digest]);
  expect(existsSync(join(directory,"legacy.json"))).toBe(false);
  writeFileSync(join(directory,"bad.json"),JSON.stringify({version:1,holder:"bad",pid:process.pid,startMs:"x",expiresAtMs:Date.now()+60_000,digests:["d".repeat(64)]}));
  await expect(readCasPinsV1(root,Date.now())).rejects.toThrow(/startMs/u);
 }finally{rmSync(root,{recursive:true,force:true});}
});
