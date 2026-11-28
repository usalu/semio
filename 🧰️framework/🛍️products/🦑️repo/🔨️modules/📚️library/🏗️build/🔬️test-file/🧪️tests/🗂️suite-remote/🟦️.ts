import {expect,test} from "bun:test";
import {createSuiteRemoteV1} from "../../🌐️remote/🗂️suite/🟦️.ts";
import type {TestFileRemoteInputV1,TestFileRemoteOutcomeV1,TestFileRemoteV1} from "../../🌐️remote/🟦️.ts";
import type {TestFileResultV1,TestFileRouteV1} from "../../🟦️.ts";

const route=(patch:Partial<TestFileRouteV1>={}):TestFileRouteV1=>({file:"f/🟦️.ts",mode:"suite",table:"t",suite:"s",owner:"o",typecheck:"none",budgetMs:300000,python:false,launcher:null,runsOnHost:true,...patch});
const result=(status:"passed"|"failed",suite="s"):TestFileResultV1=>({version:1,status,seconds:1,files:[{file:"f/🟦️.ts",mode:"suite",table:"t",suite,status,seconds:1}]} as unknown as TestFileResultV1);

/** 🧪️ A fake remote port that records every ask and answers `outcome`. */
function portV1(outcome:TestFileRemoteOutcomeV1,parallel=3){
 const asks:{file:string;budgetMs:number;input:TestFileRemoteInputV1}[]=[],measured:string[]=[];
 const port:TestFileRemoteV1=Object.assign(async(file:string,budgetMs:number,input:TestFileRemoteInputV1)=>{asks.push({file,budgetMs,input});return outcome;},{parallel,measure:async(file:string)=>{measured.push(file);}});
 return{port,asks,measured};
}
const context=(patch:Partial<{budgetMs:number;index:number;total:number;signal:AbortSignal}>={})=>({budgetMs:300000,index:1,total:3,signal:new AbortController().signal,...patch});

test("a file of this suite alone leaves, the other machine's verdict is its verdict, the width and the nested guard come from the port",async()=>{
 const passed=portV1({kind:"ran",origin:"g1-m11",seconds:4.2,result:result("passed"),output:"[2/3 g1-m11] 4 pass\n"}),notices:string[]=[];
 const remote=await createSuiteRemoteV1("/checkout","s",{SEMIO_LANGUAGE:"en"},{remote:passed.port,route:()=>[route()],notice:line=>notices.push(line)});
 expect(remote!.width).toBe(3);expect(remote!.environment).toEqual({SEMIO_TEST_FILE_OFFLOAD:"never"});
 const ran=await remote!.file("f/🟦️.ts",context());
 expect(ran).toEqual({origin:"g1-m11",seconds:4.2,passed:true,output:"[2/3 g1-m11] 4 pass\n"});
 expect(passed.asks).toHaveLength(1);expect(passed.asks[0]!.input.tag).toBe("2/3 ");expect(passed.asks[0]!.budgetMs).toBeGreaterThanOrEqual(300000+120000);
 expect(notices.join("\n")).toContain("g1-m11");
 await remote!.measure!("f/🟦️.ts",3);expect(passed.measured).toEqual(["f/🟦️.ts"]);
 const failed=portV1({kind:"ran",origin:"g1-m12",seconds:1,result:result("failed"),output:""});
 const second=await createSuiteRemoteV1("/checkout","s",{},{remote:failed.port,route:()=>[route()],notice:()=>{}});
 expect((await second!.file("f/🟦️.ts",context()))!.passed).toBe(false);
});

test("a file of two suites, of another suite, with a launcher or off this host never leaves; a refusal, a local answer and a cancel are handled",async()=>{
 const port=portV1({kind:"local",reason:"no-idle-machine",detail:"",attempted:true}),notices:string[]=[];
 for(const routes of [[route(),route({suite:"other"})],[route({suite:"other"})],[route({launcher:"genuine"})],[route({runsOnHost:false})],[route({mode:"plain"})],[]]){
  const remote=await createSuiteRemoteV1("/checkout","s",{},{remote:port.port,route:()=>routes,notice:()=>{}});
  expect(await remote!.file("f/🟦️.ts",context())).toBeUndefined();
 }
 expect(port.asks).toHaveLength(0);
 const broken=await createSuiteRemoteV1("/checkout","s",{},{remote:port.port,route:()=>{throw Error("unroutable");},notice:()=>{}});
 expect(await broken!.file("f/🟦️.ts",context())).toBeUndefined();
 const local=await createSuiteRemoteV1("/checkout","s",{},{remote:port.port,route:()=>[route()],notice:line=>notices.push(line)});
 expect(await local!.file("f/🟦️.ts",context())).toBeUndefined();expect(port.asks).toHaveLength(1);expect(notices.join("\n")).toContain("no-idle-machine");
 const cancelled=portV1({kind:"cancelled"});
 const stop=await createSuiteRemoteV1("/checkout","s",{},{remote:cancelled.port,route:()=>[route()],notice:()=>{}});
 await expect(stop!.file("f/🟦️.ts",context())).rejects.toThrow();
});

test("mode never keeps every file here and gives no remote",async()=>{
 expect(await createSuiteRemoteV1("/checkout","s",{SEMIO_TEST_FILE_OFFLOAD:"never"})).toBeUndefined();
});
