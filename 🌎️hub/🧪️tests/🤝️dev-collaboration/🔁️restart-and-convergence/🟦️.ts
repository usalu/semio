/** 🔁️ Collaboration e2e restart and convergence: the hub restart step, the editor text read, the convergence wait and the undo step. */
import { awaitTcpReady } from "@semio-tech/framework-os-dev/activation/readiness";
import { spaceE2eAssert } from "@semio-tech/framework-os-dev/testing/studio";
import { waitForLocalHubExitV1 } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import type { CollabRecord } from "../⚙️constants-relay-and-recorder/🟦️.ts";
import { collabScopeV1 } from "../🧭️scope-and-dev-servers/🟦️.ts";
import { type CollabHubHandle, collabStartHub } from "../🏠️hub-start-and-credentials/🟦️.ts";
import { collabOpenSpace, collabRowAction, collabScreenshot, collabTextEditor, collabWaitForRow } from "../🖱️ui-driving-and-row-waits/🟦️.ts";
import { type CollabCommandFrameCounter, collabWaitForEditorText } from "../📡️command-frame-recording/🟦️.ts";

//#region 🔖️CollabE2e
/** 🔁️ STEP 8 — restarts the hub against the SAME `dataDir` and the SAME port, then reloads `user2` and
 * confirms the space + artifact rows survive. Deliberately the SAME port (not a fresh one, unlike lane
 * 3-E's Node-only harness): the browser's `S_HUB_URL` is baked into its bundle at Vite `define`-time
 * (contract §C0), so only a same-port restart lets a plain page reload — not a dev-server restart —
 * reconnect; the temp `dataDir` is what actually proves persistence here, matching the brief's own
 * "restart the hub against the same `OS_HUB_DATA`" wording literally. Runs at the orchestration level
 * (not inside `collabRunScenario`) since it needs the hub daemon handle, not just a base URL. */
export async function collabRunRestartStep(opts: {
  readonly record: CollabRecord;
  readonly hubDaemon: CollabHubHandle;
  readonly hubPort: number;
  readonly hubDataDir: string;
  readonly user1: import("playwright").Page;
  readonly user2: import("playwright").Page;
  readonly user2Commands: CollabCommandFrameCounter;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<CollabHubHandle> {
  if (!opts.spaceId || !opts.artifactId) {
    opts.record(9, false, "skipped — no space/artifact id from earlier steps");
    opts.record(10, false, "skipped — no space/artifact id from earlier steps");
    return opts.hubDaemon;
  }
  let liveHub = opts.hubDaemon;
  /** ✍️ The in-flight edit STEP 10 owns: typed into user1's OPEN editor while the hub is dead, so it is
   * committed to the local ledger and queued in the `ArtifactActor` outbox with no `ServerFrame::Ack`
   * behind it. `undefined` when there was no open editor to type into, which STEP 10 reports honestly
   * rather than passing on a vacuous truth. */
  let inFlightMarker: string | undefined;
  try {
    const editor1 = collabTextEditor(opts.user1);
    const hasEditor = (await editor1.count()) > 0;
    await liveHub.kill();
    // 🧵️ We hold the hub's own `child` handle — await its `exit` event via 🔖️PollHelpers's
    const terminal = await waitForLocalHubExitV1(liveHub.run), exited = terminal.status!==null||terminal.signal!==null?"exited":"pending";
    spaceE2eAssert(exited === "exited", "hub process did not exit within 30s of being killed");
    const portFreed = await awaitTcpReady("127.0.0.1", opts.hubPort, { deadlineMs: 30_000, intervalMs: 250, mode: "closed" });
    spaceE2eAssert(portFreed === "ready", `port ${opts.hubPort} never freed up after the hub exited`);
    if (hasEditor) {
      inFlightMarker = `o${Date.now() % 100_000}`;
      await editor1.click();
      await editor1.type(inFlightMarker);
    }
    liveHub = await collabStartHub(collabScopeV1().host,opts.hubPort,"restart");
    await opts.user2.reload({ waitUntil: "domcontentloaded" });
    await collabOpenSpace(opts.user2, opts.spaceId!);
    await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 60_000);
    opts.record(9, true, `hub restarted against the same OS_HUB_DATA (${opts.hubDataDir}) on the same port; user2 still sees space ${opts.spaceId} and artifact ${opts.artifactId} after reload`);
  } catch (error) {
    await collabScreenshot(opts.user2, "step9-user2");
    opts.record(9, false, error instanceof Error ? error.message : String(error));
    opts.record(10, false, "skipped — the hub never came back up");
    return liveHub;
  }
  try {
    spaceE2eAssert(
      inFlightMarker !== undefined,
      "user1 had no open editor at restart time, so nothing was ever in flight — STEP 10 needs STEP 3/4's editor surface to exist before it can prove a resume",
    );
    await collabRowAction(opts.user2, "artifact", opts.artifactId, "open");
    const editor2 = collabTextEditor(opts.user2);
    await editor2.waitFor({ state: "visible", timeout: 30_000 });
    const frames = await collabWaitForEditorText(opts.user2, editor2, opts.user2Commands, inFlightMarker!, 120_000);
    spaceE2eAssert(frames >= 1, `user2 showed the offline edit ${JSON.stringify(inFlightMarker)} without any ServerFrame::Commands frame — it cannot have travelled through the restarted hub`);
    opts.record(
      10,
      true,
      `user1's edit ${JSON.stringify(inFlightMarker)}, typed while the hub was down and never acknowledged, was relayed to user2 after the restart in ${frames} ServerFrame::Commands frame(s) — the resume-token/frontier path carried it rather than dropping it`,
    );
  } catch (error) {
    await collabScreenshot(opts.user1, "step10-user1");
    await collabScreenshot(opts.user2, "step10-user2");
    opts.record(10, false, error instanceof Error ? error.message : String(error));
  }
  return liveHub;
}

/** 📄️ One editor's current text, whichever surface the artifact mounted. */
export async function collabEditorText(editor: import("playwright").Locator): Promise<string> {
  return (await editor.inputValue().catch(() => editor.innerText().catch(() => ""))) ?? "";
}

/** ⏳️ Polls both editors until they agree, returning the settled text. Convergence is the assertion,
 * so a timeout returns the two disagreeing values rather than throwing a bare deadline. */
export async function collabAwaitConvergence(
  user1: import("playwright").Page,
  editor1: import("playwright").Locator,
  editor2: import("playwright").Locator,
  deadlineMs: number,
): Promise<{ readonly converged: boolean; readonly first: string; readonly second: string }> {
  const deadline = Date.now() + deadlineMs;
  let first = "";
  let second = "";
  while (Date.now() < deadline) {
    first = await collabEditorText(editor1);
    second = await collabEditorText(editor2);
    if (first === second && first.length > 0) return { converged: true, first, second };
    await user1.waitForTimeout(250);
  }
  return { converged: false, first, second };
}

/** 🕰️ The shell's ONLY undo affordance is the History panel's `framework.history.undo` control; the
 * chord is owned by the focused window and would be routed to whatever pane has focus. */
export async function collabUndo(page: import("playwright").Page): Promise<void> {
  const historyTab = page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"]');
  spaceE2eAssert((await historyTab.count()) > 0, "no framework.panel.history tab found — the shell offers no undo affordance to press");
  await historyTab.click();
  const undo = page.locator('[id="framework.history.undo"]').locator("button").first();
  const control = (await undo.count()) > 0 ? undo : page.getByRole("button", { name: "Undo", exact: true }).first();
  await control.waitFor({ state: "visible", timeout: 15_000 });
  await control.click();
}
//#endregion 🔖️CollabE2e
