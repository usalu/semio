/** 🎫️ Runs the declared full thesis rendering check under genuine private native authority. */
import { test } from "bun:test";
import assert from "node:assert/strict";
import { withBuildTestInvocationV1 } from "@semio-tech/process/testing/invocation";
import { withPrintExecutionV1 } from "../../../../📦️artifacts/🎛️operation/🟦️.ts";
import { createIssuedResourceControlV1, readIssuedScriptProcessEnvelopeV1 } from "@semio-tech/process/routing";
import { declareThesisTestBirth, prepareThesisTestInputs, prepareThesisPrivateDependencies } from "../../🎟️birth/🟦️.ts";
import { openThesisTestWorkspace } from "../../🧫️workspace/🟦️.ts";
import { verifyThesisRendering } from "../🟦️.ts";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts/native-orchestration/invocation";
import fixture from "../../🔣️.json";

const row = fixture.runners.rendering, source = await openThesisTestWorkspace(), workspace = source.repository.root;
await source.repository.within(async () => declareThesisTestBirth(row));
test("genuine private thesis rendering store birth", () => source.repository.within(() => withBuildTestInvocationV1(async invocation => {
  const actual = readIssuedScriptProcessEnvelopeV1(invocation).capabilities as { arguments?: unknown };
  assert.deepEqual(actual.arguments, ["repo", ...row.capabilities.grant.arguments]);
  assert.equal(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot, workspace);
  const phaseControl = createIssuedResourceControlV1(invocation, "print:thesis:rendering:store-smoke", 0);
  await prepareThesisPrivateDependencies(workspace, invocation, phaseControl);
  await withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "document" }, async () => {});
  await source.recheck();
})), 60000);
test("complete thesis rendering through genuine private native authority", () => source.repository.within(() => withBuildTestInvocationV1(async invocation => {
  const actual = readIssuedScriptProcessEnvelopeV1(invocation).capabilities as { arguments?: unknown };
  assert.deepEqual(actual.arguments, ["repo", ...row.capabilities.grant.arguments]);
  assert.equal(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot, workspace);
  const phaseControl = createIssuedResourceControlV1(invocation, "print:thesis:rendering:test", 0);
  await prepareThesisPrivateDependencies(workspace, invocation, phaseControl);
  await withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "document" }, execution => prepareThesisTestInputs(workspace, invocation, execution));
  await withPrintExecutionV1({ workspace, invocation, phaseControl, kind: "probe" }, verifyThesisRendering);
  await source.recheck();
})), row.maximumElapsedMilliseconds);
