/** 🛡️ Nested semantic owners retain exact boundaries without unsafe optional repetition. */
import {expect,test} from "bun:test";
import {createRequire,builtinModules} from "node:module";
import {readFileSync} from "node:fs";
import {buildDependencyDirectionPolicy,authoredPackageExportAuthority} from "../../🟦️.ts";
import type {DependencyPolicyTaxonomy} from "../../../🚀️bootstrap/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const safeRegex=createRequire(import.meta.url)("safe-regex") as (expression:string)=>boolean;
const taxonomy=JSON.parse(readFileSync(new URL("../../../../../🔣️taxonomy.json",import.meta.url),"utf8")) as DependencyPolicyTaxonomy&{forbiddenPathSegments:readonly string[];bannedNameStems:readonly string[];bannedNameStemExemptions:{subsetDirectories:readonly string[]};subsetsDirName:string};
const packages=fixture.owners.map(row=>({...row,...authoredPackageExportAuthority(row.owner,{exports:{".":"./🟦️.ts"}})}));
const policy=buildDependencyDirectionPolicy({taxonomy,plugins:[],workspacePackages:packages,nodeBuiltins:builtinModules});
const matches=(patterns:string|readonly string[]|undefined,path:string):boolean=>typeof patterns==="string"?new RegExp(patterns,"u").test(path):(patterns??[]).some(pattern=>new RegExp(pattern,"u").test(path));

test("portable nested owner selectors are admitted by the independent resolver safety oracle",()=>{
  const rules=policy.forbidden.filter(rule=>rule.name.startsWith(taxonomy.dependencyDirections.publicApi.rulePrefix));
  expect(rules).toHaveLength(fixture.owners.length);
  for(const rule of rules)for(const selector of [rule.from,rule.to])for(const value of [selector.path,selector.pathNot])for(const pattern of typeof value==="string"?[value]:value??[])expect(safeRegex(pattern),rule.name+": "+pattern).toBe(true);
});

test("portable nested owner selectors preserve private ownership and exact public targets",()=>{
  const rule=policy.forbidden.find(rule=>rule.name===taxonomy.dependencyDirections.publicApi.rulePrefix+fixture.owners[0]!.name)!;
  for(const row of fixture.cases)expect(matches(rule.from.path,row.from)&&!matches(rule.from.pathNot,row.from)&&matches(rule.to.path,row.to)&&!matches(rule.to.pathNot,row.to),row.id).toBe(row.forbidden);
});
