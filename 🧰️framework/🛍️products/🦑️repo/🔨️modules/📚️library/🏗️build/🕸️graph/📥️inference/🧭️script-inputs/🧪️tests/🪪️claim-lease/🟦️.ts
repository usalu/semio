/** 🪪️ Contract: a claim on a derivation is a lease with a holder: a claim whose holder process is gone is taken over at once (nobody waits for a dead holder), a claim whose holder lives is waited for.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{claimDerivationV1,withClosurePassV1}from"../../🟨️.mjs";

const directory=mkdtempSync(join(realpathSync(tmpdir()),"claim-lease-")),file=(id:string)=>join(directory,"claim-"+createHash("sha256").update(id).digest("hex").slice(0,32));
const store={load:async()=>undefined};

test("a claim of a dead holder is taken over at once",()=>withClosurePassV1(async()=>{
 const gone=Bun.spawn(["true"]);await gone.exited;
 const id="/w\0dead-holder-"+Date.now();writeFileSync(file(id),String(gone.pid));
 const started=Date.now(),claim=await claimDerivationV1(id,store,{}) as {release:()=>void};
 expect(typeof claim.release).toBe("function");expect(Date.now()-started,"no waiting for a dead holder").toBeLessThan(500);
 claim.release();
},{verdicts:directory}));

test("a claim of a live holder is waited for",()=>withClosurePassV1(async()=>{
 const id="/w\0live-holder-"+Date.now();writeFileSync(file(id),String(process.ppid));
 let answer:unknown="pending";const waiting=claimDerivationV1(id,store,{}).then(value=>{answer=value;});
 await Bun.sleep(400);expect(answer).toBe("pending");
 rmSync(file(id),{force:true});await waiting;expect(typeof (answer as {release:()=>void}).release).toBe("function");(answer as {release:()=>void}).release();
 rmSync(directory,{recursive:true,force:true});
},{verdicts:directory}));
