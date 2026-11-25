/** 🔇️ Contract: a read whose own slice report of this attempt stopped being written (an event loop that no longer turns: m03 at `taken 52 of 608` for 4,900 s while its tree spent processor time and the host repeated the last phase) is a stall once the report is older than `sliceSilentMilliseconds`; a fresh report, the final report of a finished read, a report of an earlier attempt and a malformed one are not. The verdicts equal an independent model in CPython.
 * @see ../../🟦️.ts `readySliceSilentV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readySliceSilentV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";

const P=schema.definitions.Policy.const,now=10_000_000_000,started=now-6_000_000;
const cases:readonly [string,unknown][]=[
 ["fresh interval report",{atMs:now-30_000,reason:"interval"}],
 ["silent just below the window",{atMs:now-P.sliceSilentMilliseconds+1,reason:"interval"}],
 ["silent at the window",{atMs:now-P.sliceSilentMilliseconds,reason:"interval"}],
 ["m03: silent 4,900 s",{atMs:now-4_900_000,reason:"interval"}],
 ["finished read",{atMs:now-4_900_000,reason:"end"}],
 ["report of an earlier attempt",{atMs:started-1,reason:"interval"}],
 ["no clock",{reason:"interval"}],
 ["text clock",{atMs:String(now-4_900_000),reason:"interval"}],
 ["no report",null],
];

test("a read whose report stopped for the silent window is a stall, everything else is not, as the independent model says",()=>{
 const rows=cases.map(([name,report])=>({name,report,silent:readySliceSilentV1(report,now,started)}));
 const run=spawnSync("python3",["-I","-c","import json,sys\nd=json.load(sys.stdin)\nprint(json.dumps([isinstance((r['report'] or {}).get('atMs'),int) and not isinstance((r['report'] or {}).get('atMs'),bool) and r['report']['atMs']>=d['started'] and r['report'].get('reason')!='end' and d['now']-r['report']['atMs']>=d['window'] for r in d['rows']]))"],{input:JSON.stringify({now,started,window:P.sliceSilentMilliseconds,rows}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(rows.map(row=>row.silent)).toEqual(JSON.parse(run.stdout));
 expect(rows.filter(row=>row.silent).map(row=>row.name)).toEqual(["silent at the window","m03: silent 4,900 s"]);
 expect(P.sliceSilentMilliseconds).toBeGreaterThan(P.stallStopMilliseconds);
});
