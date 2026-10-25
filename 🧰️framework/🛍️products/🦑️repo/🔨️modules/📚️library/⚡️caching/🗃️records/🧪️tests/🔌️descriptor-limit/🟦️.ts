/** 🔌️ Contract: reading records from many packs never keeps more descriptors open than the process-wide limit, every record still reads back byte-exact, and the pack indexes agree with Python hashlib. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {RECORD_STORE_DESCRIPTOR_LIMIT,openRecordStore,openRecordStoreDescriptorCount,recordKey} from "../../🟦️.ts";

const python=testPythonV1(),inside=(directory:string)=>existsSync("/dev/fd")?readdirSync("/dev/fd").filter(name=>{try{return readlinkSync("/dev/fd/"+name).startsWith(directory);}catch{return false;}}).length:undefined;
const encoder=new TextEncoder();

test("the fixture is closed under its schema and its keys follow the record key algorithm",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 expect(RECORD_STORE_DESCRIPTOR_LIMIT).toBe(fixture.limit);
 fixture.records.slice(0,8).forEach((row,index)=>expect(recordKey(fixture.namespace,String(index))).toBe(row.key));
});

test("many packs read through a bounded descriptor set and every value is exact",()=>{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"record-descriptors-"));
 try{
  const writer=openRecordStore(directory);
  for(const row of fixture.records){writer.put(row.key,encoder.encode(row.text));writer.flush();}
  writer.close();
  expect(readdirSync(directory).filter(name=>name.endsWith(".pack")).length).toBe(fixture.packs);
  const reader=openRecordStore(directory);let widest=0,fds=0;
  for(const row of fixture.records){
   expect(new TextDecoder().decode(reader.get(row.key)!)).toBe(row.text);
   widest=Math.max(widest,openRecordStoreDescriptorCount());const real=inside(directory);if(real!==undefined)fds=Math.max(fds,real);
  }
  expect(widest).toBeLessThanOrEqual(fixture.limit);expect(widest).toBeGreaterThan(0);
  if(existsSync("/dev/fd"))expect(fds).toBeLessThanOrEqual(fixture.limit);
  for(const row of [...fixture.records].reverse())expect(new TextDecoder().decode(reader.get(row.key)!)).toBe(row.text);
  expect(reader.stats.hits).toBe(fixture.records.length*2);
  reader.close();expect(openRecordStoreDescriptorCount()).toBe(0);
  if(python){
   const script="import hashlib,json,os,sys\nbad=0\nfor name in sorted(os.listdir(sys.argv[1])):\n    if not name.endswith('.idx'): continue\n    index=json.load(open(os.path.join(sys.argv[1],name),encoding='utf8'))\n    data=open(os.path.join(sys.argv[1],index['pack']+'.pack'),'rb').read()\n    for key,offset,length,digest in index['records']:\n        if hashlib.sha256(data[offset:offset+length]).hexdigest()[:32]!=digest: bad+=1\nprint(bad)";
   expect(Bun.spawnSync([python,"-I","-c",script,directory]).stdout.toString().trim()).toBe("0");
   const expected=new Map(fixture.records.map(row=>[row.key,row.digest]));let seen=0;
   for(const name of readdirSync(directory).filter(name=>name.endsWith(".idx")))for(const [key,,,digest] of JSON.parse(readFileSync(join(directory,name),"utf8")).records){expect(expected.get(key)).toBe(digest);seen++;}
   expect(seen).toBe(fixture.records.length);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("two stores share the limit and a closed store releases its descriptors",()=>{
 const first=mkdtempSync(join(realpathSync(tmpdir()),"record-descriptors-a-")),second=mkdtempSync(join(realpathSync(tmpdir()),"record-descriptors-b-"));
 try{
  for(const directory of [first,second]){const writer=openRecordStore(directory);for(const row of fixture.records.slice(0,40)){writer.put(row.key,encoder.encode(row.text));writer.flush();}writer.close();}
  const left=openRecordStore(first),right=openRecordStore(second);
  for(const row of fixture.records.slice(0,40)){left.get(row.key);right.get(row.key);expect(openRecordStoreDescriptorCount()).toBeLessThanOrEqual(fixture.limit);}
  left.close();const afterLeft=openRecordStoreDescriptorCount();right.close();
  expect(afterLeft).toBeLessThanOrEqual(fixture.limit);expect(openRecordStoreDescriptorCount()).toBe(0);
  expect(new TextDecoder().decode(openRecordStore(first).get(fixture.records[0]!.key)!)).toBe(fixture.records[0]!.text);
 }finally{rmSync(first,{recursive:true,force:true});rmSync(second,{recursive:true,force:true});}
});
