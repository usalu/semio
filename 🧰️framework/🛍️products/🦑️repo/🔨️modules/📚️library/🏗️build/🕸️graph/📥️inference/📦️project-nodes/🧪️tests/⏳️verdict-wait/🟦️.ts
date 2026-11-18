/** ⏳️ Contract: a take waits for the verdict of a farm child at most `OWNER_VERDICT_WAIT_MILLISECONDS_V1`, a late, lost or failing child means no verdict (the reader then verifies the record itself), and a take asks a child only while one is ready and never outside a take, so an owner that is inferred or a farm without a child costs the old path exactly. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{join}from"node:path";
const nodes=join(import.meta.dir,"..","..","🟨️.mjs"),farm=join(import.meta.dir,"..","..","🏊️project-farm","🟨️.mjs"),service=join(import.meta.dir,"..","..","🏊️project-farm","👷️service","🟨️.mjs");
const {verdictWithinV1,OWNER_VERDICT_WAIT_MILLISECONDS_V1}=await import(nodes) as any;

test("a quick answer passes, a late one and a failing one are no verdict",async()=>{
 expect(await verdictWithinV1(Promise.resolve({verdict:"holds"}),1000)).toEqual({verdict:"holds"});
 expect(await verdictWithinV1(new Promise(resolve=>setTimeout(()=>resolve({verdict:"holds"}),300)),20)).toBeUndefined();
 expect(await verdictWithinV1(Promise.reject(new Error("child lost")),1000)).toBeUndefined();
 expect(OWNER_VERDICT_WAIT_MILLISECONDS_V1).toBeGreaterThanOrEqual(60000);
});

test("every lookup of an owner of a read with a farm asks a child for the verdict without a time limit, the farm and the child serve the job kind",async()=>{
 const text=await Bun.file(nodes).text(),start=text.indexOf("let offered,seenHeader"),asked=text.slice(start,text.indexOf("\n",start));
 expect(asked).toContain("verdict:observeJob===undefined?undefined:");
 expect(asked).toContain("farm.observe(observeJob)");
 expect(asked).not.toContain("verdictWithinV1(");
 expect(asked).toContain("observe:true");
 expect(await Bun.file(farm).text()).toContain("async observe(job)");
 expect(await Bun.file(service).text()).toContain("if (job.observe === true) return stagedWithinV1(join(staging, \"observe\"), () => observeProjectRecordV1(environment, job));");
});
