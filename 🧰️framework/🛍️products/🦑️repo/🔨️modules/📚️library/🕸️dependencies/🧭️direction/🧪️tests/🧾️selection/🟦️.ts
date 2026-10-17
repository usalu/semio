import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createRequire} from "node:module";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {dependencyDirectionLayerRules,dependencyDirectionStrictRules} from "../../🟦️.ts";
import type {DependencyPolicySnapshot} from "../../🚀️bootstrap/🟦️.ts";
import fixture from "../🧫️fixtures/🧾️selection/🔣️.json";
import schema from "../../🧬️schema/🧾️selection/🔣️.json";

const taxonomy=JSON.parse(readFileSync(resolve(import.meta.dir,"../../../../🔣️taxonomy.json"),"utf8"));
const construct=createRequire(import.meta.url)("../../🏗️construction/🟨️.cjs").buildDependencyDirectionPolicy as (inputs:unknown)=>DependencyPolicySnapshot["policy"];
const snapshot=(plugins:readonly string[]):Pick<DependencyPolicySnapshot,"taxonomy"|"plugins"|"policy">=>({taxonomy,plugins,policy:construct({taxonomy,plugins,workspacePackages:[],nodeBuiltins:["fs","path"]})});

test("strict production selection enforces every present plugin pair and preserves owner deletion",async()=>{
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const oracle=await import("dependency-cruiser");
  const required=["framework-no-implementation","repo-no-implementation","s-modules-no-plugins","plugins-framework-sdk-only",...Object.keys(taxonomy.dependencyDirections.rules)];
  const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifacts)throw Error("Ticket-owned verification output required");
  mkdirSync(artifacts,{recursive:true});
  for(const row of fixture.cases){
    const plugins=fixture.plugins.filter(plugin=>!row.absent.includes(plugin)),source=snapshot(plugins),rules=dependencyDirectionStrictRules(source),pairs=rules.filter(rule=>rule.name.startsWith("no-cross-plugin-"));
    expect(pairs.length,row.id).toBe(row.expectedPairs);
    expect(dependencyDirectionLayerRules(source).filter(rule=>rule.name.startsWith("no-cross-plugin-")),row.id+" shared native ownership authority").toEqual(pairs);
    expect(required.every(name=>rules.some(rule=>rule.name===name)),row.id).toBe(true);
    const root=mkdtempSync(join(artifacts,"selection-")),entries=plugins.map(plugin=>`✏️s/🔌️plugins/${plugin}/📜️script.ts`);
    const expected:string[]=[];
    try{
    for(const plugin of plugins){
      const entry=`✏️s/🔌️plugins/${plugin}/📜️script.ts`,directory=join(root,dirname(entry));
      mkdirSync(directory,{recursive:true});
      writeFileSync(join(directory,"🟦️.ts"),"export const value=1;");
      writeFileSync(join(root,entry),plugins.filter(to=>to!==plugin).map(to=>`export const value${plugins.indexOf(to)}=import(${JSON.stringify(relative(dirname(entry),`✏️s/🔌️plugins/${to}/🟦️.ts`).replaceAll("\\","/"))});`).concat("export {};").join("\n"));
    }
    for(const from of plugins)for(const to of plugins){
      if(from===to)continue;
      const entry=`✏️s/🔌️plugins/${from}/📜️script.ts`,target=`✏️s/🔌️plugins/${to}/🟦️.ts`;
      const rule=pairs.find(rule=>rule.name===`no-cross-plugin-${from}-to-${to}`)!;
      expect(rule).toBeDefined();
      expect(rule.from.path.some(pattern=>new RegExp(pattern,"u").test(entry))).toBe(true);
      expect(rule.to.path.some(pattern=>new RegExp(pattern,"u").test(target))).toBe(true);
      expected.push(JSON.stringify([rule.name,entry,target]));
    }
    if(entries.length){
      const result=await oracle.cruise(entries,{baseDir:root,validate:true,ruleSet:{forbidden:rules.map(rule=>({...rule,severity:"error" as const,from:{path:[...rule.from.path],...(rule.from.pathNot?{pathNot:[...rule.from.pathNot]}:{})},to:{path:[...rule.to.path],...(rule.to.pathNot?{pathNot:[...rule.to.pathNot]}:{})}}))},outputType:"json",tsPreCompilationDeps:true},{bustTheCache:true});
      expect(result.exitCode,row.id).toBe(0);
      const graph=typeof result.output==="string"?JSON.parse(result.output):result.output;
      expect(graph.summary.violations.map((edge:{rule:{name:string};from:string;to:string})=>JSON.stringify([edge.rule.name,edge.from,edge.to])).sort(),row.id).toEqual(expected.sort());
    }
    }finally{rmSync(root,{recursive:true,force:true});}
  }
});

test("strict production selection refuses incomplete or weakened rule authority",()=>{
  for(const refusal of fixture.refusals){
    const source=snapshot(fixture.plugins),rules=structuredClone([...source.policy.forbidden]) as unknown as Record<string,any>[];
    const index=rules.findIndex(rule=>String(rule.name).startsWith("no-cross-plugin-"));
    if(refusal==="missing-pair")rules.splice(index,1);
    if(refusal==="duplicate-pair")rules.push(structuredClone(rules[index]!));
    if(refusal==="weakened-pair")rules[index]!.severity="warn";
    if(refusal==="extra-selector")rules[index]!.to.dependencyTypes=["local"];
    if(refusal==="missing-framework-rule")rules.splice(rules.findIndex(rule=>rule.name==="framework-no-implementation"),1);
    if(refusal==="empty-selector")rules[index]!.to.path=[];
    expect(()=>dependencyDirectionStrictRules({...source,policy:{...source.policy,forbidden:rules as unknown as DependencyPolicySnapshot["policy"]["forbidden"]}}),refusal).toThrow();
  }
});
