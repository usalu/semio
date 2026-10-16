/** 📟️ The console digest step: uncaught errors, hard faults and the notices of a locale. */
import { consoleRows, BENIGN_RE, pageErrors, hardFaults, readableUrlText } from "@semio-tech/framework/time-travel/testing/console";
import type { ConsoleRow } from "@semio-tech/framework/time-travel/testing/console";
import { currentLocale, renderer } from "@semio-tech/framework/time-travel/testing/state";
import { shownNoticeCodes, shownNotices, shownBandFaults } from "@semio-tech/framework/time-travel/testing/band";
import { COPY } from "@semio-tech/framework/time-travel/testing/copy";
import { verdict, note } from "@semio-tech/framework/time-travel/testing/report";

//#region 🔖️Steps

/** 📟️ Step 9 — the locale's console: uncaught page errors and hard guest faults fail; errors, warnings and lines with
 * AGENTS.md's temporary-log tag are digested with their most frequent texts. */
export const step9 = async (_ctx: object) => {
  const mine = consoleRows.filter((row) => row.locale === currentLocale);
  const errors = mine.filter((row) => row.type === "error" && !BENIGN_RE.test(row.text));
  const warnings = mine.filter((row) => row.type === "warning" && !BENIGN_RE.test(row.text));
  const debug = mine.filter((row) => /\[DEBUG\] /u.test(row.text));
  const http = mine.filter((row) => row.type === "http");
  const top = (rows: ConsoleRow[]) => [...rows.reduce((map, row) => map.set(row.text.slice(0, 160), (map.get(row.text.slice(0, 160)) ?? 0) + 1), new Map<string, number>())].sort((x, y) => y[1] - x[1]).slice(0, 12);
  const refusals = [...new Set(mine.map((row) => /refused a local batch \S+ ((?:local|sync)\.[\w.-]+)/.exec(row.text)?.[1]).filter((code): code is string => Boolean(code)))];
  const shownCodes = await shownNoticeCodes();
  const notices = await shownNotices();
  const copy = COPY[currentLocale];
  const full = notices.filter((row) => row.code === "history.full");
  if (full.length) verdict("history-full-notice-names-the-edit-count", full.every((row) => Number(copy.historyFull.exec(row.text)?.[1] ?? "0") > 0), { full, reading: "`HistoryPatch.editCount` is the `{n}` this notice names, carried as data (never digits read from a fault message)" });
  else note("no-history-full-notice", { reading: "the paged edit ledger admits far more edits than a run makes, so `HistoryPatch.editCount` shows only as this notice's `{n}`; its folding from every patch is the React law `folds the history's edit count from every patch` (W2-B suite)" });
  const laneWords: Record<string, string> = { "history.replaying": copy.noticeReplaying, "document.loading": copy.noticeLoading };
  const lane = notices.filter((row) => row.code in laneWords);
  if (lane.length) verdict("history-lane-notices-speak-the-locale", lane.every((row) => row.text.includes(laneWords[row.code]!)), { lane, expected: laneWords, reading: "kernel `HISTORY_NOTICE_LABELS` (`🧫️history-notices`): a refused history-lane command reaches the person in the UI locale, by code" });
  note("notices-shown", { notices: notices.slice(0, 24) });
  const faults = await shownBandFaults();
  if (faults.length) verdict("band-never-shows-a-raw-fault-code", faults.every((row) => row.text !== "" && (row.code === null ? false : !row.text.includes(row.code)) && (row.code === null || row.code.startsWith("timeTravel.") ? true : row.text.includes(copy.replayFaulted))), { faults, expected: copy.replayFaulted, reading: renderer === "wgpu" ? "the wgpu band's status line must never carry a fault code (any `<scope>.<code>` token here is one)" : "the band reads a known fault in its own words and any other as \"Replay failed: later mutations could not be checked\"; the code is only `data-semio-time-travel-fault`" });
  else note("no-band-fault-in-this-run", { reading: "no replay fault or cancel showed on a band (step 16's Cancel shows `timeTravel.cancelled` as \"Replay cancelled\")" });
  const restores = notices.filter((row) => row.code === "shell.document-restore");
  verdict("no-document-restore-fails", restores.length === 0, { alerts: restores, reading: renderer === "wgpu" ? "React-only observation (the wgpu shell's restore alert is not projected to the probe); none recorded" : "the shell's restore alert (`[data-semio-bootstrap-status][role=alert]`, \"Document restore failed: <fault>\") means an archive the document read back was rejected — the shell then retires the document's port, so nothing the page publishes afterwards is persisted or reaches a peer" });
  const mismatch = notices.filter((row) => row.code === "plugin.channel-mismatch");
  const mismatchLines = mine.filter((row) => /plugin\.channel-mismatch|speaks app channel \d+, the host app channel \d+/u.test(row.text)).map((row) => row.text.slice(0, 300));
  verdict("no-plugin-is-refused-for-a-channel-mismatch", mismatch.length === 0 && mismatchLines.length === 0, { mismatch, lines: mismatchLines.slice(0, 6), reading: "S4-BUMP's guest↔host channel handshake (`admit_guest_channel_version`): a component built for another app channel is refused before its first frame — on a fresh activation none is" });
  if (mismatch.length) verdict("channel-mismatch-notice-is-localized", mismatch.every((row) => copy.channelMismatch.test(row.text)), { mismatch, reading: "kernel `🧫️framework-notices`: the refusal names both channels in the UI locale, never the raw code" });
  if (!refusals.length) note("no-command-rejection-in-this-run", { shownCodes });
  else verdict("rejection-notices-carry-their-code", refusals.every((code) => shownCodes.includes(code)), { refusals, shownCodes, reading: renderer === "wgpu" ? "wgpu projects the transient notice as the polite `shell.notice` mirror node described by its code (`🧯️wgpu-transient-notice`); the main page's trace saw these codes" : "React `[data-notice-code]` over the main page's life" });
  const uncaught = pageErrors.filter((row) => row.locale === currentLocale);
  verdict("no-uncaught-page-errors", uncaught.length === 0, { count: uncaught.length, first: uncaught.slice(0, 3) });
  const hard = hardFaults.filter((row) => row.locale === currentLocale);
  verdict("no-hard-guest-faults", hard.length === 0, { count: hard.length, first: hard.slice(0, 3) });
  verdict("console-is-debug-free", debug.length === 0, { count: debug.length, top: top(debug).slice(0, 6), reading: "AGENTS.md: temporary-log tagged lines are removed before a run counts" });
  const folderRequests = mine.filter((row) => row.type === "backbone").map((row) => `${row.t}s step ${row.step} ${/HTTP \d+ \w+/u.exec(row.text)?.[0] ?? row.text.slice(0, 24)}`);
  if (folderRequests.length) note("folder-requests-of-the-run", { requests: folderRequests.slice(0, 60), puts: folderRequests.filter((row) => row.includes("PUT")).length, reading: "every folder request of this locale's pages with its run-clock time and step: a PUT is the page writing the archive, a GET 204 an empty folder, a GET 200 an archive read" });
  if (errors.length || uncaught.length) note("console-errors-in-full", { errors: errors.slice(0, 8).map((row) => `${row.t}s step ${row.step} ${row.source}: ${readableUrlText(row.text).slice(0, 1400)}`), uncaught: uncaught.slice(0, 4).map((row) => `step ${row.step}: ${readableUrlText(row.text).slice(0, 900)}`) });
  note("console-digest", { lines: mine.length, errors: errors.length, warnings: warnings.length, debug: debug.length, http: top(http), topErrors: top(errors), topWarnings: top(warnings), topDebug: top(debug) });
};
//#endregion 🔖️Steps
