/** 🚶️ Contract: a fresh slice report of this attempt whose phase counts or owner ledger changed is progress of the read although the stage log repeats its last line (a planner at `taken 559 of 616` on a swapping 16 GB machine); an equal signature, a finished read, a report of an earlier attempt and a malformed one are none. The signature equals an independent model in CPython.
 * @see ../../🟦️.ts `readySliceMovedSignatureV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readySliceMovedSignatureV1}from"../../🟦️.ts";

const started=1_000;
const rows:readonly [string,unknown][]=[
 ["taken 559 of 616",{atMs:2_000,reason:"interval",phase:{name:"taken",done:559,total:616},sources:{owners:{ledger:{done:559,running:2,total:616},finished:3,taken:557}}}],
 ["taken 560 of 616",{atMs:2_030,reason:"interval",phase:{name:"taken",done:560,total:616},sources:{owners:{ledger:{done:560,running:2,total:616},finished:3,taken:558}}}],
 ["same counts later",{atMs:2_060,reason:"interval",phase:{name:"taken",done:560,total:616},sources:{owners:{ledger:{done:560,running:2,total:616},finished:3,taken:558}}}],
 ["no phase",{atMs:2_000,reason:"interval"}],
 ["finished read",{atMs:2_000,reason:"end",phase:{name:"taken",done:616,total:616}}],
 ["earlier attempt",{atMs:started-1,reason:"interval",phase:{name:"taken",done:1,total:2}}],
 ["no clock",{reason:"interval",phase:{name:"taken",done:1,total:2}}],
 ["no report",null],
];

test("the signature of a report moves exactly when the counts of the read move, as the independent model says",()=>{
 const got=rows.map(([,report])=>readySliceMovedSignatureV1(report,started));
 const run=spawnSync("python3",["-I","-c","import json,sys\nd=json.load(sys.stdin)\ndef f(r):\n if not isinstance(r,dict) or not isinstance(r.get('atMs'),int) or r['atMs']<d['started'] or r.get('reason')=='end': return None\n o=((r.get('sources') or {}).get('owners')) or {}\n l=o.get('ledger') or {}\n p=r.get('phase') or {}\n return json.dumps([p.get('name'),p.get('done'),p.get('total'),l.get('done'),l.get('running'),l.get('total'),o.get('finished'),o.get('taken')],separators=(',',':'))\nprint(json.dumps([f(r) for r in d['rows']]))"],{input:JSON.stringify({started,rows:rows.map(([,report])=>report)}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(got).toEqual(JSON.parse(run.stdout));
 expect(got[0]).not.toBe(got[1]);
 expect(got[1]).toBe(got[2]);
 expect(got.slice(4)).toEqual([null,null,null,null]);
});
