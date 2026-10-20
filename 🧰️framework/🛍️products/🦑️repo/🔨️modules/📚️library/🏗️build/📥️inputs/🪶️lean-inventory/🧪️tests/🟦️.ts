/** 🪶️ Contract: the lean reader returns exactly the leaves whose sha256 is their name (cross-checked against Python hashlib), refuses every other shape, and a remembered identity survives only while the leaf is untouched. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {linkSync,mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {forgetInventoryRecordsV1,inventoryRecordStillHoldsV1,readInventoryRecordLeanV1,rememberInventoryRecordV1} from "../🟨️.mjs";

const python=testPythonV1(),windows=process.platform==="win32";
const scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"lean-inventory-")),directory=join(root,fixture.directories[0]!);mkdirSync(directory,{recursive:true});return {root,directory};};

test("the fixture is closed under its schema and its digests are the sha256 of their text",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 if(python){const texts=Object.values(fixture.records).map(row=>row.text),result=Bun.spawnSync([python,"-I","-c","import hashlib,sys\nfor t in sys.argv[1:]:\n    print(hashlib.sha256(t.encode('utf8')).hexdigest())",...texts]);expect(result.stdout.toString().trim().split("\n")).toEqual(Object.values(fixture.records).map(row=>row.sha256));}
});

for(const row of fixture.cases){
 test("leaf case: "+row.name,()=>{
  if(row.link&&windows)return;
  const {root,directory}=scratch();
  try{
   const key=fixture.records[row.fileKey as keyof typeof fixture.records].sha256,path=join(directory,key+".json");
   const text=row.record===null?undefined:row.record==="empty"?"":fixture.records[row.record as keyof typeof fixture.records].text;
   if(text!==undefined){
    if(row.link==="symbolic"){const target=join(root,"target.json");writeFileSync(target,text);symlinkSync(target,path);}
    else{writeFileSync(path,text);if(row.link==="hard")linkSync(path,join(root,"second.json"));}
   }
   const bound=row.maximumBytes??fixture.maximumBytes;
   if(row.expect==="value"){
    const leaf=readInventoryRecordLeanV1(directory,key,bound);
    expect(leaf.sha256).toBe(key);expect(leaf.byteLength).toBe(Buffer.byteLength(text!));expect(leaf.value).toEqual(JSON.parse(text!));expect(leaf.stamp.split(":").length).toBe(5);
   }else expect(()=>readInventoryRecordLeanV1(directory,key,bound)).toThrow(row.expect);
  }finally{rmSync(root,{recursive:true,force:true});}
 });
}

for(const row of fixture.refusals)test("identity refusal: "+row.name,()=>{expect(()=>readInventoryRecordLeanV1(row.directory.includes("/")?row.directory:join(tmpdir(),row.directory),row.key,row.maximumBytes??fixture.maximumBytes)).toThrow(row.expect);});

test("a remembered identity holds until the leaf is replaced, touched or removed",()=>{
 forgetInventoryRecordsV1();
 const {root,directory}=scratch();
 try{
  const key=fixture.records.good.sha256,path=join(directory,key+".json");writeFileSync(path,fixture.records.good.text);
  expect(inventoryRecordStillHoldsV1(root,directory,key)).toBe(false);
  const leaf=readInventoryRecordLeanV1(directory,key,fixture.maximumBytes);rememberInventoryRecordV1(root,directory,key,leaf.stamp);
  expect(inventoryRecordStillHoldsV1(root,directory,key)).toBe(true);
  expect(inventoryRecordStillHoldsV1(root+"-other",directory,key)).toBe(false);
  unlinkSync(path);writeFileSync(path,fixture.records.good.text+" ");
  expect(inventoryRecordStillHoldsV1(root,directory,key)).toBe(false);
  rememberInventoryRecordV1(root,directory,key,"stale");unlinkSync(path);
  expect(inventoryRecordStillHoldsV1(root,directory,key)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});forgetInventoryRecordsV1();}
});
