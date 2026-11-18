/** 📏️ Contract: a farm child is booked at what children measurably hold, not at the declared worst case.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {askWorkerMegabytesV1} from "../../🟨️.mjs";

test("the declared size stands until three children reported",()=>{
 expect(askWorkerMegabytesV1(1536,[])).toBe(1536);
 expect(askWorkerMegabytesV1(1536,[500,600])).toBe(1536);
});

test("then the 75th percentile plus 15 percent, between 768 MB and the declared size",()=>{
 expect(askWorkerMegabytesV1(1536,[900,1000,1100,1200])).toBe(Math.ceil(1100*1.15));
 expect(askWorkerMegabytesV1(1536,[100,120,140])).toBe(768);
 expect(askWorkerMegabytesV1(1536,[2000,2500,3000])).toBe(1536);
 expect(askWorkerMegabytesV1(1536,[NaN,0,-5,900,950,1000])).toBe(Math.ceil(1000*1.15));
});
