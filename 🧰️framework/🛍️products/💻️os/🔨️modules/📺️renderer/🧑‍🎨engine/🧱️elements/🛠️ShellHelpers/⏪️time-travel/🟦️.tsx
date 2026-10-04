// #region 🧲️Header
/** ⏪️ The React shell's time-travel chrome (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING): the persistent band a live
 * history-edit session raises from `HistoryPatch.timeTravel` whichever panel is open — stage, edited mutation, replay
 * progress with Cancel, review status, worst outcome and the stage's own verbs — the indicator every window of the editing program
 * wears, and the remappable `ui.timeTravel.accept|discard|exit` chords. The history rows and the input editor are the
 * guest's own `framework.body.history`, rendered by the interpreter; this chrome only reads the session and dispatches
 * the reserved `historyEdit*` verbs, each stamped with the session generation it was shown, so a stale press is refused
 * `timeTravel.stale` instead of acting on a session that moved on.
 * @see ../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts
 * @see ../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { FRAMEWORK_PANEL_TAB_HISTORY_ID, HISTORY_EDIT_ARG_GENERATION, historyEntryLabelText, historyReprojectionStatus, type ActionDescriptor, type HistoryEditActionId, type HistoryEntry, type HistoryReprojection, type HistoryTimeTravel, type HistoryTimeTravelReview, type HistoryTimeTravelStage, type ShellLocale } from "@semio-tech/framework";
import type { ArtifactPresenceHistoryEdit } from "@semio-tech/framework-replication";
import { ariaKeyshortcutsText, findPanelTabInDock, findPanelTabPath, Icon, resolveControlKeybindingRaw, SHELL_KEYBINDINGS, useControlKeybinding, useUiKeybindingsByControlId, type PanelDock, type PanelTabNode, type PresencePeer, type UiTranslationKey } from "@semio-tech/ui-react";
import { useEffect, useId, useRef, type ReactElement } from "react";
import type { ShellAction } from "../../🐚️Shell/🟦️.tsx";
import { HISTORY_REFUSAL_LABEL_KEYS, historyRefusalCodeV1, shellLabel } from "../🟦️.tsx";
import type { UiPresenceOverlayEntry, UiPresenceOverlayValue } from "../../🗣️Interpreter/🟦️.tsx";
// #endregion 🔌️Adapters

//#region ⏪️TimeTravelChrome
/** 🎬️ The reserved `historyEdit*` verbs the chrome dispatches, by the control that dispatches them. */
export const TIME_TRAVEL_VERBS = {
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

/** 🔘️ One band control: what it dispatches, its label, and why it is disabled (`null` when it is not). */
export type TimeTravelBandControlStateV1 = { readonly control: TimeTravelControlV1; readonly label: UiTranslationKey; readonly disabledBy: UiTranslationKey | null };

const bandControl = (id: TimeTravelControlV1, disabledBy: UiTranslationKey | null = null): TimeTravelBandControlStateV1 => ({ control: id, label: `ui.timeTravel.${id}`, disabledBy });

/** 🚧️ Why Finalize is disabled in a review, read from the session's own `review` and never inferred from a missing
 * report: nothing accepted is `empty`, a replay still owed or a blocking report is `blocked`, a `ready` review is
 * finalizable, and a review the session does not state is not possible right now. */
function finalizeDisabledBy(session: HistoryTimeTravel): UiTranslationKey | null {
  if (session.review === "noChanges" || (session.acceptedCount ?? 0) === 0) return "ui.timeTravel.refusal.empty";
  if (session.review === "needsReplay" || session.review === "blocked" || session.blocking === true) return "ui.timeTravel.refusal.blocked";
  return session.review === "ready" ? null : "ui.timeTravel.refusal.illegal";
}

/** 🚦️ The controls a stage offers, in band order: a draft is accepted or discarded while editing, a replay can be
 * cancelled while it runs, a review can replay again when the session says it would start a replay (`rerunnable`) and
 * finalize when its review is `ready` (else both stay visible and name what stops them), the finalize prompt can be
 * left for the review, and every stage but the commit itself can exit. */
export function timeTravelBandControlsV1(session: HistoryTimeTravel): readonly TimeTravelBandControlStateV1[] {
  switch (session.stage) {
    case "editing":
      return [bandControl("accept"), bandControl("discard"), bandControl("exit")];
    case "replaying":
      return [bandControl("cancelReplay"), bandControl("exit")];
    case "reviewing":
      return [bandControl("rerun", session.rerunnable === true ? null : "ui.timeTravel.refusal.illegal"), bandControl("finalize", finalizeDisabledBy(session)), bandControl("exit")];
    case "choosing":
      return [bandControl("back"), bandControl("exit")];
    case "finalizing":
      return [];
  }
}

/** 🎯️ The action a control dispatches for `session` on the program `controllerId` names. */
export function timeTravelControlActionV1(controllerId: string, session: HistoryTimeTravel, id: TimeTravelControlV1): ActionDescriptor {
  return { controllerId, action: TIME_TRAVEL_VERBS[id], args: { [HISTORY_EDIT_ARG_GENERATION]: session.generation } };
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
  return {
    stage: String(shellLabel(STAGE_LABEL_KEYS[session.stage])),
    target: target === "" ? null : String(shellLabel("ui.timeTravel.target", { target })),
    progress: session.stage === "replaying" && session.total !== undefined ? String(shellLabel("ui.timeTravel.progress", { done: session.done ?? 0, total: session.total })) : null,
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
 * selection in time travel", the history in general when the mutation is not among the local rows), and the affected
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
 * moves keyboard focus. */
export type TimeTravelTransitionV1 = { readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null };

/** 🧭️ The chrome's answer to the session moving from `previous` to `next` (`null` = no session): the edge into a new
 * session reveals the History panel (whoever began it — a person, a chord or an agent); a draft that starts (Begin, Next
 * problem, another mutation) puts focus on its first input; a replay or a review puts it on the band; the finalize
 * prompt takes it; a progress step, the commit and the close move nothing. */
export function timeTravelTransitionV1(previous: HistoryTimeTravel | null, next: HistoryTimeTravel | null): TimeTravelTransitionV1 {
  if (next === null) return { reveal: false, focus: null };
  const same = previous !== null && previous.sessionId === next.sessionId;
  const moved = !same || previous.stage !== next.stage;
  switch (next.stage) {
    case "editing":
      return { reveal: !same, focus: moved || previous?.target !== next.target ? "editor" : null };
    case "replaying":
    case "reviewing":
      return { reveal: !same, focus: moved ? "band" : null };
    case "choosing":
      return { reveal: !same, focus: moved ? "dialog" : null };
    case "finalizing":
      return { reveal: !same, focus: null };
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
 * session reveals the History panel ({@link revealHistoryPanelV1}) and every focus move is scheduled under the host's root;
 * a focus still waiting for its target yields only to a newer focus or to the close — never to a progress step — and the
 * unmount cancels it. The host is read at the change, so a dock or device change alone never re-runs the effect. */
export function useTimeTravelRevealV1(session: HistoryTimeTravel | null, host: TimeTravelRevealHostV1): void {
  const hostRef = useRef(host);
  hostRef.current = host;
  const seenRef = useRef<HistoryTimeTravel | null>(null);
  const cancelRef = useRef<(() => void) | null>(null);
  useEffect(() => {
    const transition = timeTravelTransitionV1(seenRef.current, session);
    seenRef.current = session;
    if (transition.reveal) revealHistoryPanelV1(hostRef.current);
    if (transition.focus === null && session !== null) return;
    cancelRef.current?.();
    const root = hostRef.current.root;
    cancelRef.current = transition.focus === null || root === null ? null : scheduleTimeTravelFocusV1(root, transition.focus, hostRef.current.frames ?? 120);
  }, [session]);
  useEffect(() => () => cancelRef.current?.(), []);
}

/** 🏳️ The props both chrome pieces read: the live session, the program it edits and the shell's label axes. */
export type TimeTravelChromePropsV1 = { readonly session: HistoryTimeTravel; readonly terminology: string; readonly locale: string };

/** 🪟️ The chip every window of the editing program wears while a session is live — an icon and "Time travel" in words,
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

/** 📣️ The persistent time-travel band (`role=status`, polite): the stage, the edited mutation, the replay progress with
 * its Cancel, the review's own status, the worst outcome in words, and the stage's controls — a disabled one titled by
 * its reason and, for Finalize, described by the review line — with their chords published on `aria-keyshortcuts`. The
 * chords fire only while their control is offered and enabled, never from a form field. */
export function TimeTravelBand({ session, terminology, locale, controllerId, onAction }: TimeTravelChromePropsV1 & { readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const bindings = useUiKeybindingsByControlId();
  const reviewId = useId();
  const text = timeTravelBandTextV1(session, { terminology, locale });
  const controls = timeTravelBandControlsV1(session);
  const dispatch = (id: TimeTravelControlV1) => onAction(timeTravelControlActionV1(controllerId, session, id));
  const offered = (id: TimeTravelControlV1) => controls.some((entry) => entry.control === id && entry.disabledBy === null);
  const chord = (id: TimeTravelControlV1): string | undefined => (id in TIME_TRAVEL_CHORD_IDS ? resolveControlKeybindingRaw(TIME_TRAVEL_CHORD_IDS[id as keyof typeof TIME_TRAVEL_CHORD_IDS], bindings) ?? SHELL_KEYBINDINGS[TIME_TRAVEL_CHORD_IDS[id as keyof typeof TIME_TRAVEL_CHORD_IDS]] : undefined);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.accept, () => dispatch("accept"), { enabled: offered("accept"), preventDefault: true }, [session, controllerId]);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.discard, () => dispatch("discard"), { enabled: offered("discard"), preventDefault: true }, [session, controllerId]);
  useControlKeybinding(TIME_TRAVEL_CHORD_IDS.exit, () => dispatch("exit"), { enabled: offered("exit"), preventDefault: true }, [session, controllerId]);
  return (
    <div role="status" aria-live="polite" tabIndex={-1} aria-label={String(shellLabel("ui.timeTravel.band"))} data-semio-time-travel={session.stage} data-time-travel-generation={session.generation} className="pointer-events-auto flex max-w-[90vw] flex-wrap items-center gap-single rounded-sm border border-accent bg-menu px-double py-single text-sm shadow-sm">
      <Icon icon="clock" size="small" />
      <strong data-semio-time-travel-stage="">{text.stage}</strong>
      {text.target === null ? null : <span data-semio-time-travel-target="">{text.target}</span>}
      {text.progress === null ? null : (
        <>
          <progress data-semio-time-travel-progress="" value={session.done ?? 0} max={session.total} aria-label={text.progress} />
          <span aria-hidden="true">{text.progress}</span>
        </>
      )}
      {text.review === null ? null : <span id={reviewId} data-semio-time-travel-review={session.review}>{text.review}</span>}
      {text.outcome === null ? null : <span data-semio-time-travel-outcome={session.worst}>{text.outcome}</span>}
      {text.accepted === null ? null : <span data-semio-time-travel-accepted="">{text.accepted}</span>}
      {text.fault === null ? null : <span data-semio-time-travel-fault={session.fault}>{text.fault}</span>}
      {controls.map((entry) => (
        <button
          key={entry.control}
          type="button"
          className="min-h-medium px-tiny underline disabled:opacity-50 disabled:no-underline"
          data-semio-time-travel-control={entry.control}
          disabled={entry.disabledBy !== null}
          title={entry.disabledBy === null ? undefined : String(shellLabel(entry.disabledBy))}
          aria-describedby={entry.control === "finalize" && entry.disabledBy !== null && text.review !== null ? reviewId : undefined}
          aria-keyshortcuts={ariaKeyshortcutsText(chord(entry.control))}
          onClick={() => dispatch(entry.control)}
        >
          {shellLabel(entry.label)}
        </button>
      ))}
    </div>
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
    <div data-semio-history-reprojection={kind} data-semio-history-reprojection-phase={phase} data-notice-code={status.fault ?? undefined} className="pointer-events-auto flex max-w-[90vw] flex-wrap items-center gap-single rounded-sm border border-normal bg-menu px-double py-single text-sm shadow-sm">
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
        <button type="button" className="min-h-medium px-tiny underline" data-semio-history-reprojection-control={control} onClick={() => onAction({ controllerId, action: TIME_TRAVEL_VERBS[control], args: {} })}>
          {shellLabel(`ui.timeTravel.${control}`)}
        </button>
      )}
    </div>
  );
}
//#endregion ⏪️TimeTravelChrome
