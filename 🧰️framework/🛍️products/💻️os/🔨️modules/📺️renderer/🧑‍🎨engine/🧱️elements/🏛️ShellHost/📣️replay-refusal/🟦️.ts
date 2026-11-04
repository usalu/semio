// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/replay-refusal/component.ts
/** 📣️ Every way the shell can refuse a guest's `replayShellCommand` — a durable gesture the
 * user made — with the text each one shows. Ten fail-closed gates on that route used to report
 * through `console.warn` alone, so a create, an open or a directory command the host dropped looked
 * to the user exactly like one that worked: no fault, no ledger row, no notice (measured
 * 2026-09-22, ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END §5.4). Authored here, in the unit
 * that decides the refusal, with no shell i18n import — the `🎯️input-ledger` rule.
 * Ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END. */
// #endregion 🧲️Header

import vocabulary from "./🔣️.json" with { type: "json" };

/** 👁️✏️ The closed set of reasons a shell's own `replayShellCommand` route drops a guest gesture — the keys of the
 * schema-first vocabulary `🔣️.json` the wgpu shell reads too, so both shells refuse in the same words. One reason per gate,
 * so a notice names the thing the user can act on and never the internal id. */
export type ReplayRefusalReasonV1 = keyof typeof vocabulary.reasons;

export type ReplayRefusalLabelV1 = { readonly en: string; readonly de: string };

/** 🗣️ Renders a label table row in `locale`; the primary language subtag picks the row (`de-DE` reads `de`); a tag without a row has no default language, so every language is rendered, joined in table order. */
export function localizedLabelTextV1(label: Readonly<Record<string, string>>, locale: string): string {
  const primary = locale.toLowerCase().split(/[-_]/u)[0] ?? "";
  return Object.hasOwn(label, primary) ? label[primary]! : Object.values(label).join(" | ");
}

/** 🗣️ Notice text per reason (`🔣️.json`). `locale` picks; an unknown locale renders every language. */
export const REPLAY_REFUSAL_LABELS_V1: Readonly<Record<ReplayRefusalReasonV1, ReplayRefusalLabelV1>> = vocabulary.reasons;

export function replayRefusalNoticeTextV1(reason: ReplayRefusalReasonV1, locale: string): string {
  return localizedLabelTextV1(REPLAY_REFUSAL_LABELS_V1[reason], locale);
}

/** 🩺️ Fault code carried on the notice so a probe, a law and the console line all name the same
 * gate — `console.warn` text is prose and matches no refusal word, which is why four measured runs
 * read "dispatched, no refusal" while nothing happened. */
export function replayRefusalCodeV1(reason: ReplayRefusalReasonV1): string {
  return `${vocabulary.codePrefix}${reason}`;
}
