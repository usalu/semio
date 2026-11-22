import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {lstatSync,mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {digestStampKeyV1,openDigestMemoV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";

test("the memo answers only for the exact stamp, survives a restart, and its stamp key and digest equal the python oracle",()=>{
 const directory=mkdtempSync(join(tmpdir(),"digest-memo-")),file=join(directory,"data.bin"),memoFile=join(directory,"memo.json");
 try{
  writeFileSync(file,"registry archive bytes");const row=lstatSync(file,{bigint:true}),digest=createHash("sha256").update(readFileSync(file)).digest("hex");
  const oracle=spawnSync("python3",["-I",join(import.meta.dirname,"🐍️oracle/🐍️.py"),file],{encoding:"utf8"});expect(oracle.status).toBe(0);
  const expected=JSON.parse(oracle.stdout)as{stamp:string;digest:string};expect(digestStampKeyV1(row)).toBe(expected.stamp);expect(digest).toBe(expected.digest);
  const first=openDigestMemoV1(memoFile);expect(first.get(file,row)).toBeUndefined();first.set(file,row,digest);first.flush();
  expect(new Ajv({strict:false}).compile(schema.definitions.Memo)(JSON.parse(readFileSync(memoFile,"utf8")))).toBe(true);
  const second=openDigestMemoV1(memoFile);expect(second.get(file,row)).toBe(digest);
  utimesSync(file,new Date(1e12),new Date(1e12));expect(second.get(file,lstatSync(file,{bigint:true}))).toBeUndefined();
  expect(second.get(join(directory,"other"),row)).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("an absent, malformed, foreign or ill-formed memo file is an empty memo and a write never throws",()=>{
 const directory=mkdtempSync(join(tmpdir(),"digest-memo-")),stamp={dev:1n,ino:2n,size:3n,mtimeNs:4n};
 try{
  for(const text of [undefined,"{","[]",JSON.stringify({version:2,rows:{}}),JSON.stringify({version:1,rows:{a:"bad"}}),JSON.stringify({version:1,rows:{a:"1:2:3:4:"+"g".repeat(64)}})]){
   const file=join(directory,"m.json");if(text!==undefined)writeFileSync(file,text);const memo=openDigestMemoV1(file);expect(memo.get("a",stamp)).toBeUndefined();rmSync(file,{force:true});
  }
  const unwritable=openDigestMemoV1(join(directory,"missing","m.json"));unwritable.set("a",stamp,"a".repeat(64));expect(()=>unwritable.flush()).not.toThrow();expect(unwritable.get("a",stamp)).toBe("a".repeat(64));
 }finally{rmSync(directory,{recursive:true,force:true});}
});
