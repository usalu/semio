import{expect,test}from"bun:test";
import cases from"../🧫️fixtures/🔣️.json";
import{portableMsvcCapacityV1}from"../🟦️.ts";

test("pinned compiler capacity admits only bounded volume estimates",()=>{for(const row of cases)expect(portableMsvcCapacityV1(row.allocationBytes)!==null).toBe(row.accepted);expect(portableMsvcCapacityV1(4096)!.minimumAvailableBytes).toBeLessThanOrEqual(2147483648);});
