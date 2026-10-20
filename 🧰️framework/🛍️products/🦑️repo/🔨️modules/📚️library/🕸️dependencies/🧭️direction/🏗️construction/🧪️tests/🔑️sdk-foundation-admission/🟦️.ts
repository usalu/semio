/** 🔑️ SDK foundation admission: plugins and renderer hosts reach exactly the authored public targets of the declared foundation packages, checked directly and against an independent dependency-cruiser oracle on a portable corpus; the real taxonomy names only existing workspace packages and validates against its schema. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {builtinModules} from "node:module";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {buildDependencyDirectionPolicy} from "../../🟦️.ts";
import {loadDependencyDirectionPolicy} from "../../../🚀️bootstrap/🟦️.ts";
import type {DependencyPolicyPackage,DependencyPolicyRule} from "../../../🚀️bootstrap/🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;rule:string;from:string;to:string;forbidden:boolean};
const root=resolve(import.meta.dir,"../../../../../../../../../..");
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {validateDependency}=await import(new URL("../validate/index.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const {default:normalizeRules}=await import(new URL("./rule-set/normalize.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const taxonomy=JSON.parse(readFileSync(join(library,"🔣️taxonomy.json"),"utf8"));
const schema=JSON.parse(readFileSync(join(library,"🧬️schema/🧱️dependency-direction/🔣️.json"),"utf8"));
const packagesOf=(rows:typeof corpus.packages):DependencyPolicyPackage[]=>rows.map(row=>({...row,exports:[...new Set(row.exportTargets.map(target=>target.subpath))]}));
const policyOf=(foundation:readonly string[],rows:typeof corpus.packages)=>buildDependencyDirectionPolicy({taxonomy:{...taxonomy,dependencyDirections:{...taxonomy.dependencyDirections,publicApi:{...taxonomy.dependencyDirections.publicApi,sdkFoundationPackages:foundation}}},plugins:[],workspacePackages:packagesOf(rows),nodeBuiltins:builtinModules});
const patterns=(value:string|readonly string[]|undefined):readonly string[]=>value===undefined?[]:typeof value==="string"?[value]:value;
const matches=(value:string,selector:string|readonly string[]|undefined):boolean=>patterns(selector).some(pattern=>new RegExp(pattern,"u").test(value));
const selects=(value:string,selector:{path?:string|readonly string[];pathNot?:string|readonly string[]}):boolean=>(selector.path===undefined||matches(value,selector.path))&&!matches(value,selector.pathNot);
const ruleNamed=(policy:ReturnType<typeof policyOf>,name:string):DependencyPolicyRule=>policy.forbidden.find(rule=>rule.name===name)!;

test("portable corpus: foundation targets are admitted for plugins and hosts, everything else stays forbidden, directly and by the dependency-cruiser oracle",()=>{
  const cases=corpus.cases as Case[],policy=policyOf(corpus.foundationPackages,corpus.packages);
  expect(new Set(cases.map(row=>row.id)).size).toBe(cases.length);
  for(const rule of ["plugins-framework-sdk-only","renderer-hosts-only-ui"])expect(cases.some(row=>row.rule===rule&&row.forbidden)&&cases.some(row=>row.rule===rule&&!row.forbidden),rule).toBe(true);
  for(const row of cases){
    const rule=ruleNamed(policy,row.rule);
    const direct=selects(row.from,rule.from)&&selects(row.to,rule.to);
    const oracle=validateDependency(normalizeRules({forbidden:[rule]}),{source:row.from},{resolved:row.to,dependencyTypes:[],circular:false});
    expect(direct,row.id+" direct").toBe(row.forbidden);
    expect(!oracle.valid,row.id+" oracle").toBe(row.forbidden);
  }
});

test("a declaration naming no corpus package forbids every foundation target, so the admission derives from data alone",()=>{
  const bare=policyOf(["@semio-tech/framework-undeclared"],corpus.packages);
  const foundation=(corpus.cases as Case[]).filter(row=>row.id.includes("foundation")&&!row.forbidden);
  expect(foundation.length).toBeGreaterThan(0);
  for(const row of foundation){
    const rule=ruleNamed(bare,row.rule);
    expect(selects(row.from,rule.from)&&selects(row.to,rule.to),row.id).toBe(true);
  }
});

test("the declaration is closed: unique scoped package names distinct from the SDK package",()=>{
  const {publicApi}=taxonomy.dependencyDirections;
  const policyWith=(sdkFoundationPackages:unknown)=>()=>buildDependencyDirectionPolicy({taxonomy:{...taxonomy,dependencyDirections:{...taxonomy.dependencyDirections,publicApi:{...publicApi,sdkFoundationPackages}}},plugins:[],workspacePackages:[],nodeBuiltins:builtinModules});
  for(const bad of [undefined,[],"@semio-tech/ui",["@semio-tech/ui","@semio-tech/ui"],["ui"],[publicApi.sdkPackage],[1]])expect(policyWith(bad),JSON.stringify(bad)).toThrow(/SDK foundation package|public API authority/u);
  expect(policyWith(publicApi.sdkFoundationPackages)).not.toThrow();
});

test("the real taxonomy declaration validates against its schema and names only existing workspace packages",()=>{
  const validate=new Ajv({strict:true}).compile(schema);
  expect(validate(taxonomy.dependencyDirections),JSON.stringify(validate.errors)).toBe(true);
  const names=new Set(loadDependencyDirectionPolicy(root).workspacePackages.map(pkg=>pkg.name));
  for(const name of taxonomy.dependencyDirections.publicApi.sdkFoundationPackages)expect(names.has(name),name).toBe(true);
},600000);
