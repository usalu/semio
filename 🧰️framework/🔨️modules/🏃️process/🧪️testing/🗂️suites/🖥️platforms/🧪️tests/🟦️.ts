import{test,expect}from"bun:test";
import Ajv from"ajv";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{captureOwnedCommandV1}from"../../../../🎛️owned-execution/🟦️.ts";
import{testScratchRootV1}from"../../../🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../🧰️toolchain/🟦️.ts";
import{discoverProcessTestFilesV1,expandProcessTestSuiteV1,readProcessTestTableV1}from"../../🟦️.ts";
import{type ProcessPlatformCauseV1,processPlatformCausesOfSuiteV1,processPlatformEvidenceV1,processPlatformFindingsV1,processPlatformVerdictV1,readProcessPlatformPolicyV1}from"../🟦️.ts";
import tableSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import policy from"../🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const owner=resolve(import.meta.dir,"../../../.."),library=resolve(owner,"../../🛍️products/🦑️repo/🔨️modules/📚️library"),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),hosts=["darwin","linux","win32"]as const;
const roots={process:owner,caching:join(library,"⚡️caching"),build:join(library,"🏗️build")}as const,tableFiles={process:join(owner,"🧪️testing/🗂️suites/🔣️.json"),caching:join(library,"⚡️caching/🗂️suites/🔣️.json"),build:join(library,"🏗️build/🗂️suites/🔣️.json")}as const;
const python=():string=>{const found=testPythonV1()??Bun.which("python3")??Bun.which("python");if(!found)throw Error("Independent Python oracle required");return found;};
const ask=async(args:readonly string[],input:unknown):Promise<any>=>{
 const scratch=mkdtempSync(join(testScratchRootV1("platforms"),"ask-")),path=join(scratch,"input.json");
 try{writeFileSync(path,JSON.stringify(input));return JSON.parse(await captureOwnedCommandV1(python(),[oracle,args[0]!,...args.slice(1),path],owner,{output:{maximumBytes:4194304,maximumLines:16}}));}finally{rmSync(scratch,{recursive:true,force:true});}
};
const withPolicy=async(value:unknown,mode:"evidence"|"suites",rows:unknown):Promise<any>=>{
 const scratch=mkdtempSync(join(testScratchRootV1("platforms"),"policy-")),policyPath=join(scratch,"policy.json"),rowsPath=join(scratch,"rows.json");
 try{writeFileSync(policyPath,JSON.stringify(value));writeFileSync(rowsPath,JSON.stringify(rows));return JSON.parse(await captureOwnedCommandV1(python(),[oracle,mode,policyPath,rowsPath],owner,{output:{maximumBytes:4194304,maximumLines:16}}));}finally{rmSync(scratch,{recursive:true,force:true});}
};
const read=(name:keyof typeof roots)=>readProcessTestTableV1(JSON.parse(readFileSync(tableFiles[name],"utf8")));
const required=():ReadonlyMap<string,ReadonlySet<string>>=>{
 const ci=JSON.parse(readFileSync(join(library,"🏗️build/🧾️ci-report/🗂️suites/🔣️.json"),"utf8"))as{mustRun:Record<string,{table:string;suite:string}[]>};
 return new Map(hosts.map(host=>[host,new Set([...ci.mustRun.all,...ci.mustRun[host]!].map(row=>row.table+"/"+row.suite))]));
};
const project=(name:keyof typeof roots)=>JSON.parse(readFileSync(join(name==="process"?owner:join(library,"⚡️caching"),"📋️project.json"),"utf8"))as{targets:Record<string,{cache?:boolean;options?:{command?:string};metadata?:{semio?:{cache?:{class?:string}}}}>};

test("the evidence rules are a closed document that Ajv and the module validator both admit, and the causes of the rules are the causes of the route table schema",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);
 expect(ajv.validate({$ref:schema.$id+"#/definitions/Policy"},policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(readProcessPlatformPolicyV1(policy).causes.length).toBe(policy.causes.length);
 expect(policy.causes.map(cause=>cause.id).sort()).toEqual([...tableSchema.definitions.PlatformCause.enum].sort());
 const broken:unknown[]=[{...policy,extra:1},{...policy,version:2},{...policy,causes:[]},{...policy,causes:[policy.causes[0],policy.causes[0]]},{...policy,causes:[{...policy.causes[0]!,id:"windows"}]},{...policy,header:"(["},{...policy,causes:[{...policy.causes[0]!,markers:["("]}]},{...policy,window:{linesBeforeMarker:3}}];
 for(const value of broken){expect(ajv.validate({$ref:schema.$id+"#/definitions/Policy"},value)&&(()=>{try{readProcessPlatformPolicyV1(value);return true;}catch{return false;}})(),JSON.stringify(value).slice(0,80)).toBe(false);}
});

test("every evidence vector gives the same causes in the module and in the independent Python reading, and the rules are not decoration: dropping a marker or the line guard changes a verdict both readers agree on",async()=>{
 const texts=fixture.vectors.map(vector=>vector.text);
  const actual=fixture.vectors.map(vector=>processPlatformEvidenceV1(vector.text,readProcessPlatformPolicyV1(policy)));
  const python1=await withPolicy(policy,"evidence",texts);
  fixture.vectors.forEach((vector,index)=>{expect<unknown>(actual[index],vector.name).toEqual(vector.expected);expect<unknown>(python1[index],vector.name).toEqual(vector.expected);});
  const dropped={...policy,causes:policy.causes.map(cause=>cause.id==="symlink"?{...cause,markers:["\\bnever-matches\\b"]}:cause)},ungated={...policy,causes:policy.causes.map(cause=>cause.id==="symlink"?{...cause,lineGate:false}:cause)};
  for(const[name,mutated]of[["dropped marker",dropped],["line guard removed",ungated]]as const){
   const ts=fixture.vectors.map(vector=>processPlatformEvidenceV1(vector.text,readProcessPlatformPolicyV1(mutated))),py=await withPolicy(mutated,"evidence",texts);
   expect(ts,name).toEqual(py);expect(ts,name).not.toEqual(actual);
  }
});

test("a verdict is every host without causes, every host when the CI policy requires Windows, and the POSIX hosts with the causes otherwise",()=>{
 expect(processPlatformVerdictV1([],false)).toEqual({platforms:[...hosts],platformCauses:[]});
 expect(processPlatformVerdictV1(["symlink"],true)).toEqual({platforms:[...hosts],platformCauses:[]});
 expect(processPlatformVerdictV1(["symlink","permission-bits"],false)).toEqual({platforms:["darwin","linux"],platformCauses:["symlink","permission-bits"]});
});

test("the findings of a route table are the suites without a verdict, without an explanation and with a needless one, equal to the Python reading",async()=>{
 const base={version:1,codeSuites:[],helpers:[]},row=(id:string,extra:object)=>({id,budgetMs:1000,typecheck:"none",files:["a.ts"],...extra});
 const table=readProcessTestTableV1({...base,suites:[row("judged",{platforms:[...hosts]}),row("missing",{}),row("bare",{platforms:["darwin","linux"]}),row("posix",{platforms:["darwin","linux"],platformCauses:["symlink"]}),row("needless",{platforms:[...hosts],platformCauses:["symlink"]})]});
 const actual=processPlatformFindingsV1(table);
 expect(actual).toEqual({unjudged:["missing"],unexplained:["bare"],overexplained:["needless"]});
 expect(await ask(["findings"],table)).toEqual(actual);
});

test("every suite of the process, caching and build route tables has an explicit and explained platform verdict, agreeing with the independent Python reading",async()=>{
 for(const name of Object.keys(roots)as(keyof typeof roots)[]){
  const declared=read(name),actual=processPlatformFindingsV1(declared);
  expect(actual,name).toEqual({unjudged:[],unexplained:[],overexplained:[]});
  expect(await ask(["findings"],declared),name).toEqual(actual);
 }
});

test("the platforms of every suite follow its evidence: Windows is named exactly when no test file uses a host-bound fact unconditionally or the CI policy requires the suite there, and the causes are exactly those facts; the module and the Python reading find the same facts",async()=>{
 const needed=required().get("win32")!,rows:{id:string;files:string[]}[]=[],declaredBy=new Map<string,ReturnType<typeof read>["suites"][number]>(),facts=new Map<string,readonly ProcessPlatformCauseV1[]>(),rules=readProcessPlatformPolicyV1(policy);
 for(const name of Object.keys(roots)as(keyof typeof roots)[]){
  const declared=read(name),found=discoverProcessTestFilesV1(roots[name]);
  for(const suite of declared.suites){
   const files=expandProcessTestSuiteV1(suite,found,declared),key=name+"/"+suite.id;
   declaredBy.set(key,suite);facts.set(key,processPlatformCausesOfSuiteV1(suite,roots[name],files,rules));
   rows.push({id:key,files:[...files,...(suite.function?[suite.function.module]:[])].map(file=>resolve(roots[name],file))});
  }
 }
 const independent=await withPolicy(policy,"suites",rows),wrong:string[]=[];
 for(const[key,suite]of declaredBy){
  expect(independent[key],key).toEqual(facts.get(key));
  const causes=facts.get(key)!,verdict=processPlatformVerdictV1(causes,needed.has(key));
  if(JSON.stringify(suite.platforms)!==JSON.stringify(verdict.platforms)||JSON.stringify(suite.platformCauses??[])!==JSON.stringify(verdict.platformCauses))wrong.push(key+" declares "+JSON.stringify(suite.platforms)+JSON.stringify(suite.platformCauses??[])+" but its evidence says "+JSON.stringify(verdict));
 }
 expect(wrong).toEqual([]);
});

test("a suite that leaves a host out is never replayed from a cache: its project target is uncacheable and of the live class",()=>{
 for(const name of Object.keys(roots)as(keyof typeof roots)[]){
  const targets=project(name).targets,pattern=/ test (\S+)$/u,byId=new Map<string,string>();
  for(const[target,row]of Object.entries(targets)){const match=pattern.exec(row.options?.command??"");if(match)byId.set(match[1]!,target);}
  const open:string[]=[];
  for(const suite of read(name).suites){
   if(hosts.every(host=>(suite.platforms as readonly string[]).includes(host)))continue;
   const target=targets[byId.get(suite.id)??""];
   if(!target||target.cache!==false||target.metadata?.semio?.cache?.class!=="live")open.push(suite.id);
  }
  expect(open,name).toEqual([]);
 }
});

test("the suites the CI policy requires on a host are declared for that host, so the host job and the suite sweep agree",()=>{
 const need=required(),named=new Map<string,Set<string>>();
 for(const name of Object.keys(roots)as(keyof typeof roots)[])for(const suite of read(name).suites)named.set(name+"/"+suite.id,new Set(suite.platforms));
 for(const host of hosts)for(const key of need.get(host)!){expect(named.has(key),key+" exists").toBe(true);expect(named.get(key)!.has(host),key+" on "+host).toBe(true);}
});
