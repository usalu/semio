import assert from "node:assert/strict";
import { createFlowBrowserRuntime } from "../../📦️packages/🟨️javascript/🌐️flow-browser.js";
import { MockFlowBridge } from "../🎭️mock-flow-bridge/🟦️.ts";

interface FlowOpenFailureFixture {
  readonly verifiedRejection: { readonly nativeSession: "absent"; readonly entry: "released" };
  readonly uncertainTransport: { readonly entry: "retained-until-host-terminal"; readonly globalCloseCalls: number; readonly terminal: boolean };
}

export async function testFlowOpenOwnership(fixture: FlowOpenFailureFixture): Promise<void> {
  const memory = new WebAssembly.Memory({ initial: 400 });
  const rejectedBridge = new MockFlowBridge(memory, { rejectOpenReplies: 1 });
  const rejectedRuntime = await createFlowBrowserRuntime({ source: rejectedBridge.exports });
  const rejectedSession = rejectedRuntime.openSession();
  await assert.rejects(rejectedSession.selectedWidgetIds().result, /session open rejected before admission/);
  await rejectedRuntime.close();
  assert.equal(rejectedBridge.closedSessionSlots.length === 0 ? "absent" : "present", fixture.verifiedRejection.nativeSession);
  assert.equal(rejectedRuntime.terminalIsEmpty(), fixture.verifiedRejection.entry === "released");

  const uncertainBridge = new MockFlowBridge(memory);
  let failPoll = true;
  const uncertainRuntime = await createFlowBrowserRuntime({
    source: {
      ...uncertainBridge.exports,
      flow_bridge_poll(...args: Parameters<typeof uncertainBridge.exports.flow_bridge_poll>): number {
        if (failPoll) {
          failPoll = false;
          throw new Error("uncertain open transport failure");
        }
        return uncertainBridge.exports.flow_bridge_poll(...args);
      },
    },
  });
  const uncertainSession = uncertainRuntime.openSession();
  await assert.rejects(uncertainSession.selectedWidgetIds().result, /uncertain open transport failure/);
  await assert.rejects(uncertainRuntime.close(), /uncertain open transport failure/);
  assert.deepEqual(uncertainBridge.closedSessionSlots, [1]);
  assert.equal(uncertainBridge.globalCloseCalls, fixture.uncertainTransport.globalCloseCalls);
  assert.equal(uncertainRuntime.terminalIsEmpty(), fixture.uncertainTransport.terminal);
  console.log("[DEBUG] Flow open rejection released only its verified pre-admission entry; an uncertain admitted open remained owned through exact global terminal close");
}
