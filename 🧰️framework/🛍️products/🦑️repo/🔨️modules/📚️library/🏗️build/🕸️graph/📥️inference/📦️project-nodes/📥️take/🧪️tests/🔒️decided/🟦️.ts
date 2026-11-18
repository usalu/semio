/** 🔒️ Contract: an owner that is present, fetched or inferred is never taken back to stale or missing by a later look of the take pass, so the ledger of a read never shows an owner as stale that the read already has
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{createTakeLedgerV1}from"../../🟨️.mjs";

const source=readFileSync(new URL("../../../🟨️.mjs",import.meta.url),"utf8");

test("a decided owner stays decided when a late look finds it stale or missing",()=>{
 const ledger=createTakeLedgerV1(3);
 ledger.mark("a","inferred");ledger.mark("a","stale");ledger.mark("b","present");ledger.mark("b","miss");ledger.mark("c","fetched");ledger.mark("c","stale");
 expect(ledger.counts()).toMatchObject({stale:0,here:1,present:1,fetched:1,done:3,missing:0});
});

test("an undecided owner still moves from stale to inferred and from miss to stale",()=>{
 const ledger=createTakeLedgerV1(2);
 ledger.mark("a","miss");ledger.mark("a","stale");expect(ledger.counts().stale).toBe(1);ledger.mark("a","inferred");expect(ledger.counts()).toMatchObject({stale:0,here:1,done:1});
});

test("an owner that is neither done nor running is read before the end of the owner pass waits on the fleet",()=>{
 const sweep=source.slice(source.indexOf("const sweepOrphans=async()=>{"),source.indexOf("const inferAll=async()=>{")),all=source.slice(source.indexOf("const inferAll=async()=>{"),source.indexOf("try{await runTakeBesideInferV1"));
 expect(sweep).toContain("const orphans=");
 for(const guard of["!never.has(job.name)","!taken.has(job.name)","!active.has(job.name)","!inflight.has(job.name)","!tooHeavy.has(job.name)","!unavailable.has(job.json.root)","ledger.state(job.name)","options?.skipOwner?.(job.name)!==true"])expect(sweep,guard).toContain(guard);
 expect(sweep).toContain("sliceReadStepV1(\"orphan-sweep\"");expect(sweep).toContain("Projekte sind weder fertig noch in Arbeit");
 expect(all.indexOf("await sweepOrphans();")).toBeGreaterThan(all.indexOf("await Promise.allSettled(extras);"));
 expect(all.indexOf("await waitHeavy();")).toBeGreaterThan(all.indexOf("await sweepOrphans();"));
});

test("the wait for the fleet is a named step of the read",()=>{
 expect(source).toContain("sliceReadStepV1(\"heavy-wait\",()=>wantedModule.awaitWantedV1(");
});
