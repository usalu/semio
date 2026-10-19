/** 🏺️ Vendored third-party trees declared in `dependencyDirections.vendoredPaths` are neither linted nor followed, while authored code beside them still is; checked against dependency-cruiser on a portable corpus and against the exclusion patterns evaluated directly. */
import {expect,test} from "bun:test";
import {builtinModules} from "node:module";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {cruise,type IForbiddenRuleType} from "dependency-cruiser";
import {buildDependencyDirectionPolicy} from "../../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

type Case={id:string;vendoredPaths:string[];excluded:string[];violations:string[]};
const root=resolve(import.meta.dir,"../../../../../../../../../..");
const taxonomy=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8"));
const files:Record<string,string>={...corpus.shim,...corpus.authored};
const policyFor=(vendoredPaths:string[])=>buildDependencyDirectionPolicy({taxonomy:{...taxonomy,dependencyDirections:{...taxonomy.dependencyDirections,vendoredPaths}},plugins:[],workspacePackages:[],nodeBuiltins:builtinModules});
const subject=(violation:{rule:{name:string};from:string;cycle?:{name:string}[]}):string=>violation.rule.name+" "+(violation.cycle?[...new Set([violation.from,...violation.cycle.map(step=>step.name)])].sort().join(" > "):violation.from);

test("portable corpus: vendored declarations remove exactly the declared tree from the lint graph",async()=>{
  for(const row of corpus.cases as Case[]){
    const dir=realpathSync(mkdtempSync(join(tmpdir(),"semio-vendored-")));
    try{
      for(const [name,text] of Object.entries(files)){mkdirSync(dirname(join(dir,name)),{recursive:true});writeFileSync(join(dir,name),text);}
      const policy=policyFor(row.vendoredPaths);
      const excludePatterns=policy.options.exclude.path.map(pattern=>new RegExp(pattern,"u"));
      expect(Object.keys(files).filter(name=>excludePatterns.some(pattern=>pattern.test(name))).sort(),row.id+" direct exclusion").toEqual([...row.excluded].sort());
      const rules=policy.forbidden.filter(rule=>rule.name==="no-circular"||rule.name==="no-core-path").map(rule=>({...rule,severity:"error" as const})) as unknown as IForbiddenRuleType[];
      expect(rules.map(rule=>rule.name).sort(),row.id+" rules present").toEqual(["no-circular","no-core-path"]);
      const result=await cruise([join("🧰️framework","app"),join("🧰️framework","vendor")],{baseDir:dir,validate:true,ruleSet:{forbidden:rules},tsPreCompilationDeps:true,combinedDependencies:true,exclude:{path:[...policy.options.exclude.path]},doNotFollow:{path:policy.options.doNotFollow.path}},{bustTheCache:true});
      const output=result.output as {summary:{violations:{rule:{name:string};from:string;cycle?:{name:string}[]}[]}};
      const actual=[...new Set(output.summary.violations.map(subject))].sort();
      expect(actual,row.id+" oracle").toEqual([...row.violations].sort());
    }finally{rmSync(dir,{recursive:true,force:true});}
  }
},120000);

test("the real taxonomy declares the vendored jco preview2 shim and the actual policy excludes it",()=>{
  const policy=buildDependencyDirectionPolicy({taxonomy,plugins:[],workspacePackages:[],nodeBuiltins:builtinModules});
  const shim="🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim";
  expect(taxonomy.dependencyDirections.vendoredPaths).toContain(shim);
  const patterns=policy.options.exclude.path.map((pattern:string)=>new RegExp(pattern,"u"));
  for(const name of ["common.js","in-memory-http.js","http.js"])expect(patterns.some((pattern:RegExp)=>pattern.test(`${shim}/${name}`)),name).toBe(true);
  expect(patterns.some((pattern:RegExp)=>pattern.test(`${shim}-owner/index.js`))).toBe(false);
});
