/** 🧯️ Laws of the memory ceiling of `bun test` processes: the arguments split like an independent parser splits them, a process is held to its biggest file, only a `bun test` command has a ceiling, the hook terminates once above the ceiling, and a real `bun test` process run by the owned command runners that holds more than its ceiling is terminated and named (EN and DE), within its ceiling or with the ceiling lifted it passes. */
import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{parseArgs}from"node:util";
import Ajv from"ajv";
import{createTestFileMemoryV1,readTestFileBudgetsV1}from"../../../👷️workers/📏️file-budgets/🟦️.ts";
import{testScratchRootV1}from"../../../🏖️sandbox/🟦️.ts";
import{bunTestCeilingV1,bunTestLimitsV1,readBunTestBudgetsV1,splitBunTestArgumentsV1}from"../🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";
import schema from"./🧬️schema/🔣️.json";

const table=(rows:readonly(readonly[string,number])[])=>readTestFileBudgetsV1({version:1,ceilingBasisPoints:15000,unlistedCeilingMegabytes:3072,files:[...rows].sort(([left],[right])=>left<right?-1:1).map(([path,megabytes])=>({path,megabytes,reason:"fixture"}))});
const oracle=(args:readonly string[])=>parseArgs({args:[...args],allowPositionals:true,strict:false,options:{timeout:{type:"string"},"rerun-each":{type:"string"},"test-name-pattern":{type:"string",short:"t"},preload:{type:"string",short:"r"},reporter:{type:"string"},"reporter-outfile":{type:"string"},"coverage-reporter":{type:"string"},"coverage-dir":{type:"string"},"max-concurrency":{type:"string"}}}).positionals;

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the operands and flags of a bun test command equal what an independent parser (node:util parseArgs) finds",()=>{
 for(const row of fixture.arguments){
  const split=splitBunTestArgumentsV1(row.arguments);
  expect(split.operands,row.arguments.join(" ")).toEqual(row.operands);expect(split.flags).toEqual(row.flags);
  expect([...split.operands],"oracle: "+row.arguments.join(" ")).toEqual(oracle(row.arguments));
 }
 let seed=7;const next=(bound:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%bound;};
 const pool=[["--timeout","60000"],["-t","pattern"],["--preload","./p.ts"],["--only"],["--timeout=9"],["/x/🟦️.ts"],["a.ts"],["./dir/b.ts"]];
 for(let round=0;round<300;round++){
  const args=Array.from({length:next(7)},()=>pool[next(pool.length)]!).flat(),split=splitBunTestArgumentsV1(args);
  expect([...split.operands],args.join(" ")).toEqual(oracle(args));expect([...split.operands,...split.flags].sort()).toEqual([...args].sort());
 }
});

test("a process is held to the biggest budget and the highest ceiling of its files, an undeclared file to the runaway guard",()=>{
 const rows=fixture.budgets,budgets=table(rows.table as [string,number][]),memory=createTestFileMemoryV1({budgets,checkout:"/w",defaultMegabytes:rows.defaultMegabytes,shareMegabytes:rows.defaultMegabytes,environment:{}});
 for(const row of rows.cases)expect(bunTestLimitsV1(memory,"/w",row.operands),row.operands.join(",")).toEqual({budgetMegabytes:row.budgetMegabytes,ceilingMegabytes:row.ceilingMegabytes,declared:row.declared});
});

test("only a bun test command has a ceiling, and the hook terminates a process tree once, when it first holds more than the ceiling",async()=>{
 const budgets=table([["a.ts",100]]);
 for(const[command,args]of[[process.execPath,["run","a.ts"]],["git",["test","a.ts"]],[process.execPath,[]]]as const)expect(bunTestCeilingV1(command,args,"/w",{},{budgets,checkout:"/w"}),command+" "+args.join(" ")).toBeUndefined();
 expect(bunTestCeilingV1(process.execPath,["test","./a.ts"],"/w",{SEMIO_TEST_FILE_CEILING:"off"},{budgets,checkout:"/w"}),"lifted").toBeUndefined();
 let megabytes=10;const rows=async()=>[{pid:1,parent:0,rssKilobytes:1024},{pid:7,parent:1,rssKilobytes:megabytes*1024},{pid:8,parent:7,rssKilobytes:megabytes*1024}],terminated:string[]=[];
 const hook=bunTestCeilingV1(process.execPath,["test","--timeout","5","./a.ts"],"/w",{LANG:"en_US.UTF-8"},{budgets,checkout:"/w",rows,intervalMilliseconds:5})!;
 const release=hook(7,reason=>terminated.push(reason));
 await Bun.sleep(60);expect(terminated,"2 x 10 MB is below 150 MB").toEqual([]);
 megabytes=80;await Bun.sleep(60);expect(terminated.length,"2 x 80 MB = 160 MB is above 150 MB").toBe(1);
 expect(terminated[0]).toMatch(/^\[tests\] a\.ts held 160 MB and was stopped at its ceiling of 150 MB \(100 MB declared in the test file budgets\)$/u);
 await Bun.sleep(60);expect(terminated.length,"reported once").toBe(1);release();
});

test("SEMIO_TEST_FILE_BUDGETS names the document that replaces the authored table",()=>{
 const root=mkdtempSync(join(testScratchRootV1("ceiling"),"budgets-")),path=join(root,"table.json");
 try{
  writeFileSync(path,JSON.stringify({version:1,ceilingBasisPoints:20000,unlistedCeilingMegabytes:2048,files:[{path:"x.ts",megabytes:64,reason:"fixture"}]}));
  expect(readBunTestBudgetsV1({SEMIO_TEST_FILE_BUDGETS:path}).unlistedCeilingMegabytes).toBe(2048);expect(readBunTestBudgetsV1({}).unlistedCeilingMegabytes).toBe(3072);
  writeFileSync(path,JSON.stringify({version:2}));expect(()=>readBunTestBudgetsV1({SEMIO_TEST_FILE_BUDGETS:path})).toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

const HELPER=`import{runOwnedCommand}from${JSON.stringify(new URL("../../../../🎛️owned-execution/🟦️.ts",import.meta.url).href)};
import{runBudgetedTestCommand}from${JSON.stringify(new URL("../../../🎛️execution/🟦️.ts",import.meta.url).href)};
const[root,mode]=process.argv.slice(2),env={...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_TEST_FILE_BUDGETS:root+"/table.json"},args=["test","--timeout","60000",root+"/hold.ts"];
try{
 if(mode==="owned")await runOwnedCommand(process.execPath,args,root,"ceiling",120000,{env});
 else await runBudgetedTestCommand(process.execPath,args,{cwd:root,budgetMs:120000,env,throwOnFailure:true});
 console.log("RESULT "+JSON.stringify({ok:true}));
}catch(error){console.log("RESULT "+JSON.stringify({ok:false,message:String((error as Error).message)}));}
`;
const FIXTURE=`import{test}from"bun:test";
import{writeFileSync}from"node:fs";
test("holds memory",async()=>{
 writeFileSync(process.env.CEILING_FIXTURE_PID!,String(process.pid));
 const block=Buffer.alloc(Number(process.env.CEILING_FIXTURE_MB)*1048576,1);
 await new Promise(done=>setTimeout(done,Number(process.env.CEILING_FIXTURE_MS)));
 if(block.length<1)throw Error("unreachable");
});
`;

const run=(declared:number,mode:"owned"|"budgeted",extra:Record<string,string>={})=>{
 const root=mkdtempSync(join(testScratchRootV1("ceiling"),"run-")),pidFile=join(root,"pid");
 writeFileSync(join(root,"hold.ts"),FIXTURE);writeFileSync(join(root,"helper.ts"),HELPER);mkdirSync(join(root,".git"));
 writeFileSync(join(root,"table.json"),JSON.stringify({version:1,ceilingBasisPoints:15000,unlistedCeilingMegabytes:3072,files:[{path:"hold.ts",megabytes:declared,reason:"fixture"}]}));
 try{
  const done=Bun.spawnSync([process.execPath,join(root,"helper.ts"),root,mode],{cwd:root,env:{...process.env,CEILING_FIXTURE_PID:pidFile,CEILING_FIXTURE_MB:String(fixture.run.holdMegabytes),CEILING_FIXTURE_MS:String(fixture.run.holdMilliseconds),LANG:"en_US.UTF-8",...extra},stdout:"pipe",stderr:"pipe"});
  const line=done.stdout.toString().split("\n").find(row=>row.startsWith("RESULT "));
  if(!line)throw Error("no result: "+done.stderr.toString().slice(-2000)+done.stdout.toString().slice(-2000));
  const pid=existsSync(pidFile)?Number(readFileSync(pidFile,"utf8")):0,alive=pid>0&&(()=>{try{process.kill(pid,0);return true;}catch{return false;}})();
  return{...(JSON.parse(line.slice(7)) as{ok:boolean;message?:string}),alive,stderr:done.stderr.toString()};
 }finally{rmSync(root,{recursive:true,force:true});}
};

for(const mode of["owned","budgeted"]as const)test(`a real bun test process run by the ${mode} command runner above its ceiling is terminated and named in English and German, within its budget it passes, the switch lifts the ceiling`,()=>{
 const stopped=run(fixture.run.declaredBelow,mode),words=mode==="owned"?stopped.message!:stopped.stderr;
 expect(stopped.ok,"stopped").toBe(false);expect(words).toMatch(/hold\.ts held \d+ MB and was stopped at its ceiling of 150 MB \(100 MB declared in the test file budgets\)/u);expect(stopped.alive,"the process is gone").toBe(false);
 const german=run(fixture.run.declaredBelow,mode,{LANG:"de_DE.UTF-8"});
 expect(mode==="owned"?german.message:german.stderr).toMatch(/hold\.ts belegte \d+ MB und wurde an seiner Obergrenze von 150 MB angehalten/u);
 expect(run(fixture.run.declaredAbove,mode),"within its budget").toMatchObject({ok:true,alive:false});
 expect(run(fixture.run.declaredBelow,mode,{SEMIO_TEST_FILE_CEILING:"off"}),"with the ceiling lifted").toMatchObject({ok:true,alive:false});
},240000);
