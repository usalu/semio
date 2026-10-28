/** 📏️ Laws of the test file budgets: the schema, the authored table, the model of a file's limits and of the gate against an independent Python oracle, the gate, the watch and the messages. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{existsSync}from"node:fs";
import{join}from"node:path";
import{captureOwnedCommandV1}from"../../../../🎛️owned-execution/🟦️.ts";
import{testPythonV1}from"../../../🧰️toolchain/🟦️.ts";
import{testLevelAtLeast}from"../../../🎚️budget/🟦️.ts";
import{testScratchRootV1}from"../../../🏖️sandbox/🟦️.ts";
import{processTestCheckoutV1}from"../../../🗂️suites/🏃️run/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import authored from"../🔣️.json";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🧫️fixtures/🔣️.json";
import{admitTestFileV1,createTestFileGateV1,createTestFileMemoryV1,createTestFileWatchV1,describeTestFileExceededV1,readTestFileBudgetsV1,testFileLimitsV1,testFileRatioBasisPointsV1,type TestFileBudgetsV1,type TestFileProcessRowsV1}from"../🟦️.ts";

const ajv=new Ajv({strict:false,allErrors:true}),compile=(name:string)=>ajv.compile({...(schema.definitions as Record<string,object>)[name]!,definitions:schema.definitions}),table=readTestFileBudgetsV1(corpus.table);
const checkout=processTestCheckoutV1(import.meta.dir,join(import.meta.dir,"../../../../../../.."));
const wait=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

test("Ajv independently admits the authored table and the corpus, and the in-repo reader agrees on every refusal",()=>{
 expect(compile("Table")(authored)).toBe(true);expect(compile("Corpus")(corpus)).toBe(true);
 const entry=corpus.table.files[0]!,refused:unknown[]=[{...authored,version:2},{...authored,extra:1},{...authored,ceilingBasisPoints:9999},{...authored,unlistedCeilingMegabytes:63},{...authored,files:[{...entry,megabytes:63}]},{...authored,files:[{...entry,megabytes:16385}]},{...authored,files:[{...entry,reason:""}]},{...authored,files:[{...entry,extra:1}]},{...authored,files:[{path:entry.path,megabytes:100}]}];
 for(const value of refused){expect(compile("Table")(value),JSON.stringify(value).slice(0,90)).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.Table,value,schema).length,JSON.stringify(value).slice(0,90)).toBeGreaterThan(0);expect(()=>readTestFileBudgetsV1(value)).toThrow("invalid");}
 const rows=corpus.table.files;
 for(const files of[[rows[0]!,rows[0]!],[rows[1]!,rows[0]!],[{...rows[0]!,path:"/absolute/🟦️.ts"}],[{...rows[0]!,path:"a\\b/🟦️.ts"}],[{...rows[0]!,path:"a/../b/🟦️.ts"}],[{...rows[0]!,path:"a//b/🟦️.ts"}]])expect(()=>readTestFileBudgetsV1({...corpus.table,files}),JSON.stringify(files).slice(0,80)).toThrow("invalid");
 expect(readTestFileBudgetsV1(authored)).toEqual(authored as TestFileBudgetsV1);
});

test("The authored budgets name existing files in path order, each priced at least a file process and at most the machine",()=>{
 const found=readTestFileBudgetsV1();
 for(const row of found.files){expect(existsSync(join(checkout,row.path)),row.path).toBe(true);expect(row.megabytes).toBeGreaterThanOrEqual(64);expect(row.megabytes).toBeLessThanOrEqual(16384);}
 expect(found.ceilingBasisPoints).toBeGreaterThanOrEqual(10000);
});

test("Every stored vector of a file's limits, of a cohort's size and of the admission equals the independent Python model, which is also run live",async()=>{
 expect(corpus.cases.length).toBeGreaterThan(150);const kinds=new Set<string>();
 for(const item of corpus.cases){
  kinds.add(item.kind);const input=item.input as Record<string,any>;
  if(item.kind==="limits")expect(testFileLimitsV1(table,input.path,input.default),item.name).toEqual(item.expect as never);
  else if(item.kind==="ratio")expect(testFileRatioBasisPointsV1(table,input.paths,input.default),item.name).toBe(item.expect as never);
  else expect(admitTestFileV1(input.held,input.next,input.share),item.name).toBe(item.expect as never);
 }
 expect([...kinds].sort()).toEqual(["admit","limits","ratio"]);
 const python=testPythonV1()??Bun.which("python3")??Bun.which("python");
 if(!python)throw Error("Independent Python oracle required");
 expect(await captureOwnedCommandV1(python,[join(import.meta.dir,"🐍️oracle/🐍️.py"),"--check"],join(import.meta.dir,".."),{output:{maximumBytes:4096,maximumLines:8}})).toContain("equal the model");
 const exact=testFileLimitsV1(table,"alpha/b/c/🟦️.ts",1);
 expect(exact.ceilingMegabytes,"731 x 1.5137 = 1106.5 rounds up").toBe(1107);
});

test("Properties: the ceiling is never below the budget, grows with it, the unlisted ceiling is a floor of undeclared files and the cohort size never drops below the default",()=>{
 for(const basis of[10000,10001,15000,99999])for(let budget=64;budget<=4096;budget+=37){
  const closed=readTestFileBudgetsV1({...corpus.table,ceilingBasisPoints:basis,files:[{path:"a/🟦️.ts",megabytes:budget,reason:"p"}]}),limits=testFileLimitsV1(closed,"a/🟦️.ts",1024);
  expect(limits.ceilingMegabytes).toBeGreaterThanOrEqual(budget);expect(limits.ceilingMegabytes).toBeLessThan(Math.ceil(budget*basis/10000)+1);
  expect(testFileLimitsV1(closed,"b/🟦️.ts",budget).ceilingMegabytes).toBeGreaterThanOrEqual(closed.unlistedCeilingMegabytes);
  expect(testFileRatioBasisPointsV1(closed,["a/🟦️.ts"],1024)).toBeGreaterThanOrEqual(10000);
 }
});

test("The gate admits files in the order they ask, lets one file in alone whatever it holds, counts what a file really holds and wakes the next on release",async()=>{
 const gate=createTestFileGateV1(1000),order:string[]=[],first=await gate.enter(600);
 const second=gate.enter(600).then(slot=>{order.push("second");return slot;}),third=gate.enter(300).then(slot=>{order.push("third");return slot;});
 await wait(20);expect(order,"600+600 exceeds the share and the third waits behind the second").toEqual([]);
 first.release();(await second);await third;expect(order).toEqual(["second","third"]);
 const alone=createTestFileGateV1(1000),big=await alone.enter(5000);let entered=false;const next=alone.enter(1).then(slot=>{entered=true;return slot;});
 await wait(20);expect(entered,"a file above the share holds the whole gate").toBe(false);big.release();(await next).release();expect(entered).toBe(true);
 const raised=createTestFileGateV1(1000),small=await raised.enter(300);small.raise(900);let late=false;const waiter=raised.enter(200).then(slot=>{late=true;return slot;});
 await wait(20);expect(late,"900 held + 200 exceeds the share").toBe(false);small.release();await waiter;expect(late).toBe(true);
});

test("The watch follows every tree with one table read per tick, reports a tree above its ceiling once, answers the peak and stops reading when nothing is watched; a table it cannot read stops nobody",async()=>{
 const megabytes=(count:number)=>count*1024;let reads=0,failing=false;
 const rows=async():Promise<TestFileProcessRowsV1>=>{reads++;if(failing)throw Error("no table");return[{pid:10,parent:1,rssKilobytes:megabytes(512)},{pid:11,parent:10,rssKilobytes:megabytes(512)},{pid:20,parent:1,rssKilobytes:megabytes(100)},{pid:30,parent:1,rssKilobytes:megabytes(10)}];};
 const watch=createTestFileWatchV1({intervalMilliseconds:5,rows}),hits:number[]=[],seen:number[]=[];
 const big=watch.watch(10,1000,megabytes=>hits.push(megabytes),megabytes=>seen.push(megabytes)),small=watch.watch(20,1000,()=>{throw Error("never")});
 await wait(60);const bigResult=big(),smallResult=small();
 expect(hits,"the tree of 10 holds 1024 MB and is reported once").toEqual([1024]);expect(seen.every(value=>value===1024)).toBe(true);
 expect(bigResult).toEqual({peakMegabytes:1024,exceeded:true});expect(smallResult).toEqual({peakMegabytes:100,exceeded:false});
 const settled=reads;await wait(40);expect(reads,"no timer runs while nothing is watched").toBe(settled);
 failing=true;const blind=watch.watch(10,1,()=>hits.push(-1));await wait(40);expect(blind()).toEqual({peakMegabytes:0,exceeded:false});expect(hits).toEqual([1024]);
 const gone=createTestFileWatchV1({intervalMilliseconds:5,rows:async()=>[]}).watch(99,1,()=>hits.push(-2));await wait(30);expect(gone().exceeded).toBe(false);
});

test("SEMIO_TEST_FILE_CEILING=off lifts the ceiling of every file and nothing else",()=>{
 const budgets=readTestFileBudgetsV1(corpus.table),on=createTestFileMemoryV1({budgets,checkout:"/c",defaultMegabytes:1024,shareMegabytes:4096,environment:{}}),off=createTestFileMemoryV1({budgets,checkout:"/c",defaultMegabytes:1024,shareMegabytes:4096,environment:{SEMIO_TEST_FILE_CEILING:"off"}}),other=createTestFileMemoryV1({budgets,checkout:"/c",defaultMegabytes:1024,shareMegabytes:4096,environment:{SEMIO_TEST_FILE_CEILING:"on"}});
 expect(on.limits("/c/alpha/d/🟦️.ts")).toEqual({budgetMegabytes:64,ceilingMegabytes:97,declared:true});
 expect(off.limits("/c/alpha/d/🟦️.ts")).toEqual({budgetMegabytes:64,ceilingMegabytes:Number.MAX_SAFE_INTEGER,declared:true});
 expect(other.limits("/c/omega/🟦️.ts").ceilingMegabytes).toBe(3001);
});

test("A stopped file is worded in the language of the environment, naming the file, what it held, its ceiling and where the budget comes from",()=>{
 const declared=testFileLimitsV1(table,"alpha/d/🟦️.ts",1024),unlisted=testFileLimitsV1(table,"omega/🟦️.ts",1024);
 expect(describeTestFileExceededV1("a/🟦️.ts",100,declared,{LANG:"en_US.UTF-8"})).toBe("[tests] a/🟦️.ts held 100 MB and was stopped at its ceiling of 97 MB (64 MB declared in the test file budgets)");
 expect(describeTestFileExceededV1("a/🟦️.ts",4000,unlisted,{LANG:"en_US.UTF-8"})).toBe("[tests] a/🟦️.ts held 4000 MB and was stopped at its ceiling of 3001 MB (not declared in the test file budgets)");
 expect(describeTestFileExceededV1("a/🟦️.ts",100,declared,{LANG:"de_DE.UTF-8"})).toBe("[tests] a/🟦️.ts belegte 100 MB und wurde an seiner Obergrenze von 97 MB angehalten (64 MB in den Testdatei-Budgets deklariert)");
 expect(()=>readTestFileBudgetsV1({...authored,version:2},{LANG:"de_DE.UTF-8"})).toThrow("ungültig");
});

const measured=testLevelAtLeast("long")?test:test.skip;
measured("Every declared file, run alone in its own process, holds at most its declared budget (the long level: it runs each declared file)",async()=>{
 for(const row of readTestFileBudgetsV1().files){
  const child=Bun.spawn([process.execPath,"test","--timeout","900000","./"+row.path],{cwd:checkout,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:testScratchRootV1("file-budgets")},stdout:"pipe",stderr:"pipe"}),watched=createTestFileWatchV1({intervalMilliseconds:200}).watch(child.pid,Number.MAX_SAFE_INTEGER,()=>{});
  const output=new Response(child.stderr).text();await child.exited;const{peakMegabytes}=watched();
  expect(await output,row.path+" ran tests").toMatch(/\d+ pass/u);expect(peakMegabytes,row.path+" peak MB against the declared budget").toBeLessThanOrEqual(row.megabytes);
 }
},3_600_000);
