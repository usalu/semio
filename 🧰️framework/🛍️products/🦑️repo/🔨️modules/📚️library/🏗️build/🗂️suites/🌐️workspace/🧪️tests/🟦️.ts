import{test}from"bun:test";
import{runWorkspaceBuildTestsV1}from"../🟦️.ts";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
declareLibraryBuildTestEnvelopeV1({suite:"build-route-table"});
test("mixed workspace leaves retain independent received births, native grants and cleanup lifetimes",()=>withBuildTestInvocationV1(original=>runWorkspaceBuildTestsV1(original,process.cwd(),fixture.integrationFiles)),300000);
