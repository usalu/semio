/** 🌡️ Contract: the host capacity ask and the granted-jobs rewrite read option vectors exactly like an independent Python implementation over the systematic argument vectors of `./🔣️.json`, hold its authored expectations, and the wait control cancels on interrupt and termination and releases its handlers. */
import {availableParallelism} from "node:os";
import {EventEmitter} from "node:events";
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,buildCapacityAskV1,createCapacityControlV1,grantedBuildArgumentsV1} from "../../🟦️.ts";
import {runCapacityCommandV1} from "../../⌨️command/🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {currentWorkerHostV1,planWorkersV1,readWorkerPolicyV1} from "../../../👷️workers/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),workers=buildPolicy.workers,policy=buildPolicy.hostCapacity;
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema and the policy used is the authored one",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,granted:[{args:[],operation:"run",jobs:0}]})).toBe(false);
 expect(BUILD_HOST_CAPACITY_POLICY_V1.operations.map(row=>row.name)).toEqual(policy.operations.map(row=>row.name));
});

test("the authored expectations hold",()=>{
 for(const row of corpus.authored){
  const ask=buildCapacityAskV1(row.args,row.environment);
  expect<unknown>(ask===undefined?null:{operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:[...ask.kinds]},row.name).toEqual(row.expect);
 }
});

test("every systematic argument vector is read like the Python implementation, with the host's own job count where no explicit count is given",()=>{
 const answers=ask({op:"ask",policy,workers,rows:corpus.asks}),local=planWorkersV1(readWorkerPolicyV1(workers),currentWorkerHostV1()).localJobs;let explicit=0,hosted=0,light=0;
 for(const[index,row]of corpus.asks.entries()){
  const actual=buildCapacityAskV1(row.args,row.environment),expected=answers[index],label=JSON.stringify(row);
  if(expected===null){expect(actual,label).toBeUndefined();light++;continue;}
  expect(actual,label).toBeDefined();expect(actual!.operation,label).toBe(expected.operation);expect([...actual!.kinds],label).toEqual(expected.kinds);
  if(expected.preferredJobs==="host"){hosted++;expect(actual!.preferredJobs,label).toBe(expected.operation==="cargo"?availableParallelism():local);}else{explicit++;expect(actual!.preferredJobs,label).toBe(expected.preferredJobs);}
  expect(Object.isFrozen(actual),label).toBe(true);expect(Object.isFrozen(actual!.kinds),label).toBe(true);
 }
 expect(explicit).toBeGreaterThan(100);expect(hosted).toBeGreaterThan(100);expect(light).toBeGreaterThan(50);
});

test("the granted jobs replace the parallelism option everywhere before the separator and nowhere after it",()=>{
 const answers=ask({op:"granted",rows:corpus.granted});
 for(const[index,row]of corpus.granted.entries()){
  const actual=grantedBuildArgumentsV1(row.args,row.operation,row.jobs),label=JSON.stringify(row);
  expect(actual,label).toEqual(answers[index]);
  const name=row.operation==="cargo"?"--parallelism":"--parallel",boundary=actual.indexOf("--"),head=boundary<0?actual:actual.slice(0,boundary);
  expect(head.filter(argument=>argument===name||argument.startsWith(name+"=")),label).toEqual([name+"="+row.jobs]);
 }
 const input=["run","--parallel","1"];expect(grantedBuildArgumentsV1(input,"run",4)).toEqual(["run","--parallel=4"]);expect(input).toEqual(["run","--parallel","1"]);
});

test("the wait control cancels on interrupt and termination, has no deadline and releases its handlers when disposed",async()=>{
 for(const signal of corpus.signals){
  const emitter=new EventEmitter(),handle=createCapacityControlV1(emitter as never);
  expect(emitter.listenerCount("SIGINT")).toBe(1);expect(emitter.listenerCount("SIGTERM")).toBe(1);
  expect(handle.control.remainingMilliseconds()).toBeNull();expect(handle.control.signal.aborted).toBe(false);await handle.control.yieldContinuation();expect(handle.control.publish({} as never)).toBeUndefined();
  emitter.emit(signal);expect(handle.control.signal.aborted).toBe(true);expect(isRepoErrorV1(handle.control.signal.reason)).toBe(true);expect(handle.control.signal.reason).toMatchObject({code:"cancelled",key:"capacity.wait-cancelled",values:{signal}});for(const [language,text] of [["en","Waiting for host capacity was cancelled by "+signal],["de","Das Warten auf Rechnerkapazität wurde durch "+signal+" abgebrochen"]])expect(renderRepoErrorV1(handle.control.signal.reason as never,language)).toBe(text);
  handle.dispose();expect(emitter.listenerCount("SIGINT")+emitter.listenerCount("SIGTERM")).toBe(0);expect(Object.isFrozen(handle.control)).toBe(true);
 }
 const emitter=new EventEmitter(),handle=createCapacityControlV1(emitter as never);handle.dispose();emitter.emit("SIGINT");expect(handle.control.signal.aborted).toBe(false);
});

test("the capacity command refuses every malformed option vector with the Python reading's message before it touches the host",async()=>{
 const answers=ask({op:"command",policy,rows:corpus.commands});let refused=0,accepted=0;const messages=new Set<string>();
 for(const[index,row]of corpus.commands.entries()){
  const expected=answers[index];
  if(expected===null){accepted++;continue;}
  refused++;messages.add(expected);
  await expect(runCapacityCommandV1(row,{environment:{},write:()=>undefined,policy:BUILD_HOST_CAPACITY_POLICY_V1} as never),JSON.stringify(row)).rejects.toThrow(expected);
 }
 expect(refused).toBeGreaterThan(300);expect(accepted).toBeGreaterThan(10);
 expect([...messages].sort()).toEqual(["Capacity option --cpu requires a positive integer","Capacity option --jobs requires a positive integer","Capacity option --memory requires a positive integer","Capacity run requires a command after --","Expected repo capacity status|run","Unknown capacity operation: ","Unknown capacity operation: nope"]);
});
