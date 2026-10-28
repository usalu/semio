/** 🧭️ Contract: bundle path prefixes follow the label tokens: a path belongs to the bundle with the most matching tokens (first wins a tie), a bundle owns its assigned paths and their token-carrying parents, and a numstat row resolves to the bundles owning any of its paths; the corpus is language-agnostic and `🐍️.py` replays the commits with the git CLI and recomputes the assignment. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {assignChangedPathsToBundles,buildBundlePathPrefixSets,inferPathPrefixesForBundleLabel,pathMatchesBundleIndex,resolveBundleIndicesForNumstatRow} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const put=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const root=realpathSync(mkdtempSync(join(tmpdir(),"prefixes-")));roots.push(root);git(root,"init","-q");put(root,corpus.repo.base);git(root,"add","-A");git(root,"commit","-q","-m","base");const base=git(root,"rev-parse","HEAD");
for(const commit of corpus.repo.commits){put(root,commit.files);git(root,"add","-A");git(root,"commit","-q","-m",commit.subject,"-m",commit.body);}
const head=git(root,"rev-parse","HEAD"),sets=buildBundlePathPrefixSets(root,base,head,corpus.bundles);
test("changed paths are assigned to the bundle whose label tokens they contain",()=>{expect(assignChangedPathsToBundles(root,base,head,corpus.bundles)).toEqual(corpus.expected.assigned);});
test("each bundle owns its assigned paths and every parent that still carries the label tokens",()=>{expect(sets).toEqual(corpus.expected.prefixSets);});
for(const[index,row]of corpus.expected.infer.entries())test("inferred prefix "+index+" of "+row.label,()=>{expect(inferPathPrefixesForBundleLabel(root,base,head,row.label,row.paths??undefined)).toEqual(row.expected);});
for(const row of corpus.rowMatch)test("bundle owners of the numstat row "+JSON.stringify(row.field),()=>{expect(resolveBundleIndicesForNumstatRow(row.field,sets,corpus.bundles).sort()).toEqual(row.expected);});
for(const[index,row]of corpus.pathMatch.entries())test("path match "+index,()=>{expect(pathMatchesBundleIndex(row.path,row.index,row.prefixSets,corpus.bundles)).toBe(row.expected);});
test("a revision range without label matches or with an unknown base yields no prefixes",()=>{expect(inferPathPrefixesForBundleLabel(root,"no-such-revision",head,corpus.bundles[0]!.label)).toEqual([]);expect(assignChangedPathsToBundles(root,"no-such-revision",head,corpus.bundles)).toEqual([[],[]]);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
