/** 🪢 Contract: the revision graph rules of the build inference (revision tokens, registry keys, roster admission, revision identity, revisioned imports and wiring visibility) equal the authored cases in `./🔣️.json` and an independent Python derivation; the actual component roster of the build satisfies the same admission and a failed wiring leaves every module free to be wired again. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import actualRoster from "../../../🗂️components/🔣️.json";
import {createRevisionIdentity,createRevisionLoader,importRevisionedComponents,readComponentRoster,registryKey,wireRevisionedComponents} from "../../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string|null=>{try{action();return null;}catch(error){return (error as Error).message;}};
const settle=async(action:()=>Promise<unknown>):Promise<string|null>=>{try{await action();return null;}catch(error){return (error as Error).message;}};
const BADREV="Revision graph requires a lowercase sha256 revision";

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,revision:"abc"})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,keys:[{name:"A",key:"a"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,identity:[{name:"x",ops:[{op:"peek"}]}]})).toBe(false);
});

test("a revision token is exactly 64 lowercase hex digits and a registry key camel-cases its dashes",()=>{
 const [valid,keys]=ask([{kind:"revisions",values:corpus.revisions.map(row=>row.value)},{kind:"keys",names:corpus.keys.map(row=>row.name)}]);
 for(const[index,row]of corpus.revisions.entries()){
  expect(refusal(()=>createRevisionIdentity("u").bind(row.value))===null,JSON.stringify(row.value)).toBe(row.valid);
  expect(valid[index],JSON.stringify(row.value)).toBe(row.valid);
 }
 for(const[index,row]of corpus.keys.entries()){expect(registryKey(row.name),row.name).toBe(row.key);expect(keys[index],row.name).toBe(row.key);}
});

test("a roster is admitted only with unique portable entries whose needs name earlier components, and reports the first fault",()=>{
 const answers=ask(corpus.rosters.map(row=>({kind:"roster",manifest:row.manifest})));
 for(const[index,row]of corpus.rosters.entries()){
  expect(refusal(()=>readComponentRoster(structuredClone(row.manifest))),row.name).toBe(row.error);
  expect(answers[index].error,row.name).toBe(row.error);
 }
 const roster=readComponentRoster({version:1,components:[{name:"a",path:"./a.mjs",needs:[]},{name:"b",path:"./b.mjs",needs:["a"]}]});
 expect(Object.isFrozen(roster)).toBe(true);expect(Object.isFrozen(roster[1])).toBe(true);expect(Object.isFrozen(roster[1]!.needs)).toBe(true);
 expect(refusal(()=>{(roster as any).push(1);})).not.toBeNull();
 expect(()=>readComponentRoster(actualRoster)).not.toThrow();
});

test("a revision identity is bound once, to a valid token, and cannot be read before",()=>{
 const answers=ask(corpus.identity.map(row=>({kind:"identity",ops:row.ops})));
 for(const[index,row]of corpus.identity.entries()){
  const identity=createRevisionIdentity("file:///graph.mjs");
  expect(identity.url).toBe("file:///graph.mjs");expect(Object.isFrozen(identity)).toBe(true);
  for(const[step,op]of row.ops.entries()){
   const actual:Record<string,string>={};
   try{if(op.op==="read")actual.revision=identity.revision;else identity.bind(op.value as string);}catch(error){actual.error=(error as Error).message;}
   const expected={...(op.error?{error:op.error}:{}),...("revision" in op&&op.revision?{revision:op.revision}:{})};
   expect(actual,row.name+" #"+step).toEqual(expected);
   expect(answers[index][step],row.name+" #"+step).toEqual(expected);
  }
 }
});

test("every component is imported under the revision token and a failing import rejects the load",async()=>{
 const answers=ask(corpus.imports.map(row=>({kind:"imports",paths:row.paths,revision:row.revision,failOn:(row as any).failOn})));
 for(const[index,row]of corpus.imports.entries()){
  const roster=readComponentRoster({version:1,components:row.paths.map((path,position)=>({name:"c"+position,path,needs:[]}))});
  const calls:string[]=[],load=async(specifier:string)=>{calls.push(specifier);if(specifier.split("?")[0]===(row as any).failOn)throw Error("load refused: "+(row as any).failOn);return{component:specifier,wire(){}};};
  expect(await settle(()=>importRevisionedComponents({roster,revision:row.revision,load})),row.name).toBe(row.error);
  expect(calls,row.name).toEqual(row.calls);
  expect(answers[index],row.name).toEqual({calls:row.calls,error:row.error});
  if(row.error===null){const modules=await importRevisionedComponents({roster,revision:row.revision,load});expect(Object.keys(modules)).toEqual(roster.map(entry=>entry.name));expect(Object.isFrozen(modules)).toBe(true);}
 }
});

test("wiring gives every component only its needs and the shared services, in roster order, and a failed wiring can be repeated",()=>{
 const answers=ask(corpus.wiring.map(row=>({kind:"wiring",components:row.components,attempts:row.attempts})));
 for(const[index,row]of corpus.wiring.entries()){
  const roster=readComponentRoster({version:1,components:row.components.map(component=>({name:component.name,path:"./"+component.name+".mjs",needs:component.needs}))});
  const state={fail:null as string|null,order:[] as string[],visible:{} as Record<string,string[]>},services={},register=()=>undefined,identity=createRevisionIdentity("u");
  const modules=Object.fromEntries(row.components.filter(component=>!(component as any).absent).map(component=>[component.name,{component:(component as any).component??component.name,...((component as any).wire===false?{}:{wire:(registry:any)=>{
   state.order.push(component.name);
   expect(Object.isFrozen(registry)).toBe(true);expect(registry.services).toBe(services);expect(registry.revisionIdentity).toBe(identity);expect(registry.register).toBe(register);
   state.visible[component.name]=Object.keys(registry).filter(key=>!["services","revisionIdentity","register"].includes(key));
   for(const need of component.needs)expect(registry[registryKey(need)]).toBe(modules[need]);
   if(state.fail===component.name)throw Error("wire refused: "+component.name);
  }})}]));
  for(const[step,attempt]of row.attempts.entries()){
   state.fail=attempt.fail;state.order=[];state.visible={};
   const error=refusal(()=>{const result=wireRevisionedComponents({roster,modules:modules as never,services,revisionIdentity:identity,register});expect(Object.isFrozen(result)).toBe(true);expect<unknown>(result.modules).toBe(modules);});
   const{fail:_,...expected}=attempt;
   expect({error,order:state.order,visible:error===null?state.visible:null},row.name+" #"+step).toEqual(expected);
   expect(answers[index][step],row.name+" #"+step).toEqual(expected);
  }
 }
});

test("an adopted scope answers its revision once and an invalid or repeated revision is refused",async()=>{
 const roster=readComponentRoster({version:1,components:[{name:"a",path:"./a.mjs",needs:[]}]}),loader=createRevisionLoader({roster,load:async()=>({component:"a",wire(){}}),services:{},register:()=>undefined,url:"u"});
 const scope={adopted:true};
 loader.adopt(corpus.revision,scope as never);
 expect<unknown>(await loader.load(corpus.revision)).toBe(scope);
 expect(refusal(()=>loader.adopt(corpus.revision,{} as never))).toBe("Revision graph is already loaded");
 expect(refusal(()=>loader.adopt("bad",{} as never))).toBe(BADREV);
 expect(refusal(()=>loader.load("bad"))).toBe(BADREV);
 const second=await loader.load("b".repeat(64));
 expect(Object.keys(second.modules)).toEqual(["a"]);
 expect(await loader.load("b".repeat(64))).toBe(second);
});
