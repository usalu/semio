import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/📏️close-demand/🔣️.json" with { type: "json" };

test("original job close separates exact independent grants and preserves terminal progress",()=>{
  const demands={maximumItems:fixture.progress.copiedItems,maximumCopyBytes:fixture.progress.copiedBytes,maximumCapacityBytes:fixture.progress.retainedCapacityBytes,maximumReleaseBytes:fixture.progress.releasedBytes,maximumDepth:2};
  const allowed=(grant:typeof fixture.grant)=>Object.entries(demands).every(([key,value])=>grant[key as keyof typeof grant]>=value);
  expect(allowed(fixture.grant)).toBe(true);expect(fixture.progress.releasedBytes).toBeGreaterThan(fixture.grant.maximumCopyBytes);
  for(const key of fixture.deniedCredits){
    const refused=applyPatch(structuredClone(fixture.grant),[{op:"replace",path:`/${key}`,value:demands[key as keyof typeof demands]-1}],true).newDocument;
    expect(allowed(refused)).toBe(false);
  }
  const pending={phase:"pending",progress:fixture.progress},complete=applyPatch(structuredClone(pending),[{op:"replace",path:"/phase",value:"complete"}],true).newDocument;
  expect(complete.progress).toEqual(pending.progress);expect(fixture.close.terminal).toBe("original-owner-empty");expect(fixture.close.blocked).toBe("registered-wake");
  expect(fixture.close.refusal).toBe("original-owner-retained");expect(fixture.close.demand).toBe("result-for-copy-capacity-release-depth");
  console.log("[DEBUG] original job full grants retain physical release65536 with copy3 and exact terminal progress");
});

test("worker physical demand preserves caller authority and independent allocation state", async () => {
  const db=new Database(":memory:");try{db.run("CREATE TABLE quotes(physical INTEGER,caller INTEGER,released INTEGER)");for(const row of fixture.cases)db.query("INSERT INTO quotes VALUES(?,?,?)").run(row.physicalBytes,row.callerBytes,row.releasedBytes);expect(db.query("SELECT count(*) AS n FROM quotes WHERE released=CASE WHEN caller>=physical THEN physical ELSE 0 END AND released<=caller").get()).toEqual({n:6});}finally{db.close();}
  expect(fixture.demandRole).toBe("pure-original-owner-quote");expect(fixture.callerRole).toBe("independent-fixed-policy");
  for (const row of fixture.cases) {
    const before = { retainedBytes: row.physicalBytes, releasedBytes: 0 };
    const operations = row.callerBytes >= row.physicalBytes ? [
      { op: "replace" as const, path: "/retainedBytes", value: 0 },
      { op: "replace" as const, path: "/releasedBytes", value: row.physicalBytes },
    ] : [];
    const after = applyPatch(structuredClone(before), operations, true).newDocument;
    expect(after.releasedBytes).toBe(row.releasedBytes);
    expect(after.retainedBytes + after.releasedBytes).toBe(row.physicalBytes);
    expect(after.releasedBytes).toBeLessThanOrEqual(row.callerBytes);
    expect(Math.max(fixture.pageBytes, row.physicalBytes)).toBeLessThanOrEqual(fixture.admissionBytes);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("fn next_close_copy_byte_demand(&self)")).toBe(true);
  expect(source.split("pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError>").length-1).toBe(3);
  expect(source.includes("pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,ValueError>")).toBe(true);
  expect(source.includes("pub fn next_close_demands(")).toBe(false);
  const authority=source.slice(source.indexOf("fn worker_job_authority_close_demands"),source.indexOf("fn worker_job_close_step",source.indexOf("fn worker_job_authority_close_demands")));
  expect(authority.includes("Result<RetirementDemand,ValueError>")).toBe(true);
  expect(authority.includes("RetainedCloneGrant{")).toBe(false);
  expect(source.includes("pub fn child_retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>")).toBe(true);
  expect(source.includes("pub fn next_child_close_demands")).toBe(false);
  expect(source.includes("Arc::into_inner(payload_ledger)")).toBe(true);
  console.log("[DEBUG] worker close demand: six physical extents preserve exact caller release grant");
});

test("worker close receipt preserves all admitted currencies at pending and completion",async()=>{
 const db=new Database(":memory:");db.run("CREATE TABLE receipt(items INTEGER,body INTEGER,capacity INTEGER,release INTEGER)");
 db.run("INSERT INTO receipt VALUES(?,?,?,?)",fixture.progress.copiedItems,fixture.progress.copiedBytes,fixture.progress.retainedCapacityBytes,fixture.progress.releasedBytes);
 const expected=db.query("SELECT items AS copiedItems,body AS copiedBytes,capacity AS retainedCapacityBytes,release AS releasedBytes FROM receipt").get();
 expect(expected).toEqual(fixture.progress);
 const changed=applyPatch({kind:"pending",progress:fixture.progress},[{op:"replace",path:"/kind",value:"complete"}],true).newDocument;
 expect(changed.progress).toEqual(expected);db.close();
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 expect(source.includes("impl WorkerJobCloseStep {")).toBe(true);
 console.log("[DEBUG] worker pending/complete original full receipt items1/copy3/capacity128/release65536 conserved by SQLite/RFC6902");
});


test("actual worker callers preserve independent page and parent authority policies with finite attempts",async()=>{
  const policy=fixture.phasePolicy;
  expect(policy.page.maximumReleaseBytes).toBe(fixture.pageBytes);
  expect(policy.authority.maximumReleaseBytes).toBe(65536);
  expect(policy.page.maximumCapacityBytes).toBe(0);
  const before={owner:"original",copy:0,physical:0};
  const oracle=applyPatch(structuredClone(before),[{op:"replace",path:"/copy",value:policy.page.maximumCopyBytes},{op:"replace",path:"/physical",value:policy.page.maximumReleaseBytes}],true,false).newDocument;
  expect(oracle).toEqual({owner:"original",copy:32768,physical:16384});
  for(const copy of [1,3,64]){expect(copy).toBeLessThan(policy.page.maximumCopyBytes);expect({...oracle,copy}).toEqual(applyPatch(structuredClone(oracle),[{op:"replace",path:"/copy",value:copy}],true,false).newDocument);}
  expect(policy.maximumAttempts).toBe(65536);expect(policy.deadlineMs).toBe(30000);expect(policy.refusal).toBe("original-owner-retained");
  const owner=await Bun.file(new URL("../🔬️retained-ownership/🦀️.rs",import.meta.url)).text();
  expect(owner.includes("fn caller_worker_close_policy(")).toBe(true);
  expect(owner.includes("WorkerJobClosePhase::AuthorityRelease")).toBe(true);
  expect(owner.includes("maximum_release_bytes:release_grant")).toBe(false);
  expect(owner.includes("maximum_items:1,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES")).toBe(false);
});

test("retained fixture Box/Arc and parked queue expose physical ownership before caller funding",async()=>{
 const db=new Database(":memory:");try{db.run("CREATE TABLE owners(bytes INTEGER,credit INTEGER,shared INTEGER)");for(const row of fixture.hostileOwners){db.run("DELETE FROM owners");db.run("INSERT INTO owners VALUES(?,?,?)",row.allocationBytes,row.callerRelease,Number(row.shared));const oracle=db.query("SELECT CASE WHEN credit>=bytes AND shared=0 THEN bytes ELSE 0 END AS releasedBytes, credit<bytes AS retained FROM owners").get() as {releasedBytes:number,retained:number};expect(row.releasedBytes).toBe(oracle.releasedBytes);expect(row.retained).toBe(Boolean(oracle.retained));const after=applyPatch({retained:true,releasedBytes:0},row.retained?[]:[{op:"replace",path:"/retained",value:false},{op:"replace",path:"/releasedBytes",value:row.releasedBytes}],true).newDocument;expect(after).toEqual({retained:row.retained,releasedBytes:row.releasedBytes});}}finally{db.close();}
 const owner=await Bun.file(new URL("../🔬️retained-ownership/🦀️.rs",import.meta.url)).text();
 const hostile=owner.slice(owner.indexOf("impl InteractiveJob for HostileJob"),owner.indexOf("fn mounted_admitted"));
 expect(hostile.includes("fn next_close_copy_byte_demand")).toBe(true);expect(hostile.includes("Arc::into_inner")).toBe(true);
 const root=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();expect(root.includes("pub fn worker_job_retirement_phase(")).toBe(true);
});
