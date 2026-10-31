import "./🛡️pattern-safety/🟦️.ts";
import Ajv from "ajv";
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {buildDependencyDirectionPolicy,authoredPackageExportAuthority} from "../🟦️.ts";
import {loadDependencyDirectionPolicy} from "../../🚀️bootstrap/🟦️.ts";
import {builtinModules} from "node:module";
import {pathToFileURL} from "node:url";
const corpus=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const root=resolve(import.meta.dir,"../../../../../../../../..");
const {validateDependency}=await import(new URL("../validate/index.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const {default:normalizeRules}=await import(new URL("./rule-set/normalize.mjs",pathToFileURL(Bun.resolveSync("dependency-cruiser",root))).href);
const snapshot=loadDependencyDirectionPolicy(root);
const matches=(value:string,pattern:string|readonly string[]|undefined):boolean=>pattern!==undefined&&(typeof pattern==="string"?[pattern]:pattern).some(item=>new RegExp(item,"u").test(value));
test("portable plugin direction matrix satisfies its closed schema",()=>{expect(new Ajv().compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")))(corpus)).toBe(true);});
test("every portable ordered plugin pair rejects all source contexts and dependency kinds with independent dependency-cruiser",()=>{
 const packages=corpus.plugins.map((plugin:{id:string;package:string})=>{const owner="✏️s/🔌️plugins/"+plugin.id+"/📦️packages/🟦️typescript";return {owner,name:plugin.package,...authoredPackageExportAuthority(owner,{exports:{".":"./🟦️.ts","./geometry":"./🟦️.ts"}})};});
 const policy=buildDependencyDirectionPolicy({taxonomy:snapshot.taxonomy as any,plugins:corpus.plugins.map((plugin:{id:string})=>plugin.id),workspacePackages:packages,nodeBuiltins:builtinModules});
 const rules=policy.forbidden.filter(rule=>rule.name.startsWith("no-cross-plugin-"));
 expect(rules.length).toBe(corpus.plugins.length*(corpus.plugins.length-1));
 for(const from of corpus.plugins)for(const to of corpus.plugins)for(const context of corpus.contexts)for(const dependencyType of corpus.dependencyTypes)for(const path of corpus.paths)for(const sourcePath of corpus.sourcePaths){
  const source=sourcePath==="physical"?"✏️s/🔌️plugins/"+from.id+"/"+context:sourcePath==="package"?"node_modules/"+from.package:sourcePath==="subpath"?from.package+"/"+context:"✏️s/node_modules/"+from.package+"/"+context;
  const resolved=path==="physical"?"✏️s/🔌️plugins/"+to.id+"/"+context:path==="package"?"node_modules/"+to.package:path==="subpath"?to.package+"/geometry":"✏️s/node_modules/"+to.package+"/geometry";
  const expected=from.id!==to.id?corpus.distinctForbidden:corpus.sameForbidden;
  const actual=rules.some(rule=>matches(source,rule.from.path)&&matches(resolved,rule.to.path));
  const oracle=validateDependency(normalizeRules({forbidden:rules}),{source},{resolved,dependencyTypes:[dependencyType]});
  expect(actual,source+" -> "+resolved).toBe(expected);expect(!oracle.valid).toBe(expected);
 }
 for(const plugin of corpus.plugins){const source="✏️s/🔌️plugins/"+plugin.id+"/🧪️tests/🟦️.ts";for(const resolved of ["node_modules/@semio-tech/framework/mesh/polygon",plugin.package+"-unrelated/geometry"]){expect(validateDependency(normalizeRules({forbidden:rules}),{source},{resolved,dependencyTypes:["npm"]}).valid).toBe(true);}}
});
test("actual authored plugin inventory has every distinct pair including Flow and no test or tooling exclusions",()=>{
 const rules=snapshot.policy.forbidden.filter(rule=>rule.name.startsWith("no-cross-plugin-"));
 expect(rules.length).toBe(snapshot.plugins.length*(snapshot.plugins.length-1));
 for(const from of snapshot.plugins)for(const to of snapshot.plugins)if(from!==to){const rule=rules.find(row=>row.name===`no-cross-plugin-${from}-to-${to}`)!;expect(rule).toBeDefined();expect(rule.from.pathNot).toBeUndefined();expect(rule.to.dependencyTypes).toBeUndefined();for(const context of corpus.contexts)expect(matches("✏️s/🔌️plugins/"+from+"/"+context,rule.from.path)).toBe(true);for(const pkg of snapshot.workspacePackages.filter(pkg=>pkg.owner.startsWith("✏️s/🔌️plugins/"+to+"/")))expect(matches("node_modules/"+pkg.name+"/geometry",rule.to.path)).toBe(true);}
});
