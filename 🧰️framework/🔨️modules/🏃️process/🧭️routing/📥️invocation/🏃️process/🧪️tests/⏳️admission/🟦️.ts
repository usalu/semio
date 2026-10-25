import{test,expect}from"bun:test";
import*as api from"../../🟦️.ts";
import{admissionWaitedMillisecondsV1,beginAdmissionWaitV1,creditedDeadlineRemainingV1}from"../../🐕️watchdog/🟦️.ts";

const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

test("a deadline credits the time its process waited for machine admission, and an independent Python model agrees",()=>{
 const rows=[[600,0],[600,450],[-10,0],[-10,450],[0,0],[200,-5]],model=`import json,sys
print(json.dumps([r+max(0,w) for r,w in json.loads(sys.argv[1])]))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rows)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 expect(rows.map(([remaining,waited])=>creditedDeadlineRemainingV1(remaining!,waited!))).toEqual(JSON.parse(run.stdout.toString()));
});

test("overlapping admission waits count once and an open wait is counted while it lasts",async()=>{
 const before=admissionWaitedMillisecondsV1(),first=beginAdmissionWaitV1();await sleep(60);const second=beginAdmissionWaitV1();await sleep(60);first();first();await sleep(60);second();
 const waited=admissionWaitedMillisecondsV1()-before;expect(waited).toBeGreaterThanOrEqual(170);expect(waited).toBeLessThan(400);
 const open=beginAdmissionWaitV1();await sleep(50);expect(admissionWaitedMillisecondsV1()-before).toBeGreaterThanOrEqual(waited+45);open();
});

test("a control requested before the admission wait expires only after its own time, not the queue's",async()=>{
 await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:admission",maximumElapsedMilliseconds:0},{fixture:"admission"},Date.now()),async original=>{
  const control=api.createRequestedIssuedResourceControlV1(api.createIssuedResourceControlV1(original,"fixture:admission",0),600),end=beginAdmissionWaitV1();
  await sleep(450);end();await sleep(300);
  expect(control.signal.aborted).toBe(false);expect(control.remainingMilliseconds()!).toBeGreaterThan(0);expect(control.remainingMilliseconds()!).toBeLessThanOrEqual(600-300);
  await sleep(400);
  expect(()=>control.remainingMilliseconds()).toThrow("Original resource loan deadline exhausted");
 });
});
