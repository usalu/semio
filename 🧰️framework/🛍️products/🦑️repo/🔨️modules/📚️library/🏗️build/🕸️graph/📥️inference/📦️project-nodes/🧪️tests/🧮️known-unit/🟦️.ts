/** 🧮️ Contract: the megabytes the farm admission reads from a warm job are the megabytes `enforcement.known` answers, not megabytes divided by a megabyte again: `known` answers `row.megabytes * footprintFactor`, `knownOf` of the pool takes the payload `knownMegabytes` as megabytes, so a division made every heavy owner look weightless. */
import{expect,test}from"bun:test";
import{join}from"node:path";
const nodes=join(import.meta.dir,"..","..","🟨️.mjs"),pool=join(import.meta.dir,"..","..","🏊️project-farm","🏊️pool","🟨️.mjs");

test("the warm description passes the known megabytes of an owner unchanged",async()=>{
 const text=await Bun.file(nodes).text(),start=text.indexOf("const describe=name=>{"),describe=text.slice(start,text.indexOf("\n",start));
 expect(describe).toContain("knownMegabytes:Math.round(enforcement.known?.(name)??0)");
 expect(describe).not.toContain("1048576");
});

test("the known megabytes enforcement answers are megabytes, the pool reads them as megabytes",async()=>{
 const text=await Bun.file(nodes).text(),at=text.indexOf("known: name => {"),known=text.slice(at,text.indexOf("\n",at));
 expect(known).toContain("row.megabytes * cost.ownerCostPolicyV1.footprintFactor");
 const farm=await Bun.file(pool).text();
 expect(farm).toContain("job?.payload?.knownMegabytes");
});
