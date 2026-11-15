import { readFileSync } from "node:fs";
import { evaluate } from "mathjs";
import Ajv from "ajv";
import cases from "./🧫️cases.json";
import schema from "./🧬️cases.schema.json";

/** 🌡️ Checks authored exercise coverage and independent arithmetic without a runtime solver dependency. */
export function verifyEnergyAssignmentContentV1(): void {
  if (!new Ajv().compile(schema)(cases)) throw Error("Invalid energy exercise cases");
  for (const row of cases.cases) if (Math.abs(Number(evaluate(row.expression)) - row.expected) > 1e-8) throw Error(row.name);
  for (const locale of cases.locales as ("en" | "de")[]) {
    const text = readFileSync(new URL("../" + locale + ".content.tex", import.meta.url), "utf8");
    const shared = readFileSync(new URL("../model.tex", import.meta.url), "utf8");
    const other = locale === "en" ? "de" : "en", decimal = cases.decimal[locale], performance = "\\newcommand{\\EnergyPerformance}{\\mathrm{" + cases.performance[locale] + "}}", marked = "\\newcommand{\\EnergyDecimal}{" + decimal + "}";
    if (!text.includes(marked) || !text.includes(performance)) throw Error(locale + ": locale number conventions required");
    const prose = text.replace(marked, "").replace(performance, ""), neutral = (prose + shared).replaceAll("\\EnergyDecimal", decimal).replaceAll(decimal, ".");
    if (shared.includes("{,}") || shared.split("$").some((part, index) => index % 2 === 1 && /\d\.\d/.test(part)) || (decimal === "." ? prose.includes("{,}") : /\d\.\d/.test(prose))) throw Error(locale + ": foreign decimal mark");
    if (prose.includes(cases.performance[other])) throw Error(locale + ": foreign performance factor abbreviation");
    for (const token of cases.required) if (!neutral.includes(token)) throw Error(locale + ": missing " + token);
    if (/20\.11|archday|bis nächsten|VORLAGE in Aufgabe/.test(text)) throw Error("Historical slide logistics leaked into assignment");
    if (!text.includes("\\clearpage") || !text.includes("\\section")) throw Error("Structured assignment required");
  }
}

if (import.meta.main) verifyEnergyAssignmentContentV1();
