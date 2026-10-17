/** 🧭️ The per-kind journeys of the two-human harness: the editor, the viewer and the cross-undo journey over one document, and the shared space seeded through the hub. */
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import type { Page } from "playwright";
import { clickUncovered, readMatrixPins, readShell } from "../../🧮️program-matrix/🟦️.ts";
import { hubProbeCall, hubProbeOpenDocument, hubProbeSignIn } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { createSpaceCommandV1 } from "@semio-tech/framework/directory/spaces";
import { type DirectorySpaceRole } from "@semio-tech/framework/directory/schema";
import { boot, clickRowAction, faultsSince, type Human, pause, type Session, short, until, waitRow } from "../🔑️sessions/🟦️.ts";
import { arrivalMs, awaitMounted, awaitText, docText, edit, hubHead, openSpace, seenText, SETTLE_MS, undo, watchText } from "../🏠️space-creation-and-edits/🟦️.ts";
import type { KindTimings, TwoHumanJourney, TwoHumanOptions } from "../📋️run-types/🟦️.ts";

//#region 🔖️Journeys
/** 🧭️ What one kind's journey gets once A created and opened the document and B opened it from the Space index. */
type JourneyContext = Readonly<{ A: Session; B: Session; check: (name: string, pass: boolean, detail: unknown) => void; time: (name: keyof KindTimings, ms: number | null) => void; plugin: string; pins: ReturnType<typeof readMatrixPins>; options: TwoHumanOptions; spaceId: string; artifactId: string; misses: unknown[] }>;

/** ✏️ Both humans author: A edits → B sees, B edits → A sees, A undoes their OWN edit (B's stays), B undoes theirs (both back
 * to the start), B redoes (both see it again), both reload and reopen and converge. */
async function editJourney({ A, B, check, time, plugin, pins, options, spaceId, artifactId, misses }: JourneyContext): Promise<void> {
    const initial = [await docText(A), await docText(B)] as const;
    const head0 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const watchB = watchText(B, 90_000);
    const aEdit = await edit(A, plugin, pins);
    const bSawA = await awaitText(B, (now) => now !== initial[1], 30_000);
    await pause(B, SETTLE_MS);
    time("aEditLocalMs", aEdit.localMs);
    time("aEditSeenByBMs", arrivalMs(await watchB.stop(), aEdit));
    check("A edits → B sees", aEdit.applied && typeof bSawA === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSawA)), hubHead: [head0, await hubHead(options.hub, options.adminCapabilityFile, artifactId)] });
    const afterA = [await docText(A), await docText(B)] as const;
    const watchA = watchText(A, 90_000);
    const bEdit = await edit(B, plugin, pins);
    const aSawB = await awaitText(A, (now) => now !== afterA[0], 30_000);
    await pause(A, SETTLE_MS);
    time("bEditLocalMs", bEdit.localMs);
    time("bEditSeenByAMs", arrivalMs(await watchA.stop(), bEdit));
    check("B edits → A sees", bEdit.applied && typeof aSawB === "string", { b: short(bEdit.after), a: short(seenText(aSawB)) });
    const afterBoth = [await docText(A), await docText(B)] as const;
    const headBeforeAUndo = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const undoCursor = [A.lines.length, B.lines.length] as const;
    const aUndo = await undo(A);
    const agreed = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === b ? a : null;
    }, 30_000);
    const headAfterAUndo = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const undoFaults = [...faultsSince(A, undoCursor[0]), ...faultsSince(B, undoCursor[1])];
    const headAdvanced = typeof headBeforeAUndo !== "number" || typeof headAfterAUndo !== "number" || headAfterAUndo > headBeforeAUndo;
    const additive = aUndo.undone;
    check("A undoes own (B's stays)", agreed !== null && agreed !== initial[0] && undoFaults.length === 0 && headAdvanced && (additive || agreed === afterBoth[0]), { kind: additive ? "additive (A's element removed)" : "set (B's later value stands)", a: short(await docText(A)), b: short(await docText(B)), faults: undoFaults.slice(0, 3), hubHead: [headBeforeAUndo, headAfterAUndo] });
    const bUndo = await undo(B);
    const backToInitial = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === initial[0] && b === initial[1] ? { a, b } : null;
    }, 30_000);
    check("B undoes own (both back to the start)", bUndo.undone && backToInitial !== null, { a: short(await docText(A)), b: short(await docText(B)), initial: initial.map(short) });
    const bRedo = await undo(B, "redo");
    const redone = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a !== initial[0] && b !== initial[1] && a === b ? { a } : null;
    }, 30_000);
    check("B redoes own (both see it again)", bRedo.undone && redone !== null, { b: short(bRedo.after), a: short(await docText(A)) });
    const settled = [await docText(A), await docText(B)] as const;
    for (const session of [A, B]) await session.page.reload({ waitUntil: "domcontentloaded" });
    for (const session of [A, B]) await boot(session);
    const [reopenedA, reopenedB] = await Promise.all(
      [A, B].map(async (session) => {
        await openSpace(session, spaceId, misses);
        await waitRow(session.page, "artifact", artifactId, 90_000);
        const openedAt = Date.now();
        await clickRowAction(session.page, "artifact", artifactId, /^(open|öffnen)\b/iu);
        return (await awaitMounted(session, options.mountBudgetMs)).mountedAt - openedAt;
      }),
    );
    time("reopenAToMountedMs", reopenedA!);
    time("reopenBToMountedMs", reopenedB!);
    const converged = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === settled[0] && b === settled[1] ? { a: short(a) } : null;
    }, 45_000);
    check("reload converges", converged !== null, { settled: settled.map(short), afterReload: [short(await docText(A)), short(await docText(B))], hubHead: await hubHead(options.hub, options.adminCapabilityFile, artifactId) });
}

/** 🏷️ The document surface a human's shell mounted (the navbar role chip's `data-app-id`, `<dialect>#editor|#viewer`), the
 * chip itself (`data-role` + its localized text), and what the shell tells the human about it: the chip's own description
 * (`title`) and the transient notices. */
const surfaceReading = (page: Page): Promise<{ surfaces: string[]; chips: string[]; notices: string[] }> =>
  page.evaluate(() => ({
    surfaces: [...new Set([...document.querySelectorAll('[data-slot="surface-role-chip"][data-app-id]')].map((element) => element.getAttribute("data-app-id") ?? "").filter((id) => id.includes("#")))],
    chips: [...document.querySelectorAll('[data-slot="surface-role-chip"]')].map((element) => `${element.getAttribute("data-role")}:${(element.textContent ?? "").trim()}`),
    notices: [...document.querySelectorAll('[data-slot="surface-role-chip"][title], [data-semio-transient-notice]')].map((element) => (element.getAttribute("title") ?? element.textContent ?? "").replace(/\s+/gu, " ").trim()).filter(Boolean),
  }));

/** 🎛️ The ids among `ids` a human can press: present, visible and neither `disabled` nor `aria-disabled`. */
const pressableControls = (page: Page, ids: readonly string[]): Promise<string[]> =>
  page.evaluate(
    (wanted) =>
      wanted.filter((id) =>
        [...document.querySelectorAll(`[id="${id}"]`)].some((element) => element instanceof HTMLElement && element.offsetParent !== null && !element.hasAttribute("disabled") && element.getAttribute("aria-disabled") !== "true"),
      ),
    [...ids],
  );

/** 👁️ Every way a viewer can try to change the document: typing and deleting in the first document window body (text
 * editors, focused canvases), the plugin's pinned verb and Undo/Redo when the pane lists them, and the undo shortcut. */
async function attemptViewerEdits(session: Session, verb: string): Promise<string[]> {
  const tried: string[] = [];
  const body = session.page.locator('[data-slot="window-body"]').last();
  const box = await body.boundingBox().catch(() => null);
  if (box !== null) {
    await session.page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    tried.push("click");
  }
  const editable = session.page.locator('[data-slot="window-body"] textarea, [data-slot="window-body"] [contenteditable="true"], [data-slot="window-body"] input:not([type="hidden"])').first();
  if ((await editable.count()) > 0) {
    await editable.focus().catch(() => undefined);
    tried.push("focus-editable");
  }
  await session.page.keyboard.type("viewer-attempt", { delay: 30 });
  await session.page.keyboard.press("Backspace");
  await session.page.keyboard.press("Delete");
  tried.push("keys");
  for (const id of [`action.${verb}`, "action.undo", "action.redo"]) {
    if ((await session.page.locator(`[data-slot="window-action-pane"] [id="${id}"]`).count()) === 0) continue;
    tried.push(`${id}:${await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="${id}"]`)}`);
    await pause(session, 600);
  }
  await session.page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  tried.push("undo-shortcut");
  return tried;
}

/** 👁️ B is a Spectator of the space: B's open lands on the kind's viewer surface and the navbar says so in B's language, B is
 * offered no edit control (neither the kind's verbs nor Undo/Redo nor the navbar's switch to the editor), every edit B attempts leaves both views and the hub head unchanged, B still sees A's edit live,
 * and a write crafted with B's own credential straight onto the document socket is refused by the hub (the viewer's plan
 * grants no write) while A's view stays put. */
async function viewerJourney({ A, B, check, plugin, pins, options, spaceId, artifactId }: JourneyContext): Promise<void> {
  const verb = pins.pluginVerbs[plugin] ?? "";
  const chipText = options.locale === "de" ? "Betrachter" : "Viewer";
  const readingB = await surfaceReading(B.page);
  check("B holds the viewer surface", readingB.surfaces.length > 0 && readingB.surfaces.every((id) => id.endsWith("#viewer")) && readingB.chips.includes(`viewer:${chipText}`), { b: readingB, a: (await surfaceReading(A.page)).surfaces });
  const offered = await pressableControls(B.page, [`action.${verb}`, "action.undo", "action.redo", "playground.navbar.roles.editor"]);
  check("viewer offers no edit control", offered.length === 0, { offered, actions: (await readShell(B.page)).actions.slice(0, 16) });
  const before = [await docText(A), await docText(B)] as const;
  const head0 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const cursor = B.lines.length;
  const tried = await attemptViewerEdits(B, verb);
  await pause(B, 3_000);
  const after = [await docText(A), await docText(B)] as const;
  const head1 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const notices = (await surfaceReading(B.page)).notices;
  check("viewer edit attempts change nothing", after[0] === before[0] && after[1] === before[1] && head1 === head0, { tried, a: [short(before[0]), short(after[0])], b: [short(before[1]), short(after[1])], hubHead: [head0, head1], faults: faultsSince(B, cursor).slice(0, 4) });
  check("viewer is told why, in its language", readingB.chips.includes(`viewer:${chipText}`) && notices.some((text) => (options.locale === "de" ? /schreibgeschützt/u : /read-only/iu).test(text)), { chips: readingB.chips, notices, offered });
  const aEdit = await edit(A, plugin, pins);
  const bSaw = await awaitText(B, (now) => now !== after[1], 30_000);
  check("A edits → viewer sees it live", aEdit.applied && typeof bSaw === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSaw)) });
  const settled = [await docText(A), await docText(B)] as const;
  const head2 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const crafted = await hubProbeSignIn(options.hub, B.human.email, B.human.password, "twohumanviewer")
    .then((token) => hubProbeOpenDocument(options.hub, token, spaceId, artifactId, `two-human-viewer-${Date.now()}`))
    .then(async (socket) => {
      const answered = await socket.submit(0, "");
      socket.close();
      return { role: String(socket.plan?.surface?.role), write: Boolean(socket.plan?.grant?.write), accepted: answered.accepted, stages: JSON.stringify(answered.ack.stages).slice(0, 300) };
    })
    .catch((error: unknown) => ({ role: "unopened", write: false, accepted: false, stages: String(error instanceof Error ? error.message : error).slice(0, 300) }));
  await pause(A, 3_000);
  const head3 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const aAfterCraft = await docText(A);
  check("hub refuses a write crafted with the viewer's credential", crafted.role === "viewer" && !crafted.write && !crafted.accepted && head3 === head2 && aAfterCraft === settled[0], { crafted, hubHead: [head2, head3], a: short(aAfterCraft) });
}

/** 🪪️ The code the hub's event log refuses a `Revert`/`Reinstate` with when it names another actor's operation (kernel-db
 * `FOREIGN_HISTORY_TRANSITION_CODE`). */
const FOREIGN_HISTORY_TRANSITION_CODE = "history.foreign-transition";

/** 🧾️ A refused Ack's reason and its encoded `messages`, as text — the refusal's code is a plain string inside them. */
function rejectionText(ack: any): string {
  const rejected = (ack?.stages ?? []).map((stage: any) => stage?.Applied?.outcome?.Rejected).find((outcome: any) => outcome !== undefined);
  if (!rejected) return "";
  const messages = Array.isArray(rejected.messages) || rejected.messages instanceof Uint8Array ? new TextDecoder().decode(Uint8Array.from(rejected.messages)) : "";
  return `${String(rejected.reason ?? "")} ${messages}`;
}

/** ⏪️ A `Revert` history transition naming `mutationIds`, shaped the way a replica sends one (`semio.history.transition`:
 * tag 0, varint count, varint-length UTF-8 ids) — what another actor would have to send to undo someone else's operations.
 * @see ../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs */
function revertTransitionEnvelope(documentId: string, mutationIds: readonly string[]): Record<string, unknown> {
  const payload: number[] = [0];
  const varint = (value: number): void => {
    let rest = value;
    while (rest >= 0x80) {
      payload.push((rest & 0x7f) | 0x80);
      rest >>>= 7;
    }
    payload.push(rest);
  };
  varint(mutationIds.length);
  for (const id of mutationIds) {
    const utf8 = new TextEncoder().encode(id);
    varint(utf8.length);
    payload.push(...utf8);
  }
  return { mutation_id: `transition-crafted-${crypto.randomUUID()}`, document_id: documentId, actor: "", dependencies: [...mutationIds], observed: null, target: [], diff: { schema: "semio.history.transition", payload }, inverse: { schema: "semio.history.transition", payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 }, transaction: null, verb: null };
}

/** 🪞️ B's edit arguments where the pinned verb would otherwise make B's element indistinguishable from A's (a default text
 * block next to a default text block renders the same text), so the views can tell WHOSE edit an undo withdrew. */
const CROSS_UNDO_DISTINCT_ARGS: Readonly<Record<string, Readonly<Record<string, string>>>> = { note: { kind: "table" }, draw: { kind: "shape:ellipse" } };

/** ⏪️ Undo and redo across two authors (row 3.11): each human's undo withdraws only their own newest edit and redo restores
 * only their own — B's undo takes B's edit back and a second one leaves A's edit in both views (it may still withdraw an
 * invisible edit of B's own, so the hub head is recorded, not judged); A's undo under B's later edit
 * keeps B's edit (the later edits replay on the state without A's, a later value of the same field stands) — both views
 * converge after every step and the hub head advances with every committed transition; and a `Revert` crafted by another
 * actor that names A's operations is refused by the hub's event log (`history.foreign-transition`), never relayed, and changes
 * nothing on either view (an undo belongs to its author) — B watches A's operation ids on one probe socket from the start and
 * sends the crafted `Revert` on a socket opened for it. */
async function crossUndoJourney({ A, B, check, plugin, pins, options, spaceId, artifactId }: JourneyContext): Promise<void> {
  const crafted = await hubProbeSignIn(options.hub, B.human.email, B.human.password, "twohumanundo").then((token) => hubProbeOpenDocument(options.hub, token, spaceId, artifactId, `two-human-undo-${Date.now()}`));
  const head = (): Promise<number | string | null> => hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const both = async (): Promise<readonly [string, string]> => [await docText(A), await docText(B)] as const;
  const agreeOn = (expected: readonly [string, string]) => until(async () => {
    const [a, b] = await both();
    return a === expected[0] && b === expected[1] ? a : null;
  }, 30_000);
  const advanced = (before: number | string | null, after: number | string | null): boolean => typeof before !== "number" || typeof after !== "number" || after > before;
  try {
    const initial = await both();
    const relayedBeforeA = crafted.relayedEnvelopes().length;
    const aEdit = await edit(A, plugin, pins);
    const bSawA = await awaitText(B, (now) => now !== initial[1], 30_000);
    check("A edits → B sees", aEdit.applied && typeof bSawA === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSawA)) });
    await pause(A, 2_000);
    const aOperations = crafted.relayedEnvelopes().slice(relayedBeforeA).filter((envelope) => envelope?.diff?.schema !== "semio.history.transition").map((envelope) => String(envelope.mutation_id));
    const afterA = await both();
    const bEdit = await edit(B, plugin, pins, CROSS_UNDO_DISTINCT_ARGS[plugin] ?? {});
    const aSawB = await awaitText(A, (now) => now !== afterA[0], 30_000);
    check("B edits → A sees", bEdit.applied && typeof aSawB === "string", { b: short(bEdit.after), a: short(seenText(aSawB)) });
    const afterBoth = await both();
    const cursor = [A.lines.length, B.lines.length] as const;
    const head0 = await head();
    const bUndo = await undo(B);
    const backToA = await agreeOn(afterA);
    const head1 = await head();
    check("B's undo withdraws only B's edit", bUndo.undone && backToA !== null && advanced(head0, head1), { a: short(await docText(A)), b: short(await docText(B)), expected: afterA.map(short), hubHead: [head0, head1] });
    const bUndoAgain = await undo(B);
    const stillA = await both();
    const head2 = await head();
    check("B's next undo leaves A's edit alone", !bUndoAgain.undone && stillA[0] === afterA[0] && stillA[1] === afterA[1], { a: short(stillA[0]), b: short(stillA[1]), hubHead: [head1, head2], faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])].slice(0, 3) });
    const bRedo = await undo(B, "redo");
    const redone = await agreeOn(afterBoth);
    check("B's redo restores only B's edit", bRedo.undone && redone !== null, { a: short(await docText(A)), b: short(await docText(B)), expected: afterBoth.map(short) });
    const head3 = await head();
    await undo(A);
    const agreed = await until(async () => {
      const [a, b] = await both();
      return a === b && a !== afterBoth[0] ? a : a === b && a === afterBoth[0] && advanced(head3, await head()) ? a : null;
    }, 30_000);
    const head4 = await head();
    check("A's undo under B's later edit keeps B's edit", agreed !== null && agreed !== initial[0] && agreed !== afterA[0] && advanced(head3, head4), { kind: agreed === afterBoth[0] ? "set (B's later value stands)" : "additive (A's element withdrawn, B's replayed)", a: short(await docText(A)), b: short(await docText(B)), hubHead: [head3, head4] });
    await undo(A, "redo");
    const aRedone = await agreeOn(afterBoth);
    check("A's redo restores A's edit under B's", aRedone !== null, { a: short(await docText(A)), b: short(await docText(B)), expected: afterBoth.map(short) });
    const head5 = await head();
    const craftedRevert = revertTransitionEnvelope(artifactId, aOperations);
    const answered: { readonly accepted: boolean; readonly ack: any; readonly unanswered?: string; readonly socketClosed?: { readonly code: number; readonly clean: boolean; readonly reason: string } | null } | null =
      aOperations.length === 0
        ? null
        : await hubProbeSignIn(options.hub, B.human.email, B.human.password, "twohumanundo")
            .then((token) => hubProbeOpenDocument(options.hub, token, spaceId, artifactId, `two-human-undo-submit-${Date.now()}`))
            .then(async (submitter) => {
              try {
                return await submitter.submitEnvelopes(1, [craftedRevert]).catch(async (error: unknown) => ({ accepted: false, ack: null, unanswered: String(error instanceof Error ? error.message : error).slice(0, 80), socketClosed: await submitter.ended(5_000) }));
              } finally {
                submitter.close();
              }
            });
    await pause(A, 6_000);
    const afterCraft = await both();
    const persisted = crafted.relayedEnvelopes().some((envelope) => String(envelope?.mutation_id) === craftedRevert.mutation_id);
    const refusal = rejectionText(answered?.ack);
    check("the hub refuses another actor's crafted undo of A's edit, and nothing changes", aOperations.length > 0 && answered !== null && answered.ack !== null && !answered.accepted && refusal.includes(FOREIGN_HISTORY_TRANSITION_CODE) && !persisted && afterCraft[0] === afterBoth[0] && afterCraft[1] === afterBoth[1], { aOperations, hubAccepted: answered?.accepted ?? null, refusal: refusal.slice(0, 240), persisted, unanswered: answered?.unanswered === undefined ? null : { error: answered.unanswered, socketClosed: answered.socketClosed ?? null }, a: short(afterCraft[0]), b: short(afterCraft[1]), expected: afterBoth.map(short), hubHead: [head5, await head()] });
  } finally {
    crafted.close();
  }
}

/** 🌱️ Creates a public studio as `owner` and makes `memberEmail` a member with `role`, through the same sealed directory
 * commands the Home dialogs post (a journey whose subject is not the space dialogs does not depend on them). */
export async function seedSharedSpaceThroughHub(hub: string, owner: Human, memberEmail: string, role: DirectorySpaceRole, name: string): Promise<string> {
  const token = await hubProbeSignIn(hub, owner.email, owner.password, "twohumanseed");
  const command = (body: Parameters<typeof sealDirectoryCommandRequestV1>[1]) => hubProbeCall(hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(crypto.randomUUID().replaceAll("-", ""), body)));
  const created = await command(createSpaceCommandV1(name, "studio", "public"));
  const spaceId = created.json?.events?.find((event: { body?: { kind?: string } }) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`seed create-space ${created.status} ${created.text.slice(0, 200)}`);
  const shared = await command({ kind: "upsert-member", spaceId, email: memberEmail, role });
  if (shared.status !== 202) throw new Error(`seed upsert-member ${shared.status} ${shared.text.slice(0, 200)}`);
  return spaceId;
}

/** 🗺️ The journeys a run can drive: how the shared space is set up (the Home dialogs, or the hub's directory commands), the
 * role B is invited with, and how many checks a kind passes with. */
export const JOURNEYS: Readonly<Record<TwoHumanJourney, Readonly<{ run: (context: JourneyContext) => Promise<void>; seed: "ui" | "hub"; shareRole: RegExp; memberRole: DirectorySpaceRole; checks: number; check: string }>>> = {
  edit: { run: editJourney, seed: "ui", shareRole: /author|autor/iu, memberRole: "author", checks: 10, check: "two-human" },
  viewer: { run: viewerJourney, seed: "hub", shareRole: /spectator|betrachter/iu, memberRole: "spectator", checks: 10, check: "two-human-viewer" },
  "cross-undo": { run: crossUndoJourney, seed: "hub", shareRole: /author|autor/iu, memberRole: "author", checks: 12, check: "two-human-cross-undo" },
};
//#endregion 🔖️Journeys
