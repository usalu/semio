/** 📈️ Contract: commit attribution assigns every changed path to exactly one bundle by label tokens, sums micro-commit uloc and byte deltas per bundle and per day over a real git range, and refuses unowned or doubly owned paths, missing days and non-additive totals; the corpus is language-agnostic and `🐍️oracle.py` replays the commits with the git CLI and recomputes assignment and sums from `git diff --numstat`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {assignChangedPathsToBundles,buildBundlePathPrefixSets,inferPathPrefixesForBundleLabel,pathMatchesBundleIndex,resolveBundleIndicesForNumstatRow} from "../../🧭️path-prefixes/🟦️.ts";
import {buildBundleDateDeltasMap,buildBundleDateSizeDeltasMap,formatBundleHeaderLine,formatGitDeltaSumBrief,partitionRangeDeltasByBundle,partitionRangeSizeDeltasByBundle,rangeGitDeltaTotal,sortCommitBundlesByEditTotal,validateBundleCommitAttribution,validateBundleDayDeltasAttribution,type BundleDateDeltasMap} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
type Delta={added:number;removed:number;edited:number};
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const put=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const replay=(extra:Readonly<Record<string,string|undefined>>={}):{root:string;base:string;head:string}=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"attrib-")));roots.push(root);git(root,"init","-q");put(root,corpus.repo.base);git(root,"add","-A");git(root,"commit","-q","-m","base");const base=git(root,"rev-parse","HEAD");
 for(const[index,commit]of corpus.repo.commits.entries()){put(root,commit.files);if(index===corpus.repo.commits.length-1)put(root,extra);git(root,"add","-A");git(root,"commit","-q","-m",commit.subject,"-m",commit.body);}
 return{root,base,head:git(root,"rev-parse","HEAD")};
};
const asRows=(map:BundleDateDeltasMap):unknown=>[...map.keys()].sort().map(index=>[...map.get(index)!.entries()].sort(([a],[b])=>a<b?-1:1));
const stub={countRepoUlocByLanguage:()=>({TypeScript:100}),countRepoSizeByLanguage:()=>({TypeScript:4000})};
const fixed=replay(),sets=buildBundlePathPrefixSets(fixed.root,fixed.base,fixed.head,corpus.bundles);
test("changed paths are assigned to the bundle whose label tokens they contain",()=>{expect(assignChangedPathsToBundles(fixed.root,fixed.base,fixed.head,corpus.bundles)).toEqual(corpus.expected.assigned);});
test("each bundle owns its assigned paths and every parent that still carries the label tokens",()=>{expect(sets).toEqual(corpus.expected.prefixSets);});
for(const[index,row]of corpus.expected.infer.entries())test("inferred prefix "+index+" of "+row.label,()=>{expect(inferPathPrefixesForBundleLabel(fixed.root,fixed.base,fixed.head,row.label,row.paths??undefined)).toEqual(row.expected);});
for(const row of corpus.rowMatch)test("bundle owners of the numstat row "+JSON.stringify(row.field),()=>{expect(resolveBundleIndicesForNumstatRow(row.field,sets,corpus.bundles).sort()).toEqual(row.expected);});
for(const[index,row]of corpus.pathMatch.entries())test("path match "+index,()=>{expect(pathMatchesBundleIndex(row.path,row.index,row.prefixSets,corpus.bundles)).toBe(row.expected);});
test("per-day uloc and byte deltas are summed per bundle over the micro-commits",()=>{
 expect(asRows(buildBundleDateDeltasMap(fixed.root,fixed.base,fixed.head,corpus.bundles))).toEqual(corpus.expected.dateUloc);expect(asRows(buildBundleDateSizeDeltasMap(fixed.root,fixed.base,fixed.head,corpus.bundles))).toEqual(corpus.expected.dateSize);
});
test("the range partitions across bundles with matching totals",()=>{
 expect(partitionRangeDeltasByBundle(fixed.root,fixed.base,fixed.head,corpus.bundles,sets)).toEqual(corpus.expected.partitionUloc);expect(partitionRangeSizeDeltasByBundle(fixed.root,fixed.base,fixed.head,corpus.bundles,sets)).toEqual(corpus.expected.partitionSize);
 expect(rangeGitDeltaTotal(fixed.root,fixed.base,fixed.head)).toEqual(corpus.expected.partitionUloc.rangeTotal);
});
test("bundles are ranked by their edit total",()=>{
 const reversed=[corpus.bundles[1]!,corpus.bundles[0]!],ranked=sortCommitBundlesByEditTotal(fixed.root,fixed.base,fixed.head,reversed,[["b"],["a"]]);
 expect(ranked.bundles.map(item=>item.label)).toEqual(corpus.sortReverse.expectedOrder);expect(ranked.pathAssignments).toEqual([["a"],["b"]]);expect(ranked.pathPrefixSets).toEqual(corpus.expected.prefixSets);
});
for(const row of corpus.brief)test("brief delta "+JSON.stringify(row.delta),()=>{expect(formatGitDeltaSumBrief(row.delta)).toBe(row.expected);});
for(const[index,row]of corpus.header.entries())test("bundle header line "+index,()=>{expect(formatBundleHeaderLine(row.label,row.uloc,row.size,row.ulocBloc,row.sizeBloc)).toBe(row.expected);});
for(const row of corpus.validate)test("day attribution: "+row.name,()=>{
 const bundles=row.bundles??corpus.bundles,dateDeltas:BundleDateDeltasMap=new Map(row.days.map((days,index)=>[index,new Map(days as [string,Delta][])]));
 const run=()=>validateBundleDayDeltasAttribution(bundles,[],dateDeltas,row.totals,"uloc",row.additive);
 if(row.message===null)expect(run).not.toThrow();else expect(run).toThrow(row.message);
});
test("a complete bundle body passes the whole attribution check",()=>{expect(()=>validateBundleCommitAttribution(fixed.root,fixed.base,fixed.head,corpus.bundles,stub)).not.toThrow();});
for(const row of corpus.commitErrors)test("commit attribution refuses "+row.name,()=>{const world=replay(row.extra);expect(()=>validateBundleCommitAttribution(world.root,world.base,world.head,row.bundles,stub)).toThrow(row.message);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
