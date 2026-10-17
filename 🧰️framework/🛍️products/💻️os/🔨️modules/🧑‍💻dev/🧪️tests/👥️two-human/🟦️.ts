/** 👥️ Two humans, one hub, every kind the hub can create — driven in two isolated browser profiles of the React `s` shell.
 *
 * Per kind the hub's creation catalog offers: A creates it from the Space app (the creation saga opens it for A), B opens it
 * from the Space index, both presence rosters hold two peers, every document window takes focus with no owner fault, A
 * edits → B sees it, B edits → A sees it, A undoes their OWN edit (B's stays), B undoes theirs (both back to the start), B
 * redoes (both see it again), both reload and reopen and converge. One row per kind; per-kind faults; screenshots on failure.
 * The document is witnessed as the human sees it (window-body text plus element / SVG / canvas counts and a key digest)
 * and, when an admin capability file is given, as the hub holds it (`headSeq`). Locale: the browser language both profiles
 * boot in (`en` → `en-US`, `de` → `de-DE`).
 *
 * Promoted from the session-12/13 ticket harness `wp-c10`/`wp-c11` (`c11-collab-matrix.mjs`, `c11-lib.mjs`,
 * `c11-journey.mjs`, ticket 26/09/23); the edit verbs and staged arguments are the program matrix's pins.
 * @see ../🧮️program-matrix/🟦️.ts
 * @see ../🤝️collaboration/🟦️.ts — the fresh-local-hub collaboration story
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { readMatrixPins, withDevServe } from "../🧮️program-matrix/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "@semio-tech/repo-test/acceptance/orchestration";
import { activate, boot, clickRowAction, dialog, faultsSince, type Human, openSessions, pause, readStatus, selectOption, type Session, signIn, started, submitDialog, until, waitNamedRow, waitRow } from "./🔑️sessions/🟦️.ts";
import { awaitMounted, awaitSharedSpace, creatableKinds, createArtifact, openSpace, PLUGIN_BY_KIND, settleHome } from "./🏠️space-creation-and-edits/🟦️.ts";
import type { KindRow, KindTimings, TwoHumanJourney, TwoHumanOptions, TwoHumanReport } from "./📋️run-types/🟦️.ts";
import { JOURNEYS, seedSharedSpaceThroughHub } from "./🧭️journeys/🟦️.ts";

//#region 🔖️TwoHuman
/** 👥️ Runs the two-human journey over every selected creatable kind; the report is flushed after every kind and the
 * signal ends the run after the current kind. */
export async function runTwoHuman(options: TwoHumanOptions): Promise<TwoHumanReport> {
  const pins = readMatrixPins();
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const browserLocale = options.locale === "de" ? "de-DE" : "en-US";
  const report: TwoHumanReport = { tag: options.tag, journey: options.journey, hub: options.hub, serves: options.serves, locale: browserLocale, startedAt: new Date().toISOString(), spaceId: options.spaceId, kinds: [], rows: [], misses: [] };
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
  const sessions = await openSessions(browser, options.serves, options.humans, browserLocale);
  const [A, B] = sessions as [Session, Session];
  const flush = (): void => {
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
    writeFileSync(join(outDir, "console.txt"), sessions.flatMap((session) => session.lines.map((line) => `${session.human.label} ${line}`)).join("\n"));
  };
  const shot = (session: Session, name: string): Promise<void> => session.page.screenshot({ path: join(outDir, `${name}-${session.human.label}.png`) }).then(() => undefined, () => undefined);
  const log = (line: string): void => console.log(`[two-human] ${line}`.slice(0, 1600));
  try {
    for (const session of [A, B]) await boot(session);
    for (const session of [A, B]) await signIn(session);
    for (const session of [A, B]) await settleHome(session);
    let spaceId = options.spaceId;
    if (spaceId === null && JOURNEYS[options.journey].seed === "hub") {
      spaceId = await seedSharedSpaceThroughHub(options.hub, A.human, B.human.email, JOURNEYS[options.journey].memberRole, `Two Human ${options.tag} ${Date.now() % 100000}`);
    } else if (spaceId === null) {
      const spaceName = `Two Human ${options.tag} ${Date.now() % 100000}`;
      await activate(A.page, "s-home-create-space");
      await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
      await A.page.locator("#name").fill(spaceName);
      await selectOption(A.page, "kind", /studio/iu);
      await selectOption(A.page, "visibility", /public|öffentlich/iu);
      await submitDialog(A.page);
      spaceId = await waitNamedRow(A.page, "space", spaceName, 90_000);
      await clickRowAction(A.page, "space", spaceId, /^(share|teilen)\b/iu);
      await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
      await A.page.locator("#email").fill(B.human.email);
      await selectOption(A.page, "role", JOURNEYS[options.journey].shareRole);
      await submitDialog(A.page);
      await awaitSharedSpace(B, spaceId, report.misses);
    }
    report.spaceId = spaceId;
    log(`space ${spaceId}`);
    await openSpace(A, spaceId, report.misses);
    await openSpace(B, spaceId, report.misses);
    const kinds = (await creatableKinds(A)).filter((kind) => options.kinds.length === 0 || options.kinds.includes(kind.kindId));
    report.kinds = kinds.map((kind) => kind.kindId);
    log(`kinds ${report.kinds.join(", ")}`);
    flush();
    for (const [index, kind] of kinds.entries()) {
      if (options.signal.aborted) {
        report.cancelled = true;
        log("cancelled; the report keeps every finished kind");
        break;
      }
      const cursors = [A.lines.length, B.lines.length] as const;
      const row: KindRow = { kindId: kind.kindId, label: kind.label, checks: {}, faults: { A: [], B: [] }, pass: false };
      const check = (name: string, pass: boolean, detail: unknown): void => {
        row.checks[name] = { pass, detail };
        log(`  ${kind.kindId} ${name}: ${pass ? "PASS" : "FAIL"} ${typeof detail === "string" ? detail : JSON.stringify(detail)}`);
      };
      try {
        await openSpace(A, spaceId, report.misses);
        const createdAt = Date.now();
        const artifactId = await createArtifact(A, `Two Human ${kind.kindId} ${Date.now() % 100000}`, kind, options.mountBudgetMs);
        row.artifactId = artifactId;
        const shellA = await awaitMounted(A, options.mountBudgetMs);
        row.timings = { createToMountedMs: shellA.mountedAt - createdAt, createStartedAtMs: createdAt - started };
        const plugin = /^s\.([a-z0-9-]+)\./u.exec(String((JSON.parse(kind.value) as { dialect?: { artifactKind?: string } }).dialect?.artifactKind ?? ""))?.[1] ?? PLUGIN_BY_KIND[kind.kindId] ?? "";
        row.plugin = plugin;
        check("A creates + opens", true, { artifactId, windows: shellA.windowIds });
        await openSpace(B, spaceId, report.misses);
        await waitRow(B.page, "artifact", artifactId, 90_000);
        const openedAt = Date.now();
        await clickRowAction(B.page, "artifact", artifactId, /^(open|öffnen)\b/iu);
        const shellB = await awaitMounted(B, options.mountBudgetMs);
        row.timings = { ...row.timings, openRowToMountedMs: shellB.mountedAt - openedAt, openStartedAtMs: openedAt - started };
        check("B opens via Space index", true, { windows: shellB.windowIds });
        const presence = await until(async () => {
          const [a, b] = [await readStatus(A.page), await readStatus(B.page)];
          return a.peers.length === 2 && b.peers.length === 2 ? { a: a.peerLabels, b: b.peerLabels } : null;
        }, 45_000);
        check("presence 2/2", presence !== null, presence ?? { a: (await readStatus(A.page)).peers, b: (await readStatus(B.page)).peers });
        const windowCursor = [A.lines.length, B.lines.length] as const;
        for (const session of [A, B])
          for (const windowId of shellA.windowIds) {
            await session.page.locator(`[data-window-id="${windowId}"]`).first().click({ position: { x: 40, y: 8 }, force: true }).catch(() => undefined);
            await pause(session, 800);
          }
        const windowFaults = [...faultsSince(A, windowCursor[0]), ...faultsSince(B, windowCursor[1])];
        check("every window takes focus and commands", windowFaults.length === 0 && shellA.windowIds.length > 0, { windows: shellA.windowIds, faults: windowFaults.slice(0, 4) });
        const time = (name: keyof KindTimings, value: number | null): void => {
          if (value !== null) row.timings = { ...row.timings, [name]: value };
        };
        await JOURNEYS[options.journey].run({ A, B, check, time, plugin, pins, options, spaceId, artifactId, misses: report.misses });
      } catch (error) {
        check("journey", false, String(error instanceof Error ? error.message : error).slice(0, 600));
        await shot(A, `${kind.kindId}-fail`);
        await shot(B, `${kind.kindId}-fail`);
      }
      row.faults = { A: faultsSince(A, cursors[0]), B: faultsSince(B, cursors[1]) };
      row.pass = Object.values(row.checks).every((entry) => entry.pass) && Object.keys(row.checks).length >= JOURNEYS[options.journey].checks;
      report.rows.push(row);
      flush();
      log(`${index + 1}/${kinds.length} ${row.pass ? "PASS" : "FAIL"} ${kind.kindId} (${Object.values(row.checks).filter((entry) => entry.pass).length}/${Object.keys(row.checks).length} checks, faults A ${row.faults.A.length} B ${row.faults.B.length})`);
    }
  } catch (error) {
    report.fatal = String(error instanceof Error ? (error.stack ?? error.message) : error).slice(0, 1200);
    log(`FATAL ${report.fatal}`);
    await shot(A, "run-fail");
    await shot(B, "run-fail");
  } finally {
    report.finishedAt = new Date().toISOString();
    flush();
    await browser.close();
  }
  return report;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🔑️ The local development hub's two credential users, the ones every hub gate signs in as by default. */
const DEVELOPMENT_HUMANS: readonly [Human, Human] = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

/** 🔑️ The two humans: `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` (what the goal gate hands over), else a non-empty
 * `--users` JSON file `{"users":[{"email","password"},…]}`, else the development hub's two users. Never an argument. */
function readHumans(segments: readonly string[]): readonly [Human, Human] {
  const fromEnv = [1, 2].map((index) => ({ label: `user${index}`, email: process.env[`SEMIO_TWO_HUMAN_USER${index}_EMAIL`] ?? "", password: process.env[`SEMIO_TWO_HUMAN_USER${index}_PASSWORD`] ?? "" }));
  if (fromEnv.every((human) => human.email && human.password)) return fromEnv as unknown as readonly [Human, Human];
  const file = flagValue(segments, "--users");
  if (!file) return DEVELOPMENT_HUMANS;
  const users = (JSON.parse(readFileSync(resolve(file), "utf8")) as { users?: { email: string; password: string }[] }).users ?? [];
  if (users.length < 2) throw new Error(`--users ${file} must list two users`);
  return [{ label: "user1", ...users[0]! }, { label: "user2", ...users[1]! }];
}

/** 🚪️ `verify two-human --hub <url> --serve <url> [--serve-b <url>] [--journey edit|viewer|cross-undo] [--locale en|de] [--kinds <kindId,…>]
 * [--space <id>] [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>] [--max-create-to-mounted-ms <n>]
 * [--max-open-to-mounted-ms <n>] [--mount-budget-ms <n>]` — runs the journey (a creation or an open waits up to
 * `--mount-budget-ms`, default 15 min, for the document to mount; the latency bounds judge it), writes `report.json`,
 * `console.txt` and failure screenshots under `<out>/<tag>/`, publishes the acceptance record, exits non-zero unless every
 * kind passes. */
export async function runTwoHumanCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const hub = flagValue(segments, "--hub");
  const serve = flagValue(segments, "--serve");
  if (!hub || !serve) throw new Error("usage: verify two-human --hub <url> --serve <url> [--serve-b <url>] [--journey edit|viewer|cross-undo] [--locale en|de] [--kinds …] [--space <id>] [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>]");
  const journey = flagValue(segments, "--journey") ?? "edit";
  if (!(journey in JOURNEYS)) throw new Error(`verify two-human: unknown --journey ${journey} (${Object.keys(JOURNEYS).join("|")})`);
  const { check } = JOURNEYS[journey as TwoHumanJourney];
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const tag = flagValue(segments, "--tag") ?? `${check}-${locale}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  const serveB = flagValue(segments, "--serve-b") ?? serve;
  const hubUrl = hub.replace(/\/$/u, "");
  const served = (url: string, run: (baseUrl: string) => Promise<void>): Promise<void> => withDevServe(repoRoot, check, { serveUrl: url, hubUrl, locale, signal: controller.signal, startedAt }, run);
  const body = async (serveUrl: string, serveBUrl: string): Promise<void> => {
    const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
    const report = await runTwoHuman({
      journey: journey as TwoHumanJourney,
      hub: hubUrl,
      serves: [serveUrl, serveBUrl],
      humans: readHumans(segments),
      locale,
      kinds: (flagValue(segments, "--kinds") ?? "").split(",").filter(Boolean),
      spaceId: flagValue(segments, "--space") ?? null,
      tag,
      outDir,
      adminCapabilityFile: flagValue(segments, "--admin-capability") ?? null,
      mountBudgetMs: Number(flagValue(segments, "--mount-budget-ms") ?? 900_000),
      signal: controller.signal,
    });
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failing = report.rows.filter((row) => !row.pass).map((row) => `${row.kindId}: ${Object.entries(row.checks).filter(([, entry]) => !entry.pass).map(([name]) => name).join(", ") || "faults"}`);
    const unreachable = total === 0 && /ERR_CONNECTION_REFUSED|ECONNREFUSED|Unable to connect/u.test(report.fatal ?? "");
    const bound = (flag: string): number | null => (flagValue(segments, flag) === undefined ? null : Number(flagValue(segments, flag)));
    const createBoundMs = bound("--max-create-to-mounted-ms");
    const openBoundMs = bound("--max-open-to-mounted-ms");
    const createTimes = report.rows.map((row) => row.timings?.createToMountedMs).filter((value): value is number => value !== undefined).sort((left, right) => left - right);
    const openTimes = report.rows.map((row) => row.timings?.openRowToMountedMs).filter((value): value is number => value !== undefined).sort((left, right) => left - right);
    const p50 = (values: readonly number[]): number => (values.length ? values[Math.floor((values.length - 1) / 2)]! : -1);
    const slow = report.rows.filter((row) => (createBoundMs !== null && (row.timings?.createToMountedMs ?? 0) > createBoundMs) || (openBoundMs !== null && (row.timings?.openRowToMountedMs ?? 0) > openBoundMs)).map((row) => row.kindId);
    const status = unreachable ? "blocked" : report.fatal || report.cancelled || total === 0 ? "fail" : passed === total && slow.length === 0 ? "pass" : "fail";
    const timesOf = (...names: (keyof KindTimings)[]): number[] => report.rows.flatMap((row) => names.map((name) => row.timings?.[name]).filter((value): value is number => value !== undefined)).sort((left, right) => left - right);
    const seenTimes = timesOf("aEditSeenByBMs", "bEditSeenByAMs");
    const reopenTimes = timesOf("reopenAToMountedMs", "reopenBToMountedMs");
    const latency = { en: `create→mounted p50 ${p50(createTimes)} ms max ${createTimes.at(-1) ?? -1} ms${createBoundMs === null ? "" : ` (bound ${createBoundMs})`}, open→mounted p50 ${p50(openTimes)} ms max ${openTimes.at(-1) ?? -1} ms${openBoundMs === null ? "" : ` (bound ${openBoundMs})`}${slow.length ? `; over bound: ${slow.join(", ")}` : ""}; edit → other human sees p50 ${p50(seenTimes)} ms max ${seenTimes.at(-1) ?? -1} ms; warm reopen p50 ${p50(reopenTimes)} ms max ${reopenTimes.at(-1) ?? -1} ms`, de: `Anlegen→eingebunden p50 ${p50(createTimes)} ms max ${createTimes.at(-1) ?? -1} ms${createBoundMs === null ? "" : ` (Grenze ${createBoundMs})`}, Öffnen→eingebunden p50 ${p50(openTimes)} ms max ${openTimes.at(-1) ?? -1} ms${openBoundMs === null ? "" : ` (Grenze ${openBoundMs})`}${slow.length ? `; über der Grenze: ${slow.join(", ")}` : ""}; Bearbeitung → andere Person sieht sie p50 ${p50(seenTimes)} ms max ${seenTimes.at(-1) ?? -1} ms; warmes Wiederöffnen p50 ${p50(reopenTimes)} ms max ${reopenTimes.at(-1) ?? -1} ms` };
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check,
        status,
        startedAt,
        measured: { locale, kinds: total, passed, failed: total - passed, routeMisses: report.misses.length, fatal: Boolean(report.fatal), cancelled: Boolean(report.cancelled), createToMountedP50Ms: p50(createTimes), createToMountedMaxMs: createTimes.at(-1) ?? -1, openRowToMountedP50Ms: p50(openTimes), openRowToMountedMaxMs: openTimes.at(-1) ?? -1, createBoundMs: createBoundMs ?? -1, openBoundMs: openBoundMs ?? -1, overBound: slow.length, editSeenByOtherP50Ms: p50(seenTimes), editSeenByOtherMaxMs: seenTimes.at(-1) ?? -1, warmReopenP50Ms: p50(reopenTimes), warmReopenMaxMs: reopenTimes.at(-1) ?? -1 },
        summary: {
          en: `${passed}/${total} kinds pass the two-human ${journey} journey in ${locale}; ${latency.en}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} Arten bestehen den Zwei-Personen-Weg (${journey}) in ${locale}; ${latency.de}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, tag, "report.json")],
      }),
    );
    console.log(`[two-human] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  };
  await withAcceptanceRecord(repoRoot, check, () => served(serve, (serveUrl) => (serveB === serve ? body(serveUrl, serveUrl) : served(serveB, (serveBUrl) => body(serveUrl, serveBUrl)))));
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️TwoHuman
