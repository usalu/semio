/** 📊️ The mark every figure of the quiz is drawn with: a column that rises from a baseline with a share of the plot's
 * height, its value as text on its cap, in the quiet tone of the others or — `emphasis` — in the accent of the learner's
 * own. The column is paint only (`aria-hidden`): whatever holds it says the value in words, so no figure depends on
 * seeing it, on its colour or on hovering it.
 *
 * @see ../🗳️crowd/🟦️.tsx — the figures built from it
 * @see ../../🎨️.css — `.quiz-column`, its paint, its one-time rise and its forced colours
 */

import type { CSSProperties, ReactElement } from "react";
import type { QuizLocale } from "../🌐️i18n/🟦️.ts";

/** 📏️ The share of the plot's height a column of `count` takes on a scale from zero to `full`, within [0, 1]; nothing
 * on a scale that has no extent. */
export function columnShare(count: number, full: number): number {
  return full > 0 ? Math.min(1, Math.max(0, count / full)) : 0;
}

/** 🔝️ The largest of `counts`, zero for none: the scale a histogram's columns share. */
export function peakOf(counts: readonly number[]): number {
  return counts.reduce((peak, count) => Math.max(peak, count), 0);
}

/** 💯️ A share in [0, 1] as a whole percentage in `locale`; a share that is not nothing never reads as nothing (`<1 %`)
 * and one short of everything never as everything (`>99 %`). */
export function formatShare(share: number, locale: QuizLocale): string {
  const percent = new Intl.NumberFormat(locale, { style: "percent", maximumFractionDigits: 0 });
  if (share > 0 && share < 0.005) return `<${percent.format(0.01)}`;
  if (share < 1 && share >= 0.995) return `>${percent.format(0.99)}`;
  return percent.format(share);
}

/** 🧱️ One column: `share` of the plot's height above the baseline, `label` on its cap. A column with a `label` shows
 * at least a sliver, so one answer among thousands is still there to see. */
export function Column(props: { readonly share: number; readonly label?: string; readonly emphasis?: boolean }): ReactElement {
  const { share, label, emphasis } = props;
  return (
    <span aria-hidden="true" className="quiz-column" data-emphasis={emphasis ? "" : undefined} data-filled={label === undefined ? undefined : ""} style={{ ["--quiz-column" as string]: String(share) } as CSSProperties}>
      {label === undefined ? null : <span className="quiz-column-cap quiz-nowrap tabular-nums">{label}</span>}
      <span className="quiz-column-bar" />
    </span>
  );
}
