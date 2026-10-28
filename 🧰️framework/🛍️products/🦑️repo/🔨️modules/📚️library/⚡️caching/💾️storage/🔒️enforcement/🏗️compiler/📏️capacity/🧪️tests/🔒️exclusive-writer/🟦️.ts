import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,readFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const root=fileURLToPath(new URL("../../../../../../../../../../../../",import.meta.url)),definition="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
const files=():string[]=>spawnSync("git",["ls-files","-z","--cached","--others","--exclude-standard","--","🧰️framework/*.ts"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(path=>path&&existsSync(join(root,path))&&!/\/🧪️tests\//u.test(path));
const callers=(paths:readonly string[]):string[]=>paths.filter(path=>path!==definition&&/\bwithCompiler(WorkspaceWrite|InitialHeadroom)V1\s*\(/u.test(readFileSync(join(root,path),"utf8"))).sort();

test("no production flow takes the exclusive compiler writer, so a holder of the shared writer can never queue an upgrade behind its own cohort",()=>{
 const paths=files(),actual=callers(paths);
 const program=`import json,os,re,subprocess,sys
root,definition=sys.argv[1],sys.argv[2]
names=subprocess.run(["git","ls-files","-z","--","🧰️framework/*.ts"],cwd=root,capture_output=True,text=True).stdout.split("\\0")
pattern=re.compile(r"\\bwithCompiler(WorkspaceWrite|InitialHeadroom)V1\\s*\\(")
rows=sorted(p for p in names if p and os.path.exists(root+"/"+p) and "/🧪️tests/" not in p and p!=definition and pattern.search(open(root+"/"+p,encoding="utf8").read()))
print(json.dumps(rows))`;
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",program,root,definition],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 expect(paths.length).toBeGreaterThan(1000);
 expect(JSON.parse(run.stdout)).toEqual(actual);
 expect(actual).toEqual([]);
},180000);
