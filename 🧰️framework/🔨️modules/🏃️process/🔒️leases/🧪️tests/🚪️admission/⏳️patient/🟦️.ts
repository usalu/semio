/** ⏳️ Patient: 1 law covering the admission contention corpus, where a queued waiter outlives its own deadline while the directory gate is busy and then acquires. */
import{withIssuedResourceControlV1}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{declareBuildTestEnvelopeV1,registerBuildTestLawV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
import{testResourceLeaseAdmissionTimeoutV1}from"../🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:leases",maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}});
registerBuildTestLawV1("queued admission waits past the caller deadline instead of failing the build","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:admission:patient",0,async control=>{const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual admission sink required");await testResourceLeaseAdmissionTimeoutV1(sink,control);}));
