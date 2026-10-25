/** 🎯 Contract: the target cache policy of the build graph (name predicates, class lookup, the replay decision, output admission and execution contracts) equals the authored cases in `./🔣️.json` and an independent Python derivation, every decision it hands out conforms to the closed cache schema under Ajv, and unknown classes, foreign reasons and open records are refused with the authored messages. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {readFileSync} from "node:fs";
import {minimatch} from "minimatch";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import cacheSchema from "../../../../🏃️execution/🧬️schema/💾️cache.json";
import executionSchema from "../../../../🏃️execution/🧬️schema/🔣️.json";
import {buildTaskInputsV1} from "../../../../🧮️task-model/🟦️.ts";
import {bazelTaskSourcePathsV1} from "../../../../🔧️tools/🏗️bazel/📜️script.ts";
import {buildInputPathMatchesV1} from "../../../../📥️inputs/🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const inference=await import("../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cacheSchema,executionSchema,requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const plain=<T>(value:T):T=>value===undefined?value:JSON.parse(JSON.stringify(value));
const outcome=(action:()=>unknown):{result?:unknown;error?:string}=>{try{return{result:plain(action())};}catch(error){return{error:(error as Error).message};}};
const ajv=new Ajv({strict:false}),validDecision=ajv.compile(cacheSchema as object);

test("the corpus is closed by its schema",()=>{
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,names:[{...corpus.names[0],uncached:["x"]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,policies:[{name:"x",target:"t",input:{}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,executions:[{name:"x",target:{},kind:"shell"}]})).toBe(false);
});

test("name predicates, the uncached reason and the emoji strip equal the authored rows and the oracle",()=>{
 const names=corpus.names.map(row=>row.name),[oracle]=ask([{kind:"predicates",names}]);
 for(const[index,row]of corpus.names.entries()){
  const actual={mutating:cacheInternals.mutatingName(row.name),live:cacheInternals.liveName(row.name),uncached:cacheInternals.uncachedDecisionV1(row.name),normalized:cacheInternals.normalizedTargetName(row.name)};
  expect(actual,row.name).toEqual({mutating:row.mutating,live:row.live,uncached:row.uncached,normalized:row.normalized});
  expect(oracle[index],row.name).toEqual(actual);
 }
 const [bare]=ask([{kind:"normalize",names:corpus.normalize.map(row=>row.name)}]);
 for(const[index,row]of corpus.normalize.entries()){expect(cacheInternals.normalizedTargetName(row.name),row.name).toBe(row.bare);expect(bare[index],row.name).toBe(row.bare);}
});

test("uncached names match exactly or by command prefix and a verify command is found only as a whole word",()=>{
 const [matched,verified]=ask([{kind:"matches",policy:corpus.uncachedPolicy,names:corpus.matches.map(row=>row.name)},{kind:"verify",targets:corpus.verify.map(row=>row.target)}]);
 for(const[index,row]of corpus.matches.entries()){expect(cacheInternals.matchesUncached(row.name,corpus.uncachedPolicy),row.name).toBe(row.matches);expect(matched[index],row.name).toBe(row.matches);}
 for(const[index,row]of corpus.verify.entries()){expect(cacheInternals.verifyCommand(row.target),JSON.stringify(row.target)).toBe(row.verify);expect(verified[index],JSON.stringify(row.target)).toBe(row.verify);}
});

test("the class lookup lets an exact name decide before any head, tail or token and refuses rows the schema does not admit",()=>{
 const [oracle]=ask([{kind:"classes",rows:corpus.classRows,names:corpus.classes.map(row=>row.name)}]);
 for(const[index,row]of corpus.classes.entries()){
  const actual=cacheInternals.targetClassV1(row.name,corpus.classRows);
  expect(actual===undefined?null:plain(actual),row.name).toEqual(row.decision);
  expect(oracle[index].decision,row.name).toEqual(row.decision);
  expect(cacheInternals.cacheableFamily(row.name,corpus.classRows),row.name).toBe(row.decision?.class==="deterministic");
  if(row.decision)expect(validDecision(row.decision),row.name).toBe(true);
 }
 const [refused]=ask([{kind:"classes",rows:corpus.classRows,names:[]}]);expect(refused).toEqual([]);
 for(const row of corpus.classRefusals){
  expect(outcome(()=>cacheInternals.targetClassV1(row.target,row.rows)).error,row.name).toBe(row.error);
  expect(ask([{kind:"classes",rows:row.rows,names:[row.target]}])[0][0].error,row.name).toBe(row.error);
 }
});

test("a row names the exclusions of its targets exactly as it classes them and an explicit test read keeps the full inputs",()=>{
 const [named]=ask([{kind:"exclusions",rows:corpus.classRows,names:corpus.exclusionCases.map(row=>row.target)}]);
 for(const[index,row]of corpus.exclusionCases.entries()){
  expect(cacheInternals.targetExclusionsV1(row.target,corpus.classRows)??null,row.name).toBe(row.exclusions);
  expect(named[index]??null,row.name).toBe(row.exclusions);
 }
 const answers=ask(corpus.conflictCases.map(row=>({kind:"conflict",inputs:row.inputs,named:row.named,patterns:row.patterns,root:row.root})));
 for(const[index,row]of corpus.conflictCases.entries()){
  expect(cacheInternals.excludedInputConflictV1(row.inputs,row.named,row.patterns,buildInputPathMatchesV1,row.root),row.name).toBe(row.conflict);
  expect(answers[index],row.name).toBe(row.conflict);
 }
});

test("the exclusions keep a test, feature or story edit out of the selection of a class target and keep every source in it, equal to minimatch",()=>{
 const policyInputs=JSON.parse(readFileSync(join(import.meta.dir,"../../../../🔣️policy.json"),"utf8")).namedInputs;
 for(const row of corpus.selectionCases){
  const graph={version:1,projects:{p:{name:"p",root:row.project.root,targets:{},namedInputs:row.project.namedInputs}},dependencies:[],namedInputs:policyInputs} as any;
  const expanded=buildTaskInputsV1(graph,{project:"p",definition:{inputs:row.inputs}} as any).flatMap(input=>input.kind==="file"?[(input.exclude?"!":"")+input.pattern]:[]);
  const selected=bazelTaskSourcePathsV1(expanded,row.inventory);
  const oracle=row.inventory.filter(path=>expanded.some(pattern=>!pattern.startsWith("!")&&minimatch(path,pattern))&&!expanded.some(pattern=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1))));
  expect(selected,row.name).toEqual([...row.selected]);
  expect([...oracle].sort(),row.name).toEqual([...row.selected]);
 }
});

test("a cache decision is admitted exactly for the reasons its class names in the closed schema",()=>{
 const definitions=Object.values(cacheSchema.definitions) as any[],classes=definitions.map(row=>row.properties.class.const as string),reasons=[...new Set(definitions.flatMap(row=>row.properties.reason.enum as string[]))];
 const pairs=[...classes.flatMap(cls=>reasons.map(reason=>[cls,reason] as const)),["martian","pure-function-of-inputs"],["deterministic","martian"]] as const;
 const answers=ask(pairs.map(([cls,reason])=>({kind:"decision",class:cls,reason})));
 let accepted=0;
 for(const[index,[cls,reason]]of pairs.entries()){
  const made=outcome(()=>cacheInternals.cacheDecisionV1(cls,reason)),expected={version:1,class:cls,reason};
  const wanted=validDecision(expected);
  expect(made.error===undefined,cls+"/"+reason).toBe(wanted);
  expect(answers[index].error===undefined,cls+"/"+reason).toBe(wanted);
  if(wanted){accepted++;expect(made.result).toEqual(expected);expect(answers[index].decision).toEqual(expected);expect(Object.isFrozen(cacheInternals.cacheDecisionV1(cls,reason))).toBe(true);expect(cacheInternals.cacheDecisionV1(cls,reason)).toBe(cacheInternals.cacheDecisionV1(cls,reason));}
  else expect(made.error).toBe(`Repository target cache decision refused: ${cls}/${reason}`);
 }
 expect(accepted).toBe(definitions.reduce((sum,row)=>sum+(row.properties.reason.enum as string[]).length,0));
});

test("the replay decision follows the authored rows and the oracle and every decision it attaches conforms to the schema",()=>{
 const answers=ask(corpus.policies.map(row=>({kind:"policy",name:row.target,target:row.input,policy:corpus.policy})));
 for(const[index,row]of corpus.policies.entries()){
  const actual=outcome(()=>cacheInternals.targetPolicy(row.target,plain(row.input),corpus.policy));
  if("error" in row){expect(actual.error,row.name).toBe(row.error);expect(answers[index].error,row.name).toBe(row.error);continue;}
  expect(actual.error,row.name).toBeUndefined();
  expect(actual.result,row.name).toEqual(row.expected);
  expect(answers[index].result,row.name).toEqual(row.expected);
  const attached=(actual.result as any).metadata?.semio?.cache;
  if(attached)expect(validDecision(attached),row.name).toBe(true);
  expect((actual.result as any).cache===true,row.name).toBe(attached?.class==="deterministic");
 }
 const input={metadata:{semio:{cache:{version:1,class:"live",reason:"drives-running-process"}}}},before=JSON.stringify(input);
 cacheInternals.targetPolicy("audit",input,corpus.policy);expect(JSON.stringify(input),"the input target is not mutated").toBe(before);
});

test("a replayed producer that declares no output stops being replayable unless it is a Rust, host platform or ordering target",()=>{
 const answers=ask(corpus.outputs.map(row=>({kind:"outputs",target:row.target})));
 for(const[index,row]of corpus.outputs.entries()){
  expect(plain(cacheInternals.cachedOutputsAdmittedV1(plain(row.target))),row.name).toEqual(row.expected);
  expect(answers[index],row.name).toEqual(row.expected);
 }
});

test("an execution contract is admitted by kind against the closed schema, named in the refusal and agrees with Ajv",()=>{
 const answers=ask(corpus.executions.map(row=>({kind:"execution",target:row.target,identity:row.identity})));
 const validate=ajv.compile(executionSchema as object);
 for(const[index,row]of corpus.executions.entries()){
  const actual=outcome(()=>cacheInternals.targetExecutionKindV1(plain(row.target),row.identity));
  if("error" in row){expect(actual.error,row.name).toBe(row.error);expect(answers[index].error,row.name).toBe(row.error);}
  else{expect(actual.error,row.name).toBeUndefined();expect(actual.result??null,row.name).toBe(row.kind);expect(answers[index].kind,row.name).toBe(row.kind);}
  const declared=(row.target as any).metadata?.semio?.execution;
  if(declared!==undefined)expect(validate(declared),row.name+" "+JSON.stringify(validate.errors)).toBe(!("error" in row));
 }
});
