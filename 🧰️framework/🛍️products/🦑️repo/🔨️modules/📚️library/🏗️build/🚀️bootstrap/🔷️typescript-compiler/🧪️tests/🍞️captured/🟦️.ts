/** 🍞️ Contract: the tooling compiler interface loads in every process shape the tooling uses, among them the captured implementation loader of the farm children and the slice workers, which admits only the module requests the sources declare (an absolute path into the install is refused): a process that cannot load the compiler names the cause instead of a missing API, and the request beside the version pin is the declared literal site that loads the same 5.9.x compiler as an independent import.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {buildSourceClosureLoaderV1} from "../../../../📥️source/🔏️closure/🟦️.ts";
import {closureLiteralSitesV1} from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import {inspectTypescriptCompilerV1,typescriptCompilerV1} from "../../🟨️.mjs";

const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),here=(path:string)=>fileURLToPath(new URL(path,import.meta.url));
const sources={compiler:here("../../🟨️.mjs"),schema:here("../../🧬️schema/🔣️.json"),parsers:here("../../../📦️parsers/🟨️.mjs")},inspected=inspectTypescriptCompilerV1();
const independent=createRequire(here("../../🟨️.mjs"))(inspected.ok?join(inspected.path,"lib","typescript.js"):"typescript");

function run(){
 const base=mkdtempSync(join(realpathSync(tmpdir()),"captured-ts-")),root=join(base,"root"),dir=join(root,"b");
 for(const name of ["🔷️typescript-compiler/🧬️schema","📦️parsers/node_modules"])mkdirSync(join(dir,name),{recursive:true});
 const version=inspected.ok?inspected.version:"0.0.0",files:Record<string,string>={
  "b/🔷️typescript-compiler/🟨️.mjs":readFileSync(sources.compiler,"utf8"),
  "b/🔷️typescript-compiler/🧬️schema/🔣️.json":readFileSync(sources.schema,"utf8"),
  "b/📦️parsers/🟨️.mjs":readFileSync(sources.parsers,"utf8"),
  "entry.mjs":`import{parserTypescriptV1}from"./b/📦️parsers/🟨️.mjs";export async function runCapturedBuildSourceV1(){const ts=parserTypescriptV1();process.stdout.write("loaded "+ts.version+" "+typeof ts.preProcessFile+"\\n");}`
 },live={"bun.lock":"","b/📦️parsers/package.json":JSON.stringify({devDependencies:{typescript:version}})},imports:Record<string,Record<string,string>>={
  "entry.mjs":{"./b/📦️parsers/🟨️.mjs":"b/📦️parsers/🟨️.mjs"},
  "b/📦️parsers/🟨️.mjs":{"../🔷️typescript-compiler/🟨️.mjs":"b/🔷️typescript-compiler/🟨️.mjs",typescript:"installed:typescript@"+version},
  "b/🔷️typescript-compiler/🟨️.mjs":{"./🧬️schema/🔣️.json":"b/🔷️typescript-compiler/🧬️schema/🔣️.json"}
 };
 for(const[path,text]of Object.entries({...files,...live}))writeFileSync(join(root,path),text);
 if(inspected.ok)symlinkSync(inspected.path,join(dir,"📦️parsers","node_modules","typescript"));
 const rows=Object.entries(files).sort(([a],[b])=>a<b?-1:1).map(([path,text])=>({path,sha256:hash(text),base64:Buffer.from(text).toString("base64"),imports:imports[path]??{}})),snapshot=JSON.stringify({version:1,root,entry:"entry.mjs",files:rows}),snapshotPath=join(base,"snapshot.json"),runner=join(base,"📜️script.ts");
 writeFileSync(snapshotPath,snapshot);writeFileSync(runner,buildSourceClosureLoaderV1);
 return {base,result:spawnSync(process.execPath,[runner,snapshotPath,hash(snapshot)],{encoding:"utf8",env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}})};
}

test("the checkout holds the declared compiler this law measures against",()=>{expect(inspected.ok).toBe(true);expect(independent.version).toBe(inspected.ok?inspected.version:"");});

test("the captured implementation loader loads the declared compiler with its API and agrees with an independent import",()=>{
 const row=run();
 try{expect(row.result.status,row.result.stderr).toBe(0);expect(row.result.stdout).toBe("loaded "+independent.version+" function\n");}finally{rmSync(row.base,{recursive:true,force:true});}
},120000);

test("the request beside the version pin is a literal site the closure admits, and the interface file names no path request",()=>{
 const sites=closureLiteralSitesV1("b/parsers.mjs",readFileSync(sources.parsers,"utf8")).map(site=>site.specifier);
 expect(sites).toContain("typescript");
 expect(closureLiteralSitesV1("b/compiler.mjs",readFileSync(sources.compiler,"utf8")).map(site=>site.specifier).filter(specifier=>specifier.startsWith("/"))).toEqual([]);
});

test("a loader that throws is the typed load refusal naming the cause in English and German, never a missing API",()=>{
 let refusal:any;
 try{typescriptCompilerV1({load:()=>{throw Error("simulated loader refusal");}});}catch(error:any){refusal=error.refusal;}
 expect(refusal.code).toBe("typescript-compiler-load");expect(refusal.retry).toBe("never");expect(refusal.cause).toBe("simulated loader refusal");expect(refusal.missing).toEqual([]);
 expect(refusal.en).toContain("simulated loader refusal");expect(refusal.de).toContain("simulated loader refusal");expect(refusal.en).not.toContain("lacks the API");
});

test("a loader that answers a compiler without the API is the api refusal naming the members",()=>{
 let refusal:any;
 try{typescriptCompilerV1({load:()=>({version:inspected.ok?inspected.version:""})});}catch(error:any){refusal=error.refusal;}
 expect(refusal.code).toBe("typescript-compiler-api");expect(refusal.missing).toContain("createProgram");
});
