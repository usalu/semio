// #region 🧲️Header
/** ⏪️ The React shell's time-travel chrome (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING): the persistent band a live
 * history-edit session raises from `HistoryPatch.timeTravel` whichever panel is open — stage, edited mutation, replay
 * progress with Cancel, review status, worst outcome and the stage's own verbs — the indicator every window of the editing program
 * wears, and the remappable `ui.timeTravel.accept|discard|exit` chords. The history rows and the input editor are the
 * guest's own `framework.body.history`, rendered by the interpreter; this chrome only reads the session and dispatches
 * the reserved `historyEdit*` verbs, each stamped with the session generation it was shown, so a stale press is refused
 * `timeTravel.stale` instead of acting on a session that moved on. Every text it shows is a row of the one `labels` table
 * of the shared band corpus (`🧫️time-travel-band`), to which the i18n catalogue is pinned and which the wgpu shell embeds;
 * the person reads "History editing" everywhere — the guest's section, the band, the indicator, the peer notes.
 * @see ../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts
 * @see ../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { FRAMEWORK_PANEL_TAB_HISTORY_ID, HISTORY_EDIT_ARG_GENERATION, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_STORE, historyEntryLabelText, historyReprojectionStatus, type ActionDescriptor, type HistoryEditActionId, type HistoryEntry, type HistoryReprojection, type HistoryTimeTravel, type HistoryTimeTravelReview, type HistoryTimeTravelStage, type ShellLocale } from "@semio-tech/framework";
import type { ArtifactPresenceHistoryEdit } from "@semio-tech/framework-replication";
import { ariaKeyshortcutsText, DisabledReasonHint, findPanelTabInDock, findPanelTabPath, Icon, resolveControlKeybindingRaw, SHELL_KEYBINDINGS, Surface, useControlKeybinding, useUiKeybindingsByControlId, type PanelDock, type PanelTabNode, type PresencePeer, type UiTranslationKey } from "@semio-tech/ui-react";
import { useEffect, useId, useRef, type ReactElement } from "react";
import type { ShellAction } from "../../🐚️Shell/🟦️.tsx";
import { HISTORY_REFUSAL_LABEL_KEYS, historyRefusalCodeV1, shellLabel } from "../🟦️.tsx";
import type { UiPresenceOverlayEntry, UiPresenceOverlayValue } from "../../🗣️Interpreter/🟦️.tsx";
// #endregion 🔌️Adapters

//#region ⏪️TimeTravelChrome
/** 🎬️ The reserved `historyEdit*` verbs the chrome dispatches, by the control that dispatches them. */
export const TIME_TRAVEL_VERBS = {
  nextProblem: "historyEditBegin",
  accept: "historyEditAccept",
  discard: "historyEditDiscard",
  cancelReplay: "historyEditCancelReplay",
  rerun: "historyEditRerun",
  finalize: "historyEditFinalize",
  back: "historyEditBack",
  exit: "historyEditExit",
} as const satisfies Readonly<Record<string, HistoryEditActionId>>;

/** 🎚️ One control of {@link TIME_TRAVEL_VERBS}. */
export type TimeTravelControlV1 = keyof typeof TIME_TRAVEL_VERBS;

/** ⌨️ The remappable chord of each control that has one — never Escape, so Escape never discards a draft. */
export const TIME_TRAVEL_CHORD_IDS = {
  accept: "ui.timeTravel.accept",
  discard: "ui.timeTravel.discard",
  exit: "ui.timeTravel.exit",
} as const satisfies Partial<Record<TimeTravelControlV1, UiTranslationKey>>;

/** 🆔️ The id of each control — the one the shared band corpus owns (`controls[].controlId`), the wgpu band mirrors and
 * the React button wears as its DOM id: a control with a chord wears its keybinding id ({@link TIME_TRAVEL_CHORD_IDS}). */
export const TIME_TRAVEL_CONTROL_IDS = {
  accept: "ui.timeTravel.accept",
  discard: "ui.timeTravel.discard",
  cancelReplay: "shell.time-travel.cancel-replay",
  rerun: "shell.time-travel.rerun",
  finalize: "shell.time-travel.finalize",
  back: "shell.time-travel.back",
  exit: "ui.timeTravel.exit",
  nextProblem: "shell.time-travel.next-problem",
} as const satisfies Readonly<Record<TimeTravelControlV1, string>>;

/** 🔘️ One band control: its id, what it dispatches, its label, and why it is disabled (`null` when it is not). */
export type TimeTravelBandControlStateV1 = { readonly control: TimeTravelControlV1; readonly controlId: string; readonly label: UiTranslationKey; readonly disabledBy: UiTranslationKey | null };

const bandControl = (id: TimeTravelControlV1, disabledBy: UiTranslationKey | null = null): TimeTravelBandControlStateV1 => ({ control: id, controlId: TIME_TRAVEL_CONTROL_IDS[id], label: `ui.timeTravel.${id}`, disabledBy });

/** 🚧️ Why Finalize is disabled in a review, read from the session's own `review` and never inferred from a missing
 * report: nothing accepted is `empty`, a replay still owed or a blocking report is `blocked`, a `ready` review is
 * finalizable, and a review the session does not state is not possible right now. */
function finalizeDisabledBy(session: HistoryTimeTravel): UiTranslationKey | null {
  if (session.review === "noChanges" || (session.acceptedCount ?? 0) === 0) return "ui.timeTravel.refusal.empty";
  if (session.review === "needsReplay" || session.review === "blocked" || session.blocking === true) return "ui.timeTravel.refusal.blocked";
  return session.review === "ready" ? null : "ui.timeTravel.refusal.illegal";
}

/** 🚦️ The controls a stage offers, in band order: a draft is accepted or discarded while editing, a replay can be
 * cancelled while it runs, a review leads with Next problem while the session names the first blocking mutation
 * (`nextProblem` — the session's own answer, never computed here), can replay again when the session says it would start
 * a replay (`rerunnable`) and finalize when its review is `ready` (else both stay visible and name what stops them), the
 * finalize prompt can be left for the review, and every stage but the commit itself can exit. */
export function timeTravelBandControlsV1(session: HistoryTimeTravel): readonly TimeTravelBandControlStateV1[] {
  switch (session.stage) {
    case "editing":
      return [bandControl("accept"), bandControl("discard"), bandControl("exit")];
    case "replaying":
      return [bandControl("cancelReplay"), bandControl("exit")];
    case "reviewing":
      return [...(session.nextProblem === undefined ? [] : [bandControl("nextProblem")]), bandControl("rerun", session.rerunnable === true ? null : "ui.timeTravel.refusal.illegal"), bandControl("finalize", finalizeDisabledBy(session)), bandControl("exit")];
    case "choosing":
      return [bandControl("back"), bandControl("exit")];
    case "finalizing":
      return [];
  }
}

/** 🧱️ The reserved verbs a history row offers as its own actions: Edit (`historyEditBegin`), Withdraw and Restore — the shared
 * band corpus's `rowActions.verbs`. */
export const TIME_TRAVEL_ROW_ACTION_VERBS = ["historyEditBegin", "historyEditWithdraw", "historyEditRestore"] as const satisfies readonly HistoryEditActionId[];

/** 🚥️ Per stage, the refusal every history row action names while the stage lasts; `null` where the rows' own published
 * state rules — the shared band corpus's `rowActions.stages`. A running replay and a commit under way admit no Begin,
 * Withdraw or Restore (the session answers `timeTravel.illegal`). The shell knows the stage from the session patch at
 * once; the guest's refreshed History body, which says the same on every row, arrives only after the work (live fault F3:
 * Edit read enabled for the first 170 ms of a replay and its reason reached the page after the replay had ended). */
export const TIME_TRAVEL_ROW_ACTION_REFUSALS = { editing: null, replaying: "ui.timeTravel.refusal.illegal", reviewing: null, choosing: null, finalizing: "ui.timeTravel.refusal.illegal" } as const satisfies Readonly<Record<HistoryTimeTravel["stage"], UiTranslationKey | null>>;

/** 🛑️ The row actions the shell itself refuses for `session`, by verb, each with the reason it names in the shell's
 * current language ({@link TIME_TRAVEL_ROW_ACTION_REFUSALS}); `null` when it refuses none — no session, or a stage whose
 * rows speak for themselves. */
export function timeTravelRowActionRefusalsV1(session: Pick<HistoryTimeTravel, "stage"> | null): ReadonlyMap<string, string> | null {
  const refusal = session === null ? null : TIME_TRAVEL_ROW_ACTION_REFUSALS[session.stage];
  if (refusal === null) return null;
  const reason = String(shellLabel(refusal));
  return new Map(TIME_TRAVEL_ROW_ACTION_VERBS.map((verb) => [verb, reason]));
}

/** 🎯️ The action a control dispatches for `session` on the program `controllerId` names: its reserved verb stamped with the
 * session generation — except Next problem, which opens (`historyEditBegin`) the mutation the session's `nextProblem` names,
 * in the member store it names, exactly as the guest's own row does. */
export function timeTravelControlActionV1(controllerId: string, session: HistoryTimeTravel, id: TimeTravelControlV1): ActionDescriptor {
  if (id !== "nextProblem") return { controllerId, action: TIME_TRAVEL_VERBS[id], args: { [HISTORY_EDIT_ARG_GENERATION]: session.generation } };
  const problem = session.nextProblem;
  return { controllerId, action: TIME_TRAVEL_VERBS[id], args: problem === undefined ? {} : { [HISTORY_EDIT_ARG_MUTATION_ID]: problem.mutationId, ...(problem.store === undefined ? {} : { [HISTORY_EDIT_ARG_STORE]: problem.store }) } };
}

/** 📝️ The band's lines in the shell's current language; a line the session does not carry is `null`. */
export type TimeTravelBandTextV1 = {
  readonly stage: string;
  readonly target: string | null;
  readonly progress: string | null;
  readonly review: string | null;
  readonly outcome: string | null;
  readonly fault: string | null;
  readonly accepted: string | null;
};

const STAGE_LABEL_KEYS: Readonly<Record<HistoryTimeTravelStage, UiTranslationKey>> = {
  editing: "ui.timeTravel.stage.editing",
  replaying: "ui.timeTravel.stage.replaying",
  reviewing: "ui.timeTravel.stage.reviewing",
  choosing: "ui.timeTravel.stage.choosing",
  finalizing: "ui.timeTravel.stage.finalizing",
};

const REVIEW_LABEL_KEYS: Readonly<Record<HistoryTimeTravelReview, UiTranslationKey>> = {
  noChanges: "ui.timeTravel.review.noChanges",
  needsReplay: "ui.timeTravel.review.needsReplay",
  blocked: "ui.timeTravel.review.blocked",
  ready: "ui.timeTravel.review.ready",
};

/** 🏷️ The edited mutation's label on the shell's axes, `""` when the session names none. */
function timeTravelTargetText(session: HistoryTimeTravel, axes: { readonly terminology: string; readonly locale: string }): string {
  return session.targetLabel === undefined ? "" : historyEntryLabelText(session.targetLabel, axes.terminology, axes.locale);
}

/** 🖋️ {@link TimeTravelBandTextV1} of `session`: a review's status is the session's own `review`, never inferred from a
 * missing report; severity is always named in words (`ui.mutation.level.*`) and only when the session reports one,
 * never by colour alone; a fault code the shell knows (`timeTravel.cancelled`, the refusals) reads as its own text, any other
 * reads the replay-failed copy — the code itself is only the band's `data-semio-time-travel-fault`, never shown. */
export function timeTravelBandTextV1(session: HistoryTimeTravel, axes: { readonly terminology: string; readonly locale: string }): TimeTravelBandTextV1 {
  const target = timeTravelTargetText(session, axes);
  const fault = session.fault === undefined ? null : historyRefusalCodeV1(session.fault);
  const running = session.stage === "editing" || session.stage === "replaying";
  const progress = running && session.total !== undefined ? String(shellLabel(session.stage === "editing" ? "ui.timeTravel.preparationProgress" : "ui.timeTravel.progress", { done: session.done ?? 0, total: session.total })) : null;
  return {
    stage: String(shellLabel(STAGE_LABEL_KEYS[session.stage])),
    target: target === "" ? null : String(shellLabel("ui.timeTravel.target", { target })),
    progress: progress !== null && session.processed !== undefined ? `${progress} · ${shellLabel("ui.timeTravel.processed", { processed: session.processed })}` : progress,
    review: session.stage === "reviewing" && session.review !== undefined ? String(shellLabel(REVIEW_LABEL_KEYS[session.review])) : null,
    outcome: session.worst === undefined ? null : String(shellLabel("ui.timeTravel.worst", { level: shellLabel(`ui.mutation.level.${session.worst}`) })),
    fault: session.fault === undefined ? null : String(shellLabel(HISTORY_REFUSAL_LABEL_KEYS[fault ?? "timeTravel.replay-faulted"])),
    accepted: (session.acceptedCount ?? 0) > 0 ? String(shellLabel("ui.timeTravel.accepted", { count: session.acceptedCount })) : null,
  };
}

/** 🔖️ What a window of the editing program says about itself: the document before the edited mutation while a draft
 * is edited, else the session's stage. */
export function timeTravelIndicatorTextV1(session: HistoryTimeTravel, axes: { readonly terminology: string; readonly locale: string }): string {
  const target = timeTravelTargetText(session, axes);
  return String(session.stage === "editing" && target !== "" ? shellLabel("ui.timeTravel.indicatorTarget", { target }) : shellLabel(STAGE_LABEL_KEYS[session.stage]));
}

/** 🗝️ The node keys the framework history body gives a history row and a mutation child (`🔌️plugin/🦀️.rs`
 * `ui_history_panel`), which a peer's open history edit marks.
 * @see ../../../../../🔌️plugin/🦀️.rs */
export const HISTORY_ROW_KEY_PREFIX = "framework.history.entry.";
export const HISTORY_MUTATION_ROW_KEY_PREFIX = "framework.history.mutation.";

const EMPTY_PEER_OVERLAY: UiPresenceOverlayValue = { byKey: new Map() };

/** 👥️ Peers' open history edits, labelled from THIS replica's own history rows (the wire carries only the mutation id,
 * the stage and the draft count, never locale text): each editing peer's chip gets its activity ("Ada is editing Drag
 * selection in the history", the history in general when the mutation is not among the local rows), and the affected
 * mutation row and its history row get a note naming who edits it. */
export function timeTravelPeerPresenceV1(
  peers: readonly (PresencePeer & { readonly historyEdit?: ArtifactPresenceHistoryEdit })[],
  entries: readonly HistoryEntry[],
  axes: { readonly terminology: string; readonly locale: string },
): { readonly peers: readonly PresencePeer[]; readonly overlay: UiPresenceOverlayValue } {
  const notes = new Map<string, string[]>();
  const note = (key: string, text: string) => notes.set(key, [...(notes.get(key) ?? []), text]);
  const chips = peers.map(({ historyEdit, ...peer }): PresencePeer => {
    if (historyEdit === undefined) return peer;
    const row = entries.find((entry) => entry.mutations?.some((mutation) => mutation.mutationId === historyEdit.mutationId));
    const mutation = row?.mutations?.find((candidate) => candidate.mutationId === historyEdit.mutationId);
    const target = mutation === undefined ? "" : historyEntryLabelText(mutation.label, axes.terminology, axes.locale);
    const rowNote = String(shellLabel("ui.timeTravel.peer.editingRow", { name: peer.label }));
    note(`${HISTORY_MUTATION_ROW_KEY_PREFIX}${historyEdit.mutationId}`, rowNote);
    if (row !== undefined) note(`${HISTORY_ROW_KEY_PREFIX}${row.seq}`, rowNote);
    const text = target === "" ? shellLabel("ui.timeTravel.peer.editingHistory", { name: peer.label }) : shellLabel("ui.timeTravel.peer.editingTarget", { name: peer.label, target });
    return { ...peer, activity: { text: String(text), badge: "⏪" } };
  });
  return { peers: chips, overlay: notes.size === 0 ? EMPTY_PEER_OVERLAY : { byKey: new Map([...notes].map(([key, lines]): [string, UiPresenceOverlayEntry] => [key, { notes: lines }])) } };
}

/** 🔦️ Where keyboard focus belongs after a session change: the first input of the draft editor, the band, or the
 * finalize prompt. */
export type TimeTravelFocusTargetV1 = "editor" | "band" | "dialog";

/** 🗺️ What one change of the focused program's session asks of the chrome: `reveal` opens the History panel, `focus`
 * moves keyboard focus, `scrollTo` names the history-body row (a node key) to bring into view. */
export type TimeTravelTransitionV1 = { readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null; readonly scrollTo: string | null };

/** 🧭️ The chrome's answer to the session moving from `previous` to `next` (`null` = no session): the edge into a new
 * session reveals the History panel (whoever began it — a person, a chord or an agent); a draft that starts (Begin, Next
 * problem, another mutation) puts focus on its first input; a replay or a review puts it on the band; the edge into a
 * review that names its first blocking mutation (`nextProblem` — a replay that completed blocked) reveals the panel again
 * and scrolls to that mutation's row; the finalize prompt takes focus; a progress step, the commit and the close move
 * nothing. */
export function timeTravelTransitionV1(previous: HistoryTimeTravel | null, next: HistoryTimeTravel | null): TimeTravelTransitionV1 {
  if (next === null) return { reveal: false, focus: null, scrollTo: null };
  const same = previous !== null && previous.sessionId === next.sessionId;
  const moved = !same || previous.stage !== next.stage;
  switch (next.stage) {
    case "editing":
      return { reveal: !same, focus: moved || previous?.target !== next.target ? "editor" : null, scrollTo: null };
    case "replaying":
      return { reveal: !same, focus: moved ? "band" : null, scrollTo: null };
    case "reviewing": {
      const problem = moved ? next.nextProblem : undefined;
      return { reveal: !same || problem !== undefined, focus: moved ? "band" : null, scrollTo: problem === undefined ? null : `${HISTORY_MUTATION_ROW_KEY_PREFIX}${problem.mutationId}` };
    }
    case "choosing":
      return { reveal: !same, focus: moved ? "dialog" : null, scrollTo: null };
    case "finalizing":
      return { reveal: !same, focus: null, scrollTo: null };
  }
}

const FOCUSABLE_SELECTOR = 'input:not([disabled]):not([type="hidden"]),select:not([disabled]),textarea:not([disabled]),button:not([disabled]),[tabindex]:not([tabindex="-1"]),[contenteditable]:not([contenteditable="false"])';
const EDITABLE_SELECTOR = 'input,select,textarea,[contenteditable]:not([contenteditable="false"])';
const HISTORY_PANEL_ID_PART = "framework.panel.history";

/** 🔎️ `element` itself when it takes focus, else its first focusable descendant. */
function focusableWithin(element: Element): HTMLElement | null {
  return element.matches(FOCUSABLE_SELECTOR) ? (element as HTMLElement) : element.querySelector<HTMLElement>(FOCUSABLE_SELECTOR);
}

/** 🔍️ The element that takes focus for `target` under `root`: the first focusable control of the editor's input rows
 * (`…/framework.history.editor.input.<pointer>`), else its Accept (the button, or the activatable row a lone button
 * becomes); the band (`[data-semio-time-travel]`); the open dialog's first control. `null` while it is not mounted yet. */
export function timeTravelFocusElementV1(root: ParentNode, target: TimeTravelFocusTargetV1): HTMLElement | null {
  switch (target) {
    case "band":
      return root.querySelector<HTMLElement>("[data-semio-time-travel]");
    case "dialog": {
      const dialog = root.querySelector('[role="dialog"]');
      return dialog === null ? null : focusableWithin(dialog);
    }
    case "editor": {
      for (const element of root.querySelectorAll('[id*="/framework.history.editor.input."]')) {
        const key = element.id.slice(element.id.lastIndexOf("/") + 1);
        if (!key.startsWith("framework.history.editor.input.") || key.endsWith(".row")) continue;
        const control = focusableWithin(element);
        if (control !== null) return control;
      }
      const accept = root.querySelector('[id$="/framework.history.editor.accept"], [id$="/framework.history.editor.accept.row"]');
      return accept === null ? null : focusableWithin(accept);
    }
  }
}

/** ✋️ Whether moving focus now would take it from someone typing elsewhere: an editable field outside the History
 * panel and the finalize prompt keeps its focus. */
export function timeTravelFocusIsHeldV1(active: Element | null): boolean {
  if (active === null || !active.matches(EDITABLE_SELECTOR)) return false;
  return active.closest('[role="dialog"]') === null && !active.id.includes(HISTORY_PANEL_ID_PART) && active.closest(`[id*="${HISTORY_PANEL_ID_PART}"]`) === null;
}

/** ⏳️ Moves focus to `target` under the shell's `root` (the prompt anywhere in its document, being portalled) once it is
 * mounted, retried across animation frames while the panel or the prompt mounts. A person typing elsewhere keeps their
 * focus for the editor and the band; the modal prompt always takes it (focus returns when it closes), and a focus already
 * inside the prompt stays. Answers the cancel. */
export function scheduleTimeTravelFocusV1(root: Element, target: TimeTravelFocusTargetV1, frames = 30): () => void {
  const document = root.ownerDocument;
  let handle = 0;
  let remaining = frames;
  const attempt = () => {
    if (target !== "dialog" && timeTravelFocusIsHeldV1(document.activeElement)) return;
    const element = timeTravelFocusElementV1(target === "dialog" ? document : root, target);
    if (element !== null) {
      if (!(target === "dialog" && element.closest('[role="dialog"]')?.contains(document.activeElement))) element.focus();
      return;
    }
    if (remaining-- > 0) handle = requestAnimationFrame(attempt);
  };
  handle = requestAnimationFrame(attempt);
  return () => cancelAnimationFrame(handle);
}

/** 🧷️ The element of the history-body node `key` under `root` — its DOM id is the node's path, ending `/<key>` — or
 * `null` while it is not mounted. */
export function timeTravelRowElementV1(root: ParentNode, key: string): HTMLElement | null {
  for (const element of root.querySelectorAll<HTMLElement>('[id*="/framework.history."]')) if (element.id.endsWith(`/${key}`)) return element;
  return null;
}

/** 🧲️ Brings the history row `key` into view under the shell's `root` once it is mounted, retried across animation frames
 * while the panel opens and the guest's body arrives — the first blocking row after a replay that completed blocked. It
 * moves no focus (the band keeps it). Answers the cancel. */
export function scheduleTimeTravelScrollV1(root: Element, key: string, frames = 30): () => void {
  let handle = 0;
  let remaining = frames;
  const attempt = () => {
    const row = timeTravelRowElementV1(root, key);
    if (row !== null) {
      row.scrollIntoView?.({ block: "nearest" });
      return;
    }
    if (remaining-- > 0) handle = requestAnimationFrame(attempt);
  };
  handle = requestAnimationFrame(attempt);
  return () => cancelAnimationFrame(handle);
}

/** 🏠️ What the History reveal speaks to: the device, the dock (or the merged mobile panel's tabs), the shell's dispatch, the
 * root focus moves inside, and how many frames a focus waits for its target to mount. */
export type TimeTravelRevealHostV1 = { readonly mobile: boolean; readonly dock: PanelDock; readonly mobilePanelTabs: readonly PanelTabNode[]; readonly dispatch: (action: ShellAction) => void; readonly root: Element | null; readonly frames?: number };

/** 📂️ Opens the History panel where the shell keeps it — its own dock anchor, selected and shown, or on mobile the merged
 * panel on the History tab — and answers whether one was found; a shell without the History tab dispatches nothing. */
export function revealHistoryPanelV1(host: Pick<TimeTravelRevealHostV1, "mobile" | "dock" | "mobilePanelTabs" | "dispatch">): boolean {
  if (host.mobile) {
    const path = findPanelTabPath(host.mobilePanelTabs, FRAMEWORK_PANEL_TAB_HISTORY_ID);
    if (path === undefined) return false;
    host.dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
    host.dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
    return true;
  }
  const located = findPanelTabInDock(host.dock, FRAMEWORK_PANEL_TAB_HISTORY_ID);
  const path = located === null ? undefined : findPanelTabPath(host.dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_HISTORY_ID);
  if (located === null || path === undefined) return false;
  host.dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: path });
  host.dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
  return true;
}

/** 🛰️ The shell's answer to each change of the focused program's session ({@link timeTravelTransitionV1}): the edge into a
 * session — and into a review that names its first blocking mutation — reveals the History panel
 * ({@link revealHistoryPanelV1}), that mutation's row is scrolled into view ({@link scheduleTimeTravelScrollV1}) and every
 * focus move is scheduled under the host's root; a focus or a scroll still waiting for its target yields only to a newer
 * one or to the close — never to a progress step — and the unmount cancels both. The host is read at the change, so a dock
 * or device change alone never re-runs the effect. */
export function useTimeTravelRevealV1(session: HistoryTimeTravel | null, host: TimeTravelRevealHostV1): void {
  const hostRef = useRef(host);
  hostRef.current = host;
  const seenRef = useRef<HistoryTimeTravel | null>(null);
  const cancelRef = useRef<(() => void) | null>(null);
  const scrollCancelRef = useRef<(() => void) | null>(null);
  useEffect(() => {
    const transition = timeTravelTransitionV1(seenRef.current, session);
    seenRef.current = session;
    if (transition.reveal) revealHistoryPanelV1(hostRef.current);
    const root = hostRef.current.root;
    if (transition.scrollTo !== null || session === null) {
      scrollCancelRef.current?.();
      scrollCancelRef.current = transition.scrollTo === null || root === null ? null : scheduleTimeTravelScrollV1(root, transition.scrollTo, hostRef.current.frames ?? 120);
    }
    if (transition.focus === null && session !== null) return;
    cancelRef.current?.();
    cancelRef.current = transition.focus === null || root === null ? null : scheduleTimeTravelFocusV1(root, transition.focus, hostRef.current.frames ?? 120);
  }, [session]);
  useEffect(
    () => () => {
      cancelRef.current?.();
      scrollCancelRef.current?.();
    },
    [],
  );
}

/** 🏳️ The props both chrome pieces read: the live session, the program it edits and the shell's label axes. */
export type TimeTravelChromePropsV1 = { readonly session: HistoryTimeTravel; readonly terminology: string; readonly locale: string };

/** 🪟️ The chip every window of the editing program wears while a session is live — an icon and "History editing" in words,
 * named by what the window shows ({@link timeTravelIndicatorTextV1}). It takes the stage and that text rather than the
 * session, so the window descriptors that carry it do not rebuild on every replay progress step. */
export function TimeTravelWindowIndicator({ stage, description }: { readonly stage: HistoryTimeTravelStage; readonly description: string }): ReactElement {
  return (
    <span role="note" aria-label={description} title={description} data-semio-time-travel-indicator={stage} className="pointer-events-auto inline-flex items-center gap-tiny rounded-sm border border-accent bg-base px-single text-xs">
      <Icon icon="clock" size="small" />
      <span aria-hidden="true">{shellLabel("ui.timeTravel.indicator")}</span>
    </span>
  );
}

/** 🕹️ One control of the band. A refused one is never `disabled`: it stays in the Tab order as `aria-disabled`, dispatches
 * nothing, and names what stops it through {@link DisabledReasonHint} — always its description (`aria-describedby`), and
 * visible text while it is hovered, keyboard-focused or pressed — exactly as a refused row action does and as the wgpu band
 * mirrors it (parity audit gap 3: a `disabled` button with a `title` told neither a keyboard nor a screen-reader user why).
 * Every control is a touch target of the `large` size token on both axes (`--size-large` = 9 × `--ui-spacing`: 28.8 px at
 * compact density, 39.6 px at touch density — WCAG 2.5.8 asks 24 × 24; the control-height token `medium` is 22.4 px). */
function TimeTravelBandControl({ entry, shortcut, onPress }: { readonly entry: TimeTravelBandControlStateV1; readonly shortcut: string | undefined; readonly onPress: () => void }): ReactElement {
  const reasonId = `${useId()}-reason`;
  const refused = entry.disabledBy !== null;
  const button = (
    <button
      type="button"
      id={entry.controlId}
      className={refused ? "inline-flex min-h-large min-w-large cursor-not-allowed items-center justify-center px-tiny opacity-50" : "inline-flex min-h-large min-w-large items-center justify-center px-tiny underline"}
      data-semio-time-travel-control={entry.control}
      aria-disabled={refused ? true : undefined}
      aria-describedby={refused ? reasonId : undefined}
      aria-keyshortcuts={shortcut}
      onClick={refused ? undefined : onPress}
    >
      {shellLabel(entry.label)}
    </button>
  );
  return (
    <DisabledReasonHint id={reasonId} reason={entry.disabledBy === null ? undefined : String(shellLabel(entry.disabledBy))}>
      {button}
    </DisabledReasonHint>
  );
}

/** 📣️ The persistent history-editing band (`role=status`, polite): the stage, the edited mutation, the replay progress with
 * its Cancel, the review's own status, the worst outcome in words, and the stage's controls ({@link TimeTravelBandControl})
 * — a refused one focusable and telling its reason — with their chords published on `aria-keyshortcuts`. The controls sit
 * in an `aria-live="off"` group, so a reason revealed or hidden by focus never re-announces the status. The chords fire only
 * while their control is offered and enabled, never from a form field. The band paints its own opaque panel-level surface
 * and the shell mounts it in the layout's `subfooter` row — in flow, the last row of the shell, under the footer — so it never
 * lies over other text (live faults O1 / O5: an undefined `bg-menu` and an absolute overlay let it) nor under a docked panel's
 * cap (live fault F8: a row above the footer lay under the bottom-right tab bar at tablet width). */
export function TimeTravelBand({ session, terminology, locale, controllerId, onAction }: TimeTravelChromePropsV1 & { readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const bindings = useUiKeybindingsByControlId();
  const text = timeTravelBandTextV1(session, { terminology, locale });
  const controls = timeTravelBandControlsV1(session);
  const dispatch = (id: TimeTravelControlV1) => onAction(timeTravelControlActionV1(controllerId, session, id));
  const offered = (id: TimeTravelControlV1) => controls.some((entry) => entry.control === id && entry.disabledBy === null);
  const chord = (id: TimeTravelControlV1): string | undefined => (id in TIME_TRAVEL_CHORD_IDS ? resolveControlKeybindingRaw(TIME_TRAVEL_CHORD_IDS[id as keyof typeof TIME_TRAVEL_CHORD_IDS], bindings) ?? SHELL_KEYBINDINGS[TIME_TRAVEL_CHORD_IDS[id as keyof typeof TIME_TRAVEL_CHORD_IDS]] : undefined);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.accept, () => dispatch("accept"), { enabled: offered("accept"), preventDefault: true }, [session, controllerId]);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.discard, () => dispatch("discard"), { enabled: offered("discard"), preventDefault: true }, [session, controllerId]);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.exit, () => dispatch("exit"), { enabled: offered("exit"), preventDefault: true }, [session, controllerId]);
  return (
    <Surface level="panel" role="status" aria-live="polite" tabIndex={-1} aria-label={String(shellLabel("ui.timeTravel.band"))} data-semio-time-travel={session.stage} data-time-travel-generation={session.generation} className="pointer-events-auto flex max-w-[90vw] flex-wrap items-center gap-single rounded-sm border border-accent px-double py-single text-sm">
      <Icon icon="clock" size="small" />
      <strong data-semio-time-travel-stage="">{text.stage}</strong>
      {text.target === null ? null : <span data-semio-time-travel-target="">{text.target}</span>}
      {text.progress === null ? null : (
        <>
          <progress data-semio-time-travel-progress="" value={session.done ?? 0} max={session.total} aria-label={text.progress} />
          <span aria-hidden="true">{text.progress}</span>
        </>
      )}
      {text.review === null ? null : <span data-semio-time-travel-review={session.review}>{text.review}</span>}
      {text.outcome === null ? null : <span data-semio-time-travel-outcome={session.worst}>{text.outcome}</span>}
      {text.accepted === null ? null : <span data-semio-time-travel-accepted="">{text.accepted}</span>}
      {text.fault === null ? null : <span data-semio-time-travel-fault={session.fault}>{text.fault}</span>}
      <span className="contents" aria-live="off" data-semio-time-travel-controls="">
        {controls.map((entry) => (
          <TimeTravelBandControl key={entry.control} entry={entry} shortcut={ariaKeyshortcutsText(chord(entry.control))} onPress={() => dispatch(entry.control)} />
        ))}
      </span>
    </Surface>
  );
}

/** 🔁️ What the reprojection status offers while no history-edit session is open: Cancel replay while a remote change, a
 * history step or a document load replays (the history body's own control), Replay again while a remote change is paused,
 * nothing for a refused adoption — a session's band owns both controls while it is open. */
export function historyReprojectionControlV1(reprojection: HistoryReprojection, sessionOpen: boolean): "cancelReplay" | "rerun" | null {
  if (sessionOpen || reprojection.total === 0) return null;
  return reprojection.paused === true && (reprojection.kind ?? "remote") === "remote" ? "rerun" : "cancelReplay";
}

/** 📡️ The polite status a history change replaying before adoption raises OUTSIDE the History panel (audit W1E-3) — another
 * replica's change, this replica's own deferred history step, a whole-document load — read from `HistoryPatch.reprojection`
 * through the kernel's one copy (`historyReprojectionStatus`, the same bytes the wgpu shell mirrors as
 * `shell.history.reprojection`): the kind's title, its progress (announced when it starts, then shown and carried by the
 * progress bar without re-announcing every step), a pause or a refused adoption in words — its code only as
 * `data-notice-code` — and Cancel replay / Replay again ({@link historyReprojectionControlV1}). */
export function HistoryReprojectionStatus({ reprojection, locale, sessionOpen, controllerId, onAction }: { readonly reprojection: HistoryReprojection; readonly locale: ShellLocale; readonly sessionOpen: boolean; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const status = historyReprojectionStatus(reprojection, "native", locale);
  const kind = reprojection.kind ?? "remote";
  const phase = status.fault !== null && status.total === 0 ? "refused" : status.paused ? "paused" : "progress";
  const announced = useRef<{ readonly key: string; readonly text: string } | null>(null);
  const key = `${kind}:${phase}:${locale}`;
  if (announced.current?.key !== key) announced.current = { key, text: status.text };
  const control = historyReprojectionControlV1(reprojection, sessionOpen);
  return (
    <Surface level="panel" data-semio-history-reprojection={kind} data-semio-history-reprojection-phase={phase} data-notice-code={status.fault ?? undefined} className="pointer-events-auto flex max-w-[90vw] flex-wrap items-center gap-single rounded-sm border border-normal px-double py-single text-sm">
      <span role="status" aria-live="polite" aria-atomic="true" className="sr-only" data-semio-history-reprojection-announcement="">
        {`${status.title}: ${announced.current.text}`}
      </span>
      <Icon icon={phase === "refused" ? "triangle-alert" : kind === "load" ? "download" : "loader-2"} size="small" />
      <strong aria-hidden="true">{status.title}</strong>
      <span aria-hidden="true" data-semio-history-reprojection-text="">
        {status.text}
      </span>
      {phase === "progress" && status.total > 0 ? <progress data-semio-history-reprojection-progress="" value={status.done} max={status.total} aria-label={status.title} aria-valuetext={status.text} /> : null}
      {control === null ? null : (
        <button type="button" className="inline-flex min-h-large min-w-large items-center justify-center px-tiny underline" data-semio-history-reprojection-control={control} onClick={() => onAction({ controllerId, action: TIME_TRAVEL_VERBS[control], args: {} })}>
          {shellLabel(`ui.timeTravel.${control}`)}
        </button>
      )}
    </Surface>
  );
}
//#endregion ⏪️TimeTravelChrome
