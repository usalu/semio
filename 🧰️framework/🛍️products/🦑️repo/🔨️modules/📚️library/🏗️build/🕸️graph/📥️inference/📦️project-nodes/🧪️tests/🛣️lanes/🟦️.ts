import{expect,test}from"bun:test";
import{inferenceLanesV1}from"../../🟨️.mjs";

/** 🛣️ The inference lanes of a read follow the cores and the memory one owner is measured to hold, not the cut ladder alone: a cut load keeps the lanes its memory fits. */
test("lanes are the cores, the memory the parent may grow by divided by the measured owner, and never fewer than the cut allows",()=>{
 expect(inferenceLanesV1({cores:10,cut:1,parentMegabytes:2400,ownerMegabytes:800})).toBe(3);
 expect(inferenceLanesV1({cores:2,cut:1,parentMegabytes:8000,ownerMegabytes:800})).toBe(2);
 expect(inferenceLanesV1({cores:10,cut:Infinity,parentMegabytes:80000,ownerMegabytes:800})).toBe(8);
 expect(inferenceLanesV1({cores:10,cut:4,parentMegabytes:800,ownerMegabytes:800})).toBe(4);
 expect(inferenceLanesV1({cores:10,cut:1,parentMegabytes:100,ownerMegabytes:800})).toBe(1);
 expect(inferenceLanesV1({cores:10,cut:1,parentMegabytes:2400,ownerMegabytes:0})).toBe(3);
 expect(inferenceLanesV1({cores:10,cut:1,parentMegabytes:2400,ownerMegabytes:800,explicit:2})).toBe(1);
 expect(inferenceLanesV1({cores:10,cut:4,parentMegabytes:2400,ownerMegabytes:800,explicit:2})).toBe(2);
});
