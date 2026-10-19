/** ⏪️ `time-travel` — the plugin-neutral end-to-end harness of non-destructive history editing. One headless Chromium, one fresh browser context per UI locale (`en`, then `de`, chosen through `navigator.language`), the steps of a plugin scenario and the shared universal journey, each verdict compared against the localized texts of the framework's own history panel.
 *
 * A scenario ({@link TimeTravelScenarioV1}) names the playground it drives, where its serve listens and the steps of its journey; a plugin owns its scenario as data, a repository-relative module whose `timeTravelScenarioV1` export this runner loads for `--scenario <module>`; a run without a scenario is universal and drives any declared playground through the mirror's own controls. Outputs under `--out`: `probe-<renderer>-<stamp>.md` (step lines, verdicts, notes, timeline, console digest), `.ndjson` (one record per verdict/note/dump) and `probe-<renderer>-<stamp>-<locale>-s<step>-<tag>.png|json`; the acceptance record `time-travel`.
 * @see ../../../../../../🔨️modules/⏪️time-travel/🧪️testing/🧩️scenario/🟦️.ts
 * @see ../../../🔌️plugin/⏪️time-travel/🦀️.rs
 * @see ../👥️two-human/🟦️.ts */
import { checkScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withAcceptanceRecord, publishAcceptanceCheckResult, acceptanceCheckResult } from "@semio-tech/repo-test/acceptance/orchestration";
import { timeTravelServeVariantV1 } from "./🚀️boot/🟦️.ts";
import { playgroundCatalog } from "@semio-tech/framework-plugin/build/plan";
import { ensureDevServe, devServePortV1 } from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import { existsSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework-time-travel-ts/testing/scenario";
import { configure, routeUrl, flagValue } from "@semio-tech/framework-time-travel-ts/testing/arguments";
import { serveOrigin, universalRoute, renderer, setUniversalRoute, OUT, base, locales, chordLocales, ndjsonPath } from "@semio-tech/framework-time-travel-ts/testing/state";
import { runProbe } from "./🎪️run/🟦️.ts";

//#region 🔖️Arguments

const CHECK_ID = "time-travel";

/** 🧩️ Loads the scenario a plugin owns from the repository-relative module named by `--scenario`; the module must stay inside the repository and export `timeTravelScenarioV1`. */
async function loadScenario(repoRoot: string, modulePath: string): Promise<TimeTravelScenarioV1<object>> {
  const absolute = resolve(repoRoot, modulePath);
  const inside = relative(repoRoot, absolute);
  if (isAbsolute(modulePath) || inside.startsWith("..") || isAbsolute(inside) || !existsSync(absolute)) throw new Error(`--scenario names a module inside the repository that exists: ${modulePath}`);
  const exported = ((await import(pathToFileURL(absolute).href)) as { timeTravelScenarioV1?: TimeTravelScenarioV1<object> }).timeTravelScenarioV1;
  if (exported === undefined || typeof exported.variant !== "string" || typeof exported.context !== "function") throw new Error(`${modulePath} exports no timeTravelScenarioV1`);
  return exported;
}
//#endregion 🔖️Arguments
//#region 🔖️Main

/** 🚦️ `time-travel [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--chords en,de] [--only 1,2,…]
 * [--folder-at 1|5] [--long-history <mutations>] [--out <dir>] [--scenario <module>] [--universal] [--variant <declared variant>] [--explore]` — reuses the serve answering at `--serve` (the
 * scenario's default per renderer) or starts the catalog-admitted variant there for the run ({@link ensureDevServe}; stopped
 * after it), drives every locale, writes the report under `--out`, publishes the `time-travel` acceptance record (pass only
 * when every verdict passes with no uncaught page error and no hard guest fault) and exits non-zero otherwise. Ctrl-C stops
 * after the running step and still writes the report. */
export async function runTimeTravelCli(invocation: ScriptInvocation, repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  checkScriptInvocation(invocation);
  const scenarioModule = flagValue(segments, "--scenario");
  if (scenarioModule === undefined && !segments.includes("--universal")) throw new Error("a run without a scenario is universal: pass --universal --variant <declared variant> --serve <url>, or --scenario <repository-relative module of the plugin that owns a scenario>");
  const scenarioDefinition = scenarioModule === undefined ? undefined : await loadScenario(repoRoot, scenarioModule);
  configure(segments, defaultOutDir, invocation.control.signal, scenarioDefinition);
  const startedAt = new Date();
  const serveUrl = `${serveOrigin}/`;
  await withAcceptanceRecord(
    repoRoot,
    CHECK_ID,
    async () => {
      let variant: string | null = null;
      const startServe = async () => {
        const selectedVariant = timeTravelServeVariantV1({ universal: universalRoute !== null, url: routeUrl(), renderer, explicit: flagValue(segments, "--variant"), scenarioVariant: scenarioDefinition?.variant }, playgroundCatalog());
        variant = selectedVariant;
        if (universalRoute !== null) {
          const route = new URL(universalRoute);
          route.searchParams.set("plugin", variant);
          setUniversalRoute(route.href);
        }
        return ensureDevServe({ repoRoot, port: devServePortV1(serveUrl), variant, renderer, signal: invocation.control.signal, logPath: join(OUT, `${base}-serve.log`), beforeSpawn: async () => (await import("../../♻️activation/🏃️execution/🟦️.ts")).activatePlaygroundRuntime(invocation, selectedVariant, "prototype", renderer), onProgress: (_status, line) => console.log(line) });
      };
      const serve = await startServe().catch((error: unknown) => (error instanceof Error ? error : new Error(String(error))));
      if (serve instanceof Error) {
        const reason = serve.message.split("\n")[0]!.slice(0, 200);
        publishAcceptanceCheckResult(repoRoot, acceptanceCheckResult({ check: CHECK_ID, status: "blocked", startedAt, measured: { serve: serveUrl, renderer, variant: variant ?? "none", cancelled: invocation.control.signal.aborted }, summary: { en: `no ${renderer} ${variant ?? "declared playground"} serve at ${serveUrl}: ${reason}`, de: `kein ${renderer}-Server für ${variant ?? "den deklarierten Playground"} unter ${serveUrl}: ${reason}` } }));
        process.exitCode = 1;
        return;
      }
      try {
        const run = await runProbe();
        const status = run.verdicts > 0 && run.failed === 0 && run.uncaught === 0 && run.hard === 0 && !invocation.control.signal.aborted ? "pass" : "fail";
        const failing = run.failing.slice(0, 8).join(", ");
        publishAcceptanceCheckResult(
          repoRoot,
          acceptanceCheckResult({
            check: CHECK_ID,
            status,
            startedAt,
            measured: { renderer, variant: variant ?? "none", serve: routeUrl(), locales: locales.join(","), chords: [...chordLocales].join(","), verdicts: run.verdicts, passed: run.passed, failed: run.failed, uncaught: run.uncaught, hard: run.hard, cancelled: invocation.control.signal.aborted },
            summary: {
              en: `${run.passed}/${run.verdicts} time-travel verdicts pass on ${renderer} (${locales.join(", ")}); ${run.uncaught} uncaught page errors, ${run.hard} hard faults${failing ? `; failing: ${failing}` : ""}`.slice(0, 1900),
              de: `${run.passed}/${run.verdicts} Zeitreise-Prüfungen bestehen auf ${renderer} (${locales.join(", ")}); ${run.uncaught} unbehandelte Seitenfehler, ${run.hard} harte Fehler${failing ? `; fehlgeschlagen: ${failing}` : ""}`.slice(0, 1900),
            },
            evidence: [run.mdPath, ndjsonPath],
          }),
        );
        if (status !== "pass") process.exitCode = 1;
      } finally {
        await serve.stop();
      }
    },
    (error) => /ECONNREFUSED|ERR_CONNECTION_REFUSED/u.test(String(error)),
  );
}
//#endregion 🔖️Main
