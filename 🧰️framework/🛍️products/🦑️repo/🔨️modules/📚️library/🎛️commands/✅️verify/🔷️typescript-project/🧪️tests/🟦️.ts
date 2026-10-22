/** 🔷️ Contract: `verify typescript` parses its flags strictly, selects the workspace roots (excludes, shadowed declarations, scopes, changed and listed sets), generates the relative project document, finds excluded applications, counts diagnostics by column-zero lines and type-checks a real project in a Node child; the corpus is language-agnostic, minimatch and the TypeScript compiler API are the third-party oracles and `🐍️oracle.py` re-implements the selection and the log counting. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import ts from "typescript";
import {minimatch} from "minimatch";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {relative} from "node:path";
import {listTypescriptAppProjects,listTypescriptChangedSources,listTypescriptWorkspaceSources,parseTypescriptProjectArguments,readTypescriptRootList,readTypescriptWorkspaceExcludes,runTypescriptCompiler,selectTypescriptRoots,selectsTypescriptAppProject,summarizeTypescriptDiagnostics,summarizeTypescriptSyntaxFiles,typescriptProjectDocument,writeTypescriptProject,type TypescriptProjectSelection} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"tsproject-")));roots.push(root);return root;};
const place=(root:string,files:Record<string,string>):void=>{for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const nullable=<T,>(value:T|undefined):T|null=>value===undefined?null:value;
const selectionOf=(row:{scopes:string[];changedOnly:boolean;listed?:string[]}):TypescriptProjectSelection=>row.listed===undefined?{scopes:row.scopes,changedOnly:row.changedOnly}:{scopes:row.scopes,changedOnly:row.changedOnly,listed:row.listed};
for(const row of corpus.argsOk)test("parses "+JSON.stringify(row.args),()=>{const parsed=parseTypescriptProjectArguments(row.args);expect({selection:parsed.selection,out:nullable(parsed.out),filesFrom:nullable(parsed.filesFrom),heapMegabytes:nullable(parsed.heapMegabytes),timeoutMinutes:parsed.timeoutMinutes,maximumDiagnostics:parsed.maximumDiagnostics,list:parsed.list}).toEqual(row.expected);});
for(const row of corpus.argsErr)test("refuses "+JSON.stringify(row.args),()=>{expect(()=>parseTypescriptProjectArguments(row.args)).toThrow(row.message);});
for(const row of corpus.roots)test("roots: "+row.name,()=>{expect(selectTypescriptRoots(row.sources,row.excludes,selectionOf(row.selection),row.changed)).toEqual(row.expected);});
test("the root selection agrees with the minimatch model",()=>{
 const excluded=(path:string,patterns:string[]):boolean=>patterns.some(pattern=>{const clean=pattern.replace(/^\.\//u,"");const segments=path.split("/");for(let end=1;end<=segments.length;end++)if(minimatch(segments.slice(0,end).join("/"),clean))return true;return false;});
 for(const row of corpus.roots){
  const live=new Set(row.sources.filter(path=>!excluded(path,row.excludes))),kept=[...live].filter(path=>!(path.endsWith(".d.ts")&&(live.has(path.slice(0,-5)+".ts")||live.has(path.slice(0,-5)+".tsx")))&&(!row.selection.changedOnly||[...row.changed].includes(path))&&(row.selection.listed===undefined||row.selection.listed.includes(path))&&(row.selection.scopes.length===0||row.selection.scopes.some(scope=>path===scope||path.startsWith(scope+"/")))).sort();
  expect(kept,row.name).toEqual(row.expected);
 }
});
for(const row of corpus.apps.cases)test("applications: "+row.name,()=>{
 const root=scratch();place(root,corpus.apps.files);
 if(row.message!==null)expect(()=>listTypescriptAppProjects(root,row.sources,row.excludes)).toThrow(row.message);else expect(listTypescriptAppProjects(root,row.sources,row.excludes)).toEqual(row.expected);
});
for(const[index,row]of corpus.appSelect.entries())test("application selection "+index+": "+row.app.folder+" "+JSON.stringify(row.selection),()=>{expect(selectsTypescriptAppProject(row.app,selectionOf(row.selection),row.changed)).toBe(row.expected);});
for(const[index,row]of corpus.document.entries())test("project document "+index,()=>{expect(typescriptProjectDocument(row.roots,row.toRoot,Object.fromEntries(Object.entries(row.existing).filter((entry):entry is[string,string]=>typeof entry[1]==="string")))).toEqual(row.expected);});
test("a diagnostic log is counted by column-zero lines only",()=>{expect(summarizeTypescriptDiagnostics(corpus.log.text)).toEqual(corpus.log.summary);expect(summarizeTypescriptSyntaxFiles(corpus.log.text)).toEqual(corpus.log.syntax);expect(summarizeTypescriptDiagnostics("")).toEqual({total:0,files:0,byCode:{}});});
test("the summary agrees with the TypeScript compiler's own diagnostics for a program",()=>{
 const root=scratch();place(root,corpus.e2e.files);
 const options:ts.CompilerOptions={strict:true,noEmit:true,module:ts.ModuleKind.ESNext,target:ts.ScriptTarget.ES2022,types:[],allowImportingTsExtensions:true};
 const names=Object.keys(corpus.e2e.files).filter(name=>name.endsWith(".ts")).map(name=>join(root,...name.split("/")));
 const program=ts.createProgram(names,options),syntactic=names.flatMap(name=>program.getSyntacticDiagnostics(program.getSourceFile(name)!)),semantic=names.flatMap(name=>program.getSemanticDiagnostics(program.getSourceFile(name)!));
 const all=[...syntactic,...semantic],host:ts.FormatDiagnosticsHost={getCurrentDirectory:()=>root,getCanonicalFileName:name=>name,getNewLine:()=>"\n"},log=ts.sortAndDeduplicateDiagnostics(all).map(item=>ts.formatDiagnostic(item,host)).join("");
 const summary=summarizeTypescriptDiagnostics(log),histogram:Record<string,number>={};for(const item of ts.sortAndDeduplicateDiagnostics(all))histogram["TS"+item.code]=(histogram["TS"+item.code]??0)+1;
 expect(summary.total).toBe(ts.sortAndDeduplicateDiagnostics(all).length);expect(summary.byCode).toEqual(histogram);expect(summary.files).toBe(new Set(all.map(item=>item.file?.fileName)).size);
 expect(summarizeTypescriptSyntaxFiles(log).map(item=>item.file).sort()).toEqual([...new Set(all.map(item=>relative(root,item.file!.fileName).split("\\").join("/")))].sort());
});
test("a root list skips comments and blanks, relativises absolute paths and deduplicates",()=>{
 const root=scratch();writeFileSync(join(root,"list.txt"),corpus.rootList.text.replace("/ROOT/",root.split("\\").join("/")+"/"));
 expect(readTypescriptRootList(root,"list.txt")).toEqual(corpus.rootList.expected);
});
test("tsconfig excludes are read as JSONC and must be strings",()=>{
 const root=scratch();place(root,{"tsconfig.json":corpus.git.files["tsconfig.json"]!});expect(readTypescriptWorkspaceExcludes(root)).toEqual(corpus.git.excludes);
 writeFileSync(join(root,"tsconfig.json"),"{}");expect(()=>readTypescriptWorkspaceExcludes(root)).toThrow("verify.typescript.tsconfig-json-must-declare-an");
 writeFileSync(join(root,"tsconfig.json"),'{"exclude":[1]}');expect(()=>readTypescriptWorkspaceExcludes(root)).toThrow("verify.typescript.tsconfig-json-must-declare-an");
});
test("git lists sources, changes and writes the generated project",()=>{
 const root=scratch();git(root,"init","-q");place(root,corpus.git.files);git(root,"add","--",...corpus.git.commit);git(root,"commit","-q","-m","c");
 place(root,corpus.git.later);rmSync(join(root,corpus.git.removeTracked));
 expect(listTypescriptWorkspaceSources(root).sort()).toEqual(corpus.git.sources);expect(listTypescriptChangedSources(root).sort()).toEqual(corpus.git.changed);
 const written=writeTypescriptProject(root,parseTypescriptProjectArguments(["--out","cfg/tsconfig.generated.json"]));
 expect([...written.roots]).toEqual(corpus.git.outRoots);expect(written.stale).toEqual([]);
 const document=JSON.parse(readFileSync(written.path,"utf8"));expect(document.extends).toBe("../tsconfig.json");expect(document.files).toEqual(corpus.git.outRoots.map(path=>"../"+path));expect(document.include).toEqual([]);
 const scoped=writeTypescriptProject(root,parseTypescriptProjectArguments(["--out","cfg/s.json","--scope","a.ts"]));expect([...scoped.roots]).toEqual(corpus.git.scopedRoots);
 writeFileSync(join(root,"list.txt"),"c.d.ts\nmissing.ts\nskip/s.ts\n");const listed=writeTypescriptProject(root,parseTypescriptProjectArguments(["--out","cfg/l.json","--files-from","list.txt"]));
 expect([...listed.roots]).toEqual(corpus.git.listedRoots);expect([...listed.stale].sort()).toEqual(["missing.ts","skip/s.ts"]);
 expect(()=>writeTypescriptProject(root,parseTypescriptProjectArguments(["--out","cfg/n.json","--scope","nowhere"]))).toThrow("verify.typescript.the-selection-matches-no-source");
});
test("the compiler child type-checks every parsable file and reports syntax errors apart",async()=>{
 if(Bun.spawnSync(["node","--version"]).exitCode!==0)return;
 const root=scratch();place(root,corpus.e2e.files);const log=join(root,"out.tsc.txt"),syntax=join(root,"out.syntax.txt");
 const status=await runTypescriptCompiler(root,join(root,"tsconfig.json"),log,syntax,1024,5);
 expect(status).toBe(2);const text=readFileSync(log,"utf8"),summary=summarizeTypescriptDiagnostics(text);
 expect(text.includes(corpus.e2e.semanticFile+"(")).toBe(true);expect(text.includes(corpus.e2e.syntaxFile+"(")).toBe(true);for(const clean of corpus.e2e.cleanFiles)expect(text.includes(clean+"(")).toBe(false);
 expect(summary.byCode["TS2322"]).toBe(1);expect(Object.keys(summary.byCode).some(code=>code.startsWith(corpus.e2e.syntaxCodePrefix))).toBe(true);
 const syntaxText=readFileSync(syntax,"utf8");expect(syntaxText.includes(corpus.e2e.syntaxFile)).toBe(true);expect(syntaxText.includes(corpus.e2e.semanticFile)).toBe(false);
 writeFileSync(join(root,corpus.e2e.syntaxFile),"export const fixed = 1;\n");writeFileSync(join(root,corpus.e2e.semanticFile),"export const bad: number = 1;\n");
 expect(await runTypescriptCompiler(root,join(root,"tsconfig.json"),log,syntax,1024,5)).toBe(0);expect(readFileSync(log,"utf8")).toBe("");
},120000);
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
