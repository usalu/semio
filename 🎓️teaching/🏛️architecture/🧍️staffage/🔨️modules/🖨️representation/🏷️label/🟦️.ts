/** 🏷️ The label a user interface shows for a resolved scale, in both languages: exact drawings are named plainly, derived uses name the source drawing and its valid range, unavailable and unsuitable requests say so and list the supported scales.
 *
 * @see ../🟦️.ts — `resolveRepresentation`
 * @see ../../🔗️contracts/🟦️.ts — `RepresentationResolution`
 */
import type { Language, Text } from "../../../🧬️schema/🟦️.ts";
import type { RepresentationResolution } from "../../🔗️contracts/🟦️.ts";

const scales = (list: readonly number[]): string => (list.length > 0 ? list.map((d) => `1:${d}`).join(", ") : "–");

/** 🏷️ The bilingual label of a resolution. */
export function resolutionLabel(resolution: RepresentationResolution): Text {
  switch (resolution.kind) {
    case "authored":
      return resolution.exact
        ? { en: `Drawing authored for 1:${resolution.denominator}`, de: `Für 1:${resolution.denominator} gezeichnet` }
        : {
            en: `Derived: 1:${resolution.requested} uses the drawing authored for 1:${resolution.denominator} (valid ${resolution.representation.validRange.minDenominator}…${resolution.representation.validRange.maxDenominator}); paper size follows 1:${resolution.requested}`,
            de: `Abgeleitet: 1:${resolution.requested} nutzt die für 1:${resolution.denominator} gezeichnete Darstellung (gültig ${resolution.representation.validRange.minDenominator}…${resolution.representation.validRange.maxDenominator}); die Papiergröße folgt 1:${resolution.requested}`,
          };
    case "unavailable":
      return { en: `Not available at 1:${resolution.requested}: ${resolution.reason.en} Supported scales: ${scales(resolution.supported)}.`, de: `Bei 1:${resolution.requested} nicht verfügbar: ${resolution.reason.de} Unterstützte Maßstäbe: ${scales(resolution.supported)}.` };
    case "unsuitable":
      return { en: `Not printable at 1:${resolution.requested} with profile ${resolution.profile}: ${resolution.suitability.violations.length} violations. Supported scales: ${scales(resolution.supported)}.`, de: `Bei 1:${resolution.requested} mit Profil ${resolution.profile} nicht druckbar: ${resolution.suitability.violations.length} Verletzungen. Unterstützte Maßstäbe: ${scales(resolution.supported)}.` };
    case "symbol":
      return { en: `Symbol "${resolution.symbol.title.en}" (${resolution.symbol.semantics}); its size follows the symbol's own semantics, not 1:${resolution.requested}`, de: `Symbol „${resolution.symbol.title.de}“ (${resolution.symbol.semantics}); die Größe folgt der Symbol-Semantik, nicht 1:${resolution.requested}` };
  }
}

/** 🏷️ The label in one language. */
export function resolutionText(resolution: RepresentationResolution, language: Language): string {
  return resolutionLabel(resolution)[language];
}
