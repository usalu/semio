// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/replay-refusal/component.ts
/** @emoji 📣️ Every way the shell can refuse a guest's `replayShellCommand` — a durable gesture the
 * user made — with the text each one shows. Ten fail-closed gates on that route used to report
 * through `console.warn` alone, so a create, an open or a directory command the host dropped looked
 * to the user exactly like one that worked: no fault, no ledger row, no notice (measured
 * 2026-09-22, ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END §5.4). Authored here, in the unit
 * that decides the refusal, with no shell i18n import — the `🎯️input-ledger` rule.
 * Ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END. */
// #endregion 🧲️Header

/** 👁️✏️ The closed set of reasons the shell's own `replayShellCommand` route drops a guest gesture.
 * One reason per gate, so a notice names the thing the user can act on and never the internal id. */
export type ReplayRefusalReasonV1 =
  | "sign-in-required"
  | "space-required"
  | "space-index-required"
  | "invalid-request"
  | "router-not-ready"
  | "open-rejected"
  | "unrouted-command";

export type ReplayRefusalLabelV1 = { readonly en: string; readonly de: string };

/** 🗣️ Notice text per reason. `locale` picks; only an unknown locale falls back to English. */
export const REPLAY_REFUSAL_LABELS_V1: Readonly<Record<ReplayRefusalReasonV1, ReplayRefusalLabelV1>> = {
  "sign-in-required": { en: "Sign in to run that command.", de: "Zum Ausführen dieses Befehls bitte anmelden." },
  "space-required": { en: "Open a space first — that command needs one.", de: "Zuerst einen Space öffnen — dieser Befehl braucht einen." },
  "space-index-required": { en: "Open the space itself to create a document there.", de: "Zum Anlegen eines Dokuments den Space selbst öffnen." },
  "invalid-request": { en: "That command was incomplete and was not run.", de: "Dieser Befehl war unvollständig und wurde nicht ausgeführt." },
  "router-not-ready": { en: "The workspace is still loading — try again in a moment.", de: "Der Arbeitsbereich lädt noch — bitte gleich erneut versuchen." },
  "open-rejected": { en: "That document could not be opened.", de: "Dieses Dokument konnte nicht geöffnet werden." },
  "unrouted-command": { en: "This shell has no route for that command.", de: "Diese Shell kennt keinen Weg für diesen Befehl." },
};

export function replayRefusalNoticeTextV1(reason: ReplayRefusalReasonV1, locale: string): string {
  const label = REPLAY_REFUSAL_LABELS_V1[reason];
  return locale === "de" ? label.de : label.en;
}

/** 🩺️ Fault code carried on the notice so a probe, a law and the console line all name the same
 * gate — `console.warn` text is prose and matches no refusal word, which is why four measured runs
 * read "dispatched, no refusal" while nothing happened. */
export function replayRefusalCodeV1(reason: ReplayRefusalReasonV1): string {
  return `shell.replayShellCommand.${reason}`;
}
