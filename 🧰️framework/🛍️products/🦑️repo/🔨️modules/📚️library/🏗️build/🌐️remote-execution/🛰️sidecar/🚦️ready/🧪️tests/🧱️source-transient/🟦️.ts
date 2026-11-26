import{expect,test}from"bun:test";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,readyDecideV1,readySourceRetryV1,readySourceTransientV1,readyRefusalV1}from"../../🟦️.ts";

/** 🧱️ Contract: a stage that fails on a source-level cause of the checkout (a missing or refused source file, import or module: a peer's half-finished edit) is classified `source-transient`; its next attempt starts a debounce after the next change of the replicated tree instead of after the exponential back-off, and keeps the back-off when the tree does not change. Equal to an independent Python implementation of both rules.
 * @see ../../🟦️.ts */
const oracle=(mode:string,payload:unknown)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),mode,JSON.stringify(payload)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const root="/Users/ueli/Documents/semio";
const cases:{lines:string[];root:string;want:boolean}[]=[
 {lines:[`Internal error: ENOENT: no such file or directory, lstat '${root}/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/x/🐚️shell' (reference e6bf3cf2)`],root,want:true},
 {lines:["Private Nx engine import missing: ../../a/b.ts from /x/y.ts"],root,want:true},
 {lines:['RepoError: refused process.source.crate-refused {"crate":"c","reason":"Cargo Source outside admitted owner"}'],root,want:true},
 {lines:["\u001b[31merror: Cannot find module './gone.ts'\u001b[0m"],root,want:true},
 {lines:["ENOENT: no such file or directory, open '/elsewhere/cache/file'"],root,want:false},
 {lines:["No space left (ENOSPC) in /x: the disk"],root,want:false},
 {lines:[`ENOENT: no such file or directory, lstat '${root}/a'`],root:"",want:false},
 {lines:["The plan cannot continue: the slice worker ended 3 times in a row with exit code 75 and the same text: Unexpected } (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:100)"],root,want:true},
 {lines:[`SyntaxError: Unexpected token '}' at ${root}/🧰️framework/x/🟦️.ts:3:1`],root,want:true},
 {lines:[`error: Expected ")" but found "}" (${root}/a/b.ts:9:4)`],root,want:true},
 {lines:["The plan cannot continue: the slice worker ended 3 times in a row with exit code 75 and the same text: out of memory"],root,want:false},
 {lines:["SyntaxError: Unexpected token '}' at /elsewhere/x.ts:3:1"],root,want:false},
 {lines:["stage failed with exit code 1"],root,want:false},
 {lines:["The resident service met an unexpected failure (reference 1c7c6ae4): Error [BUILD_SOURCE_CONFLICT]: Conflicting inference observations: global:[\"closure-graph\"]"],root,want:true},
 {lines:["Error [BUILD_SOURCE_CONFLICT]: Something else entirely"],root,want:false},
 {lines:[`error: failed to write \`${root}/.cache/b/r/m/b/p/prototype/build/semio-framework-pack-http/cf085c40b268a8e7/fingerprint/invoked.timestamp\``],root,want:true},
 {lines:["error: failed to write `/elsewhere/cache/x.timestamp`"],root,want:false},
 {lines:["The resident service met an unexpected failure (reference 145eb0b1): Error: Complete retained closure publication required"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 84438681): Error: Closure reference proof leaf changed"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference d4797b0e): Error: Slice worker exited with code null: [graph-closure] {}"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 1): Error: Slice worker exited with code 1: out of memory"],root,want:false},
 {lines:["The resident service met an unexpected failure (reference 92b15f5b): Error: Closure index descriptor changed"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f5c): Error: Closure proof descriptor changed"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f5d): Error: Original group retirement descriptor changed"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f5e): ConcurrentEditErrorV1: Closure index file changed while it was open / Abschlussindex Datei hat sich geändert, während sie offen war: /x/y.idx"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f5f): ConcurrentEditErrorV1: Closure index append-only file was replaced, linked again or shortened while it was open / Abschlussindex anhängbare Datei wurde ersetzt, neu verlinkt oder gekürzt, während sie offen war: /x/y.idx"],root,want:true},
 {lines:["Slice worker failed: Closure proof leaf descriptor stamp mismatch (ino)"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f60): Error: Closure index is required"],root,want:false},
 {lines:["The resident service met an unexpected failure (reference 92b15f61): ConcurrentEditErrorV1: Cache charge receipt file changed while it was open / Cache-Belastungsbeleg Datei hat sich geändert, während sie offen war: /x/y.json"],root,want:true},
 {lines:["The resident service met an unexpected failure (reference 92b15f62): ConcurrentEditErrorV1: Retiring closure leaf file was replaced again while it was opened by name / Abzuräumendes Abschlussblatt Datei wurde beim Öffnen über den Namen erneut ersetzt: /x/y.json"],root,want:true},
 {lines:["The file changed while it was open in another editor"],root,want:false},
];
test("the source-transient classification equals an independent Python implementation",()=>{
 const expected=oracle("classify",cases.map(row=>({lines:row.lines,root:row.root,patterns:P.sourceTransientPatterns})));
 cases.forEach((row,index)=>{expect(readySourceTransientV1(row.lines,row.root),JSON.stringify(row)).toBe(row.want);expect(expected[index]).toBe(row.want);});
});
const failure=(extra:Record<string,unknown>)=>({code:"source-transient",atMs:1000,tree:"a",nextAttemptMs:1000+600000,...extra});
test("the next attempt of a source-transient failure waits for a changed tree plus the debounce, else the back-off, equal to Python",()=>{
 const rows=[];
 for(const code of["source-transient","stage-failed"])for(const tree of[null,{stamp:"a",changedMs:2000},{stamp:"b",changedMs:500},{stamp:"b",changedMs:5000},{stamp:"b",changedMs:700000}])for(const recorded of["a",undefined])rows.push({failure:failure({code,...recorded===undefined?{tree:undefined}:{}}),tree,debounce:P.sourceTransientDebounceMilliseconds});
 const expected=oracle("retry",rows);
 rows.forEach((row,index)=>{const input={code:row.failure.code as never,atMs:row.failure.atMs,nextAttemptMs:row.failure.nextAttemptMs,...row.failure.tree===undefined?{}:{tree:row.failure.tree}};expect(readySourceRetryV1(input,row.tree),JSON.stringify(row)).toBe(expected[index]);});
 expect(expected.some((value:number)=>value<600000+1000)&&expected.some((value:number)=>value===601000)).toBe(true);
});
test("the decision retries a source-transient failure shortly after the tree changed and keeps waiting while it did not",()=>{
 const digest="d".repeat(64),stages=[{name:"workspace",digest:"1".repeat(64)}],base={server:false,digest,stages,done:{},marker:false,repo:false,human:false,lease:"free" as const,inflight:null,memoryOk:true};
 const recorded={digest,code:"source-transient" as const,atMs:1000,tree:"a",nextAttemptMs:601000};
 const waiting=readyDecideV1({...base,failure:recorded,tree:{stamp:"a",changedMs:1},now:20000});
 expect(waiting.action).toBe("backoff");expect(waiting.waitMilliseconds).toBe(P.sourceTransientPollMilliseconds);
 const changed=readyDecideV1({...base,failure:recorded,tree:{stamp:"b",changedMs:2000},now:2000+P.sourceTransientDebounceMilliseconds-1000});
 expect(changed.action).toBe("backoff");expect(changed.waitMilliseconds).toBe(1000);
 expect(readyDecideV1({...base,failure:recorded,tree:{stamp:"b",changedMs:2000},now:2000+P.sourceTransientDebounceMilliseconds}).action).toBe("run");
 expect(readyDecideV1({...base,failure:{...recorded,code:"stage-failed"},tree:{stamp:"b",changedMs:2000},now:20000}).action).toBe("backoff");
 expect(readyDecideV1({...base,failure:{...recorded,code:"stage-failed"},tree:{stamp:"b",changedMs:2000},now:20000}).waitMilliseconds).toBe(581000);
});
test("the refusal has English and German texts that name the stage and the debounce",()=>{
 const refusal=readyRefusalV1("source-transient",{stage:"workspace",ending:{en:"exit 1",de:"Exit 1"},seconds:15});
 expect(refusal.en).toContain("workspace");expect(refusal.en).toContain("15 seconds");expect(refusal.de).toContain("workspace");expect(refusal.de).toContain("15 Sekunden");
});
