import {resolve} from "node:path";
import {writeSync} from "node:fs";
import {createIssuedResourceControlV1,receiveScriptProcessInvocation,withIssuedResourceOperationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {ConcurrentEditErrorV1,churnProtocolV1,concurrentEditV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {nativeHostSourceEditProtocolV1, takeNativeHostSourceEditReporterV1} from "../🟦️.ts";

const [fixture,path,mode]=process.argv.slice(2) as [string,string,string];
const {environment,marker}=nativeHostSourceEditProtocolV1;
const request=process.env[environment];
const observe=takeNativeHostSourceEditReporterV1(process.env,process.cwd());
if(process.env[environment]!==undefined||process.env.SEMIO_BUILD_MODE!==mode)throw Error("Child receipt environment or profile disagrees");
const write=(value:unknown)=>writeSync(2,marker+JSON.stringify(value)+"\n");
const row={version:1,request,path,reason:"input"};
if(fixture==="ordinary-75")process.exit(75);
if(fixture==="attempt-diagnostic"){
  writeSync(2,'[cargo-plan-drift] {"field":"source","category":"graph-comparison-limit"}\n');
  process.exit(75);
}
if(fixture==="wrong-request"){write({...row,request:"a".repeat(32)});process.exit(75);}
if(fixture==="malformed"){write({...row,extra:true});process.exit(75);}
if(fixture==="duplicate"){write(row);write(row);process.exit(75);}
if(fixture==="foreign-path"){write({...row,path:resolve(process.cwd(),"../foreign.rs")});process.exit(75);}
if(fixture==="ordinary-nonzero"){write(row);process.exit(1);}
if(fixture==="success"){write(row);process.exit(0);}
if(fixture==="wrong-stream"){writeSync(1,marker+JSON.stringify(row)+"\n");process.exit(75);}
if(fixture==="cancelled"){write(row);await Bun.sleep(10000);process.exit(75);}
await receiveScriptProcessInvocation(process.env,async original=>{
  const phase=createIssuedResourceControlV1(original,"test:native-host-source-edit-drain",0);
  void withIssuedResourceOperationV1(phase,async()=>{await Bun.sleep(5);writeSync(2,"fixture:drained\n");});
  const error=concurrentEditV1("input",path,"Fixture verified compiler source changed","build");
  if(fixture==="mixed-failure")throw new AggregateError([error,Error("Ordinary cleanup failed")],"fixture cleanup");
  if(fixture==="terminal-exhausted-input")throw new ConcurrentEditErrorV1("input",path,undefined,churnProtocolV1().profiles.build.rank,{attempts:2,message:"Fixture exhausted compiler Source plan",cause:error});
  throw error;
},observe);
