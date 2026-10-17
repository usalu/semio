/** 👥️ Collaboration e2e presence and peer cursors: the presence rows, roster and colors, the peer cursors and the cursor and caret movement assertions. */
import { spaceE2eAssert } from "@semio-tech/framework-os-dev/testing/studio";
import { collabTextEditor } from "../🖱️ui-driving-and-row-waits/🟦️.ts";

//#region 🔖️CollabE2e
//#region 🔖️CollabE2eDom
/** 👥️ One shell's presence roster: the `peer:` rows `PresenceBar` paints inside `#s-presence-peers`,
 * with the `peer:overflow` chip excluded — it is a "+N" count, not a peer. */
function collabPresenceRows(page: import("playwright").Page): import("playwright").Locator {
  return page.locator('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])');
}

/** ⏳️ Polls one shell until its presence roster holds `expected` peers, returning the count it saw.
 * Presence is heartbeat-driven, so a single read right after the editors open races the first beat;
 * the previous one-shot `count()` is why this step could report "1 peer" on a shell that was about to
 * show two. Returns the LAST count on timeout so the failure names what the roster actually held. */
export async function collabWaitForPresenceRoster(page: import("playwright").Page, expected: number, deadlineMs: number): Promise<number> {
  const deadline = Date.now() + deadlineMs;
  let seen = -1;
  while (Date.now() < deadline) {
    seen = await collabPresenceRows(page).count();
    if (seen === expected) return seen;
    await page.waitForTimeout(500);
  }
  return seen;
}

/** 🎨️ The RESOLVED avatar border colour of every peer row in one shell, in DOM order. `PresenceBar`
 * paints the hub-assigned session colour (`ServerFrame::Session.color`, stamped onto every outbound
 * presence beat by the backbone worker's `sessionColor`) as an inline `borderColor` on each peer's
 * `TableAvatar`, and the first twelve palette slots are CSS custom properties — so this has to read
 * the computed colour, or two different slots would both read back as the same `var(--…)` text and the
 * distinctness assertion would be vacuous. */
export async function collabPresenceColors(page: import("playwright").Page): Promise<readonly string[]> {
  return await collabPresenceRows(page).evaluateAll((rows) => rows.map((row) => getComputedStyle((row.firstElementChild as HTMLElement | null) ?? (row as HTMLElement)).borderTopColor));
}

/** Peer-cursor overlay markers painted by CanvasPresenceOverlayV1 (`data-testid="peer-cursor"`). */
function collabPeerCursors(page: import("playwright").Page): import("playwright").Locator {
  return page.locator("[data-peer-cursor], [data-testid=\"peer-cursor\"]");
}

/** Wait until at least one peer cursor marker is painted, returning its CSS left/top. */
async function collabWaitForPeerCursor(page: import("playwright").Page, deadlineMs: number): Promise<{ left: number; top: number; count: number }> {
  const deadline = Date.now() + deadlineMs;
  let last = { left: 0, top: 0, count: 0 };
  while (Date.now() < deadline) {
    const cursors = collabPeerCursors(page);
    const count = await cursors.count();
    if (count > 0) {
      const box = await cursors.first().boundingBox();
      if (box) {
        last = { left: box.x, top: box.y, count };
        return last;
      }
    }
    await page.waitForTimeout(200);
  }
  return last;
}

/** Move the local pointer and assert the peer's overlay cursor relocates. */
export async function collabAssertPeerCursorMoves(opts: {
  readonly mover: import("playwright").Page;
  readonly observer: import("playwright").Page;
  readonly host: string;
  readonly label: string;
}): Promise<string> {
  const host = opts.mover.locator(opts.host).first();
  spaceE2eAssert((await host.count()) > 0, `${opts.label}: no host surface matching ${opts.host}`);
  const box = await host.boundingBox();
  spaceE2eAssert(!!box, `${opts.label}: host surface has no bounding box`);
  await opts.mover.mouse.move(box!.x + box!.width * 0.3, box!.y + box!.height * 0.3);
  const first = await collabWaitForPeerCursor(opts.observer, 20_000);
  spaceE2eAssert(first.count > 0, `${opts.label}: observer never painted [data-peer-cursor] after peer pointer move`);
  await opts.mover.mouse.move(box!.x + box!.width * 0.7, box!.y + box!.height * 0.7);
  const deadline = Date.now() + 20_000;
  let moved = first;
  while (Date.now() < deadline) {
    moved = await collabWaitForPeerCursor(opts.observer, 500);
    if (moved.count > 0 && Math.hypot(moved.left - first.left, moved.top - first.top) > 4) break;
  }
  spaceE2eAssert(
    moved.count > 0 && Math.hypot(moved.left - first.left, moved.top - first.top) > 4,
    `${opts.label}: peer cursor did not move while painted (before=${JSON.stringify(first)}, after=${JSON.stringify(moved)})`,
  );
  return `${opts.label}: peer-cursor moved from (${first.left.toFixed(1)},${first.top.toFixed(1)}) to (${moved.left.toFixed(1)},${moved.top.toFixed(1)})`;
}

/** ✏️ A text peer's marker is its CARET, not its pointer (`TextPeerCaretsOverlayV1`): the mover puts its caret at the start
 * and then the end of its first text line, and the observer's painted caret for that peer must move right. */
export async function collabAssertPeerCaretMoves(opts: { readonly mover: import("playwright").Page; readonly observer: import("playwright").Page; readonly label: string }): Promise<string> {
  const sink = collabTextEditor(opts.mover);
  spaceE2eAssert((await sink.count()) > 0, `${opts.label}: the mover has no text editor open`);
  const caret = async (): Promise<{ readonly x: number; readonly count: number }> => {
    const marks = opts.observer.locator("[data-peer-caret]");
    const count = await marks.count();
    return { x: count > 0 ? ((await marks.first().boundingBox())?.x ?? Number.NaN) : Number.NaN, count };
  };
  const settle = async (moved: (value: { readonly x: number; readonly count: number }) => boolean): Promise<{ readonly x: number; readonly count: number }> => {
    const deadline = Date.now() + 20_000;
    let seen = await caret();
    while (Date.now() < deadline && !moved(seen)) {
      await opts.observer.waitForTimeout(200);
      seen = await caret();
    }
    return seen;
  };
  await sink.focus();
  await opts.mover.keyboard.press("Home");
  const start = await settle((value) => value.count > 0);
  spaceE2eAssert(start.count > 0, `${opts.label}: the observer never painted the mover's caret`);
  await opts.mover.keyboard.press("End");
  const end = await settle((value) => value.count > 0 && value.x > start.x + 4);
  spaceE2eAssert(end.count > 0 && end.x > start.x + 4, `${opts.label}: the peer caret did not follow Home → End (${JSON.stringify(start)} → ${JSON.stringify(end)})`);
  return `${opts.label}: peer caret moved from x=${start.x.toFixed(1)} to x=${end.x.toFixed(1)}`;
}
//#endregion 🔖️CollabE2eDom
//#endregion 🔖️CollabE2e
