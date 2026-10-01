//#region ✂️TextSplice
/** ✂️ One range-text operation as its author saw it: at scalar `start` of the author's text, `deleted` was replaced by
 * `insert`; `before`/`after` are up to {@link TEXT_SPLICE_CONTEXT_SCALARS} scalars the author saw around the replaced run.
 * Positions and lengths count Unicode scalar values (Rust `char`s), the same in every language.
 * @see ../🧬️schema/✂️text-splice/🔣️.json */
export type TextSpliceV1 = { readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };

/** 📍️ Where a splice lands in a concrete text: the run it replaces there, and whether its `deleted` run was already gone
 * (`clamped`: nothing is deleted, the insert still lands) or its context was nowhere to be found. */
export type LocatedTextSpliceV1 = { readonly start: number; readonly deleteLength: number; readonly clamped: boolean };

/** 🧾️ A splice applied to a text: the new text, where it landed, and the inverse splice that restores the text. */
export type AppliedTextSpliceV1 = { readonly text: string; readonly located: LocatedTextSpliceV1; readonly inverse: TextSpliceV1 };

/** 📏️ Context scalars a splice carries on each side. */
export const TEXT_SPLICE_CONTEXT_SCALARS = 32;

/** 🔗️ Two-sided context patterns shorter than this many scalars per side are never searched: two scalars of context
 * match all over a real document, one-sided context of the same length near the author's position is the better witness. */
export const TEXT_SPLICE_MIN_TWO_SIDED_SCALARS = 4;

const scalars = (text: string): string[] => Array.from(text);

/** 🔎️ Every start index of `needle` in `hay` (scalar arrays, overlapping). */
function occurrencesV1(hay: readonly string[], needle: readonly string[]): number[] {
  const found: number[] = [];
  if (needle.length === 0) {
    for (let index = 0; index <= hay.length; index += 1) found.push(index);
    return found;
  }
  for (let index = 0; index + needle.length <= hay.length; index += 1) {
    let match = true;
    for (let offset = 0; offset < needle.length && match; offset += 1) match = hay[index + offset] === needle[offset];
    if (match) found.push(index);
  }
  return found;
}

/** 🎯️ The candidate position nearest to `expected`; ties take the lowest position. */
function nearestV1(positions: readonly number[], expected: number): number | null {
  let best: number | null = null;
  for (const position of positions) {
    if (best === null || Math.abs(position - expected) < Math.abs(best - expected) || (Math.abs(position - expected) === Math.abs(best - expected) && position < best)) best = position;
  }
  return best;
}

/** 📍️ Locates `splice` in `text`, deterministically and identically in every language twin:
 * 1. the run with context on both sides, `k` = the longer side's length down to {@link TEXT_SPLICE_MIN_TWO_SIDED_SCALARS}
 *    (each side capped at its own length);
 * 2. the run with context on one side, `k` = the longer side's length down to 1 (both sides of one `k` compete);
 * 3. the run alone (only when it is not empty);
 * 4. a run that is gone deletes nothing (`clamped`): steps 1–2 again with an empty run find the insertion point;
 * 5. otherwise the author's `start`, clamped to the text (`clamped` when the splice carried any context).
 * Every step takes the match nearest to the author's `start`, ties to the lowest position. */
export function locateTextSpliceV1(text: string, splice: TextSpliceV1): LocatedTextSpliceV1 {
  const hay = scalars(text);
  const before = scalars(splice.before), deleted = scalars(splice.deleted), after = scalars(splice.after);
  const longest = Math.max(before.length, after.length);
  const anchored = (run: readonly string[]): number | null => {
    for (let k = longest; k >= TEXT_SPLICE_MIN_TWO_SIDED_SCALARS; k -= 1) {
      const head = before.slice(before.length - Math.min(k, before.length)), tail = after.slice(0, Math.min(k, after.length));
      if (head.length === 0 || tail.length === 0) break;
      const found = nearestV1(occurrencesV1(hay, [...head, ...run, ...tail]).map((index) => index + head.length), splice.start);
      if (found !== null) return found;
    }
    for (let k = longest; k >= 1; k -= 1) {
      const head = before.slice(before.length - Math.min(k, before.length)), tail = after.slice(0, Math.min(k, after.length));
      const candidates = [...(head.length > 0 ? occurrencesV1(hay, [...head, ...run]).map((index) => index + head.length) : []), ...(tail.length > 0 ? occurrencesV1(hay, [...run, ...tail]) : [])];
      const found = nearestV1(candidates, splice.start);
      if (found !== null) return found;
    }
    return null;
  };
  const exact = anchored(deleted) ?? (deleted.length > 0 ? nearestV1(occurrencesV1(hay, deleted), splice.start) : null);
  if (exact !== null) return { start: exact, deleteLength: deleted.length, clamped: false };
  const insertion = deleted.length > 0 ? anchored([]) : null;
  if (insertion !== null) return { start: insertion, deleteLength: 0, clamped: true };
  return { start: Math.min(Math.max(splice.start, 0), hay.length), deleteLength: 0, clamped: deleted.length > 0 || before.length > 0 || after.length > 0 };
}

/** ✂️ Applies `splice` to `text` where it locates, answering the new text and the inverse splice (the removed run back
 * in place of the inserted one, with the context of the new text). */
export function applyTextSpliceV1(text: string, splice: TextSpliceV1, context = TEXT_SPLICE_CONTEXT_SCALARS): AppliedTextSpliceV1 {
  const located = locateTextSpliceV1(text, splice);
  const hay = scalars(text), insert = scalars(splice.insert);
  const removed = hay.slice(located.start, located.start + located.deleteLength).join("");
  const next = [...hay.slice(0, located.start), ...insert, ...hay.slice(located.start + located.deleteLength)];
  const end = located.start + insert.length;
  return {
    text: next.join(""),
    located,
    inverse: { start: located.start, deleted: splice.insert, insert: removed, before: next.slice(Math.max(0, located.start - context), located.start).join(""), after: next.slice(end, end + context).join("") },
  };
}

/** ⌨️ The ONE splice an editor sends for its own change `previous → next`: the common scalar prefix and suffix bound the
 * changed run, the context is what the author saw around it; `null` when nothing changed. */
export function textSpliceFromEditV1(previous: string, next: string, context = TEXT_SPLICE_CONTEXT_SCALARS): TextSpliceV1 | null {
  const a = scalars(previous), b = scalars(next);
  let prefix = 0;
  while (prefix < a.length && prefix < b.length && a[prefix] === b[prefix]) prefix += 1;
  let suffix = 0;
  while (suffix < a.length - prefix && suffix < b.length - prefix && a[a.length - 1 - suffix] === b[b.length - 1 - suffix]) suffix += 1;
  if (prefix === a.length && prefix === b.length) return null;
  return {
    start: prefix,
    deleted: a.slice(prefix, a.length - suffix).join(""),
    insert: b.slice(prefix, b.length - suffix).join(""),
    before: a.slice(Math.max(0, prefix - context), prefix).join(""),
    after: a.slice(a.length - suffix, Math.min(a.length, a.length - suffix + context)).join(""),
  };
}

/** 🧷️ A zero-width splice marking scalar `position` of `text` — how a caret or a selection end travels through other
 * authors' splices: locating the marker in the new text is the position's new place. */
export function textMarkerV1(text: string, position: number, context = TEXT_SPLICE_CONTEXT_SCALARS): TextSpliceV1 {
  const hay = scalars(text);
  const at = Math.min(Math.max(position, 0), hay.length);
  return { start: at, deleted: "", insert: "", before: hay.slice(Math.max(0, at - context), at).join(""), after: hay.slice(at, at + context).join("") };
}

/** 🔗️ How a typed splice joins its run ({@link composeTextSplicesV1}): one net splice, a run that changed nothing, or a caret
 * that jumped away (the run ends and a new one begins). */
export type TextSpliceCompositionV1 = { readonly kind: "composed"; readonly splice: TextSpliceV1 } | { readonly kind: "cancelled" } | { readonly kind: "disjoint" };

/** 🔗️ The ONE splice of a typing run (twin of Rust `TextSplice::then`): `net` (the run so far) took the author's text `T0` to
 * `T1`, `next` (an edit of `T1`, same author, same coordinates) takes `T1` to `T2`. `next` must lie inside the run's window
 * `before + insert + after` of `T1` and agree with it wherever both carry text, and the two changes must touch once each is placed
 * anywhere it may equivalently sit among repeated scalars. The run then grows to the union of both changes, untrimmed — a scalar the
 * run deleted and typed again stays inside it; `cancelled` when the run no longer changes anything, `disjoint` when the caret
 * jumped away. Placements are tried nearest first, left before right. */
export function composeTextSplicesV1(net: TextSpliceV1, next: TextSpliceV1, context = TEXT_SPLICE_CONTEXT_SCALARS): TextSpliceCompositionV1 {
  const before = scalars(net.before), deleted = scalars(net.deleted), insert = scalars(net.insert), after = scalars(net.after);
  const nextBefore = scalars(next.before), nextDeleted = scalars(next.deleted), nextInsert = scalars(next.insert), nextAfter = scalars(next.after);
  const window = [...before, ...insert, ...after];
  const origin = net.start - before.length;
  const at = next.start - origin;
  const end = at + nextDeleted.length;
  if (at < 0 || end > window.length) return { kind: "disjoint" };
  const head = window.slice(0, at), tail = window.slice(end);
  const seenBefore = Math.min(nextBefore.length, head.length), seenAfter = Math.min(nextAfter.length, tail.length);
  const same = (a: readonly string[], b: readonly string[]) => a.length === b.length && a.every((scalar, index) => scalar === b[index]);
  if (!same(window.slice(at, end), nextDeleted) || !same(head.slice(head.length - seenBefore), nextBefore.slice(nextBefore.length - seenBefore)) || !same(tail.slice(0, seenAfter), nextAfter.slice(0, seenAfter))) return { kind: "disjoint" };
  const runs = textSplicePlacementsV1(window, before.length, insert, deleted);
  const nexts = textSplicePlacementsV1(window, at, nextDeleted, nextInsert);
  let pair: readonly [TextSplicePlacementV1, TextSplicePlacementV1] | undefined;
  for (const run of runs) {
    const candidate = nexts.find((placement) => placement.at <= run.at + insert.length && placement.at + nextDeleted.length >= run.at);
    if (candidate !== undefined) {
      pair = [run, candidate];
      break;
    }
  }
  if (pair === undefined) return { kind: "disjoint" };
  const [run, typing] = pair;
  const original = [...window.slice(0, run.at), ...run.outside, ...window.slice(run.at + insert.length)];
  const typed = [...window.slice(0, typing.at), ...typing.outside, ...window.slice(typing.at + nextDeleted.length)];
  if (same(original, typed)) return { kind: "cancelled" };
  const lo = Math.min(run.at, typing.at), hi = Math.max(run.at + insert.length, typing.at + nextDeleted.length);
  const originalHi = hi - insert.length + deleted.length, typedHi = hi - nextDeleted.length + nextInsert.length;
  const leading = [...nextBefore.slice(0, nextBefore.length - seenBefore), ...window.slice(0, lo)];
  const trailing = [...original.slice(originalHi), ...nextAfter.slice(seenAfter)];
  return {
    kind: "composed",
    splice: { start: origin + lo, deleted: original.slice(lo, originalHi).join(""), insert: typed.slice(lo, typedHi).join(""), before: leading.slice(Math.max(0, leading.length - context)).join(""), after: trailing.slice(0, context).join("") },
  };
}

/** 🪜️ One equivalent placement of a change inside a window: where its `T1` side starts, that side and its other side. */
type TextSplicePlacementV1 = { readonly at: number; readonly inside: readonly string[]; readonly outside: readonly string[] };

/** 🪜️ Every equivalent placement of one change inside `window` (`T1`), twin of Rust `placements`: shifting it by one scalar `c`
 * is the same change while both sides end (left) or start (right) with `c`; ordered by distance, left before right. */
function textSplicePlacementsV1(window: readonly string[], at: number, inside: readonly string[], outside: readonly string[]): TextSplicePlacementV1[] {
  const shift = (placement: TextSplicePlacementV1, leftward: boolean): TextSplicePlacementV1 | undefined => {
    const scalar = leftward ? window[placement.at - 1] : window[placement.at + placement.inside.length];
    if (scalar === undefined || (placement.inside.length === 0 && placement.outside.length === 0)) return undefined;
    const matches = (side: readonly string[]) => side.length === 0 || (leftward ? side.at(-1) : side[0]) === scalar;
    if (!matches(placement.inside) || !matches(placement.outside)) return undefined;
    const rotate = (side: readonly string[]) => (side.length === 0 ? [] : leftward ? [scalar, ...side.slice(0, -1)] : [...side.slice(1), scalar]);
    return { at: placement.at + (leftward ? -1 : 1), inside: rotate(placement.inside), outside: rotate(placement.outside) };
  };
  const walk = (leftward: boolean) => {
    const found: TextSplicePlacementV1[] = [];
    for (let placement = shift({ at, inside, outside }, leftward); placement !== undefined; placement = shift(placement, leftward)) found.push(placement);
    return found;
  };
  const lefts = walk(true), rights = walk(false);
  const ordered: TextSplicePlacementV1[] = [{ at, inside, outside }];
  for (let index = 0; index < Math.max(lefts.length, rights.length); index += 1) {
    if (lefts[index] !== undefined) ordered.push(lefts[index]!);
    if (rights[index] !== undefined) ordered.push(rights[index]!);
  }
  return ordered;
}

/** 🔁️ The editor host's view after the guest published `remote`: every splice the guest has not applied yet, in the
 * order the host sent them, folded onto `remote`, and the local selection (scalar anchor/caret in `local`) carried along. */
export function rebaseTextEditsV1(remote: string, unapplied: readonly TextSpliceV1[], local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly text: string; readonly anchor: number; readonly caret: number } {
  let text = remote;
  for (const splice of unapplied) text = applyTextSpliceV1(text, splice).text;
  const place = (position: number) => locateTextSpliceV1(text, textMarkerV1(local, position)).start;
  return { text, anchor: place(selection.anchor), caret: place(selection.caret) };
}

/** 🔢️ Scalar index of UTF-8 byte offset `bytes` in `text` (the wasm editor session's offsets). */
export function scalarOfUtf8OffsetV1(text: string, bytes: number): number {
  let consumed = 0, index = 0;
  for (const scalar of text) {
    if (consumed >= bytes) return index;
    consumed += new TextEncoder().encode(scalar).length;
    index += 1;
  }
  return index;
}

/** 🔢️ UTF-8 byte offset of scalar index `index` in `text`. */
export function utf8OffsetOfScalarV1(text: string, index: number): number {
  return new TextEncoder().encode(scalars(text).slice(0, index).join("")).length;
}
//#endregion ✂️TextSplice

//#region ⌨️TextEditorTyping
/** ⌨️ How a text editor host types into one window (`settingsJson.typing`, schema `TextEditorTypingV1`): `splice` —
 * every typed run is one {@link TextSpliceV1} (`textSplice`, with the host's `seq`), and the scene's `selectionJson.splice`
 * names the last `seq` the window applied. Absent: the window takes whole-text `textEdit`. */
export type TextEditorTypingV1 = { readonly mode: "splice" };

/** ⌨️ The typing contract a scene's `settingsJson` declares, or `null`. */
export function textEditorTypingV1(settingsJson: string | undefined): TextEditorTypingV1 | null {
  if (!settingsJson) return null;
  try {
    const typing = (JSON.parse(settingsJson) as { readonly typing?: { readonly mode?: unknown } }).typing;
    return typing?.mode === "splice" ? { mode: "splice" } : null;
  } catch {
    return null;
  }
}

/** 🔢️ The last host splice `seq` a scene's `selectionJson` says its window applied (`0` when none). */
export function textEditorAppliedSpliceV1(selectionJson: string | undefined): number {
  if (!selectionJson) return 0;
  try {
    const splice = (JSON.parse(selectionJson) as { readonly splice?: unknown }).splice;
    return typeof splice === "number" && Number.isSafeInteger(splice) && splice >= 0 ? splice : 0;
  } catch {
    return 0;
  }
}

/** 🧾️ One splice-typing host's knowledge of one window (the Jupiter client model, no CRDT): `remote` — the text the
 * window last published, `base` — the text the window holds once it applied every splice this host sent (the text the next
 * splice is computed against), `seq` — the last splice sent, `acknowledged` — the last splice a scene said the window applied (a
 * scene naming a lower one comes from a reincarnated window), `unapplied` — the sent splices no scene acknowledged yet, oldest
 * first. */
export type TextEditorSpliceHostV1 = {
  readonly remote: string;
  readonly base: string;
  readonly seq: number;
  readonly acknowledged: number;
  readonly unapplied: readonly { readonly seq: number; readonly splice: TextSpliceV1 }[];
};

/** 🖼️ What a host's editor shows after a scene: the text and the scalar selection, or `null` when it already shows it. */
export type TextEditorSpliceViewV1 = { readonly text: string; readonly anchor: number; readonly caret: number } | null;

/** 🌱️ A host that shows `buffer` of a window that applied splice `applied`: its numbers continue after `applied`, so a
 * remounted host never reuses a `seq` the window already took. */
export function textEditorSpliceHostV1(buffer: string, applied: number): TextEditorSpliceHostV1 {
  return { remote: buffer, base: buffer, seq: applied, acknowledged: applied, unapplied: [] };
}

/** ⌨️ The host's editor now shows `local`: the ONE splice to send (numbered `seq`) and the host after it, or `null` when
 * the window will already hold `local`. */
export function sendTextEditorSpliceV1(host: TextEditorSpliceHostV1, local: string): { readonly host: TextEditorSpliceHostV1; readonly splice: TextSpliceV1; readonly seq: number } | null {
  const splice = textSpliceFromEditV1(host.base, local);
  if (splice === null) return null;
  const seq = host.seq + 1;
  return { host: { ...host, base: local, seq, unapplied: [...host.unapplied, { seq, splice }] }, splice, seq };
}

function textEditorSpliceShowV1(host: TextEditorSpliceHostV1, remote: string, unapplied: TextEditorSpliceHostV1["unapplied"], local: string, selection: { readonly anchor: number; readonly caret: number }, undelivered: TextSpliceV1 | null): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const base = unapplied.reduce((text, entry) => applyTextSpliceV1(text, entry.splice).text, remote);
  const view = rebaseTextEditsV1(remote, undelivered === null ? unapplied.map((entry) => entry.splice) : [...unapplied.map((entry) => entry.splice), undelivered], local, selection);
  return { host: { ...host, remote, base, unapplied }, show: view.text === local ? null : view };
}

/** 🔁️ The window published `remote` having applied this host's splices up to `applied`, while the editor shows `local` with
 * the scalar `selection`: the editor shows `remote` with every still-unapplied splice and the not-yet-sent local change folded
 * on, the selection carried along by its context — nothing when that is what it shows already (an echo of its own typing),
 * so neither a lagging echo nor a collaborator's run ever reverts or scrambles what this host typed. A scene naming a splice BELOW
 * the one the window already acknowledged comes from a reincarnated window (the document reopened, silently on a hub-ordered
 * reorder too): this host's unapplied splices never reached it, so they are dropped instead of being folded onto its text as if
 * still in flight (which duplicated them once the reopened document carried them); the numbering continues. */
export function receiveTextEditorSceneV1(host: TextEditorSpliceHostV1, remote: string, applied: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const unapplied = applied < host.acknowledged ? [] : host.unapplied.filter((entry) => entry.seq > applied);
  return textEditorSpliceShowV1({ ...host, seq: Math.max(host.seq, applied), acknowledged: applied }, remote, unapplied, local, selection, textSpliceFromEditV1(host.base, local));
}

/** ✅️ The round trip of splice `seq` settled APPLIED — the window's typed operation completed, so the window holds it and
 * every splice this host sent before it, whether or not a scene said so yet: they leave the unapplied set, so no later scene (a
 * reincarnated window's included, which may carry them from the hub) folds them onto a text that already holds them. */
export function settleTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number): TextEditorSpliceHostV1 {
  return { ...host, unapplied: host.unapplied.filter((entry) => entry.seq > seq) };
}

/** 🚫️ The window refused splice `seq`: it leaves the unapplied set and the editor shows the last published text with the
 * remaining unapplied splices — the refused run, and what was typed after it and not sent yet, disappear instead of looking
 * saved; the caret collapses where the refused run was (located by the run's own context, which is still there). A refusal of a
 * run the host no longer holds (dropped when its window reincarnated) changes nothing. */
export function refuseTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const refused = host.unapplied.find((entry) => entry.seq === seq);
  if (refused === undefined) return { host, show: null };
  const shown = textEditorSpliceShowV1(host, host.remote, host.unapplied.filter((entry) => entry.seq !== seq), local, selection, null);
  if (shown.show === null) return shown;
  const at = locateTextSpliceV1(shown.show.text, { ...refused.splice, deleted: "", insert: "" }).start;
  return { host: shown.host, show: { text: shown.show.text, anchor: at, caret: at } };
}
//#endregion ⌨️TextEditorTyping

//#region ⏱️TextEditorTypingRun
/** ⌨️ The host side of the typing-run protocol (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING; owner constants
 * `🛠️tool-machine` `TYPING_BUFFER_ARG`/`TYPING_COMMIT_ARG`/`TYPING_IDLE_MS`, pinned against the typing-law fixture by this
 * module's laws): every live delivery names the buffer it types into, a commit signal names its reason and carries no edit. */
export const TEXT_EDITOR_TYPING_BUFFER_ARG = "typing";
export const TEXT_EDITOR_TYPING_COMMIT_ARG = "typingCommit";
/** ⏱️ How long after its last delivery a host ends the run on its own (the runtime's idle lapse is the same bound). */
export const TEXT_EDITOR_TYPING_IDLE_MS = 750;

/** 🏁️ Why a host ends its typing run. */
export type TextEditorTypingCommitV1 = "idle" | "selectionJump" | "blur" | "enter" | "hidden" | "apply" | "otherVerb";

/** ⌨️ One editor's typing run as its host sees it: `typed(verb, before)` after every delivery the window took (`before` = the text
 * the delivery was computed against), `commit(reason)` to end the run (blur, page hidden, Enter in a single-line buffer; the idle
 * bound fires on its own), `closed()` when the window ended it itself (a caret move it was told about), `preview(local)` = the
 * run's pending change as ONE splice of the text it started from (the ephemeral shared preview peers render), `dispose()` ends a
 * run that is still open like a blur. Every commit is dispatched exactly once per run, through the verb that typed it. */
export type TextEditorTypingRunV1 = {
  readonly typed: (verb: string, before: string) => void;
  readonly commit: (reason: TextEditorTypingCommitV1) => void;
  readonly closed: () => void;
  readonly preview: (local: string) => TextSpliceV1 | null;
  readonly open: () => boolean;
  readonly dispose: () => void;
};

/** ⌨️ Creates one editor's typing run: `send(verb, reason)` dispatches the commit signal, `schedule(run, ms)` arms the idle bound
 * and answers its cancel (a host timer; the laws drive a fake one). */
export function createTextEditorTypingRunV1(send: (verb: string, reason: TextEditorTypingCommitV1) => void, schedule: (run: () => void, ms: number) => () => void): TextEditorTypingRunV1 {
  let run: { readonly verb: string; readonly base: string } | null = null;
  let cancel: (() => void) | null = null;
  const disarm = () => {
    cancel?.();
    cancel = null;
  };
  const commit = (reason: TextEditorTypingCommitV1) => {
    disarm();
    const ended = run;
    run = null;
    if (ended !== null) send(ended.verb, reason);
  };
  return {
    typed: (verb, before) => {
      if (run !== null && run.verb !== verb) commit("otherVerb");
      run ??= { verb, base: before };
      disarm();
      cancel = schedule(() => commit("idle"), TEXT_EDITOR_TYPING_IDLE_MS);
    },
    commit,
    closed: () => {
      disarm();
      run = null;
    },
    preview: (local) => (run === null ? null : textSpliceFromEditV1(run.base, local)),
    open: () => run !== null,
    dispose: () => commit("blur"),
  };
}
//#endregion ⏱️TextEditorTypingRun
