/** 🚫️ Resolved-path boundary classification: builtins, bun-store packages and subset vocabulary are classified like the filesystem law and an independent dependency-cruiser oracle. */
import {expect,test} from "bun:test";
import {builtinModules} from "node:module";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {buildDependencyDirectionPolicy} from "../../🟦️.ts";
import {loadDependencyDirectionPolicy} from "../../../🚀️bootstrap/🟦️.ts";
import type {DependencyPolicyRule} from "../../../🚀️bootstrap/🟦️.ts";
import {policyBannedNameStemBreaches} from "../../../../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;rule:string;from:string;to:string;forbidden:boolean;dependencyTypes?:string[];circular?:boolean};
const root=resolve(import.meta.dir,"../../../../../../../../../..");
const {validateDependency}=await import(new URL("../validate/index.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const {default:assertRuleSetValid}=await import(new URL("./rule-set/assert-validity.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const {default:normalizeRules}=await import(new URL("./rule-set/normalize.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const taxonomy=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"));
const policy=buildDependencyDirectionPolicy({taxonomy,plugins:[],workspacePackages:[],nodeBuiltins:builtinModules});
const patterns=(value:string|readonly string[]|undefined):readonly string[]=>value===undefined?[]:typeof value==="string"?[value]:value;
const matches=(value:string,selector:string|readonly string[]|undefined):boolean=>patterns(selector).some(pattern=>new RegExp(pattern,"u").test(value));
const selects=(value:string,selector:{path?:string|readonly string[];pathNot?:string|readonly string[];dependencyTypesNot?:readonly string[];circular?:boolean},types:readonly string[]=[],circular=false):boolean=>(selector.path===undefined||matches(value,selector.path))&&!matches(value,selector.pathNot)&&!types.some(type=>selector.dependencyTypesNot?.includes(type))&&(selector.circular===undefined||selector.circular===circular);
const ruleNamed=(name:string):DependencyPolicyRule=>policy.forbidden.find(rule=>rule.name===name)!;

test("portable boundary corpus names every case once and only known rules",()=>{
  const cases=corpus.cases as Case[];
  expect(new Set(cases.map(row=>row.id)).size).toBe(cases.length);
  for(const row of cases)expect(ruleNamed(row.rule),row.id).toBeDefined();
  expect(cases.some(row=>row.forbidden)&&cases.some(row=>!row.forbidden)).toBe(true);
});

test("actual rules agree with the independent dependency-cruiser oracle on every portable case",()=>{
  for(const row of corpus.cases as Case[]){
    const rule=ruleNamed(row.rule);
    const direct=selects(row.from,rule.from)&&selects(row.to,rule.to,row.dependencyTypes,row.circular);
    const oracle=validateDependency(normalizeRules({forbidden:[rule]}),{source:row.from},{resolved:row.to,dependencyTypes:row.dependencyTypes??[],circular:row.circular??false});
    expect(direct,row.id+" direct").toBe(row.forbidden);
    expect(!oracle.valid,row.id+" oracle").toBe(row.forbidden);
  }
});

test("the ui boundary declaration is required, closed over declared roles and names only existing, disjoint owners",()=>{
  const policyOf=(dependencyDirections:object)=>()=>buildDependencyDirectionPolicy({taxonomy:{...taxonomy,dependencyDirections:{...taxonomy.dependencyDirections,...dependencyDirections}},plugins:[],workspacePackages:[],nodeBuiltins:builtinModules});
  const {uiBoundary,roles}=taxonomy.dependencyDirections;
  expect(policyOf({uiBoundary:undefined})).toThrow(/canonical ui boundary declaration/u);
  expect(policyOf({uiBoundary:{...uiBoundary,foundationRoles:["no-such-role"]}})).toThrow(/Unknown dependency direction role/u);
  expect(policyOf({uiBoundary:{...uiBoundary,subjectRole:"no-such-role"}})).toThrow(/canonical ui boundary declaration/u);
  const owners=(ids:string[])=>ids.flatMap(id=>roles[id].ownerPaths as string[]);
  const foundation=owners(uiBoundary.foundationRoles),support=owners(uiBoundary.testSupportRoles);
  for(const owner of [...foundation,...support])expect(existsSync(join(root,owner)),owner).toBe(true);
  expect(foundation.filter(owner=>support.includes(owner))).toEqual([]);
  expect([...foundation,...support].filter(owner=>owner===roles[uiBoundary.subjectRole].ownerPaths[0]||owner.startsWith(roles[uiBoundary.subjectRole].ownerPaths[0]+"/"))).toEqual([]);
});

test("banned-name-stem filesystem law applies the same subset exemption as the dependency policy",()=>{
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!output)throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output,{recursive:true});
  const tree=mkdtempSync(join(output,"banned-stem-"));
  for(const path of corpus.filesystemLaw.directories)mkdirSync(join(tree,path),{recursive:true});
  for(const path of corpus.filesystemLaw.files)writeFileSync(join(tree,path),"");
  const scopes=policyBannedNameStemBreaches(tree).map(row=>row.scope).sort();
  expect(scopes).toEqual([...corpus.filesystemLaw.breaches].sort());
});

test("the complete actual policy is admitted by dependency-cruiser's own rule-set validation including its regular expression safety check",()=>{
  const actual=loadDependencyDirectionPolicy(root).policy;
  expect(()=>assertRuleSetValid({forbidden:actual.forbidden,options:actual.options})).not.toThrow();
},600000);
