/** 📤️ Contract: the commit analysis resolves the range from the newest bundle subject, lists comparable lines of prior commit bodies (without subjects, days, metrics and sign-offs), refuses a bullet that copies one of them case- and whitespace-insensitively and prints log and diff to stderr with the truncation notice; the corpus is language-agnostic and `🐍️.py` replays the commits with the git CLI and recomputes the comparable lines. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {bulletMatchesCommitHistory,commitHistoryCompareLines,emitCommitAnalysis,emitCommitDiff,emitCommitLog,resolveCommitBundleRange,validateBundleBulletsFresh} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const replay=(commits:{subject:string;body:string}[]):{root:string;shas:string[]}=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"emit-")));roots.push(root);git(root,"init","-q");const shas:string[]=[];for(const commit of commits){git(root,"commit","-q","--allow-empty","-m",commit.subject,...(commit.body?["-m",commit.body]:[]));shas.push(git(root,"rev-parse","HEAD"));}return{root,shas};};
const captured=(action:()=>void):string=>{const lines:string[]=[],original=console.error,write=process.stderr.write;console.error=(...parts:unknown[])=>{lines.push(parts.join(" "));};process.stderr.write=((chunk:unknown)=>{lines.push(String(chunk).replace(/\n$/u,""));return true;}) as typeof process.stderr.write;try{action();}finally{console.error=original;process.stderr.write=write;}return lines.join("\n");};
const world=replay(corpus.commits);
test("the range starts at the newest bundle subject",()=>{
 const resolved=resolveCommitBundleRange(world.root);expect(resolved).not.toBeNull();expect(resolved!.wip.sha).toBe(world.shas[corpus.expectedRange.index]);expect(resolved!.wip.subject).toBe(corpus.expectedRange.subject);expect(resolved!.range).toBe(world.shas[corpus.expectedRange.index]+"..HEAD");
 const bare=replay(corpus.noWip.map(subject=>({subject,body:""})));expect(resolveCommitBundleRange(bare.root)).toBeNull();
});
test("prior commit bodies yield normalised comparable lines without bookkeeping lines",()=>{
 const lines=commitHistoryCompareLines(world.root,world.shas[0]!,"HEAD");expect([...lines].sort()).toEqual([...corpus.history].sort());
 for(const text of corpus.excludedLines)expect(lines.has(text.trim().replace(/\s+/gu," ").toLowerCase())).toBe(false);
 expect(commitHistoryCompareLines(world.root,"no-such-revision","HEAD").size).toBe(0);
});
for(const row of corpus.bullets)test("copy detection of "+JSON.stringify(row.bullet),()=>{expect(bulletMatchesCommitHistory(row.bullet,new Set(corpus.history))).toBe(row.expected);});
test("fresh bullets pass and a copied bullet is refused",()=>{
 expect(()=>validateBundleBulletsFresh(world.root,world.shas[0]!,"HEAD",corpus.fresh.ok)).not.toThrow();expect(()=>validateBundleBulletsFresh(world.root,world.shas[0]!,"HEAD",corpus.fresh.copy)).toThrow(corpus.copyMessage);
});
test("the analysis streams name the last bundle, the range and the diff",()=>{
 const log=captured(()=>emitCommitLog(world.root));expect(log).toContain("Last bundle WIP: "+corpus.expectedRange.subject+" ("+world.shas[corpus.expectedRange.index]!.slice(0,7)+")");expect(log).toContain("Range: "+world.shas[corpus.expectedRange.index]+"..HEAD");expect(log).toContain("prior commit messages (context only");for(const line of corpus.history.slice(0,1))expect(log).toContain(line);
 const diff=captured(()=>emitCommitDiff(world.root));expect(diff).toContain("=== git diff --stat (overview) ===");expect(diff).toContain("=== git diff (write bullets from this");expect(diff).toContain("(empty diff)");
 const both=captured(()=>emitCommitAnalysis(world.root));expect(both).toContain("prior commit messages");expect(both).toContain("git diff --stat");
});
test("a large diff is truncated at the byte ceiling with a notice",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"emit-big-")));roots.push(root);git(root,"init","-q");git(root,"commit","-q","--allow-empty","-m",corpus.commits[1]!.subject);
 writeFileSync(join(root,"big.txt"),("x".repeat(99)+"\n").repeat(16000));git(root,"add","-A");git(root,"commit","-q","-m","big");
 const text=captured(()=>emitCommitDiff(root));expect(text).toContain("[commit diff truncated at 1500000 bytes of ");
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
