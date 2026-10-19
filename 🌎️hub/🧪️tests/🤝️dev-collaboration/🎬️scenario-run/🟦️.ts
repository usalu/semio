/** 🎬️ Collaboration e2e scenario run: the scripted collaboration scenario over two signed-in browsers. */
import { spaceE2eAssert } from "@semio-tech/framework-os-dev/testing/studio";
import { COLLAB_E2E_LOCALES, COLLAB_E2E_USER2_EMAIL, type CollabLocale, type CollabRecord } from "../⚙️constants-relay-and-recorder/🟦️.ts";
import { COLLAB_E2E_KINDS, collabAwaitCheckinStatus, collabAwaitRowUpdatedMinute, collabClickToolbarButton, collabCreateArtifact, collabCurrentCheckpointId, collabHubUserId, collabOpenCheckin, collabOpenSpace, collabRowAction, collabScreenshot, collabSelectOption, collabSubmitDialog, collabTextEditor, collabWaitForDialog, collabWaitForEditor, collabWaitForNamedRow, collabWaitForRow } from "../🖱️ui-driving-and-row-waits/🟦️.ts";
import { collabPresenceColors, collabWaitForPresenceRoster } from "../👥️presence-and-peer-cursors/🟦️.ts";
import { type CollabCommandFrameCounter, type CollabSentCommands, collabWaitForEditorText } from "../📡️command-frame-recording/🟦️.ts";

//#region 🔖️CollabE2e
/** 🎬️ The whole 8-step scenario, run against two already-booted `s` react dev servers and a live hub.
 * Each step is wrapped so a failure is recorded and the run continues to the next step wherever the
 * remaining steps can still be meaningfully attempted. */
export async function collabRunScenario(
  record: CollabRecord,
  user1: import("playwright").Page,
  user2: import("playwright").Page,
  hubBaseUrl: string,
  user2Commands: CollabCommandFrameCounter,
  adminCapability: () => string,
  user1Sent: CollabSentCommands,
  locale: CollabLocale,
): Promise<{ readonly spaceId: string | undefined; readonly artifactId: string | undefined }> {
  let spaceId: string | undefined;
  let artifactId: string | undefined;

  try {
    const spaceName = `Collab Studio ${Date.now()}`;
    await collabClickToolbarButton(user1, "s-home-create-space");
    await collabWaitForDialog(user1);
    await user1.locator("#name").fill(spaceName);
    await collabSelectOption(user1, "kind", COLLAB_E2E_LOCALES[locale].studio);
    await collabSelectOption(user1, "visibility", COLLAB_E2E_LOCALES[locale].public);
    await collabSubmitDialog(user1);
    spaceId = await collabWaitForNamedRow(user1, "space", spaceName, 30_000);
    record(1, true, `space ${spaceId} created; user1's Home lists it`);
  } catch (error) {
    await collabScreenshot(user1, "step1-user1");
    await collabScreenshot(user2, "step1-user2");
    record(1, false, error instanceof Error ? error.message : String(error));
  }

  if (spaceId) {
    try {
      await collabRowAction(user1, "space", spaceId, "share");
      await collabWaitForDialog(user1);
      await user1.locator("#email").fill(COLLAB_E2E_USER2_EMAIL);
      await collabSelectOption(user1, "role", COLLAB_E2E_LOCALES[locale].author);
      await collabSubmitDialog(user1);
      const live = await collabWaitForRow(user2, "space", spaceId, 60_000).then(() => true).catch(() => false);
      if (!live) {
        await user2.reload({ waitUntil: "domcontentloaded" });
        await collabWaitForRow(user2, "space", spaceId, 120_000);
      }
      await collabOpenSpace(user2, spaceId);
      record(
        2,
        live,
        live
          ? `user2's Home listed ${spaceId} live once shared as author; /spaces/${spaceId} mounted the Space app with its create-artifact affordance`
          : `user2's Home was NOT told live that ${spaceId} was shared (no directory event within 60 s); it listed the space only after a reload — later steps continue on it`,
      );
    } catch (error) {
      await collabScreenshot(user1, "step2-user1");
      await collabScreenshot(user2, "step2-user2");
      record(2, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(2, false, "skipped — no space id from STEP 1");
  }

  if (spaceId) {
    try {
      await collabOpenSpace(user1, spaceId);
      artifactId = await collabCreateArtifact(user1, "Collab Writer", COLLAB_E2E_KINDS.writer);
      const editorOpened = await collabWaitForEditor(user1, 240_000);
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      spaceE2eAssert(
        editorOpened,
        "no editable text surface appeared for user1 after createArtifact — inspect the direct Space opening effect, exact Shell session and scope, Hub open-plan/catalog verification, socket Session and retained document UI; the Space relay already includes documentId, spaceId and schema",
      );
      record(3, true, `artifact ${artifactId} created, row replicated to user2, editor surface present for user1`);
    } catch (error) {
      await collabScreenshot(user1, "step3-user1");
      await collabScreenshot(user2, "step3-user2");
      record(3, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(3, false, "skipped — no space id from STEP 1");
  }

  if (spaceId && artifactId) {
    try {
      await collabRowAction(user2, "artifact", artifactId, "open");
      await collabWaitForEditor(user2, 240_000);
      const editor1 = collabTextEditor(user1);
      const editor2 = collabTextEditor(user2);
      spaceE2eAssert((await editor1.count()) > 0, "user1 has no editable text surface open (see STEP 3)");
      spaceE2eAssert((await editor2.count()) > 0, "user2 has no editable text surface open after clicking the artifact row's open button");
      const probeText = `collab-probe-${Date.now()}`;
      await editor1.click();
      await editor1.type(probeText);
      const deadline = Date.now() + 30_000;
      let seen = "";
      while (Date.now() < deadline) {
        seen = (await editor2.inputValue().catch(() => editor2.innerText().catch(() => ""))) ?? "";
        if (seen.includes(probeText)) break;
        await user2.waitForTimeout(500);
      }
      spaceE2eAssert(
        seen.includes(probeText),
        `user2's editor never showed user1's typed text ${JSON.stringify(probeText)} (last seen: ${JSON.stringify(seen.slice(-200))}) — both editors are likely unbound ephemeral instances rather than the same hub-synced document (same root cause as STEP 3)`,
      );
      spaceE2eAssert(user1Sent.undecodable.length === 0, `user1 sent document frames this harness could not decode: ${JSON.stringify(user1Sent.undecodable.slice(0, 3))}`);
      spaceE2eAssert(user1Sent.documentIds.length > 0, "user1 typed into the fresh door artifact but sent no ClientFrame::Commands envelope");
      spaceE2eAssert(
        user1Sent.documentIds[0] === artifactId && user1Sent.documentIds.every((documentId, index) => documentId === user1Sent.socketDocumentIds[index]),
        `the fresh door artifact's first outbound Commands envelope named ${JSON.stringify(user1Sent.documentIds[0])}, not the artifact ${JSON.stringify(artifactId)} (socket documents ${JSON.stringify([...new Set(user1Sent.socketDocumentIds)])})`,
      );
      record(4, true, `user1's typed text propagated to user2's editor; the fresh door artifact's first ${user1Sent.documentIds.length} outbound Commands envelope(s) all name documentId ${artifactId}`);
    } catch (error) {
      await collabScreenshot(user1, "step4-user1");
      await collabScreenshot(user2, "step4-user2");
      record(4, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(4, false, "skipped — no artifact id from STEP 3");
  }

  try {
    const peers1 = user1.locator('[id="s-presence-peers"]');
    const peers2 = user2.locator('[id="s-presence-peers"]');
    spaceE2eAssert(
      (await peers1.count()) > 0,
      "#s-presence-peers does not exist in the React shell (🧰️framework/…/renderer/…/ShellHost/🟦️.tsx never imports or renders PresenceBar — confirmed by grep; lane 2-D wired presence only into the wgpu Shell, 🧊️component.rs, which per the ticket brief does not compile this wave)",
    );
    spaceE2eAssert((await peers2.count()) > 0, "#s-presence-peers does not exist in user2's shell either");
    const roster1 = await collabWaitForPresenceRoster(user1, 2, 30_000);
    const roster2 = await collabWaitForPresenceRoster(user2, 2, 30_000);
    spaceE2eAssert(roster1 === 2, `user1's presence roster has ${roster1} peer(s), expected 2`);
    spaceE2eAssert(roster2 === 2, `user2's presence roster has ${roster2} peer(s), expected 2`);
    const colors1 = await collabPresenceColors(user1);
    const colors2 = await collabPresenceColors(user2);
    for (const [label, colors] of [
      ["user1", colors1],
      ["user2", colors2],
    ] as const) {
      spaceE2eAssert(colors.every((color) => color.length > 0), `${label}'s presence roster painted an empty avatar colour: ${JSON.stringify(colors)}`);
      spaceE2eAssert(
        new Set(colors).size === colors.length,
        `${label}'s two presence avatars are painted the SAME colour (${JSON.stringify(colors)}) — the hub's per-session ServerFrame::Session.color never reached the roster, so the two sessions are indistinguishable on screen`,
      );
    }
    record(5, true, `both shells show a 2-peer presence roster with distinct session colours (user1: ${JSON.stringify(colors1)}, user2: ${JSON.stringify(colors2)})`);
  } catch (error) {
    await collabScreenshot(user1, "step5-user1");
    await collabScreenshot(user2, "step5-user2");
    record(5, false, error instanceof Error ? error.message : String(error));
  }

  if (spaceId && artifactId) {
    try {
      await collabOpenSpace(user1, spaceId);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowBefore1 =
        (await user1
          .locator(`[data-ui-node-key="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      const rowBefore2 =
        (await user2
          .locator(`[data-ui-node-key="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      await collabRowAction(user1, "artifact", artifactId, "open");
      spaceE2eAssert(await collabWaitForEditor(user1, 120_000), "user1's writer editor never mounted, so there is no document to check in");
      const checkinButton = await collabOpenCheckin(user1);
      const checkpointBefore = await collabCurrentCheckpointId(user1);
      await checkinButton.click();
      const message = `collab check-in ${Date.now()}`;
      await user1.locator('[id="s-checkin-message"]').fill(message);
      const checkedInMinute = Math.floor(Date.now() / 60_000);
      await user1.locator('[id="s-checkin-message"]').press("Enter");
      const status = await collabAwaitCheckinStatus(user1, 60_000);
      spaceE2eAssert(status === COLLAB_E2E_LOCALES[locale].checkedIn, `the hub Check In ended as ${JSON.stringify(status)}, not ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}`);
      const checkpointAfter = await collabCurrentCheckpointId(user1);
      spaceE2eAssert(checkpointAfter !== null && checkpointAfter !== checkpointBefore, `the active checkpoint did not move (${checkpointBefore} → ${checkpointAfter})`);
      await collabOpenSpace(user1, spaceId);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowAfter1 = await collabAwaitRowUpdatedMinute(user1, artifactId, checkedInMinute, 30_000);
      spaceE2eAssert(rowAfter1.minute !== null && rowAfter1.minute >= checkedInMinute, `user1's space table row for ${artifactId} does not show the check-in's minute (before: ${JSON.stringify(rowBefore1)}, after: ${JSON.stringify(rowAfter1.text)})`);
      await collabOpenSpace(user2, spaceId);
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      const rowAfter2 = await collabAwaitRowUpdatedMinute(user2, artifactId, checkedInMinute, 30_000);
      spaceE2eAssert(rowAfter2.minute !== null && rowAfter2.minute >= checkedInMinute, `user2's space table row for ${artifactId} does not show user1's check-in minute (before: ${JSON.stringify(rowBefore2)}, after: ${JSON.stringify(rowAfter2.text)})`);
      record(6, true, `check-in "${message}" reads ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}, checkpoint moved, both users' space table rows show the check-in minute`);
    } catch (error) {
      await collabScreenshot(user1, "step6-user1");
      await collabScreenshot(user2, "step6-user2");
      record(6, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(6, false, "skipped — no space/artifact id from earlier steps");
  }

  const capability = adminCapability();
  if (capability === "") record(7, null, "skipped — this run joined an external hub without an admin-relay capability");
  else try {
    if (spaceId && artifactId) {
      await collabOpenSpace(user1, spaceId);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      await collabRowAction(user1, "artifact", artifactId, "open");
      spaceE2eAssert(await collabWaitForEditor(user1, 120_000), "user1's writer editor never re-mounted after STEP 6, so user1 holds no document connection to list");
    }
    const [user1Id, user2Id] = [await collabHubUserId(user1), await collabHubUserId(user2)];
    let text = "";
    for (const deadline = Date.now() + 30_000; ; ) {
      const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${capability}` } });
      spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
      text = JSON.stringify(await connectionsRes.json());
      if ((user1Id !== null && text.includes(user1Id) && user2Id !== null && text.includes(user2Id)) || Date.now() >= deadline) break;
      await user1.waitForTimeout(1_000);
    }
    spaceE2eAssert(user1Id !== null && text.includes(user1Id), `/admin/api/connections does not name user1's hub user ${JSON.stringify(user1Id)}: ${text.slice(0, 500)}`);
    spaceE2eAssert(user2Id !== null && text.includes(user2Id), `/admin/api/connections does not name user2's hub user ${JSON.stringify(user2Id)}: ${text.slice(0, 500)}`);
    const adminRes = await fetch(`${hubBaseUrl}/admin`, { headers: { authorization: `Bearer ${capability}` } });
    spaceE2eAssert(adminRes.ok, `GET /admin returned ${adminRes.status}`);
    const contentType = adminRes.headers.get("content-type") ?? "";
    spaceE2eAssert(contentType.includes("html"), `GET /admin content-type is ${contentType}, expected html`);
    record(
      7,
      true,
      "/admin/api/connections names both humans' hub user ids; /admin returns HTML — note: /admin is a client-rendered SPA shell, so the raw HTML byte stream itself does not literally embed the user names (verified via /admin/api/connections instead)",
    );
  } catch (error) {
    record(7, false, error instanceof Error ? error.message : String(error));
  }

  if (spaceId && artifactId) {
    try {
      await collabOpenSpace(user1, spaceId);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      await collabRowAction(user1, "artifact", artifactId, "open");
      const editor1 = collabTextEditor(user1);
      const editor2 = collabTextEditor(user2);
      await editor1.waitFor({ state: "visible", timeout: 30_000 });
      spaceE2eAssert((await editor2.count()) > 0, "user2 has no editable text surface open — STEP 8 measures a round trip between two OPEN editors (see STEP 4)");
      const marker = `r${Date.now() % 100_000}`;
      await editor1.click();
      await editor1.type(marker);
      const frames = await collabWaitForEditorText(user2, editor2, user2Commands, marker, 30_000);
      spaceE2eAssert(
        frames >= 1,
        `user2's editor showed ${JSON.stringify(marker)} without a single ServerFrame::Commands frame arriving — the two editors are not the same hub-synced document`,
      );
      spaceE2eAssert(
        frames <= marker.length,
        `user1's edit took ${frames} ServerFrame::Commands frames to reach user2 for ${marker.length} typed character(s) — more than one relay round trip per keystroke means the tail is being re-sent rather than relayed`,
      );
      record(8, true, `user1's ${marker.length}-keystroke edit reached user2 in ${frames} ServerFrame::Commands frame(s)`);
    } catch (error) {
      await collabScreenshot(user1, "step8-user1");
      await collabScreenshot(user2, "step8-user2");
      record(8, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(8, false, "skipped — no space/artifact id from earlier steps");
  }

  return { spaceId, artifactId };
}
//#endregion 🔖️CollabE2e
