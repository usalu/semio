/** 🧾️ The report of one run: verdicts, notes, the timeline and the ndjson stream. */
import { appendFileSync } from "node:fs";
import { ndjsonPath, t0, currentLocale, currentStep, setCurrentStep } from "../🧠️state/🟦️.ts";
import type { Locale } from "../🧠️state/🟦️.ts";

//#region 🔖️Report

type VerdictRow = { locale: Locale; step: number; name: string; ok: boolean; detail: Record<string, unknown> };

export const verdicts: VerdictRow[] = [];

export const notes: string[] = [];

export const timeline: string[] = [];

/** 📮️ One ndjson record, stamped with the run clock, the locale and the step. */
export const emit = (record: Record<string, unknown>) => appendFileSync(ndjsonPath, `${JSON.stringify({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, ...record })}\n`);

/** 🕰️ One timeline line, printed and kept for the markdown report. */
export const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${currentLocale}/${currentStep} ${message}`;
  timeline.push(row);
  console.log(row);
};

/** ⚖️ One PASS/FAIL verdict of the current locale and step. */
export const verdict = (name: string, ok: boolean, detail: Record<string, unknown> = {}) => {
  verdicts.push({ locale: currentLocale, step: currentStep, name, ok, detail });
  emit({ kind: "verdict", name, verdict: ok ? "PASS" : "FAIL", ...detail });
  log(`${ok ? "PASS" : "FAIL"} ${name} ${JSON.stringify(detail).slice(0, 600)}`);
  return ok;
};

/** 🔖️ Attributes the verdicts `record` makes to `step` — a check taken later in the run than the step whose gap it proves. */
export const atStep = <T,>(step: number, record: () => T) => {
  const previous = currentStep;
  setCurrentStep(step);
  try {
    return record();
  } finally {
    setCurrentStep(previous);
  }
};

/** 📝️ An observation that is neither PASS nor FAIL (timing, a design reading, an inventory). */
export const note = (name: string, detail: Record<string, unknown> = {}) => {
  const row = `${currentLocale}/${currentStep} ${name} ${JSON.stringify(detail).slice(0, 900)}`;
  notes.push(row);
  emit({ kind: "note", name, ...detail });
  log(`NOTE ${name} ${JSON.stringify(detail).slice(0, 600)}`);
};
//#endregion 🔖️Report
