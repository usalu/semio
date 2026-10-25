/** 🚦️ The CLI configuration of one time-travel run: the flags it reads and the output files it opens. */
import { resolve, join } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import type { TimeTravelScenarioV1 } from "../🧩️scenario/🟦️.ts";
import { renderer, setScenario, setRenderer, setServeOrigin, setOnly, setReloadSelected, setLocales, setChordLocales, setExplore, setWgpuSeedTerminology, setUniversalRoute, universalRoute, setUniversalEditor, setLastBootFault, setFolderAt, setLongHistory, setOUT, OUT, setStamp, setBase, stamp, setNdjsonPath, base, ndjsonPath, setT0, setInvalidatedBy, setCancelled, serveOrigin, scenario } from "../🧠️state/🟦️.ts";
import type { Renderer, Locale } from "../🧠️state/🟦️.ts";
import { verdicts, notes, timeline } from "../🧾️report/🟦️.ts";
import { consoleRows, pageErrors, hardFaults } from "../🎧️console/🟦️.ts";
import { harvestedNotices, harvestedBandFaults } from "../🎗️band/🟦️.ts";

//#region 🔖️Arguments

/** 🏷️ The value after `flag` in `segments` (`--flag value`), or undefined when absent or followed by another flag. */
export const flagValue = (segments: readonly string[], flag: string): string | undefined => {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
};

/** 🧭️ The serve a run drives when `--serve` is absent: the scenario's default for `renderer`. */
const defaultServe = (scenarioDefinition: TimeTravelScenarioV1<object> | undefined, renderer: Renderer): string => {
  const serve = scenarioDefinition?.defaultServes[renderer];
  if (serve === undefined) throw new Error("a run without a scenario names its serve with --serve");
  return serve;
};

/** ⚙️ Seats one run's configuration from its CLI segments, opens its output directory and its ndjson stream. */
export const configure = (segments: readonly string[], defaultOutDir: string, signal: AbortSignal, scenarioDefinition: TimeTravelScenarioV1<object> | undefined) => {
  setScenario(scenarioDefinition);
  setRenderer(flagValue(segments, "--renderer") === "wgpu" ? "wgpu" : "react");
  setServeOrigin(new URL(flagValue(segments, "--serve") ?? defaultServe(scenarioDefinition, renderer)).origin);
  const steps = flagValue(segments, "--only");
  setOnly(steps ? new Set(steps.split(",").map((entry) => Number(entry.trim())).filter(Number.isFinite)) : null);
  setReloadSelected(!steps || steps.split(",").some((entry) => entry.trim() === "reload"));
  setLocales((flagValue(segments, "--locales") ?? "en,de").split(",").map((entry) => entry.trim()).filter((entry): entry is Locale => entry === "en" || entry === "de"));
  setChordLocales(new Set((flagValue(segments, "--chords") ?? "en,de").split(",").map((entry) => entry.trim()).filter(Boolean)));
  setExplore(segments.includes("--explore"));
  setWgpuSeedTerminology(segments.includes("--wgpu-seed-terminology"));
  setUniversalRoute(segments.includes("--universal") ? (flagValue(segments, "--serve") ?? defaultServe(scenarioDefinition, renderer)) : null);
  if (universalRoute !== null && !steps) setOnly(new Set([24, 9]));
  setUniversalEditor(flagValue(segments, "--editor") ?? null);
  setLastBootFault("");
  setFolderAt(Number(flagValue(segments, "--folder-at") ?? "1"));
  setLongHistory(Math.max(1, Number(flagValue(segments, "--long-history") ?? "200") || 200));
  setOUT(resolve(flagValue(segments, "--out") ?? defaultOutDir));
  mkdirSync(OUT, { recursive: true });
  setStamp(new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19));
  setBase(`probe-${renderer}-${stamp}`);
  setNdjsonPath(join(OUT, `${base}.ndjson`));
  writeFileSync(ndjsonPath, "");
  setT0(Date.now());
  setInvalidatedBy(null);
  setCancelled(signal);
  for (const rows of [verdicts, notes, timeline, consoleRows, pageErrors, hardFaults, harvestedNotices, harvestedBandFaults] as unknown[][]) rows.length = 0;
};

/** 🌐️ The route of the serve under test: the universal route, else the scenario's playground at the serve origin. */
export const routeUrl = () => universalRoute ?? `${serveOrigin}/?plugin=${scenario?.variant ?? ""}`;
//#endregion 🔖️Arguments
