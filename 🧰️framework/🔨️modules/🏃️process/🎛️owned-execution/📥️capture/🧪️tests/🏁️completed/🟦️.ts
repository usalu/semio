import{test,expect}from"bun:test";
import{requireCompletedCaptureAuthorityV1}from"../../🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
/** 🏁️ A command that exited by itself keeps its result after the loan ran out; only cancellation refuses it. */
test("a finished owned command is admitted after an expired loan and refused only when cancelled",()=>{
 for(const row of fixture.cases){
  const controller=new AbortController();if(row.aborted)controller.abort(Error("cancelled"));
  const control={signal:controller.signal,remainingMilliseconds:()=>row.remainingMilliseconds,publish:()=>undefined,yieldContinuation:async()=>undefined};
  const admitted=(()=>{try{requireCompletedCaptureAuthorityV1(control);return true;}catch{return false;}})();
  expect(admitted,row.name).toBe(row.admitted);
  expect(admitted,row.name).toBe(!controller.signal.aborted);
 }
});
