import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,mirrorLinesV1}from"../../🟦️.ts";

/** 🪞️ Contract: a stage that waits for hours keeps its last real words in the live log mirror; the heartbeat sentences of a hung stage never push them out, only the newest one stays as the sign of life.
 * @see ../../🟦️.ts */
const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=()=>{const path=mkdtempSync(join(tmpdir(),"mirror-"));scratch.push(path);return path;};

test("the mirror of a hung stage shows its last real lines and the newest heartbeat, and equals a Python oracle",()=>{
 const log=join(temp(),"stage.log"),real=["[compose] Verified Node.js 24.15.0 at C:\\Users\\Ueli\\AppData\\Local\\semio\\node\\v24.15.0\\node.exe.","[tools] warming helpers","dependency-store.retiring prefix a1b2"],lines=[...real];
 for(let seconds=60;seconds<=60*600;seconds+=60)lines.push(`[repository:command] still running after ${seconds} s`,`[repo] still working after ${seconds+3} s; last phase started: source.served`);
 writeFileSync(log,lines.join("\r\n")+"\r\n");
 const mirrored=mirrorLinesV1(log);
 expect(mirrored.slice(0,real.length)).toEqual(real);
 expect(mirrored.length).toBe(real.length+1);
 expect(mirrored.at(-1)).toContain("still working after 36003 s");
 const python=spawnSync("python3",["-I","-c","import json,re,sys\nnoise=[re.compile(p) for p in json.loads(sys.argv[1])]\nrows=[l for l in open(sys.argv[2],encoding='utf8').read().splitlines() if l]\nquiet=[l for l in rows if not any(p.search(l) for p in noise)]\nprint(json.dumps(quiet+[rows[-1]]))",JSON.stringify(P.failureNoise),log],{encoding:"utf8"});
 expect(python.status).toBe(0);expect(mirrored).toEqual(JSON.parse(python.stdout));
});

test("a log that holds only heartbeats keeps its own last lines and never exceeds the mirror size",()=>{
 const log=join(temp(),"beats.log"),lines:string[]=[];
 for(let seconds=1;seconds<=3000;seconds++)lines.push(`[repository:command] still running after ${seconds} s`);
 writeFileSync(log,lines.join("\n")+"\n");
 const mirrored=mirrorLinesV1(log);
 expect(mirrored.length).toBeLessThanOrEqual(P.mirrorLines);expect(mirrored.at(-1)).toBe("[repository:command] still running after 3000 s");
 expect(mirrorLinesV1(join(temp(),"missing.log"))).toEqual([]);
});
