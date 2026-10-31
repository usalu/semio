import {test,expect} from "bun:test";
import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

const root=resolve(dirname(fileURLToPath(import.meta.url)),corpus.rootDepth.map(()=>"..").join("/")),pattern=new RegExp(corpus.pattern,"gu");

/** 🏷️ Lists every `with {type:"json"}` import whose specifier resolves to a file that is not JSON; specifiers that do not resolve to an existing file are skipped. */
export function jsonImportAttributeMismatchesV1(base:string,scopes:readonly string[]):{imports:number;mismatches:{file:string;specifier:string;resolved:string}[]}{
 const mismatches:{file:string;specifier:string;resolved:string}[]=[];let imports=0;
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(corpus.skipDirectories.includes(entry.name))continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path);else if(/\.(?:ts|tsx|mts|cts)$/u.test(entry.name)){const text=readFileSync(path,"utf8");if(!text.includes("json"))continue;for(const match of text.matchAll(pattern)){imports++;const specifier=(match[1]??match[2])!;let resolved:string;try{resolved=Bun.resolveSync(specifier,dirname(path));}catch{continue;}if(!resolved.endsWith(".json")&&!corpus.fixtureStrings.some(row=>row.file===path.slice(base.length+1)&&row.specifier===specifier))mismatches.push({file:path.slice(base.length+1),specifier,resolved:resolved.slice(base.length+1)});}}}};
 for(const scope of scopes)walk(join(base,scope));
 return {imports,mismatches};
}

/** 🧪️ The attribute only belongs on imports that resolve to JSON: a `.ts` wrapper behind a package export is not JSON and Node/Bun refuse it with a JSON parse error. */
test("no json import attribute sits on a specifier that resolves to a non-JSON file",()=>{
 const found=jsonImportAttributeMismatchesV1(root,corpus.scopes);expect(found.imports).toBeGreaterThan(corpus.minimumImports);expect(found.mismatches).toEqual([]);
},600000);

/** 🧫️ The scan sees a planted mismatch and a planted good import in a synthetic scope. */
test("scan control: planted wrapper is found, planted json is not",()=>{
 const {mkdtempSync,mkdirSync,rmSync}=require("node:fs"),{tmpdir}=require("node:os"),base=mkdtempSync(join(tmpdir(),"jia-"));
 try{mkdirSync(join(base,"scope"));writeFileSync(join(base,"scope","wrapper.ts"),"export default {};");writeFileSync(join(base,"scope","data.json"),"{}");
  writeFileSync(join(base,"scope","use.ts"),'import a from "./wrapper.ts" with {type:"json"};\nimport b from "./data.json" with {type:"json"};\n');
  const found=jsonImportAttributeMismatchesV1(base,["scope"]);expect(found.imports).toBe(2);expect(found.mismatches.map(row=>row.specifier)).toEqual(["./wrapper.ts"]);
 }finally{rmSync(base,{recursive:true,force:true});}
});

/** 🐍️ The Python twin runs the same scan with its own walker and Node as resolver and must find the same (empty) set. */
test("python twin agrees",()=>{
 const done=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{encoding:"utf8",maxBuffer:1<<26,timeout:1700000});expect(done.status,done.stderr).toBe(0);
},1800000);
