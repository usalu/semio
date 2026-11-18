/** 🗃️ Contract: a farm child holds no custody of the graph store of its parent, so the job that resolves closure references in the child (which writes optional program receipts under the graph data directory) runs inside a private staging directory and never asks the store for custody. A real child process against the store that enforces custody: the bare call is refused with the custody error that stopped a real read, the job inside `stagedWithinV1` gets a directory under its staging.
 * @see ../../👷️service/🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";

test("the store refuses a bare writer directory in a real child and gives the staged job a private one",async()=>{
 const child=Bun.spawn([process.execPath,join(import.meta.dir,"👷️child.ts")],{stdout:"pipe",stderr:"pipe",env:{...process.env}});
 const out=await new Response(child.stdout).text();await child.exited;
 const line=out.split("\n").find(each=>each.startsWith("REPORT "));expect(line,out).toBeDefined();
 const{report,staging}=JSON.parse(line!.slice(7));
 expect(report.bare).toContain("Original bounded graph writer custody required");
 expect(report.job.startsWith("ok:"),report.job).toBe(true);expect(report.job).toContain(staging);
});

test("the observe job of the service runs staged and an owner whose job failed is set aside with its reason instead of ending the read",()=>{
 const service=readFileSync(join(import.meta.dir,"..","..","👷️service","🟨️.mjs"),"utf8"),nodes=readFileSync(join(import.meta.dir,"..","..","..","🟨️.mjs"),"utf8");
 expect(service).toContain('stagedWithinV1(join(staging, "observe"), () => observeProjectRecordV1(environment, job))');
 expect(nodes).toContain('const failedOwner=error=>{if(error?.code!=="graph.farm-job-failed")return false;');
 expect(nodes).toContain("deferOwner(error)||failedOwner(error)||");
 expect(nodes).toContain("unavailable.set(root,{root,project:owner.name");
});
