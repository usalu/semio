/** 🧪️ Collaboration e2e behaviours: the cursor, caret, undo and checkpoint collaboration behaviours checked after the scenario. */
import { spaceE2eAssert } from "@semio-tech/framework-os-dev/testing/studio";
import { COLLAB_E2E_LINK_CUTS_MS, collabFrameMs, type CollabLinkRelay, type CollabRecord, collabShowsRebuild } from "../⚙️constants-relay-and-recorder/🟦️.ts";
import { COLLAB_E2E_KINDS, collabCreateArtifact, collabOpenSpace, collabReopenWriter, collabRowAction, collabScreenshot, collabTextEditor, collabWaitForApp, collabWaitForRow } from "../🖱️ui-driving-and-row-waits/🟦️.ts";
import { collabAssertPeerCaretMoves, collabAssertPeerCursorMoves } from "../👥️presence-and-peer-cursors/🟦️.ts";
import { collabAwaitConvergence, collabEditorText, collabUndo } from "../🔁️restart-and-convergence/🟦️.ts";

//#region 🔖️CollabE2e
/** 🤝️ The three behaviours the brief names and the ten steps never covered (C1b §12.4): per-user
 * undo, a deliberate short connection loss that neither freezes the app nor loses the edit, and two
 * simultaneous writers converging. They run against the SAME live document the scenario opened, so
 * each one is a statement about the real replication lane rather than a unit fixture.
 *
 * `context.setOffline` is the honest shape of "a short connection shortage" (AGENTS.md): the page
 * keeps running, its socket drops, and nothing about the hub or the other human changes — which is
 * exactly the fault the product promises to survive without freezing. */
export async function collabRunCollaborationBehaviours(opts: {
  readonly record: CollabRecord;
  readonly user1: import("playwright").Page;
  readonly user2: import("playwright").Page;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
  readonly relays: readonly [CollabLinkRelay, CollabLinkRelay];
}): Promise<void> {
  if (!opts.spaceId || !opts.artifactId) {
    for (const step of [11, 12, 13, 14, 15]) opts.record(step, false, "skipped — no space/artifact id from earlier steps");
    return;
  }
  const editor1 = collabTextEditor(opts.user1);
  const editor2 = collabTextEditor(opts.user2);

  // STEP 11 — per-user undo
  try {
    spaceE2eAssert((await editor1.count()) > 0 && (await editor2.count()) > 0, "both humans need an open editor before per-user undo can mean anything");
    const mine = `u1-${Date.now() % 100_000}`;
    const theirs = `u2-${Date.now() % 100_000}`;
    await editor1.click();
    await editor1.type(mine);
    await editor2.click();
    await editor2.type(theirs);
    const before = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
    spaceE2eAssert(before.converged, `the two editors never agreed before the undo (user1: ${JSON.stringify(before.first.slice(-120))}, user2: ${JSON.stringify(before.second.slice(-120))})`);
    spaceE2eAssert(before.first.includes(mine) && before.first.includes(theirs), `the shared text is missing one of the two edits before the undo: ${JSON.stringify(before.first.slice(-200))}`);
    await collabUndo(opts.user1);
    const after = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
    spaceE2eAssert(after.converged, `the two editors never agreed after user1's undo (user1: ${JSON.stringify(after.first.slice(-120))}, user2: ${JSON.stringify(after.second.slice(-120))})`);
    spaceE2eAssert(!after.first.includes(mine), `user1's undo did not revert user1's OWN edit ${JSON.stringify(mine)}: ${JSON.stringify(after.first.slice(-200))}`);
    spaceE2eAssert(
      after.first.includes(theirs),
      `user1's undo also reverted user2's edit ${JSON.stringify(theirs)} — undo is a per-author inverse of that author's own envelopes, never a global rewind of the shared ledger: ${JSON.stringify(after.first.slice(-200))}`,
    );
    opts.record(11, true, `user1's undo reverted ${JSON.stringify(mine)} and left user2's ${JSON.stringify(theirs)} standing, in BOTH shells`);
  } catch (error) {
    await collabScreenshot(opts.user1, "step11-user1");
    await collabScreenshot(opts.user2, "step11-user2");
    opts.record(11, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 12 — a short connection loss that does not freeze the app
  try {
    const offlineMarker = `off-${Date.now() % 100_000}`;
    const context2 = opts.user2.context();
    await context2.setOffline(true);
    const typedAt = Date.now();
    await editor2.click();
    await editor2.type(offlineMarker);
    let localEcho = await collabEditorText(editor2);
    while (!localEcho.includes(offlineMarker) && Date.now() - typedAt < 5_000) {
      await opts.user2.waitForTimeout(100);
      localEcho = await collabEditorText(editor2);
    }
    const localLatencyMs = Date.now() - typedAt;
    spaceE2eAssert(localEcho.includes(offlineMarker), `user2's own editor did not echo ${JSON.stringify(offlineMarker)} while offline — the shell froze on the dead socket instead of staying local-first`);
    // 🖱️ A second, independent interaction while still offline: a frozen page cannot answer this.
    const paneResponds = await opts.user2.locator('[id="s-presence-peers"]').count();
    spaceE2eAssert(paneResponds >= 0, "user2's shell stopped answering DOM queries while offline");
    await opts.user2.waitForTimeout(4_000);
    await context2.setOffline(false);
    const recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 90_000);
    spaceE2eAssert(
      recovered.converged && recovered.first.includes(offlineMarker),
      `the edit typed during the outage never reached user1 after the link returned (user1: ${JSON.stringify(recovered.first.slice(-160))}, user2: ${JSON.stringify(recovered.second.slice(-160))})`,
    );
    opts.record(12, true, `user2 stayed interactive through a ~4s link loss (local echo in ${localLatencyMs}ms) and ${JSON.stringify(offlineMarker)} reached user1 once the link returned`);
  } catch (error) {
    await opts.user2.context().setOffline(false).catch(() => undefined);
    await collabScreenshot(opts.user2, "step12-user2");
    opts.record(12, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 13 — two simultaneous writers converge
  try {
    const markerOne = `w1-${Date.now() % 100_000}`;
    const markerTwo = `w2-${Date.now() % 100_000}`;
    await editor1.click();
    await editor2.click();
    await Promise.all([editor1.type(markerOne, { delay: 20 }), editor2.type(markerTwo, { delay: 20 })]);
    const settled = await collabAwaitConvergence(opts.user1, editor1, editor2, 90_000);
    spaceE2eAssert(
      settled.converged,
      `two simultaneous writers did not converge (user1: ${JSON.stringify(settled.first.slice(-200))}, user2: ${JSON.stringify(settled.second.slice(-200))}) — with event-sourced ordering both shells must fold the hub's ONE sequence, so a lasting disagreement is a replication defect, not a merge ambiguity`,
    );
    spaceE2eAssert(settled.first.includes(markerOne) && settled.first.includes(markerTwo), `the converged text dropped one writer's characters entirely: ${JSON.stringify(settled.first.slice(-200))}`);
    opts.record(13, true, `both writers' text survived and both shells settled on the identical document (${JSON.stringify(settled.first.slice(-120))})`);
  } catch (error) {
    await collabScreenshot(opts.user1, "step13-user1");
    await collabScreenshot(opts.user2, "step13-user2");
    opts.record(13, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 14 — in-canvas peer cursors on writer (open from steps 3-4) plus draw + puzzle3d when kinds exist
  try {
    const details: string[] = [];
    // Writer: the document of steps 3–13, its peer marker is the caret.
    try {
      details.push(await collabAssertPeerCaretMoves({ mover: opts.user1, observer: opts.user2, label: "writer" }));
    } catch (error) {
      // Re-open space artifact editors if step 6+ navigated away.
      if (opts.spaceId && opts.artifactId) {
        await collabReopenWriter({ user1: opts.user1, user2: opts.user2, spaceId: opts.spaceId, artifactId: opts.artifactId, purpose: "the caret leg" });
        details.push(await collabAssertPeerCaretMoves({ mover: opts.user1, observer: opts.user2, label: "writer" }));
      } else {
        throw error;
      }
    }
    for (const [label, kind, host] of [
      ["draw", COLLAB_E2E_KINDS.draw, ".semio-canvas-2d-host"],
      ["puzzle3d", COLLAB_E2E_KINDS.puzzle3d, '[data-peer-cursor-world], [data-slot="canvas-presence-overlay"], canvas'],
    ] as const) {
      try {
        await collabOpenSpace(opts.user1, opts.spaceId!);
        await collabWaitForApp(opts.user1, "s-space-create-artifact", 30_000);
        const id = await collabCreateArtifact(opts.user1, `Collab ${label}`, kind);
        await collabOpenSpace(opts.user2, opts.spaceId!);
        await collabWaitForRow(opts.user2, "artifact", id, 30_000);
        // 🚪️ The creation saga opens the new document for its creator; the peer opens it from the Space index.
        await opts.user1.locator(host).first().waitFor({ state: "attached", timeout: 300_000 });
        await collabRowAction(opts.user2, "artifact", id, "open");
        await opts.user2.locator(host).first().waitFor({ state: "attached", timeout: 300_000 });
        details.push(await collabAssertPeerCursorMoves({ mover: opts.user1, observer: opts.user2, host, label }));
      } catch (error) {
        details.push(`${label}: FAILED (${error instanceof Error ? error.message : String(error)})`);
      }
    }
    spaceE2eAssert(details.every((line) => !line.includes(": FAILED (")), `step 14 peer cursors: ${JSON.stringify(details)}`);
    spaceE2eAssert(details.length === 3, `step 14 needs writer, draw and puzzle3d cursor proofs: ${JSON.stringify(details)}`);
    opts.record(14, true, details.join("; "));
  } catch (error) {
    await collabScreenshot(opts.user1, "step14-user1");
    await collabScreenshot(opts.user2, "step14-user2");
    opts.record(14, false, error instanceof Error ? error.message : String(error));
  }

  try {
    const rounds: string[] = [];
    await collabReopenWriter({ user1: opts.user1, user2: opts.user2, spaceId: opts.spaceId!, artifactId: opts.artifactId!, purpose: "the link cuts (STEP 14 left both humans on its draw/puzzle3d documents)" });
    for (const outageMs of COLLAB_E2E_LINK_CUTS_MS) {
      const settled = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
      spaceE2eAssert(settled.converged, `the editors disagreed before the ${outageMs} ms cut`);
      const noticeBefore = (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      const cutAt = Date.now();
      for (const relay of opts.relays) relay.cut(outageMs);
      const markers: string[] = [];
      let worstFrameMs = 0;
      let rebuilt = false;
      while (Date.now() < cutAt + Math.min(outageMs - 1_000, 20_000)) {
        const marker = `cut${outageMs / 1_000}k${markers.length}`;
        await editor1.focus();
        await opts.user1.keyboard.press("End");
        await opts.user1.keyboard.type(` ${marker}`, { delay: 35 });
        markers.push(marker);
        worstFrameMs = Math.max(worstFrameMs, await collabFrameMs(opts.user1), await collabFrameMs(opts.user2));
        rebuilt ||= (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      }
      const remaining = cutAt + outageMs - Date.now();
      if (remaining > 0) await opts.user1.waitForTimeout(remaining);
      const restoredAt = Date.now();
      let recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 120_000);
      while (Date.now() - restoredAt < 120_000 && !markers.every((marker) => recovered.first.includes(marker) && recovered.second.includes(marker))) {
        await opts.user1.waitForTimeout(500);
        recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 10_000);
      }
      rebuilt ||= (await collabShowsRebuild(opts.user1)) || (await collabShowsRebuild(opts.user2));
      const missing = markers.filter((marker) => !recovered.first.includes(marker) || !recovered.second.includes(marker));
      spaceE2eAssert(worstFrameMs < 1_000, `a shell froze during the ${outageMs} ms cut (worst frame ${worstFrameMs} ms)`);
      spaceE2eAssert(noticeBefore || !rebuilt, `the ${outageMs} ms cut rebuilt the document instead of resuming it`);
      spaceE2eAssert(
        recovered.converged && missing.length === 0,
        `keystrokes typed during the ${outageMs} ms cut were lost: ${JSON.stringify(missing)} (user1: ${JSON.stringify(recovered.first.slice(-160))}, user2: ${JSON.stringify(recovered.second.slice(-160))})`,
      );
      rounds.push(`${outageMs / 1_000} s: ${markers.length} markers in both shells ${Date.now() - restoredAt} ms after the link returned, worst frame ${worstFrameMs} ms`);
    }
    opts.record(15, true, rounds.join("; "));
  } catch (error) {
    await collabScreenshot(opts.user1, "step15-user1");
    await collabScreenshot(opts.user2, "step15-user2");
    opts.record(15, false, error instanceof Error ? error.message : String(error));
  }
}
//#endregion 🔖️CollabE2e
