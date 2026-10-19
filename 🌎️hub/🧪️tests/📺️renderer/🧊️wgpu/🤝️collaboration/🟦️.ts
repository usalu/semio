import { type AddressInfo, createServer } from "node:net";
import { devServePortV1, ensureDevServe } from "@semio-tech/framework-os-dev/local-hub/execution";
import { join, resolve } from "node:path";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { hubProbeSignIn } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { ensureParityPlaywrightBrowsersPath } from "@semio-tech/framework-os-dev/parity/execution";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "@semio-tech/repo-test/acceptance/orchestration";
import { HUB_COLLABORATION_CHECKS, HUB_COLLABORATION_KINDS, HubCollaborationBlocked, hubCollaborationHumans, type HubCollaborationJourneyV1, type HubCollaborationOptionsV1, type HubCollaborationReportV1 } from "./📋️contract/🟦️.ts";
import { prepareDocument } from "./🧭️native-law-and-follow/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "@semio-tech/framework-plugin/build/plan";
import { JOURNEY_BODIES } from "./🎬️journeys/🟦️.ts";

/** 🔌️ A free loopback port for a serve this run starts itself. */
async function freeLoopbackPort(): Promise<number> {
  const probe = createServer();
  await new Promise<void>((ready, fail) => probe.once("error", fail).listen(0, "127.0.0.1", ready));
  const { port } = probe.address() as AddressInfo;
  await new Promise<void>((closed) => probe.close(() => closed()));
  return port;
}

/** 🛎️ The serve a journey drives, through S18's shared `ensureDevServe`: the one `--serve` / `--react-serve` names (reused when it
 * answers, started on that port otherwise), or — flag omitted — one started on a free loopback port (the wgpu release serve of
 * the journey's variant, the React `s` dev serve); only what this run started is stopped at its end. A serve that cannot be had
 * is a missing precondition (`blocked`). */
async function provisionServe(options: HubCollaborationOptionsV1, url: string | null, shell: Readonly<{ variant: string; renderer: "wgpu" | "react"; profile: "release" | "prototype" }>, cleanups: (() => Promise<void>)[]): Promise<string> {
  const port = url ? devServePortV1(url) : await freeLoopbackPort();
  const fixture = await ensureDevServe({ repoRoot: options.repoRoot, port, variant: shell.variant, renderer: shell.renderer, profile: shell.profile, hubUrl: options.hub, locale: options.locale, signal: options.signal, onProgress: (_status, line) => console.log(line) }).catch((error: unknown) => {
    throw new HubCollaborationBlocked(`${shell.renderer} serve on port ${port}: ${error instanceof Error ? error.message : String(error)}`);
  });
  if (!fixture.reused) cleanups.push(() => fixture.stop());
  return url ?? (shell.renderer === "wgpu" ? `${fixture.url}?plugin=${shell.variant}` : fixture.url);
}

/** 🌐️ Refuses a hub or serve that does not answer: a missing precondition, never a failed journey. */
async function requireAnswering(url: string, what: string): Promise<void> {
  const answers = await fetch(url, { signal: AbortSignal.timeout(10_000) }).then((response) => response.status < 500, () => false);
  if (!answers) throw new HubCollaborationBlocked(`${what} ${url} does not answer`);
}

/** 🚦️ Runs one journey and writes `<out>/<tag>/report.json` (+ console, hub lines, screenshots, native log). */
export async function runHubCollaboration(options: HubCollaborationOptionsV1): Promise<HubCollaborationReportV1> {
  const directory = join(options.outDir, options.tag);
  rmSync(directory, { recursive: true, force: true });
  mkdirSync(directory, { recursive: true });
  const started = Date.now();
  const clock = (): number => Date.now() - started;
  const report: HubCollaborationReportV1 = { schema: "semio.wgpu.hub-collaboration-report/v1", journey: options.journey, locale: options.locale, hub: options.hub, spaceId: null, documentId: null, startedAt: new Date(started).toISOString(), finishedAt: null, steps: [], measured: {}, fatal: null };
  const flush = (): void => writeFileSync(join(directory, "report.json"), JSON.stringify(report, null, 2));
  const record = (name: string, pass: boolean, detail: unknown): void => {
    report.steps.push({ name, pass, atMs: clock(), detail });
    console.log(`[hub-collaboration] ${pass ? "PASS" : "FAIL"} ${name} +${(clock() / 1000).toFixed(1)}s ${JSON.stringify(detail).slice(0, 400)}`);
    flush();
  };
  await requireAnswering(`${options.hub}/readyz`, "hub");
  const kind = HUB_COLLABORATION_KINDS[options.journey];
  const cleanups: (() => Promise<void>)[] = [];
  const document = { spaceId: "", documentId: "" };
  try {
    const serving: HubCollaborationOptionsV1 = {
      ...options,
      wgpuServe: kind.wgpu ? await provisionServe(options, options.wgpuServe, { variant: kind.variant, renderer: "wgpu", profile: "release" }, cleanups) : null,
      reactServe: kind.react ? await provisionServe(options, options.reactServe, { variant: "s", renderer: "react", profile: "prototype" }, cleanups) : null,
    };
    const token = await hubProbeSignIn(options.hub, options.humans[0].email, options.humans[0].password, "wgpucollab");
    if (!kind.native) {
      const prepared = await prepareDocument(options, kind.schema, token);
      Object.assign(document, { spaceId: prepared.spaceId, documentId: prepared.documentId });
      report.measured.documentCreatedMs = prepared.createdMs;
    }
    ensureParityPlaywrightBrowsersPath();
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
    cleanups.push(() => browser.close());
    await JOURNEY_BODIES[options.journey]({ options: serving, browser, record, measured: report.measured, clock, out: (name) => join(directory, name), document, onCleanup: (cleanup) => cleanups.push(cleanup), token });
  } catch (error) {
    if (error instanceof HubCollaborationBlocked) throw error;
    report.fatal = String(error instanceof Error ? (error.stack ?? error.message) : error).slice(0, 1_500);
    console.log(`[hub-collaboration] FATAL ${report.fatal}`);
  } finally {
    for (const cleanup of cleanups.reverse()) await cleanup().catch(() => undefined);
    report.spaceId = document.spaceId || null;
    report.documentId = document.documentId || null;
    report.finishedAt = new Date().toISOString();
    flush();
  }
  return report;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `collaboration-acceptance --journey <wasm32|wasm32-react|wasm32-native|native-react|native-react-cursors|cursors|agent-pixels>
 * --hub <url> [--serve <wgpu serve url>] [--react-serve <url>] [--locale en|de] [--space <id> --document <id>] (an omitted serve is
 * started by the run itself on a free loopback port and stopped at its end)
 * [--native-binary <path>] [--native-modules <dir>] [--tag <t>] [--out <dir>]` — runs one journey, publishes its acceptance
 * record (`<check>-<locale>`), exits non-zero unless every step passes. */
export async function runHubCollaborationCli(repoRoot: string, defaultOutDir: string, segments: readonly string[], defaults: Readonly<{ nativeModules?: () => string }> = {}): Promise<void> {
  const journey = flagValue(segments, "--journey") as HubCollaborationJourneyV1 | undefined;
  if (!journey || !(journey in HUB_COLLABORATION_CHECKS)) throw new Error(`usage: collaboration-acceptance --journey <${Object.keys(HUB_COLLABORATION_CHECKS).join("|")}> --hub <url> [--serve <url>] [--react-serve <url>] [--locale en|de]`);
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const check = `${HUB_COLLABORATION_CHECKS[journey]}-${locale}`;
  const tag = flagValue(segments, "--tag") ?? check;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  await withAcceptanceRecord(
    repoRoot,
    check,
    async () => {
      const hub = flagValue(segments, "--hub");
      if (!hub) throw new HubCollaborationBlocked("--hub is required");
      const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
      const native = HUB_COLLABORATION_KINDS[journey].native;
      const report = await runHubCollaboration({
        repoRoot,
        journey,
        hub: hub.replace(/\/$/u, ""),
        wgpuServe: flagValue(segments, "--serve") ?? null,
        reactServe: flagValue(segments, "--react-serve") ?? null,
        locale,
        humans: hubCollaborationHumans(),
        spaceId: flagValue(segments, "--space") ?? null,
        documentId: flagValue(segments, "--document") ?? null,
        nativeBinary: flagValue(segments, "--native-binary") ?? null,
        nativeModules: flagValue(segments, "--native-modules") ?? (native ? (defaults.nativeModules?.() ?? null) : null),
        outDir,
        tag,
        signal: controller.signal,
      });
      const passed = report.steps.filter((step) => step.pass).length;
      const failing = report.steps.filter((step) => !step.pass).map((step) => step.name);
      const status = !report.fatal && report.steps.length > 0 && failing.length === 0 ? "pass" : "fail";
      publishAcceptanceCheckResult(
        repoRoot,
        acceptanceCheckResult({
          check,
          status,
          startedAt,
          measured: { ...report.measured, locale, steps: report.steps.length, passed, failed: failing.length, fatal: Boolean(report.fatal) },
          summary: {
            en: `${passed}/${report.steps.length} steps of the wgpu ${journey} collaboration journey pass in ${locale}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; stopped: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
            de: `${passed}/${report.steps.length} Schritte des wgpu-Zusammenarbeitswegs ${journey} bestehen in ${locale}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; abgebrochen: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
          },
          evidence: [join(outDir, tag, "report.json")],
        }),
      );
      if (status !== "pass") process.exitCode = 1;
    },
    (error) => error instanceof HubCollaborationBlocked,
  );
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
