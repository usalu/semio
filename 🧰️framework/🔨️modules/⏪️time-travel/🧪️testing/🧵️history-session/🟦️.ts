/** 🧵️ History sessions shared by every scenario: opening the editor, the alternatives list and leaving a session. */
import { waitUntil, sleep, evalSafe, shot } from "../📄️page/🟦️.ts";
import { editor, revealHistory } from "../🖊️editor/🟦️.ts";
import { log } from "../🧾️report/🟦️.ts";
import { openHistory, findMutationRow, pressRowAction, scrollHistory, treeWindow, allHistoryRows, documentEntries, expandEntry, isShellChromeRow } from "../📜️history/🟦️.ts";
import type { HistoryRow } from "../📜️history/🟦️.ts";
import { band, pressBand, finalizeDialog, dialogChoose, prepareChord } from "../🎗️band/🟦️.ts";
import type { Band } from "../🎗️band/🟦️.ts";
import { COPY } from "../🌍️copy/🟦️.ts";
import { currentLocale, renderer, page } from "../🧠️state/🟦️.ts";
import { mirror, mirrorAwait, wgpuPress, mirrorFind, mirrorLocator } from "../🧊️wgpu-mirror/🟦️.ts";
import { openPanelTabIds, markToken } from "../🧱️chrome/🟦️.ts";

//#region 🔖️Steps

/** 🪄️ Waits for the draft editor of the session that just began on the mutation labelled `label`: the band (host chrome) reads
 * `editing` first, the editor rows are part of the History body and arrive with its next refresh (measured 0.3–1.5 s under
 * load). Answers the wait; a person sees the editor before acting on it, so every step acts after this. */
export const editorArrived = async (label: string) => {
  const prefix = label.slice(0, 12);
  const arrived = await waitUntil(editor, (value) => Boolean(value?.heading && value.heading.includes(prefix)), 15000, 150);
  if (!arrived.ok) log(`editor of "${prefix}…" did not arrive within ${arrived.waitedMs} ms (heading ${JSON.stringify(arrived.value?.heading ?? null)})`);
  return arrived.waitedMs;
};

/** 🖍️ Presses Edit on the mutation row `key` names (also from a review: Begin on another target) and answers the band once
 * it reads `editing`. */
export const beginEditOf = async (key: string) => {
  await openHistory();
  const row = await findMutationRow(key);
  if (!row) return { row: null as HistoryRow | null, via: "absent", band: null as Band | null, waitedMs: 0, editorWaitedMs: 0 };
  const via = await pressRowAction(row.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing" && (b.target ?? b.text).includes(row.label.slice(0, 12)), 20000);
  const editorWaitedMs = settled.ok ? await editorArrived(row.label) : 0;
  return { row: row as HistoryRow | null, via, band: settled.value, waitedMs: settled.waitedMs, editorWaitedMs };
};

type AlternativeRow = { id: string; key: string; text: string; active: boolean };

/** 🌲️ The Alternatives section's rows (`framework.history.alternative.<id>`), with whether each reads active. */
export const readAlternatives = async (): Promise<AlternativeRow[]> => {
  await openHistory();
  if (renderer === "wgpu") {
    const mirrored = async () => (await mirror()).filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.alternative\.[^\/␟\u001f]+$/.test(node.key)).map((node) => ({ id: node.key, key: node.key.replace(/^.*framework\.history\.alternative\./, ""), text: `${node.label} ${node.description}`.trim(), active: node.selected === "true" || node.pressed === "true" }));
    for (const edge of ["start", "end"] as const) {
      await scrollHistory(edge);
      await sleep(500);
      const rows = await mirrored();
      if (rows.length) return rows;
    }
    return [];
  }
  await scrollHistory("start");
  await sleep(250);
  await revealHistory("framework.history.alternatives");
  await waitUntil(() => treeWindow("framework.history.alternatives"), (read) => read === null || read.total === 0 || read.length > 0, 3000, 150);
  return evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /(^|[\/␟])framework\.history\.alternative\.[^\/␟]+$/.test(el.id))
        .map((el) => ({
          id: el.id,
          key: el.id.replace(/^.*framework\.history\.alternative\./, ""),
          text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
          active: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-current") !== null || el.getAttribute("data-selected") === "true",
        })),
    [] as AlternativeRow[],
  );
};

/** 💡️ Whether an alternatives row is the current one: its selected state, or the "Current" word its description carries. */
export const isCurrentAlternative = (row: AlternativeRow) => row.active || new RegExp(`\\b${COPY[currentLocale].current}\\b`).test(row.text);

/** 🔃️ Switches to one alternative through its Switch row action, else the row's own activation. */
export const switchAlternative = async (row: AlternativeRow) => pressRowAction(row.id, COPY[currentLocale].switchAction);
//#region 🔖️Gaps

/** 🚪️ Exits the live session through the band and waits for it to close. */
export const exitSession = async () => {
  await pressBand("exit");
  return waitUntil(band, (b) => b === null, 20000);
};

/** 🏆️ Accepted drafts → Finalize → the prompt's destructive Overwrite; answers whether the session closed. */
export const finalizeOverwrite = async () => {
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  await dialogChoose("overwrite");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  return { dialog: dialog.ok, closed: gone.ok, band: gone.value?.stage ?? null };
};
//#endregion 🔖️Gaps

/** 🪚️ The History rows newer than `seq`, waited for until `want` document rows are listed: the document rows with their
 * mutation rows, and the command rows that are neither document rows nor the probe's own panel toggles. */
export const documentRowsSince = async (seq: number, want: number, timeoutMs: number) => {
  const read = async () => (await allHistoryRows()).filter((row) => row.kind === "entry" && Number(row.key) > seq);
  const settled = await waitUntil(read, (rows) => documentEntries(rows).length >= want, timeoutMs, 1000);
  const documents: { id: string; label: string; mutations: string[]; keys: string[]; count: number }[] = [];
  for (const entry of documentEntries(settled.value)) {
    const expected = 1 + Number(/\(\+(\d+)\)\s*$/u.exec(entry.label)?.[1] ?? "0");
    let mutations = entry.expandable ? (await expandEntry(entry.id)).mutations : [];
    for (let attempt = 0; attempt < 4 && entry.expandable && mutations.length < expected; attempt++) {
      await sleep(800);
      mutations = (await expandEntry(entry.id)).mutations;
    }
    const total = renderer === "wgpu" ? null : ((await treeWindow(`framework.history.entry.${entry.key}`))?.total ?? null);
    documents.push({ id: entry.id, label: entry.label, mutations: mutations.map((row) => row.label), keys: mutations.map((row) => row.key), count: total !== null && Number.isFinite(total) ? total : mutations.length });
  }
  const commands = settled.value.filter((row) => documentEntries([row]).length === 0 && !isShellChromeRow(row)).map((row) => row.label);
  return { documents, commands, waitedMs: settled.waitedMs, newest: Math.max(seq, ...settled.value.map((row) => Number(row.key)).filter(Number.isFinite)) };
};

/** 📂️ Picks the sync card's Folder kind: the sync utilities render as one grouped toggle (`ui.utilities.group.sync`)
 * whose menu lists File / Folder / Remote. */
export const pickSyncFolder = async () => {
  if (renderer === "wgpu") {
    if (!(await mirrorAwait("framework.sync.folder", 6000))) {
      await wgpuPress("ui.utilities.group.sync");
      await mirrorAwait("framework.sync.folder", 6000);
    }
    return (await wgpuPress("framework.sync.folder")) ?? "absent";
  }
  const direct = page.locator('[id="framework.sync.folder"]').first();
  if (!(await direct.count().catch(() => 0))) {
    await page.locator('[id="ui.utilities.group.sync"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(700);
  }
  if (await direct.count().catch(() => 0)) {
    await direct.click({ timeout: 4000 }).catch(() => {});
    return "id";
  }
  const item = page.locator('[role="menuitem"], [role="menuitemradio"], [role="option"], button').filter({ hasText: /^\s*(folder|ordner)\s*$/i }).first();
  if (await item.count().catch(() => 0)) {
    await item.click({ timeout: 4000 }).catch(() => {});
    return "text";
  }
  return "absent";
};

/** 🗂️ Attaches the session document to the local folder `path` through the sync chip (`s-sync-status` →
 * `framework.sync.folder` → `framework.sync.folder.path` → Attach): `openSyncTarget` opens it with a
 * `persistedLocalOnly` folder binding the backbone worker reads and writes through the dev serve's `/semio-backbone`.
 * `afterPress` runs the moment Attach is pressed — a person turning to the History panel while the folder's archive loads —
 * instead of the settle and the Escape that close the card. */
export const attachFolder = async (path: string, afterPress?: () => Promise<unknown>) => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await pickSyncFolder();
    const shown = await waitUntil(async () => mirrorFind(await mirror(), "framework.sync.folder.path") ?? null, (node) => node !== null, 10000, 300);
    const field = shown.value ? await mirrorLocator(shown.value.key, shown.value.window) : null;
    await field?.fill(path, { force: true, timeout: 4000 }).catch(() => {});
    const typed = field ? await field.inputValue({ timeout: 2000 }).catch(() => null) : null;
    await field?.press("Tab", { timeout: 2000 }).catch(() => {});
    const ready = await mirrorAwait("framework.sync.attach", 8000, true);
    const attached = ready ? await wgpuPress("framework.sync.attach") : null;
    if (afterPress) {
      await afterPress();
      return { card: shown.ok, typed, attachButton: attached !== null };
    }
    const active = await mirrorAwait("framework.sync.active", 10000);
    const after = { cardText: active ? active.label.slice(0, 200) : null, chip: mirrorFind(await mirror(), "s-sync-status")?.label ?? null, alerts: (await mirror()).filter((node) => node.key === "shell.notice").map((node) => `${node.description}: ${node.label}`.slice(0, 200)) };
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return { card: shown.ok, typed, attachButton: attached !== null, after };
  }
  const chip = page.locator('[id="s-sync-status"]').first();
  if (!(await openPanelTabIds()).includes("s-sync-status")) await chip.click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const field = page.locator('[id="framework.sync.folder.path"]').first();
  const shown = await waitUntil(() => field.count().catch(() => 0), (count) => count > 0, 8000);
  await field.fill(path, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  const attach = page.locator("button").filter({ hasText: /^\s*(attach|verbinden)\s*$/i }).first();
  const attachCount = await attach.count().catch(() => 0);
  if (attachCount) await attach.click({ timeout: 4000 }).catch(() => {});
  else await field.press("Enter").catch(() => {});
  if (afterPress) {
    await afterPress();
    return { card: shown.ok, typed, attachButton: attachCount > 0 };
  }
  await sleep(1500);
  const after = await evalSafe(() => {
    const field = document.querySelector<HTMLElement>('[id="framework.sync.folder.path"]');
    const card = field?.closest('[data-slot="panel"], [role="dialog"], [data-radix-popper-content-wrapper]') ?? field?.parentElement?.parentElement?.parentElement ?? null;
    const chip = document.querySelector<HTMLElement>('[id="s-sync-status"]');
    return {
      cardText: (card as HTMLElement | null)?.innerText.replace(/\s+/g, " ").trim().slice(0, 400) ?? null,
      chip: chip ? `${chip.getAttribute("aria-label") ?? ""} ${chip.innerText}`.replace(/\s+/g, " ").trim().slice(0, 120) : null,
      alerts: Array.from(document.querySelectorAll<HTMLElement>('[role="alert"], [data-notice-code]')).map((el) => `${el.getAttribute("data-notice-code") ?? el.getAttribute("role")}: ${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200)}`).slice(0, 6),
    };
  }, { cardText: null as string | null, chip: null as string | null, alerts: [] as string[] });
  await shot(`sync-card-after-attach-${markToken()}`);
  await page.keyboard.press("Escape").catch(() => {});
  return { card: shown.ok, typed, attachButton: attachCount > 0, after };
};
//#endregion 🔖️Steps
