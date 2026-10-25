import{withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {join} from "node:path";
import {mkdirSync,mkdtempSync,rmSync,symlinkSync,unlinkSync,writeFileSync} from "node:fs";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as leases from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {readJavascriptDependencySourceOwnerExecutionV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";

/** 🔭️ Walks a complete synthetic Store once to count checkpoints, then replays it with one physical change injected at a chosen checkpoint, so memoized observations must still be refused by the final replays. */
test("declared provider walk refuses every change after the first observation of a path",async()=>{
 const validate=new Ajv({strict:true}).compile(corpus.schema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original provider walk artifact root required");
 await withBuildTestInvocationV1(async original=>{
  const control=issued.createIssuedResourceControlV1(original,"dependencies:provider-walk",0),run=async(mutation:string,position:number,total:number)=>{
   const root=mkdtempSync(join(artifacts,"pw-")),workspace=join(root,"s"),cache=join(root,"n"),a="a".repeat(64),b="b".repeat(64),ga=join(cache,"j",a.slice(0,20)),gb=join(cache,"j",b.slice(0,20)),pa=join(ga,"w/0"),pb=join(gb,"w/0"),provider=join(ga,"node_modules/.bun/dep"),copy=join(gb,"node_modules/.bun/dep");
   try{
    for(const path of[join(workspace,"p"),join(workspace,"b"),join(pa,"node_modules"),join(pb,"node_modules"),provider,copy])mkdirSync(path,{recursive:true});
    const document={name:"shared-p",version:"1.0.0",dependencies:{dep:"^1.0.0"}};writeFileSync(join(workspace,"p","package.json"),JSON.stringify(document));
    for(const path of[workspace,join(workspace,"b")])writeFileSync(join(path,"package.json"),JSON.stringify({name:"scope",version:"1.0.0"}));
    for(const path of[pa,pb])writeFileSync(join(path,"package.json"),JSON.stringify(document));
    for(const path of[provider,copy])writeFileSync(join(path,"package.json"),JSON.stringify({name:"dep",version:"1.2.3"}));
    symlinkSync(provider,join(pa,"node_modules/dep"),"dir");symlinkSync(provider,join(pb,"node_modules/dep"),"dir");
    for(const[folder,target]of[[join(workspace,"p"),join(pa,"node_modules")],[workspace,join(ga,"node_modules")],[join(workspace,"b"),join(gb,"node_modules")]]as const)symlinkSync(target,join(folder,"node_modules"),"dir");
    let checks=0,fired=false;
    const check=()=>{leases.checkResourceLeaseControl(control);if(!fired&&mutation!=="none"&&checks++>=Math.floor(position*total)){fired=true;switch(mutation){
      case"manifest-rewritten":writeFileSync(join(provider,"package.json"),JSON.stringify({name:"dep",version:"1.2.4"}));break;
      case"provider-removed":rmSync(provider,{recursive:true,force:true});break;
      case"provider-relinked":unlinkSync(join(pb,"node_modules/dep"));symlinkSync(copy,join(pb,"node_modules/dep"),"dir");break;
      case"sibling-added":writeFileSync(join(root,"sibling.txt"),"unrelated");break;
    }}else if(mutation==="none")checks++;};
    const context=Object.freeze({root:cache,invocation:original,control,check:()=>check(),recheck:async()=>{check();}});
    const input={version:1,requested:[".","b"],owners:{version:1,scopes:[{manifest:"package.json",packages:["p"]},{manifest:"b/package.json",packages:["p"]}]},generations:[{scope:".",generation:a},{scope:"b",generation:b}]};
    let result:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerExecutionV1>>|undefined,failure:unknown;
    try{result=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context as never,control,input);}catch(error){failure=error;}
    return{result,failure,checks,fired};
   }finally{rmSync(root,{recursive:true,force:true});}
  };
  const baseline=await run("none",0,0);expect(baseline.failure).toBeUndefined();const total=baseline.checks;expect(total).toBeGreaterThan(40);
  const summary=(value:NonNullable<typeof baseline.result>)=>value.relations.flatMap(relation=>relation.edges.map(edge=>({name:edge.name,kind:edge.kind,range:edge.range,version:String(edge.provider?.version)}))),edges=summary(baseline.result!);
  expect(edges.length).toBe(2);for(const edge of edges)expect(corpus.expectedEdges).toContainEqual(edge);
  const failures:string[]=[];
  for(const row of corpus.cases){
   expect(validate(row),row.id).toBe(true);const outcome=await run(row.mutation,row.at,total);
   if(row.mutation!=="none"&&!outcome.fired)failures.push(row.id+": mutation never fired");
   if((outcome.failure===undefined)!==row.accepted)failures.push(row.id+": expected "+(row.accepted?"accepted":"refused")+(outcome.failure instanceof Error?" ("+outcome.failure.message+")":""));
   if(row.accepted&&outcome.result&&JSON.stringify(summary(outcome.result))!==JSON.stringify(edges))failures.push(row.id+": result differs from the unmutated walk");
  }
  expect(failures).toEqual([]);
 });
},120000);
