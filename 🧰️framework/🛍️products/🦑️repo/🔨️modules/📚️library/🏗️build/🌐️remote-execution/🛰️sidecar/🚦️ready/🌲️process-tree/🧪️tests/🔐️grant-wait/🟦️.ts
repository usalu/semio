import {expect,test} from "bun:test";
import {readyGrantWaitV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const report=(atMs:number,waits:readonly number[])=>({atMs,sources:{owners:{presence:{mappingQueue:{waiting:waits.map(waitedMilliseconds=>({kind:"shared",step:"grant",waitedMilliseconds}))}}}}});

/** 🔐️ Contract: a read queued for the mapping is not stalled while its fresh report lists a waiter younger than the maximum; an old report, an empty queue, an overlong wait or a missing section change nothing, and the maximum is a safety of minutes, not the half hour of the old queue. */
test("a fresh report with a pending mapping wait holds the stage, anything else does not",()=>{
 const now=10_000_000;
 expect(readyGrantWaitV1(report(now-1000,[2_000]),now)).toBe(true);
 expect(readyGrantWaitV1(report(now-1000,[900_000,2_000]),now)).toBe(true);
 expect(readyGrantWaitV1(report(now-1000,[]),now)).toBe(false);
 expect(readyGrantWaitV1(report(now-600_000,[2_000]),now)).toBe(false);
 expect(readyGrantWaitV1(report(now-1000,[300_000]),now)).toBe(false);
 expect(readyGrantWaitV1({atMs:now},now)).toBe(false);
 expect(readyGrantWaitV1({atMs:now,sources:{owners:{presence:{nativeVerification:{postVerify:{"mounted-image":{mappingGrant:{active:1,milliseconds:1}}}}}}}},now),"a report without the queue (older code) never holds the stage").toBe(false);
 expect(readyGrantWaitV1(null,now)).toBe(false);
 expect(schema.definitions.Policy.const.grantWaitMaximumMilliseconds).toBeLessThanOrEqual(300_000);
});
