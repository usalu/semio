/** 🫧️ Contract: a farm child releases what a cold job left in the cache shared by its owners, after a job that finished and after one that failed, so the next heavy owner does not land on the state of every earlier one; the release is the growth-driven one of a look (`shrink` off).
 * @see ../../../🟨️.mjs `produceProjectRecordV1` */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";

const here=dirname(fileURLToPath(import.meta.url)),nodes=join(here,"..","..","..","🟨️.mjs"),fixture=join(here,"🧫️fixtures","🟨️.mjs");
const run=(mode:string)=>{const done=Bun.spawnSync([process.execPath,fixture,nodes,mode],{env:{...process.env}});expect(done.exitCode,done.stderr.toString()).toBe(0);return JSON.parse(done.stdout.toString().trim().split("\n").pop()!);};

test("a cold job releases the owner state of its child once, whether it finished or failed",()=>{
 const finished=run("finishes"),failed=run("throws");
 expect(finished.failed).toBe(false);expect(failed.failed).toBe(true);
 for(const row of[finished,failed]){expect(row.calls.length).toBe(1);expect(row.calls[0].scripts).toBe(true);expect(row.calls[0].options).toEqual({shrink:false});}
});
