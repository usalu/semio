import {expect,test} from "bun:test";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import*as issuer from "@semio-tech/process/routing/invocation/process";

/** 📨️ Asserts the Original a test file is born with is exactly the one the orchestrator hands a suite: its budget comes from the caller's expectation, never from the budget of the suite row. */
test("this file process receives the envelope of the orchestrator",()=>withBuildTestInvocationV1(async original=>{
 const wire=issuer.readIssuedScriptProcessEnvelopeV1(original),expected=JSON.parse(process.env.SEMIO_TEST_ENVELOPE_EXPECT!) as {maximumElapsedMilliseconds:number};
 expect(wire.policy.version).toBe(1);
 expect(wire.policy.maximumElapsedMilliseconds).toBe(expected.maximumElapsedMilliseconds);
 expect(wire.deadlineEpochMilliseconds===null).toBe(expected.maximumElapsedMilliseconds===0);
}));
