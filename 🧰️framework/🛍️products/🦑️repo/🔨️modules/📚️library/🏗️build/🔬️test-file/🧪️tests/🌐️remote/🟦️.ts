import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import type {offloadCheckoutTestV1} from "../../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🔌️client/🟦️.ts";
import {renderTestFileResultV1,validateTestFileDocumentV1,type TestFileResultV1,type TestFileRouteV1} from "../../🟦️.ts";
import {runTestFileCliV1} from "../../⌨️command/🟦️.ts";
import {runTestFileRoutesV1} from "../../🏃️run/🟦️.ts";
import {createTestFileRemoteV1,type TestFileRemoteV1} from "../../🌐️remote/🟦️.ts";

const root=(()=>{for(const start of [dirname(fileURLToPath(import.meta.url)),process.cwd()]){let directory=start;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);if(existsSync(join(directory,"MODULE.bazel")))return directory;}throw Error("checkout root not found");})();
type Offload=typeof offloadCheckoutTestV1;
const sharedMemo=mkdtempSync(join(tmpdir(),"semio-h44-"));
afterAll(()=>{rmSync(sharedMemo,{recursive:true,force:true});});

const passed=(file:string,suite="plain"):TestFileResultV1=>({version:1,status:"passed",seconds:0.2,bail:false,counts:{passed:1,failed:0,skipped:0,notRun:0},files:[{file,mode:"plain",table:null,suite,status:"passed",seconds:0.2,pass:3,fail:0}]});
const failed=(file:string):TestFileResultV1=>({version:1,status:"failed",seconds:0.2,bail:false,counts:{passed:0,failed:1,skipped:0,notRun:0},files:[{file,mode:"plain",table:null,suite:"plain",status:"failed",seconds:0.2,pass:1,fail:2,error:"boom",tail:"1 fail"}]});
const route=(file:string,suite="plain",extra:Partial<TestFileRouteV1>={}):TestFileRouteV1=>({file,mode:"plain",table:null,suite,owner:"test-file",typecheck:"none",budgetMs:60000,python:false,launcher:null,runsOnHost:true,...extra});
const input=(notices:string[]=[])=>({language:"en" as const,notice:(line:string)=>{notices.push(line);}});

test("the remote port does not exist while the mode keeps every file here",async()=>{
 expect(await createTestFileRemoteV1(root,{SEMIO_TEST_FILE_OFFLOAD:"never"})).toBeUndefined();
 expect(await createTestFileRemoteV1(root,{})).toBeTypeOf("function");
 expect(await createTestFileRemoteV1(root,{SEMIO_TEST_FILE_OFFLOAD:"always"})).toBeTypeOf("function");
});

test("a file leaves only when the host verdict is wait and a machine is idle; a calm host and an empty fleet keep it here and nothing is attempted",async()=>{
 const calls:string[]=[],offload=(async()=>{calls.push("offload");return{kind:"local",reason:"no-idle-machine",detail:"x"};}) as unknown as Offload;
 const remote=(await createTestFileRemoteV1(root,{},{host:async()=>"admit",idle:async()=>5,offload}))!;
 expect(await remote("a.ts",1000,input())).toEqual({kind:"local",reason:"local-slot",detail:"",attempted:false});
 const crowded=(await createTestFileRemoteV1(root,{},{host:async()=>"wait",idle:async()=>0,offload}))!;
 expect(await crowded("a.ts",1000,input())).toEqual({kind:"local",reason:"no-idle-machine",detail:"",attempted:false});
 expect(calls).toEqual([]);
});

test("the host is asked once for a run of files, not once per file",async()=>{
 let asked=0;
 const remote=(await createTestFileRemoteV1(root,{},{host:async()=>{asked++;return "admit";},idle:async()=>3}))!;
 for(const file of ["a.ts","b.ts","c.ts"])await remote(file,1000,input());
 expect(asked).toBe(1);
});

test("an offered file is handed over with its budget, mode and language, the result is attributed, and every fault returns the file with a reason",async()=>{
 const seen:Record<string,unknown>[]=[],notices:string[]=[];
 const answer=(outcome:unknown,stages:readonly string[]=[]):Offload=>(async(call:Parameters<Offload>[0])=>{seen.push({file:call.file,budgetSeconds:call.budgetSeconds,language:call.language,workspace:call.environment.SEMIO_WORKSPACE_ROOT});for(const stage of stages)call.onStage?.(stage as never);return outcome;}) as Offload;
 const port=async(offload:Offload,environment:Record<string,string>={})=>(await createTestFileRemoteV1(root,environment,{memoRoot:sharedMemo,host:async()=>"wait",idle:async()=>2,offload}))!;
 const ran=await (await port(answer({kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"worker output\n",seconds:1.5},["QUEUED","EXECUTING","UNKNOWN"])))("a.ts",90500,input(notices));
 expect(ran).toMatchObject({kind:"ran",origin:"g1-m06",seconds:1.5,output:"worker output\n"});
 expect(seen[0]).toEqual({file:"a.ts",budgetSeconds:91,language:"en",workspace:root});
 expect(notices).toHaveLength(2);
 expect(notices[0]).toContain("a.ts");
 const always=await (await port(answer({kind:"done",origin:"g1-m07",result:passed("a.ts"),exitCode:0,stderr:"",seconds:1}),{SEMIO_TEST_FILE_OFFLOAD:"always"}))("a.ts",1,input());
 expect(always.kind).toBe("ran");
 expect(seen[1]).toMatchObject({budgetSeconds:1});
 expect(await (await port(answer({kind:"local",reason:"cancelled",detail:""})))("a.ts",1000,input())).toEqual({kind:"cancelled"});
 expect(await (await port(answer({kind:"local",reason:"queue-timeout",detail:"waited"})))("a.ts",1000,input())).toEqual({kind:"local",reason:"queue-timeout",detail:"waited",attempted:true});
 expect(await (await port(answer({kind:"done",origin:"g1-m06",result:{status:"passed"},exitCode:0,stderr:"",seconds:1})))("a.ts",1000,input())).toMatchObject({kind:"local",reason:"remote-failed-local-redo",attempted:true});
 expect(await (await port(answer({kind:"done",origin:"g1-m06",result:passed("other.ts"),exitCode:0,stderr:"",seconds:1})))("a.ts",1000,input())).toMatchObject({kind:"local",reason:"remote-failed-local-redo",detail:expect.stringContaining("does not name")});
});

test("a run keeps the rows of the machine that ran the file, marks them with its origin, runs a file once however many suites own it, stops after the first failing file with bail, and ends when cancelled",async()=>{
 const asked:string[]=[],remote:TestFileRemoteV1=async file=>{asked.push(file);return file==="c.ts"?{kind:"cancelled"}:{kind:"ran",origin:"g1-m06",seconds:2.5,result:file==="b.ts"?failed(file):passed(file),output:"output of "+file+"\n"};};
 const notices:string[]=[],routes=[route("a.ts","one"),route("a.ts","two"),route("b.ts"),route("c.ts")],options={bail:true,json:false,language:"en" as const};
 await expect(runTestFileRoutesV1(root,routes,{...options,bail:false},{remote,notice:line=>{notices.push(line);},tables:new Map()})).rejects.toMatchObject({key:expect.stringContaining("run.cancelled")});
 expect(asked).toEqual(["a.ts","b.ts","c.ts"]);
 asked.length=0;
 const printed:string[]=[],write=process.stderr.write;
 process.stderr.write=((chunk:unknown)=>{printed.push(String(chunk));return true;}) as typeof process.stderr.write;
 let result;
 try{result=await runTestFileRoutesV1(root,[route("a.ts","one"),route("a.ts","two"),route("b.ts"),route("d.ts")],options,{remote,notice:line=>{notices.push(line);},tables:new Map()});}finally{process.stderr.write=write;}
 expect(printed.join("")).toBe("output of a.ts\noutput of b.ts\n");
 expect(asked).toEqual(["a.ts","b.ts"]);
 expect(result.files.map(row=>[row.file,row.status,row.ranOn])).toEqual([["a.ts","passed","g1-m06"],["b.ts","failed","g1-m06"],["d.ts","not-run",undefined]]);
 expect(result.status).toBe("failed");
 expect(validateTestFileDocumentV1("Result",result)).toEqual([]);
 expect(notices.some(line=>line.includes("a.ts ran on g1-m06 (2.5 s)"))).toBe(true);
 const text=renderTestFileResultV1(result,"en"),german=renderTestFileResultV1(result,"de");
 expect(text).toContain("passed  a.ts (plain, 0.2 s, 3 pass, 0 fail, ran on g1-m06)");
 expect(text).toContain("FAILED  b.ts");
 expect(german).toContain("lief auf g1-m06");
 expect(text).not.toContain("d.ts (plain, 0 s, ran on");
});

test("the command line wires the port in: a file the other machine ran is printed and counted without a local run, and the mode never leaves the old behaviour",async()=>{
 const own="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🧪️tests/🟦️.ts";
 const lines:string[]=[],remote:TestFileRemoteV1=async file=>({kind:"ran",origin:"g1-m11",seconds:3,output:"",result:{...passed(file,"build-test-file"),files:[{...passed(file).files[0]!,mode:"suite",table:"build",suite:"build-test-file"}]}});
 const code=await runTestFileCliV1(root,["--json",own],{...process.env,SEMIO_LANGUAGE:"en"},line=>{lines.push(line);},{notice:()=>{},run:{remote}});
 expect(code).toBe(0);
 const document=JSON.parse(lines.join("\n")) as TestFileResultV1;
 expect(document.files).toHaveLength(1);
 expect(document.files[0]).toMatchObject({file:own,status:"passed",ranOn:"g1-m11"});
 expect(validateTestFileDocumentV1("Result",document)).toEqual([]);
 const plan:string[]=[];
 expect(await runTestFileCliV1(root,["--dry-run","--json",own],{...process.env,SEMIO_TEST_FILE_OFFLOAD:"never"},line=>{plan.push(line);},{notice:()=>{}})).toBe(0);
 expect(JSON.parse(plan.join("\n")).files[0].file).toBe(own);
 const failing:TestFileRemoteV1=async file=>({kind:"ran",origin:"g1-m11",seconds:1,output:"",result:failed(file)});
 const bad:string[]=[];
 expect(await runTestFileCliV1(root,[own],{...process.env,SEMIO_LANGUAGE:"de"},line=>{bad.push(line);},{notice:()=>{},run:{remote:failing}})).toBe(1);
 expect(bad.join("\n")).toContain("FEHLGESCHLAGEN");
 expect(bad.join("\n")).toContain("lief auf g1-m11");
});

test("H44: a saturated host offers the file even while its memory verdict is admit, a host that is neither saturated nor under pressure keeps it, and the measured durations are recorded for the next verdict",async()=>{
 const memoRoot=mkdtempSync(join(tmpdir(),"semio-h44-")),calls:string[]=[],offload=(async()=>{calls.push("offload");return{kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"",streamed:false,seconds:4.2};}) as unknown as Offload;
 try{
  const calm=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"admit",load:async()=>"keep",idle:async()=>3,offload}))!;
  expect(await calm("a.ts",1000,input())).toEqual({kind:"local",reason:"local-slot",detail:"",attempted:false});
  const saturated=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"admit",load:async file=>file==="a.ts"?"offload":"keep",idle:async()=>3,offload}))!;
  expect(await saturated("a.ts",1000,input())).toMatchObject({kind:"ran",origin:"g1-m06"});
  expect(await saturated("b.ts",1000,input())).toMatchObject({kind:"local",reason:"local-slot"});
  expect(calls).toEqual(["offload"]);
  const {readCheckoutTestMeasuresV1}=await import("../../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🚦️gate/🟦️.ts");
  expect(readCheckoutTestMeasuresV1(memoRoot).files["a.ts"]?.remote?.seconds).toBe(4.2);
  await saturated.measure!("a.ts",31.4);
  expect(readCheckoutTestMeasuresV1(memoRoot).files["a.ts"]).toMatchObject({local:{seconds:31.4},remote:{seconds:4.2}});
  expect(saturated.parallel).toBeGreaterThan(1);
 }finally{rmSync(memoRoot,{recursive:true,force:true});}
});

test("H44: the live output of a remote file is prefixed with its position and machine, complete lines at a time, and the rest of the output is prefixed the same way",async()=>{
 const memoRoot=mkdtempSync(join(tmpdir(),"semio-h44-")),lines:string[]=[];
 const offload=(async(call:Parameters<Offload>[0])=>{call.onOutput?.("first\nsec","g1-m06");call.onOutput?.("ond\nthi","g1-m06");return{kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"rd\nfourth",streamed:true,seconds:1};}) as unknown as Offload;
 try{
  const remote=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"wait",idle:async()=>1,offload}))!;
  const outcome=await remote("a.ts",1000,{...input(),output:text=>{lines.push(text);},tag:"2/3 "});
  expect(lines).toEqual(["[2/3 g1-m06] first\n","[2/3 g1-m06] second\n"]);
  expect(outcome).toMatchObject({kind:"ran",output:"[2/3 g1-m06] third\n[2/3 g1-m06] fourth\n"});
  const quiet=(await createTestFileRemoteV1(root,{},{memoRoot,host:async()=>"wait",idle:async()=>1,offload:(async()=>({kind:"done",origin:"g1-m06",result:passed("a.ts"),exitCode:0,stderr:"whole output\n",streamed:false,seconds:1})) as unknown as Offload}))!;
  expect(await quiet("a.ts",1000,input())).toMatchObject({kind:"ran",output:"whole output\n"});
 }finally{rmSync(memoRoot,{recursive:true,force:true});}
});

test("H44: several files of one run go out in parallel up to the width of the port, rows come back in the order of the routes, a width of one or bail keeps them one after the other, and live output reaches stderr as it arrives",async()=>{
 const run=async(parallel:number|undefined,bail=false)=>{
  let active=0,peak=0;
  const asked:string[]=[],remote:TestFileRemoteV1=Object.assign(async(file:string,_budget:number,call:Parameters<TestFileRemoteV1>[2])=>{
   asked.push(file);active++;peak=Math.max(peak,active);
   call.output?.(`[${call.tag}g1-m0${file.charCodeAt(0)-96}] live ${file}\n`);
   await new Promise(done=>setTimeout(done,file==="a.ts"?600:300));
   active--;
   return{kind:"ran",origin:"g1-m0"+(file.charCodeAt(0)-96),seconds:0.1,result:passed(file),output:"tail of "+file+"\n"} as const;
  },parallel===undefined?{}:{parallel});
  const printed:string[]=[],write=process.stderr.write;
  process.stderr.write=((chunk:unknown)=>{printed.push(String(chunk));return true;}) as typeof process.stderr.write;
  const started=Date.now();
  let result;
  try{result=await runTestFileRoutesV1(root,[route("a.ts"),route("b.ts"),route("c.ts")],{bail,json:false,language:"en"},{remote,notice:()=>{},tables:new Map()});}finally{process.stderr.write=write;}
  return{result,peak,printed:printed.join(""),asked,elapsed:Date.now()-started};
 };
 const wide=await run(3);
 expect(wide.peak).toBe(3);
 expect(wide.result.files.map(row=>[row.file,row.status,row.ranOn])).toEqual([["a.ts","passed","g1-m01"],["b.ts","passed","g1-m02"],["c.ts","passed","g1-m03"]]);
 expect(wide.printed).toContain("[1/3 g1-m01] live a.ts\n");
 expect(wide.printed).toContain("[3/3 g1-m03] live c.ts\n");
 expect(wide.printed.endsWith("tail of c.ts\n")).toBe(true);
 expect(wide.printed.indexOf("tail of a.ts")).toBeLessThan(wide.printed.indexOf("tail of b.ts"));
 expect(wide.elapsed,"three files of 600/300/300 ms in parallel take about the longest, not the sum (1200 ms one after the other)").toBeLessThan(1000);
 const narrow=await run(2);
 expect(narrow.peak).toBe(2);
 expect(narrow.result.files.map(row=>row.file)).toEqual(["a.ts","b.ts","c.ts"]);
 for(const [name,outcome] of [["no width",await run(undefined)],["width one",await run(1)],["bail",await run(3,true)]] as const){
  expect(outcome.peak,name).toBe(1);
  expect(outcome.asked,name).toEqual(["a.ts","b.ts","c.ts"]);
  expect(outcome.result.files.map(row=>row.file),name).toEqual(["a.ts","b.ts","c.ts"]);
 }
 const same=await run(3),again=await run(2);
 expect(JSON.stringify(same.result.files.map(row=>[row.file,row.status,row.pass,row.fail,row.ranOn])),"the aggregate is deterministic whatever the width").toBe(JSON.stringify(again.result.files.map(row=>[row.file,row.status,row.pass,row.fail,row.ranOn])));
});

test("H44: a file that runs on no host here is never offered in a parallel run",async()=>{
 const asked:string[]=[],remote:TestFileRemoteV1=Object.assign(async(file:string)=>{asked.push(file);return{kind:"ran",origin:"g1-m06",seconds:0.1,result:passed(file),output:""} as const;},{parallel:3}),notices:string[]=[];
 const result=await runTestFileRoutesV1(root,[route("a.ts"),route("b.ts",undefined,{runsOnHost:false}),route("c.ts")],{bail:false,json:false,language:"en"},{remote,notice:line=>{notices.push(line);},tables:new Map()});
 expect(result.files.map(row=>[row.file,row.status])).toEqual([["a.ts","passed"],["b.ts","skipped"],["c.ts","passed"]]);
 expect([...asked].sort()).toEqual(["a.ts","c.ts"]);
 expect(notices.filter(line=>line.includes("a.ts ran on")||line.includes("c.ts ran on"))).toHaveLength(2);
});

test("H44: a remote row that failed on an envelope deadline refusal is no verdict: the file runs here, a real failure stays a verdict",async()=>{
 const refusing:TestFileResultV1={...failed("a.ts"),files:[{...failed("a.ts").files[0]!,error:"Original Script deadline refused"}]};
 const port=async(result:TestFileResultV1)=>(await createTestFileRemoteV1(root,{},{memoRoot:sharedMemo,host:async()=>"wait",idle:async()=>1,offload:(async()=>({kind:"done",origin:"g1-m13",result,exitCode:1,stderr:"",streamed:false,seconds:1})) as unknown as Offload}))!;
 expect(await (await port(refusing))("a.ts",1000,input())).toMatchObject({kind:"local",reason:"remote-failed-local-redo",detail:expect.stringContaining("deadline refused")});
 expect(await (await port(failed("a.ts")))("a.ts",1000,input())).toMatchObject({kind:"ran",origin:"g1-m13"});
});
