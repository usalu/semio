import { readFileSync } from "node:fs";
import { evaluate } from "mathjs";
import Ajv from "ajv";
import cases from "./🧫️cases.json";
import schema from "./🧬️cases.schema.json";

const TEX_LABEL_ESCAPES: readonly (readonly [string, string])[] = [["_", "\\_"], ["^", "\\textasciicircum{}"], ["ρ", "$\\rho$"], ["ΔΘ", "$\\Delta\\Theta$"], ["'", "\\textquotesingle{}"], ["…", "\\ldots{}"]];

/** 🌡️ Checks exact geometry, the exact CSV template listing, the informal German address and the absence of prescribed energy values without a runtime solver dependency. */
export function verifyEnergyAssignmentContentV1(): void {
  if (!new Ajv().compile(schema)(cases)) throw Error("Invalid energy exercise cases");
  for (const row of cases.cases) if (Math.abs(Number(evaluate(row.expression)) - row.expected) > 1e-8) throw Error(row.name);
  const shared = readFileSync(new URL("../model.tex", import.meta.url), "utf8");
  const template = readFileSync(new URL("../" + cases.template.file, import.meta.url), "utf8");
  const rows = template.split("\r\n");
  if (!template.startsWith("﻿") || rows.pop() !== "" || rows.length !== cases.template.rows || rows.some(row => !row.endsWith(";") || row.indexOf(";") !== row.length - 1)) throw Error("CSV template: BOM, CRLF and one empty value per row required");
  rows.forEach((row, index) => {
    const label = TEX_LABEL_ESCAPES.reduce((text, [from, to]) => text.replaceAll(from, to), row.slice(index === 0 ? 1 : 0, -1).trimEnd());
    if (!shared.includes("\n" + (index + 1) + "&" + label + "\\\\\n")) throw Error("CSV listing row " + (index + 1));
  });
  if (shared.includes("{,}") || shared.split("$").some((part, index) => index % 2 === 1 && /\d\.\d/.test(part))) throw Error("Shared model: foreign decimal mark");
  const cover = shared.slice(shared.indexOf("\\newcommand{\\EnergyCoverAxonometry}"), shared.indexOf("\\newcommand{\\EnergyElements}"));
  if (!cover.includes("\\EnergyStaffagePerson") || /\\EnergyN|\\EnergyWord|dashed|\$|node/.test(cover)) throw Error("Cover axonometry: staffage without dimensions, labels and hidden edges required");
  const technical = shared.slice(shared.indexOf("\\newcommand{\\EnergyAxonometry}"), shared.indexOf("\\newcommand{\\EnergyAxonometryFigure}"));
  if (technical.includes("\\EnergyStaffage") || !technical.includes("\\EnergyN{9}{0}") || !technical.includes("dashed")) throw Error("Geometry axonometry: technical drawing with dimensions required");
  for (const locale of cases.locales as ("en" | "de")[]) {
    const text = readFileSync(new URL("../" + locale + ".content.tex", import.meta.url), "utf8");
    const decimal = cases.decimal[locale], marked = "\\newcommand{\\EnergyDecimal}{" + decimal + "}";
    if (!text.includes(marked)) throw Error(locale + ": locale decimal mark required");
    const prose = text.replace(marked, ""), neutral = (prose + shared).replace(/\\EnergyN\{(\d+)\}\{(\d+)\}/g, "$1.$2").replace(/\\EnergyRange\{(\d+)\}\{(\d+)\}\{(\d+)\}\{(\d+)\}/g, "$1.$2 $3.$4").replace(/(\d)\\EnergyDecimal (\d)/g, "$1.$2").replaceAll(decimal, ".");
    if (decimal === "." ? prose.includes("{,}") : /\d\.\d/.test(prose)) throw Error(locale + ": foreign decimal mark");
    for (const token of cases.required) if (!neutral.includes(token)) throw Error(locale + ": missing " + token);
    for (const token of cases.withheld) if (neutral.includes(token)) throw Error(locale + ": prescribed energy value " + token);
    for (const token of cases.forbidden[locale]) if (prose.includes(token)) throw Error(locale + ": forbidden form of address " + token);
    if (/20\.11|19\.11|archda|bis nächsten|VORLAGE in Aufgabe/.test(text)) throw Error("Historical slide logistics leaked into assignment");
    if (!text.includes("\\makeassignmentcover") || !text.includes("cover-content={\\EnergyCoverAxonometry}") || !text.includes("\\section")) throw Error("Structured assignment with cover required");
    for (const macro of ["\\EnergyAxonometryFigure", "\\EnergyElements", "\\EnergyCsvTemplate"]) if (!text.includes(macro)) throw Error(locale + ": missing " + macro);
  }
}

if (import.meta.main) verifyEnergyAssignmentContentV1();
