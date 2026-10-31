/** 🪧️ The explanation box shown instead of a drawing when a scale is unavailable or unsuitable: what is not drawn and why, the limiting feature, the measured violations, the supported scales and the alternatives.
 *
 * It is plain SVG markup in paper millimetres (a framed box with wrapped text), usable inline next to real drawings; nothing is drawn silently in its place.
 *
 * @see ../../🖨️representation/🟦️.ts — `resolveRepresentation`
 * @see ../../🔗️contracts/🟦️.ts — `RepresentationResolution`
 */
import type { Alternative, Asset, Language } from "../../../🧬️schema/🟦️.ts";
import type { RepresentationResolution } from "../../🔗️contracts/🟦️.ts";
import { ANNOTATION_TEXT_MM, LABEL_TEXT_MM, formatNumber } from "../../📐️units/🟦️.ts";
import { wrap } from "../🖍️palette/🟦️.ts";

/** 🪧️ A rendered box: size in paper millimetres and its inner markup (origin top-left). */
export type ExplanationBox = { readonly widthMm: number; readonly heightMm: number; readonly lines: readonly { readonly text: string; readonly bold: boolean }[]; readonly markup: string };

const FONT_STACK = "Anta, 'Segoe UI', 'Helvetica Neue', Helvetica, Arial, 'DejaVu Sans', sans-serif";
const escape = (text: string): string => text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;");

function alternativeText(asset: Asset, alternative: Alternative, language: Language): string {
  if (alternative.kind === "scale") return language === "de" ? `Zeichnung bei 1:${alternative.denominator} verwenden` : `Use the drawing at 1:${alternative.denominator}`;
  if (alternative.kind === "symbol") {
    const symbol = asset.symbols.find((candidate) => candidate.id === alternative.symbol);
    return language === "de" ? `Symbol „${symbol?.title.de ?? alternative.symbol}“ verwenden` : `Use the symbol "${symbol?.title.en ?? alternative.symbol}"`;
  }
  return language === "de" ? `Asset „${alternative.asset}“ verwenden` : `Use the asset "${alternative.asset}"`;
}

/** 🪧️ The box for an `unavailable` or `unsuitable` resolution of an asset in a language, `widthMm` wide (default 80 mm) with text of the annotation height. */
export function explanationBox(asset: Asset, resolution: Extract<RepresentationResolution, { readonly kind: "unavailable" | "unsuitable" }>, language: Language, widthMm = 80): ExplanationBox {
  const de = language === "de";
  const out: { text: string; bold: boolean }[] = [];
  const push = (text: string, bold = false): void => {
    for (const line of wrap(text, Math.floor((widthMm - 4) / (LABEL_TEXT_MM * 0.56)))) out.push({ text: line, bold });
  };
  const mm = (value: number): string => formatNumber(value, 2, language);
  if (resolution.kind === "unavailable") {
    push(de ? `Bei 1:${resolution.requested} nicht dargestellt` : `Not drawn at 1:${resolution.requested}`, true);
    push(resolution.reason[language]);
    if (resolution.limitingFeature !== null && resolution.limitingSizeM !== null) push(`${de ? "Begrenzendes Merkmal" : "Limiting feature"}: ${resolution.limitingFeature[language]} (${formatNumber(resolution.limitingSizeM * 1000, 1, language)} mm ${de ? "real" : "real"}, ${mm((resolution.limitingSizeM * 1000) / resolution.requested)} mm ${de ? "auf Papier" : "on paper"})`);
  } else {
    push(de ? `Bei 1:${resolution.requested} nicht druckbar (${resolution.profile})` : `Not printable at 1:${resolution.requested} (${resolution.profile})`, true);
    for (const violation of resolution.suitability.violations.slice(0, 5)) {
      const names = { stroke: de ? "Strichstärke" : "Stroke", gap: de ? "Lücke" : "Gap", feature: de ? "Einzelheit" : "Feature", hatch: de ? "Schraffurabstand" : "Hatch spacing" }[violation.check];
      push(`${names} ${violation.shapes.join(" / ")}: ${mm(violation.measuredMm)} mm < ${mm(violation.requiredMm)} mm`);
    }
    if (resolution.suitability.violations.length > 5) push(de ? `… und ${resolution.suitability.violations.length - 5} weitere` : `… and ${resolution.suitability.violations.length - 5} more`);
  }
  push(`${de ? "Verfügbare Maßstäbe" : "Available scales"}: ${resolution.supported.length ? resolution.supported.map((d) => `1:${d}`).join(", ") : "–"}`);
  for (const alternative of resolution.alternatives) push(`→ ${alternativeText(asset, alternative, language)}`);
  const pitch = LABEL_TEXT_MM * 1.45;
  const heightMm = out.length * pitch + 4;
  const text = out.map((line, i) => `<text x="2" y="${(2 + (i + 0.85) * pitch).toFixed(3)}" font-family="${FONT_STACK}" font-size="${line.bold ? ANNOTATION_TEXT_MM : LABEL_TEXT_MM}"${line.bold ? ` font-weight="bold"` : ""} fill="#001117">${escape(line.text)}</text>`).join("");
  const markup = `<rect x="0.1" y="0.1" width="${widthMm - 0.2}" height="${(heightMm - 0.2).toFixed(3)}" fill="#fff8e1" stroke="#001117" stroke-width="0.25" stroke-dasharray="1.5 1"/>${text}`;
  return { widthMm, heightMm, lines: out, markup };
}
