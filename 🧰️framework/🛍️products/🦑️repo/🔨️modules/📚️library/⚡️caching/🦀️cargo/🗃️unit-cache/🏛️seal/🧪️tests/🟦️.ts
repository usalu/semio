/** 🏛️ Contract: a build session runs a sealed, content-addressed copy of the tooling taken at its start; a later edit reaches the next session only, and a tree that is mid-edit (does not parse, does not load, changes while it is read) is retried and never sealed.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {requirePinnedTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {ensureUnitCacheShimV1,unitCacheWrapperV1,sealedUnitCacheV1,UNIT_CACHE_SCRIPT_V1} from "../../🚪️launch/🟦️.ts";
import {sealModulesV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const policy=UNIT_CACHE_POLICY_V1.seal,python=requirePinnedTestToolV1("python"),instant={sleep:()=>undefined,runtime:process.execPath};
const scratch=():string=>mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"seal-"));
const write=(live:string,path:string,text:string):string=>{const file=join(live,path);mkdirSync(dirname(file),{recursive:true});try{chmodSync(file,0o644);}catch{}writeFileSync(file,text);return file;};
const populate=(live:string):string=>{for(const [path,text] of Object.entries(fixture.files))write(live,path,text);return join(live,fixture.entry);};
const run=(entry:string):string=>{const result=spawnSync(process.execPath,[entry],{encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return result.stdout;};
const lib=(live:string):string=>join(live,"set","lib","x.ts");
const sandbox=(body:(live:string,cache:string,entry:string)=>void):void=>{const root=scratch(),live=join(root,"live"),cache=join(root,"cache");try{body(live,cache,populate(live));}finally{rmSync(root,{recursive:true,force:true});}};

test("the digest of a sealed set equals the independent Python computation and the fixture vector", () => sandbox((live,cache,entry) => {
 const sealed=sealModulesV1(cache,[entry],{...instant,checked:[entry]});
 expect(sealed.digest).toBe(fixture.digest);expect(sealed.fresh).toBe(true);expect(sealed.files).toBe(Object.keys(fixture.files).length);
 const oracle=spawnSync(python,["-I","-c","import hashlib,os,sys\nroot=sys.argv[1];rows=[]\nfor base,_,names in os.walk(root):\n for name in names:\n  path=os.path.join(base,name);rel=os.path.relpath(path,root).replace(os.sep,'/')\n  if rel!='sealed.json':rows.append((rel,hashlib.sha256(open(path,'rb').read()).hexdigest()))\nrows.sort()\nprint(hashlib.sha256(''.join(f'{p}\\0{h}\\n' for p,h in rows).encode()).hexdigest()[:16])",sealed.directory],{encoding:"utf8"});
 expect(oracle.stdout.trim(),oracle.stderr).toBe(sealed.digest);
 expect(run(sealed.entries[0]!)).toBe(fixture.output);
 expect(statSync(sealed.entries[0]!).mode&0o222,"sealed files are read-only").toBe(process.platform==="win32"?statSync(sealed.entries[0]!).mode&0o222:0);
}));

test("an edit during a session does not change the code the session runs, only the next session", () => sandbox((live,cache,entry) => {
 const session=sealModulesV1(cache,[entry],{...instant,checked:[entry]});
 write(live,"set/lib/x.ts","export const value=1000;\n");
 expect(run(session.entries[0]!),"the running session keeps its snapshot").toBe(fixture.output);
 expect(run(entry),"the live tree moved").toBe("1002:true\n");
 const next=sealModulesV1(cache,[entry],{...instant,checked:[entry]});
 expect(next.digest).not.toBe(session.digest);expect(run(next.entries[0]!)).toBe("1002:true\n");
 expect(run(session.entries[0]!),"the older snapshot is still intact").toBe(fixture.output);
 expect(sealModulesV1(cache,[entry],{...instant,checked:[entry]})).toMatchObject({digest:next.digest,fresh:false});
}));

test("a half-finished edit is never sealed: the last good snapshot serves and the blocking place is named", () => sandbox((live,cache,entry) => {
 const good=sealModulesV1(cache,[entry],{...instant,checked:[entry]});
 write(live,"set/lib/x.ts","export const value=;\n");
 const served=sealModulesV1(cache,[entry],{...instant,checked:[entry],attempts:3});
 expect(served.digest).toBe(good.digest);expect(served.stale.length).toBeGreaterThan(0);expect(served.stale[0]!.path.endsWith("x.ts")).toBe(true);expect(served.stale[0]!.line).toBeGreaterThan(0);
 expect(readdirSync(join(cache,policy.directory)).filter(name=>/^[0-9a-f]{16}$/u.test(name))).toEqual([good.digest]);
 expect(run(served.entries[0]!)).toBe(fixture.output);
}));

test("without a good snapshot a tree that does not parse refuses with the first blocking place", () => sandbox((live,cache,entry) => {
 write(live,"set/lib/x.ts","export const value=;\n");
 expect(()=>sealModulesV1(cache,[entry],{...instant,attempts:2})).toThrow(/after 2 attempt\(s\): .*x\.ts:\d+:\d+/u);
 expect(existsSync(join(cache,policy.directory,fixture.digest))).toBe(false);
}));

test("an edit that parses but does not load (a missing export) is not sealed", () => sandbox((live,cache,entry) => {
 const good=sealModulesV1(cache,[entry],{...instant,checked:[entry]});
 write(live,"set/lib/x.ts","export const other=1;\n");
 const served=sealModulesV1(cache,[entry],{...instant,checked:[entry],attempts:1});
 expect(served.digest).toBe(good.digest);expect(served.stale[0]!.message).toContain("does not load");
 expect(readdirSync(join(cache,policy.directory)).filter(name=>name.startsWith(".stage-")),"a refused stage never stays").toEqual([]);
}));

test("a tree that is still being written is retried after the settle pause and sealed once it holds still", () => sandbox((live,cache,entry) => {
 write(live,"set/lib/x.ts","export const value=;\n");
 const pauses:number[]=[];
 const sealed=sealModulesV1(cache,[entry],{runtime:process.execPath,checked:[entry],sleep:milliseconds=>{pauses.push(milliseconds);write(live,"set/lib/x.ts","export const value=40;\n");}});
 expect(pauses).toEqual([policy.settleMilliseconds]);expect(sealed.digest).toBe(fixture.digest);expect(sealed.stale).toEqual([]);
}));

test("a file that changes between the parse and the second read is not sealed", () => sandbox((live,cache,entry) => {
 let edits=0;
 const parse=()=>{write(live,"set/lib/x.ts",`export const value=${40+ ++edits};\n`);return[];};
 expect(()=>sealModulesV1(cache,[entry],{...instant,attempts:3,parse})).toThrow(/changed while they were read/u);
 expect(edits).toBe(3);
}));

test("snapshots outside the newest few and the retention window are collected, the current one never", () => sandbox((live,cache,entry) => {
 const day=policy.retainMilliseconds+1,digests:string[]=[];let removed:string[]=[];
 for(let step=0;step<7;step++){write(live,"set/lib/x.ts",`export const value=${step};\n`);const sealed=sealModulesV1(cache,[entry],{...instant,nowMs:Date.now()+step*2*day});digests.push(sealed.digest);removed=[...removed,...sealed.removed];}
 const kept=readdirSync(join(cache,policy.directory)).filter(name=>/^[0-9a-f]{16}$/u.test(name));
 expect(kept.length).toBe(policy.keepSnapshots);expect(kept).toContain(digests.at(-1)!);expect(new Set([...kept,...removed])).toEqual(new Set(digests));
}));

test("the compiler wrapper stand-in names the sealed copy, never the live tooling", () => {
 const root=scratch();
 try{
  const cache=join(root,"u"),shim=ensureUnitCacheShimV1(cache),sealed=sealedUnitCacheV1(cache),script=sealed.entries[0]!;
  expect(unitCacheWrapperV1(cache)).toBe(shim);expect(script).toBe(sealed.entries[0]!);expect(script.startsWith(join(cache,policy.directory))).toBe(true);expect(script).not.toBe(UNIT_CACHE_SCRIPT_V1);
  if(process.platform!=="win32"){const text=readFileSync(shim,"utf8");expect(text).toContain(script);expect(text).not.toContain(UNIT_CACHE_SCRIPT_V1);expect(shim.endsWith(sealed.digest)).toBe(true);}
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
