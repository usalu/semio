/** 🕰️ The history wire every host renders: transactions, mutation entries, time travel, history entries and their labels, reprojection and its status, and the history patch.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { ShellLocale, ShellTerminology, LocalizedLabel } from "@semio-tech/framework-manifest/generated/ui-axes";
import { type Severity } from "../🚨️fault/🟦️.ts";
import { historyNotice } from "../🏷️notice-labels/🟦️.ts";

// #region 🎠️Kernel
//#region InvocationResponse
/** 🛠️ The committed tool transaction a history row's edit carries, mirrored from Rust `HistoryTransaction`. */
export type HistoryTransaction = {
  readonly id: string;
  readonly tool: string;
};

/** 📨️ One outcome message of a history mutation row, mirrored from Rust `HistoryMutationMessage`. */
export type HistoryMutationMessage = {
  readonly level: Severity;
  readonly code: string;
  readonly message: string;
  readonly target?: readonly string[];
  readonly opIndex?: number;
};

/** ✏️ One applied mutation of a history row, mirrored from Rust `HistoryMutationEntry`: `withdrawable` = the store's
 * supersede law lets this actor withdraw it and it is not withdrawn yet, `pending` = downstream of
 * the mutation being edited and not applied in the preview, `edited` = the session holds a draft for it, `introduced` =
 * the session's replay outcome carries a message (level and code) its pre-edit outcome does not, `store` = the composed
 * member store that holds it (`<slot>/<childId>`), absent for the document's own. */
export type HistoryMutationEntry = {
  readonly mutationId: string;
  readonly position: number;
  readonly opIndex: number;
  readonly label: LocalizedLabel;
  readonly worst?: Severity;
  readonly messages?: readonly HistoryMutationMessage[];
  readonly superseded?: boolean;
  readonly withdrawn?: boolean;
  readonly editable?: boolean;
  readonly withdrawable?: boolean;
  readonly pending?: boolean;
  readonly edited?: boolean;
  readonly introduced?: boolean;
  readonly store?: string;
};

/** 🚦️ Stage of a live history-edit session, mirrored from Rust `HistoryTimeTravelStage`. */
export type HistoryTimeTravelStage = "editing" | "replaying" | "reviewing" | "choosing" | "finalizing";

/** 🧭️ What a reviewing session shows, mirrored from Rust `HistoryTimeTravelReview`. */
export type HistoryTimeTravelReview = "noChanges" | "needsReplay" | "blocked" | "ready";

/** ⛔️ The first mutation whose replay outcome blocks finalizing, mirrored from Rust `HistoryTimeTravelProblem`: the
 * arguments of `historyEditBegin` (`store` = the composed member store that holds it, absent for the document's own). */
export type HistoryTimeTravelProblem = {
  readonly mutationId: string;
  readonly store?: string;
};

/** ⏪️ The live history-edit session of one instance, mirrored from Rust `HistoryTimeTravel`; absent while none is open. */
export type HistoryTimeTravel = {
  readonly sessionId: string;
  readonly generation: number;
  readonly stage: HistoryTimeTravelStage;
  readonly target?: string;
  readonly targetLabel?: LocalizedLabel;
  readonly done?: number;
  readonly total?: number;
  readonly processed?: number;
  readonly worst?: Severity;
  readonly blocking?: boolean;
  readonly fault?: string;
  readonly acceptedCount?: number;
  /** 🔍️ While reviewing: what the review shows. */
  readonly review?: HistoryTimeTravelReview;
  /** 🔁️ Whether `historyEditRerun` would start a replay now. */
  readonly rerunnable?: boolean;
  /** 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`. */
  readonly nextProblem?: HistoryTimeTravelProblem;
};

/** 🔑️ The key a host folds a history row under: its edit id, else its history transition id, else its session sequence. */
export function historyEntryKey(entry: Pick<HistoryEntry, "seq" | "editId" | "transitionId">): string {
  if (entry.editId !== undefined) return `edit:${entry.editId}`;
  return entry.transitionId === undefined ? `seq:${entry.seq}` : `transition:${entry.transitionId}`;
}

/** 🧾️ One host-projectable command-history row, mirrored from Rust `HistoryEntry`: one row per committed tool
 * transaction (else per edit), keyed by {@link historyEntryKey}. */
export type HistoryEntry = {
  readonly seq: number;
  readonly editId?: string;
  readonly transaction?: HistoryTransaction;
  /** 🔀️ The `Supersede` history transition this row is — a history edit, or the undo or redo of one. */
  readonly transitionId?: string;
  /** 🖋️ The actor who authored this row's edit or history transition. */
  readonly author?: string;
  readonly actionId: string;
  /** 🏷️ Every locale's text for this row, mirrored from Rust `LocalizedLabel` — the renderer
   * resolves it with {@link historyEntryLabelText} against the locale it is showing right now, so a
   * locale switch re-renders the whole ledger instead of leaving logged rows in their dispatch locale. */
  readonly label: LocalizedLabel;
  readonly kind: string;
  readonly timestamp: string;
  /** 📜️ The newest forward operations of this row's edit, newest last — a bounded preview. */
  readonly opLines?: readonly string[];
  /** 🔢️ Forward operations of this row's edit; more than `opLines` holds means the preview omits the older ones. */
  readonly opCount?: number;
  readonly applied?: boolean;
  readonly revertible?: boolean;
  readonly count?: number;
  readonly worst?: Severity;
  readonly mutations?: readonly HistoryMutationEntry[];
};

/** 🔖️ The one text a history row shows on the requested axes. There is deliberately NO
 * English fallback: `LocalizedLabel` is a `Record<ShellTerminology, Record<ShellLocale, string>>`
 * that the Rust carrier always fills for every axis, so an axis value the carrier does not carry is
 * a wire defect and renders as empty — visibly wrong — rather than silently as English. The shell's
 * `uiTerminology`/`uiLocale` preferences are stringly typed, hence the widened parameters; the same
 * contract as the neighbouring `resolveManifestLabel`.
 *
 * ⚠️ Both axes are read defensively, and the TYPE is not the guarantee here. The locale axis was
 * already optional-chained; the terminology axis was not, so a carrier that filled neither level
 * made `label[terminology]` `undefined` and `row[locale]` THREW. This function runs inside a
 * `useMemo` of `FrameworkOsShellInner`'s render, so that throw did not render one row wrong — it
 * unmounted the entire shell: measured on 2026-09-21 as `TypeError: Cannot read properties of
 * undefined (reading 'en')` followed by `plugin-handle.closed` and
 * `noteShellCommand refused: instance-retired (user window=s-home-main)`, with every window gone and
 * an empty `document.body`, every time the studio was opened from the command palette while ONE
 * staged guest still carried the pre-`LocalizedLabel` label shape (ticket 26/09/18 S10 §2.1).
 * A wire defect in one ledger row must cost that row its text, which is what the paragraph above
 * promises — never the product. The no-English-fallback law is unchanged: a missing axis is still
 * empty and still visibly wrong. */
export function historyEntryLabelText(label: LocalizedLabel, terminology: string, locale: string): string {
  if (terminology !== "native" && terminology !== "reuse") return "";
  const row: Readonly<Record<string, string>> | undefined = label?.[terminology];
  return row?.[locale] ?? "";
}

/** 🦀️ Which history change replays before adoption, mirrored from Rust `HistoryReprojectionKind`: another replica's change,
 * this replica's own deferred history step (interior undo/redo, checkout, alternative switch) or a whole-document load. */
export type HistoryReprojectionKind = "remote" | "step" | "load";

/** 🎞️ The replay a history change needs before this replica adopts it, mirrored from Rust `HistoryReprojection`: its
 * `kind` (absent = remote), replayed operations of the total, whether the user paused a remote one, and the code of a
 * refused adoption. */
export type HistoryReprojection = {
  readonly done: number;
  readonly total: number;
  readonly processed?: number;
  readonly kind?: HistoryReprojectionKind;
  readonly paused?: boolean;
  readonly fault?: string;
};

//#region 🔖️HistoryReprojectionStatus
/** 🦜️ Framework-owned EN/DE copy of a history change replaying before adoption, by `<kind>.<part>` key, mirrored from Rust
 * `HISTORY_REPROJECTION_LABELS`: every shell announces it in a polite status region outside the History panel and the history
 * body titles its reprojection section with it. `{done}`/`{total}` are replayed operations, `{reason}` a refusal's notice. */
export const HISTORY_REPROJECTION_LABELS = [
  { key: "remote.title", en: "Remote history change", de: "Entfernte Verlaufsänderung" },
  { key: "remote.progress", en: "Replaying a remote history change: {done} of {total} steps", de: "Entfernte Verlaufsänderung wird angewendet: {done} von {total} Schritten" },
  { key: "remote.paused", en: "Remote history change paused: this replica still shows the history before it", de: "Entfernte Verlaufsänderung pausiert: dieses Replikat zeigt noch den Verlauf davor" },
  { key: "remote.refused", en: "Remote history change refused: {reason}", de: "Entfernte Verlaufsänderung abgelehnt: {reason}" },
  { key: "step.title", en: "History step", de: "Verlaufsschritt" },
  { key: "step.progress", en: "Replaying history: {done} of {total} steps", de: "Verlauf wird neu angewendet: {done} von {total} Schritten" },
  { key: "step.refused", en: "History step refused: {reason}", de: "Verlaufsschritt abgelehnt: {reason}" },
  { key: "load.title", en: "Document load", de: "Dokument laden" },
  { key: "load.progress", en: "Loading document: {done} of {total}", de: "Dokument wird geladen: {done} von {total}" },
  { key: "load.refused", en: "Document load refused: {reason}", de: "Laden des Dokuments abgelehnt: {reason}" },
  { key: "reason.unnamed", en: "the change could not be applied", de: "die Änderung konnte nicht angewendet werden" },
  { key: "work.processed", en: "Work completed: {processed}", de: "Arbeitsfortschritt: {processed}" },
] as const;

/** 📢️ What a shell announces while a history change replays before adoption — TS twin of Rust `HistoryReprojectionStatus`. */
export type HistoryReprojectionStatus = Readonly<{ title: string; text: string; done: number; total: number; paused: boolean; fault: string | null }>;

/** 🚥️ The one status copy of a history change replaying before adoption — TS twin of Rust `kernel::history_reprojection_status`:
 * a refused adoption (a fault with nothing left to replay) reads `<kind>.refused` with the fault's placeholder-free
 * {@link historyNotice} or `reason.unnamed` (never a raw code); a paused remote change reads `remote.paused` (a step or a load is
 * never paused); anything else its kind's progress. Terminology-invariant framework copy. */
export function historyReprojectionStatus(reprojection: HistoryReprojection, _terminology: ShellTerminology, locale: ShellLocale): HistoryReprojectionStatus {
  const label = (key: string): string => HISTORY_REPROJECTION_LABELS.find((row) => row.key === key)?.[locale] ?? "";
  const kind = reprojection.kind ?? "remote";
  const paused = reprojection.paused === true && kind === "remote";
  const notice = reprojection.fault === undefined ? undefined : historyNotice(reprojection.fault);
  const reason = notice !== undefined && !notice.en.includes("{") && !notice.de.includes("{") ? notice[locale] : label("reason.unnamed");
  let text =
    reprojection.fault !== undefined && reprojection.total === 0
      ? label(`${kind}.refused`).replaceAll("{reason}", () => reason)
      : paused
        ? label("remote.paused")
        : label(`${kind}.progress`).replaceAll("{done}", () => String(reprojection.done)).replaceAll("{total}", () => String(reprojection.total));
  if (reprojection.processed !== undefined && !paused && reprojection.total > 0) text += ` · ${label("work.processed").replaceAll("{processed}", () => String(reprojection.processed))}`;
  return { title: label(`${kind}.title`), text, done: reprojection.done, total: reprojection.total, paused, fault: reprojection.fault ?? null };
}
//#endregion 🔖️HistoryReprojectionStatus

/** 🩹️ Ordered history delta carried with an accepted invocation response. */
export type HistoryPatch = {
  readonly cursor: number;
  readonly upserts?: readonly HistoryEntry[];
  readonly canUndo?: boolean;
  readonly canRedo?: boolean;
  readonly activeAlternativeId?: string;
  readonly currentCheckpointId?: string;
  readonly commandFilter?: string;
  /** ⌛️ The live history-edit session in full; absent while none is open. */
  readonly timeTravel?: HistoryTimeTravel;
  /** 📡️ The history change (remote, or this replica's own step) waiting for its replay; absent while none waits. */
  readonly reprojection?: HistoryReprojection;
  /** 🧮️ How many edits the document's history holds — the `{n}` of the `history.full` notice (absent = 0). */
  readonly editCount?: number;
};
//#endregion InvocationResponse
// #endregion 🎠️Kernel
