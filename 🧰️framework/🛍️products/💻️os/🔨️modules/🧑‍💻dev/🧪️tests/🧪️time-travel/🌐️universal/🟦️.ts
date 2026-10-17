/** 🌐️ The plugin-neutral universal step: the mirror's controls and verbs, one clean change and the journey over them. */
import { universalMirrorControlsV1, universalMirrorVerbsV1 } from "../🪞️inventory/🟦️.ts";
import type { UniversalControlV1, UniversalVerbV1 } from "../🪞️inventory/🟦️.ts";
import { join } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";
import { renderer, page, currentLocale, universalEditor, invalidatedBy, stamp, OUT } from "@semio-tech/framework-time-travel-ts/testing/state";
import { mirror, mirrorLocator, mirrorActivate, wgpuPress, mirrorFind, BAND_WORDS } from "@semio-tech/framework-time-travel-ts/testing/wgpu-mirror";
import { evalSafe, sleep, waitUntil } from "@semio-tech/framework-time-travel-ts/testing/page";
import { pressAuthored, documentEntries, allHistoryRows, pressRowAction, openHistory, newestEntrySeq, expandEntry, rowActionState, findMutationRow } from "@semio-tech/framework-time-travel-ts/testing/history";
import type { HistoryRow } from "@semio-tech/framework-time-travel-ts/testing/history";
import { pressUseSelection, editor } from "@semio-tech/framework-time-travel-ts/testing/editor";
import { COPY } from "@semio-tech/framework-time-travel-ts/testing/copy";
import { closePanels, textOfKey } from "@semio-tech/framework-time-travel-ts/testing/chrome";
import { band, pressBand, installBandTrace, bandTrace, finalizeDialog, dialogChoose, dialogFillName } from "@semio-tech/framework-time-travel-ts/testing/band";
import type { Band } from "@semio-tech/framework-time-travel-ts/testing/band";
import { verdict, note } from "@semio-tech/framework-time-travel-ts/testing/report";
import { documentRowsSince, beginEditOf, exitSession, readAlternatives, isCurrentAlternative, switchAlternative } from "@semio-tech/framework-time-travel-ts/testing/history-session";
import { pageErrors, hardFaults } from "@semio-tech/framework-time-travel-ts/testing/console";

//#region 🔖️Steps

type UniversalControl = UniversalControlV1;

type UniversalVerb = UniversalVerbV1;

/** 🧭 The input controls of the open history editor, read without knowing the editor: every `framework.history.editor.input.<pointer>.row`
 * with the contract role of the control it holds (slider, spinbutton, radiogroup, listbox, combobox, switch, textbox, or a
 * reference list when the row has Use selection / chips), its accessible name, value and min / max / step where the DOM
 * carries them. Each control is stamped `data-probe-u=<index>` so it can be operated. */
const universalControls = async (): Promise<UniversalControl[]> => renderer === "wgpu" ? universalMirrorControlsV1(await mirror()) :
  evalSafe(
    () => {
      const all = Array.from(document.querySelectorAll<HTMLElement>("[id]"));
      const rows = all.filter((el) => /framework\.history\.editor\.input\.[^/␟]+\.row$/u.test(el.id));
      const pointers = rows.map((row) => row.id.replace(/^.*framework\.history\.editor\.input\./u, "").replace(/\.row$/u, ""));
      const out: UniversalControl[] = [];
      document.querySelectorAll("[data-probe-u]").forEach((el) => el.removeAttribute("data-probe-u"));
      rows.forEach((row, at) => {
        const pointer=pointers[at];if(pointer===undefined)throw Error("Actual rendered history pointer required");
        if (/\.(useSelection|chip\.\d+)$/u.test(pointer)) return;
        const name = ((row.querySelector('[data-slot="tree-label"]') as HTMLElement | null)?.innerText ?? row.getAttribute("aria-label") ?? "").replace(/\s+/gu, " ").trim().slice(0, 80);
        const own = all.find((el) => el.id.startsWith(`${row.id}/`)) ?? row;
        const within = (selector: string) => (own.matches(selector) ? own : own.querySelector<HTMLElement>(selector)) ?? row.querySelector<HTMLElement>(selector);
        const chips = pointers.filter((other) => other.startsWith(`${pointer}.chip.`)).length;
        const reference = pointers.includes(`${pointer}.useSelection`) || chips > 0;
        const slider = within('[role="slider"]');
        const number = within('input[type="number"], input[data-stepper-input="true"], [role="spinbutton"]');
        const radiogroup = within('[role="radiogroup"]');
        const listbox = within('[role="listbox"]');
        const combobox = within('[role="combobox"], select');
        const toggle = within('[role="switch"], [role="checkbox"], input[type="checkbox"]');
        const text = within('textarea, input[type="text"], input:not([type]), [role="textbox"]');
        const pick = reference ? row : (slider ?? number ?? radiogroup ?? listbox ?? combobox ?? toggle ?? text);
        if (!pick) return;
        const role = reference ? "reference list" : slider ? "slider" : number ? "spinbutton" : radiogroup ? "radiogroup" : listbox ? "listbox" : combobox ? "combobox" : toggle ? "switch" : "textbox";
        const input = pick as HTMLInputElement;
        const value = reference
          ? String(chips)
          : slider
            ? pick.getAttribute("aria-valuenow")
            : number
              ? (input.value ?? pick.getAttribute("aria-valuenow"))
              : radiogroup
                ? (pick.querySelector('[role="radio"][aria-checked="true"]')?.textContent?.trim() ?? null)
                : listbox
                  ? (pick.querySelector('[role="option"][aria-selected="true"]')?.textContent?.trim() ?? null)
                  : combobox
                    ? (pick instanceof HTMLSelectElement ? pick.value : (pick.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60))
                    : toggle
                      ? (pick.getAttribute("aria-checked") ?? String(input.checked))
                      : (input.value ?? pick.textContent ?? null);
        pick.setAttribute("data-probe-u", String(out.length));
        out.push({ index: out.length, pointer, role, name, value, min: pick.getAttribute("min") ?? pick.getAttribute("aria-valuemin"), max: pick.getAttribute("max") ?? pick.getAttribute("aria-valuemax"), step: pick.getAttribute("step") ?? pick.getAttribute("data-step"), options: radiogroup ? pick.querySelectorAll('[role="radio"]').length : listbox ? pick.querySelectorAll('[role="option"]').length : null, disabled: input.disabled === true || pick.getAttribute("aria-disabled") === "true" || pick.getAttribute("aria-readonly") === "true" || input.readOnly === true });
      });
      return out;
    },
    [] as UniversalControl[],
  );

/** 🧑‍🚀 The verbs the open Actions rail lists (`action.<verb>` rows under their `action.category.<name>` groups), in order. */
const universalVerbs = async (): Promise<UniversalVerb[]> => renderer === "wgpu" ? universalMirrorVerbsV1(await mirror()) :
  evalSafe(
    () => {
      const pane = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window-action-pane"]')).find((el) => el.offsetParent !== null);
      if (!pane) return [];
      let category = "";
      const out: UniversalVerb[] = [];
      for (const el of Array.from(pane.querySelectorAll<HTMLElement>('[id^="action."]'))) {
        if (el.id.startsWith("action.category.")) {
          category = el.id.slice("action.category.".length);
          continue;
        }
        if (!/^action\.[A-Za-z0-9_-]+$/u.test(el.id)) continue;
        const label = ((el.querySelector('[data-slot="tree-label"]') as HTMLElement | null)?.innerText ?? el.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60);
        out.push({ id: el.id, verb: el.id.slice("action.".length), category, label, disabled: el.getAttribute("aria-disabled") === "true" });
      }
      return out;
    },
    [] as UniversalVerb[],
  );

/** 🛰 Changes one editor control by the smallest step its role offers: ArrowRight on a slider, ArrowUp on a spinbutton, the
 * next radio / option, a click on a switch, one appended character (committed by Tab) in a textbox. Answers what it did. */
const universalChange = async (control: UniversalControl) => {
  const target = renderer === "wgpu" && control.key ? await mirrorLocator(control.key) : page.locator(`[data-probe-u="${control.index}"]`).first();
  if (!target) return "the control left the retained mirror";
  await target.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
  if (control.role === "reference list") {
    const chips = Number(control.value ?? "0");
    if (chips > 0) {
      const removed = await pressAuthored(`framework.history.editor.input.${control.pointer}.chip.${chips - 1}`, false);
      return `Remove on the last of ${chips} chip(s): ${removed.present ? "pressed" : "absent"}`;
    }
    return `Use selection (no chip to remove): ${(await pressUseSelection(control.pointer)) ?? "not pressed"}`;
  }
  if (control.role === "slider" || control.role === "spinbutton") {
    const atMax = control.max !== null && control.value !== null && Number(control.value) >= Number(control.max);
    const key = control.role === "slider" ? (atMax ? "ArrowLeft" : "ArrowRight") : atMax ? "ArrowDown" : "ArrowUp";
    await target.focus({ timeout: 3000 }).catch(() => {});
    await page.keyboard.press(key).catch(() => {});
    return key;
  }
  if (control.role === "textbox") {
    await target.focus({ timeout: 3000 }).catch(() => {});
    await page.keyboard.press("End").catch(() => {});
    await page.keyboard.type("x").catch(() => {});
    await page.keyboard.press("Tab").catch(() => {});
    return 'typed "x", committed by Tab';
  }
  if (control.role === "switch") {
    if (renderer === "wgpu" && control.key) return await mirrorActivate(control.key) ? "accessibility activation" : "activation absent";
    await target.click({ timeout: 3000 }).catch(() => {});
    return "click";
  }
  if (renderer === "wgpu" && control.key && ["radiogroup", "listbox", "combobox"].includes(control.role)) {
    if (control.role === "combobox") { await mirrorActivate(control.key); await sleep(500); }
    const option = (await mirror()).find(node => (node.key.startsWith(`${control.key}::`) || node.key.startsWith(`${control.key}.`)) && ["radio", "option"].includes(node.role) && node.checked !== "true" && node.selected !== "true" && !node.disabled);
    if (option) return await mirrorActivate(option.key, option.window) ? "the first other accessible option" : "option activation absent";
    await page.keyboard.press("Escape");
    return "no other accessible option offered";
  }
  if (control.role === "radiogroup") {
    await target.locator('[role="radio"][aria-checked="false"]').first().click({ timeout: 3000 }).catch(() => {});
    return "the first unchecked radio";
  }
  if (control.role === "listbox") {
    await target.locator('[role="option"]:not([aria-selected="true"])').first().click({ timeout: 3000 }).catch(() => {});
    return "the first unselected option";
  }
  if (control.role === "combobox") {
    if ((await target.evaluate((el) => el.tagName).catch(() => "")) === "SELECT") {
      await target.evaluate((el) => {
        const select = el as HTMLSelectElement;
        select.selectedIndex = (select.selectedIndex + 1) % Math.max(1, select.options.length);
        select.dispatchEvent(new Event("change", { bubbles: true }));
      }).catch(() => {});
      return "the next option of the select";
    }
    await target.click({ timeout: 3000 }).catch(() => {});
    const option = page.locator('[role="option"]:not([aria-selected="true"]):not([aria-disabled="true"])').first();
    if ((await waitUntil(() => option.count().catch(() => 0), (count) => count > 0, 3000, 200)).ok) {
      await option.click({ timeout: 3000 }).catch(() => {});
      return "the first other option";
    }
    await page.keyboard.press("Escape").catch(() => {});
    return "no other option offered";
  }
  return "not operated";
};

/** 🧯️ The clean-input helpers of the universal journey: a control's settled value and the shown notices. */
const settledControl = async (pointer: string) => {
  await sleep(1200);
  let last: string | null | undefined;
  const read = await waitUntil(universalControls, (list) => {
    const now = list.find((row) => row.pointer === pointer);
    const same = now !== undefined && now.value === last;
    last = now?.value;
    return same;
  }, 6000, 350);
  return read.value.find((row) => row.pointer === pointer) ?? null;
};
const visibleNoticeCodes = async () => renderer === "wgpu" ? (await mirror()).filter(node => node.key === "shell.notice").map(node => node.description) : evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]")).filter((el) => el.offsetParent !== null).map((el) => el.getAttribute("data-notice-code") ?? ""), [] as string[]);
const visibleNoticeTexts = async () => renderer === "wgpu" ? (await mirror()).filter(node => node.key === "shell.notice").map(node => node.label) : evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]")).filter((el) => el.offsetParent !== null).map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)), [] as string[]);
const dismissNotices = async () => {
  if (renderer === "wgpu") { const dismiss = (await mirror()).find(node => /^shell\.notice\.(?:dismiss|close)$/u.test(node.key)); if (dismiss) await mirrorActivate(dismiss.key); return; }
  await evalSafe(() => document.querySelectorAll<HTMLElement>("[data-notice-code] button").forEach((button) => button.click()), undefined);
  await sleep(250);
};

/** 🌱 Changes one clean history input through its published control and confirms the settled draft. */
const universalChangeOneInput = async () => {
  const attempts: Record<string, unknown>[] = [];
  let changed: string | null = null;
  const clean = (await waitUntil(universalControls, (list) => list.length > 0, 6000, 400)).value;
  for (const control of clean.filter((row) => !row.disabled && row.role !== "reference list").slice(0, 6)) {
    await dismissNotices();
    const how = await universalChange(control);
    const after = await waitUntil(universalControls, (list) => {
      const now = list.find((row) => row.pointer === control.pointer);
      return now !== undefined && now.value !== control.value;
    }, 5000, 300);
    const settled = await settledControl(control.pointer);
    const refusedWith = (await visibleNoticeCodes()).filter((code) => /^(timeTravel|app\.command)\./u.test(code));
    attempts.push({ control: `${control.role} "${control.name}"`, how, before: control.value, after: settled?.value ?? null, ...(refusedWith.length ? { refusedWith } : {}) });
    if (after.ok && settled !== null && settled.value !== control.value && refusedWith.length === 0) {
      changed = `${control.role} "${control.name}": ${control.value} → ${settled.value} (${how})`;
      break;
    }
  }
  return { changed, attempts };
};

/** 🌐 Step 24 — design §23.3, the UNIVERSAL LIVE JOURNEY (batch U): the history-edit journey on whatever editor the serve
 * boots, with framework selectors only (Actions rail, History panel, time-travel band, history editor, finalize prompt).
 * u1 boot + rail verbs + History; u2 TWO mutation rows without editor knowledge (rail actions run with their declared
 * defaults, the second preferably one that acts on what the first left selected; else rows of the booted example); u3 Edit
 * on the OLDER row → band `editing` + the editor's controls by contract role (the inventory, also written to
 * `<out>/<editor>-controls.json`); `u-control-<role>` one control of every inventoried role operated, each on a clean draft;
 * u4 one control changed → the draft is acknowledged, the later row reads "not applied"; u5 Accept → replay (stage + progress
 * when a frame shows it) → review, blockers resolved through Next problem → Withdraw; `u-conflict-…` the blocked → resolved
 * path (taken in u5, else by withdrawing the older row); u6 Finalize → Overwrite, then a second input edit → Finalize → named New alternative;
 * u7 Withdraw → Restore on a row, Exit, zero trace; u8 no faults, labels in the locale, none a raw key. What an editor does not offer is a
 * NOTE named `u<n>-NOT-OFFERED-…` with its evidence — never a PASS. */
export const stepU = async (_ctx: object) => {
  const copy = COPY[currentLocale];
  const started = Date.now();
  const shell = await evalSafe((wgpu) => ({ url: location.href, title: document.title, windows: wgpu ? [...new Set(Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-window]")).map(el => el.dataset.window ?? "").filter(Boolean))].slice(0, 8) : Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window"]')).map((el) => el.id).slice(0, 8), canvases: document.querySelectorAll("canvas").length }), { url: "", title: "", windows: [] as string[], canvases: 0 }, renderer === "wgpu");
  const editor = universalEditor ?? (shell.title.toLowerCase().replace(/semio/gu, "").replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "") || "editor");
  await closePanels();
  const pane = page.locator('[data-slot="window"]').filter({ has: page.locator('[id$=".engagement.toggle"]') }).first();
  const retainedToggle = renderer === "wgpu" ? (await mirror()).find(node => node.key.endsWith(".engagement.toggle"))?.key ?? null : null;
  const hasRail = renderer === "wgpu" ? retainedToggle !== null : (await pane.count().catch(() => 0)) > 0;
  const toggle = pane.locator('[id$=".engagement.toggle"]').first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const retainedRailVisible = async () => (await mirror()).some(node => /(?:^|[\/␟\u001f])action\.(?:category\.)?[A-Za-z0-9_-]+$/u.test(node.key));
  const openRail = async () => {
    if (renderer === "wgpu") {
      if (retainedToggle && !await retainedRailVisible()) { await wgpuPress(retainedToggle); await waitUntil(retainedRailVisible, Boolean, 8000, 200); }
      return;
    }
    if (!hasRail || (await rail.isVisible().catch(() => false))) return;
    await toggle.click({ timeout: 4000 }).catch(() => {});
    await waitUntil(() => rail.isVisible().catch(() => false), Boolean, 6000, 200);
    await sleep(500);
  };
  const closeRail = async () => {
    if (renderer === "wgpu") { if (retainedToggle && await retainedRailVisible()) await wgpuPress(retainedToggle); return; }
    if (hasRail && (await rail.isVisible().catch(() => false))) await toggle.click({ timeout: 4000 }).catch(() => {});
  };
  const stableDocumentLabels = async () => {
    let last = "";
    const read = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (labels) => {
      const key = JSON.stringify(labels);
      const same = key === last;
      last = key;
      return same;
    }, 12000, 700);
    return read.value;
  };
  const isHistoryEdit = (row: HistoryRow) => /History edit|Verlauf bearbeitet|Verlaufsbearbeitung/u.test(row.label);
  const reviewed = () => waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 90000, 100);
  const resolveBlockers = async (first: Band | null, path: string[]) => {
    let now = first;
    for (let round = 0; round < 4 && now?.review === "blocked"; round++) {
      const next = await pressBand("nextProblem");
      const reopened = await waitUntil(band, (state) => state?.stage === "editing", 8000, 100);
      const blockerText = reopened.value?.target ?? reopened.value?.text ?? "";
      const blocker = (await allHistoryRows()).find((row) => row.kind === "mutation" && row.label.length > 3 && blockerText.includes(row.label)) ?? null;
      const pulled = await pressAuthored("framework.history.editor.withdraw");
      let how = pulled.present ? "the editor's Withdraw" : "";
      if (!pulled.present) how = `the blocker's row action Withdraw: ${blocker ? await pressRowAction(blocker.id, copy.withdraw) : "absent"}`;
      await sleep(900);
      if ((await band())?.stage === "editing") await pressBand("accept");
      now = (await reviewed()).value;
      path.push(`${copy === COPY.de ? "Nächstes Problem" : "Next problem"} (${next}) → ${reopened.ok ? `the editor opens on "${(blocker?.label ?? blockerText).slice(0, 60)}"` : "no editor"} → ${how} → ${now?.review ?? "no review"}`);
    }
    return now;
  };
  await openRail();
  const verbs = await universalVerbs();
  await closeRail();
  const historyOpened = await openHistory();
  const commandsLabel = await textOfKey("framework.history.commands");
  const bootRows = await stableDocumentLabels();
  verdict("u1-boots-and-lists-its-actions-and-its-history", historyOpened && shell.windows.length > 0, { ...shell, editor, railOffered: hasRail, railVerbs: verbs.length, verbs: verbs.slice(0, 48).map((row) => `${row.category || "-"}/${row.verb}=${row.label}${row.disabled ? " [disabled]" : ""}`), historyRowsAtBoot: bootRows.slice(0, 8), commandsSection: commandsLabel.slice(0, 40), reading: "the editor boots into a shell with at least one window; its Actions rail lists the verbs the manifest declares and the History panel opens" });
  let seq = Math.max(-1, newestEntrySeq(await allHistoryRows()));
  const seqAtBoot = seq;
  const skipCategory = /^(transfer|file|view|selection|navigation|clipboard|help|window|history)$/iu;
  const skipVerb = /^(select|deselect|clear|copy|cut|paste|delete|remove|import|export|open|close|load|save|undo|redo|focus|zoom|fit|toggle|show|hide|reset|commit|checkout|switch|createAlternative|set(Active)?(Example|Document|Fixture|Camera))/iu;
  const rank = (category: string) => (/^create$/iu.test(category) ? 0 : /^transform$/iu.test(category) ? 1 : 2);
  const candidates = verbs
    .map((row, index) => ({ ...row, index }))
    .filter((row) => !row.disabled && !skipCategory.test(row.category) && !skipVerb.test(row.verb))
    .sort((left, right) => rank(left.category) - rank(right.category) || left.index - right.index)
    .slice(0, 14);
  type Candidate = (typeof candidates)[number];
  const tried: Record<string, unknown>[] = [];
  const runVerb = async (candidate: Candidate, phase: string) => {
    await openRail();
    if (renderer === "wgpu") {
      const item = mirrorFind(await mirror(), candidate.id);
      if (!item) { tried.push({ phase, verb: candidate.verb, outcome: "its row left the rail" }); return null; }
      await mirrorActivate(item.key, item.window);
      const execute = await waitUntil(async () => (await mirror()).find(node => node.key.endsWith(`.action.${candidate.verb}.execute`)) ?? null, node => node !== null, 4000, 200);
      const executeDisabled = execute.value?.disabled ?? false;
      if (execute.ok && execute.value && !executeDisabled) await mirrorActivate(execute.value.key, execute.value.window);
      const made = executeDisabled ? null : await documentRowsSince(seq, 1, execute.ok ? 8000 : 5000);
      if ((await mirror()).some(node => node.role === "dialog")) { await page.keyboard.press("Escape"); await sleep(400); }
      const row = made?.documents[0] ?? null;
      tried.push({ phase, verb: candidate.verb, label: candidate.label, category: candidate.category, form: execute.ok, executeDisabled, row: row ? `${row.label} [${row.count} mutation(s)]` : null });
      if (made) seq = made.newest;
      if (!row && execute.ok) await mirrorActivate(item.key, item.window);
      return row ? { id: row.id, label: row.label, form: execute.ok } : null;
    }
    const item = pane.locator(`[id="${candidate.id}"]`).first();
    if (!(await item.count().catch(() => 0))) {
      tried.push({ phase, verb: candidate.verb, outcome: "its row left the rail" });
      return null;
    }
    const label = item.locator('[data-slot="tree-label"]').first();
    const press = (await label.count().catch(() => 0)) ? label : item;
    await item.scrollIntoViewIfNeeded({ timeout: 2000 }).catch(() => {});
    const execute = pane.locator(`[id$=".action.${candidate.verb}.execute"]`).first();
    if (!(await execute.count().catch(() => 0))) await press.click({ timeout: 3000 }).catch(() => {});
    const form = (await waitUntil(() => execute.count().catch(() => 0), (count) => count > 0, 1800, 200)).ok;
    let executeDisabled: boolean | null = null;
    let staged: string[] = [];
    if (form) {
      staged = await evalSafe((verb) => Array.from(document.querySelectorAll<HTMLElement>(`[id^="action.${verb}.arg."]`)).filter((el) => !el.id.endsWith(".disclosureLabel")).map((el) => `${el.id.replace(/^.*\.arg\./u, "")}=${(el.querySelector("input") as HTMLInputElement | null)?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? (el.querySelector('[role="combobox"]')?.textContent ?? "").trim()}`.slice(0, 48)).slice(0, 8), [] as string[], candidate.verb);
      executeDisabled = (await execute.isDisabled({ timeout: 1500 }).catch(() => false)) || (await execute.getAttribute("aria-disabled").catch(() => null)) === "true";
      if (!executeDisabled) {
        await execute.scrollIntoViewIfNeeded({ timeout: 2000 }).catch(() => {});
        await execute.click({ timeout: 3000 }).catch(() => {});
      }
    }
    const made = executeDisabled ? null : await documentRowsSince(seq, 1, form ? 5000 : 3500);
    if (await page.locator('[role="dialog"]').first().isVisible().catch(() => false)) {
      await page.keyboard.press("Escape").catch(() => {});
      await sleep(400);
    }
    const row = made?.documents[0] ?? null;
    tried.push({ phase, verb: candidate.verb, label: candidate.label, category: candidate.category, form, staged, executeDisabled, row: row ? `${row.label} [${row.count} mutation(s)]` : null });
    if (made) seq = made.newest;
    if (!row && form) {
      await openRail();
      await press.click({ timeout: 2000 }).catch(() => {});
    }
    return row ? { id: row.id, label: row.label, form } : null;
  };
  let producer: Candidate | null = null;
  let first: { id: string; label: string; form: boolean } | null = null;
  for (const candidate of candidates) {
    if (first !== null || Date.now() - started > 150000 || invalidatedBy !== null) break;
    first = await runVerb(candidate, "first row");
    if (first) producer = candidate;
  }
  let second: { id: string; label: string; form: boolean } | null = null;
  let secondHow = "";
  if (first && producer) {
    const others = candidates.filter((row) => row.id !== producer!.id).sort((left, right) => (rank(left.category) === 1 ? 0 : 1) - (rank(right.category) === 1 ? 0 : 1) || left.index - right.index);
    for (const candidate of others.slice(0, 7)) {
      if (second !== null || Date.now() - started > 230000 || invalidatedBy !== null) break;
      second = await runVerb(candidate, "second row, acting on what the first left selected");
      if (second) secondHow = `the rail action "${candidate.label}" (${candidate.verb}) run after the first row — it acts on what the first action left selected`;
    }
    const selectAll = verbs.find((row) => row.verb === "selectAll" && !row.disabled) ?? null;
    if (!second && selectAll && invalidatedBy === null) {
      await openRail();
      if (renderer === "wgpu") await wgpuPress(selectAll.id);
      else {
        const item = pane.locator(`[id="${selectAll.id}"]`).first();
        const label = item.locator('[data-slot="tree-label"]').first();
        await ((await label.count().catch(() => 0)) ? label : item).click({ timeout: 3000 }).catch(() => {});
      }
      await sleep(1200);
      for (const candidate of others.filter((row) => rank(row.category) === 1).slice(0, 3)) {
        if (second !== null || Date.now() - started > 260000) break;
        second = await runVerb(candidate, "second row, after Select All");
        if (second) secondHow = `the rail action "${candidate.label}" (${candidate.verb}) run after Select All`;
      }
    }
    if (!second && invalidatedBy === null) {
      second = await runVerb(producer, "second row, the same action again");
      if (second) secondHow = `the rail action "${producer.label}" once more (a row that does not depend on the first)`;
    }
  }
  await closeRail();
  await openHistory();
  const entriesNow = documentEntries(await allHistoryRows()).sort((left, right) => Number(right.key) - Number(left.key));
  const route = first && producer ? `the rail action "${producer.label}" (${producer.verb}) ${first.form ? "executed with its staged defaults" : "run without a form"}` : entriesNow.length > 0 ? "History rows the editor booted with (no rail action produced a row with its defaults)" : "";
  if (first !== null || entriesNow.length > 0) verdict("u2-two-mutation-rows-without-editor-knowledge", (first !== null && second !== null) || (first === null && entriesNow.length >= 2), { route, firstRow: first?.label ?? entriesNow[1]?.label ?? entriesNow[0]?.label ?? null, secondRow: second?.label ?? (first === null ? (entriesNow[0]?.label ?? null) : null), secondRowBy: secondHow || null, railActionsTried: tried, documentRows: entriesNow.length, reading: "two document rows are made without knowing the editor — rail actions executed with the defaults their forms stage (the second preferably one that acts on what the first left selected), else rows of the booted example; the journey edits the OLDER one so that a later row exists" });
  else note("u2-NOT-OFFERED-no-mutation-row", { railActionsTried: tried, verbs: verbs.length, candidates: candidates.map((row) => row.verb), reading: "no rail action produced a document row with its declared defaults and the editor booted with an empty history: the journey has no row to edit" });
  const older = first ? entriesNow.filter((row) => row.id === first!.id) : entriesNow.filter((row) => !isHistoryEdit(row)).slice(1, 2);
  const inspect = [...older, ...entriesNow.filter((row) => !older.includes(row) && !isHistoryEdit(row))].slice(0, 4);
  const offers: string[] = [];
  let target: { key: string; label: string; entry: number } | null = null;
  for (const entry of inspect) {
    if (target) break;
    const mutations = entry.expandable ? (await expandEntry(entry.id)).mutations.slice(0, 6) : [];
    for (const mutation of mutations) {
      let edit = await rowActionState(mutation.id, copy.edit, false);
      let withdraw = await rowActionState(mutation.id, copy.withdraw, false);
      for (let attempt = 0; attempt < 5 && !edit.present && !withdraw.present; attempt++) {
        await sleep(800);
        const again = (await expandEntry(entry.id)).mutations.find((row) => row.key === mutation.key);
        if (!again) continue;
        edit = await rowActionState(again.id, copy.edit, false);
        withdraw = await rowActionState(again.id, copy.withdraw, false);
      }
      offers.push(`${entry.label.slice(0, 48)} › ${mutation.label.slice(0, 60)}: edit ${!edit.present ? "absent" : edit.disabled ? `disabled: ${edit.reason}` : "offered"}, withdraw ${!withdraw.present ? "absent" : withdraw.disabled ? `disabled: ${withdraw.reason}` : "offered"}`);
      if (edit.present && edit.disabled !== true) {
        target = { key: mutation.key, label: mutation.label, entry: Number(entry.key) };
        break;
      }
    }
  }
  let editingWords = false;
  let inventory: UniversalControl[] = [];
  const operated: Record<string, unknown> = {};
  let conflict: { by: string; path: string[] } | null = null;
  let reviewReady = false;
  if (!target) note("u3-NOT-OFFERED-no-mutation-offers-edit", { offers: offers.slice(0, 12), reading: "no mutation of the inspected rows offers Edit enabled (each is withdraw-only or its inputs are not published): the editor, the draft, the replay and the finalize of an EDIT cannot be driven; u4–u6 are not reached" });
  else {
    await installBandTrace();
    const begun = await beginEditOf(target.key);
    const controlsRead = begun.band?.stage === "editing" ? await waitUntil(universalControls, (list) => list.length > 0, 8000, 400) : null;
    inventory = controlsRead?.value ?? [];
    editingWords = BAND_WORDS.find(([stage]) => stage === "editing")?.[1].test(begun.band?.text ?? "") ?? false;
    const describe = (row: UniversalControl) => `${row.role} "${row.name}" [${row.pointer}] value=${row.value ?? ""}${row.min !== null ? ` min=${row.min}` : ""}${row.max !== null ? ` max=${row.max}` : ""}${row.step !== null ? ` step=${row.step}` : ""}${row.options !== null ? ` options=${row.options}` : ""}${row.disabled ? " [read-only]" : ""}`;
    verdict("u3-edit-opens-an-editor-with-contract-controls", begun.band?.stage === "editing" && inventory.length > 0, { mutation: target.label, via: begun.via, band: begun.band?.text?.slice(0, 140) ?? null, controls: inventory.length, inventory: inventory.map(describe), offers: offers.slice(0, 8), reading: "Edit on a mutation row opens the session (band `editing`) and the history editor with at least one input control of a contract role; the inventory is the per-editor evidence that the leaf's inputs carry UI metadata" });
    let open = begun.band?.stage === "editing" && inventory.length > 0;
    const roles = [...new Set(inventory.filter((row) => !row.disabled).map((row) => row.role))];
    for (const role of open ? roles : []) {
      const name = `u-control-${role.replace(/\s+/gu, "-")}`;
      if (Date.now() - started > 330000 || invalidatedBy !== null) {
        note(`${name}-NOT-REACHED`, { reason: invalidatedBy !== null ? "the run was invalidated" : "the journey's time budget for the control roles was spent" });
        continue;
      }
      const attemptsOfRole: Record<string, unknown>[] = [];
      let acknowledged = false;
      for (let nth = 0; nth < 3 && !acknowledged && open; nth++) {
        const control = (await waitUntil(universalControls, (list) => list.some((row) => row.role === role && !row.disabled), 6000, 400)).value.filter((row) => row.role === role && !row.disabled)[nth];
        if (!control) break;
        await dismissNotices();
        const how = await universalChange(control);
        const after = await waitUntil(universalControls, (list) => {
          const now = list.find((row) => row.pointer === control.pointer);
          return now !== undefined && now.value !== control.value;
        }, 5000, 300);
        const settled = await settledControl(control.pointer);
        const refusedWith = (await visibleNoticeCodes()).filter((code) => /^(timeTravel|app\.command)\./u.test(code));
        const stage = (await band())?.stage ?? null;
        acknowledged = after.ok && settled !== null && settled.value !== control.value && stage === "editing" && refusedWith.length === 0;
        let shownLater: string | null = null;
        if (refusedWith.length) {
          await sleep(1500);
          shownLater = (await universalControls()).find((row) => row.pointer === control.pointer)?.value ?? null;
        }
        attemptsOfRole.push({ control: describe(control), how, before: control.value, shownAfter: settled?.value ?? null, ...(refusedWith.length ? { refusedWith, notice: await visibleNoticeTexts(), shownTwoSecondsAfterTheRefusal: shownLater } : {}), band: stage });
        if (await band()) {
          await pressBand("discard");
          await sleep(900);
        }
        if (await band()) await exitSession();
        open = (await beginEditOf(target.key)).band?.stage === "editing";
      }
      operated[role] = { acknowledged, attempts: attemptsOfRole };
      const refused = attemptsOfRole.some((row) => Array.isArray(row.refusedWith));
      if (attemptsOfRole.length === 0) note(`${name}-NOT-OPERABLE`, { reason: "the role is not in the editor after it was reopened" });
      else if (role === "reference list" && !acknowledged && !refused) note(`${name}-NOT-OPERABLE`, { attempts: attemptsOfRole, reading: "the reference list holds no chip to remove and Use selection took nothing (the editor has no selection of the referenced kind at this point): the role is inventoried, its operation is not judged" });
      else verdict(name, acknowledged, { attempts: attemptsOfRole, reading: "a control of the role is operated by the smallest step the role offers (up to three controls of the role are tried); it counts when the editor republishes it with the new value, the band stays `editing` and the session raises no refusal notice — a value the session refuses (`timeTravel.invalid-input`) is not a draft, whatever the field shows. The draft is discarded afterwards so the next role starts from the recorded inputs" });
      if (!open) {
        note("u-control-loop-stopped-the-session-did-not-reopen", { after: role });
        break;
      }
    }
    if (open) {
      const { changed, attempts } = await universalChangeOneInput();
      const bandDrafted = await band();
      let whileEditing = await allHistoryRows();
      const downstream = whileEditing.filter((row) => row.kind === "entry" && Number(row.key) > target!.entry && documentEntries([row]).length > 0);
      let pending = whileEditing.filter((row) => row.text.includes(copy.pending));
      const latest=downstream.at(-1);
      if (pending.length === 0 && latest?.expandable) {
        await expandEntry(latest.id);
        whileEditing = await allHistoryRows();
        pending = whileEditing.filter((row) => row.text.includes(copy.pending));
      }
      verdict("u4-changing-one-input-is-acknowledged-as-a-draft", changed !== null && bandDrafted?.stage === "editing" && downstream.length > 0 && pending.length > 0, { changed, attempts, band: bandDrafted?.text?.slice(0, 140) ?? null, laterRows: downstream.map((row) => row.label.slice(0, 60)), rowsReadingNotApplied: pending.map((row) => row.text.slice(0, 90)).slice(0, 4), expected: copy.pending, reading: "one control is changed by one step on a clean draft; the editor republishes it, the band stays `editing`, and the later document row reads \"not applied\" while the draft is open" });
      const viaAccept = await pressBand("accept");
      const firstReviewed = await reviewed();
      const firstReview = firstReviewed.value?.review ?? null;
      const firstOutcome = firstReviewed.value?.outcome ?? null;
      const path: string[] = [`Accept (${viaAccept}) → ${firstReview ?? `no review (band ${firstReviewed.value?.stage ?? "gone"})`}${firstOutcome ? `, worst outcome ${firstOutcome}` : ""}`];
      const resolved = await resolveBlockers(firstReviewed.value, path);
      const trace = (await bandTrace()).map((row) => row.key.split("|"));
      const stages = trace.map((parts) => parts[0]).filter((stage, index, list) => stage && stage !== list[index - 1]);
      const progress = trace.filter((parts) => parts[0] === "replaying").map((parts) => parts[2] ?? "").filter(Boolean);
      reviewReady = resolved?.review === "ready";
      if (firstReview === "blocked" && reviewReady) conflict = { by: "the edit itself (u5)", path };
      verdict("u5-accept-replays-to-a-review-and-blockers-resolve", reviewReady, { path, firstReview, firstOutcome, stagesSeen: stages.slice(0, 14), text: resolved?.text?.slice(0, 180) ?? null, reading: "Accept replays the later mutations and ends in a review; a blocked review is resolved through Next problem → Withdraw of the blocker → Accept until it reads ready; the path taken is recorded" });
      if (stages.includes("replaying")) verdict("u5-the-replay-is-shown-as-a-stage-with-its-progress", progress.length > 0, { stagesSeen: stages.slice(0, 14), progressFrames: [...new Set(progress)].slice(0, 8), reading: "the band reads `replaying` while the later rows are applied again and its progress names done / total" });
      else note("u5-the-replay-finished-within-one-refresh", { stagesSeen: stages.slice(0, 14), laterRows: downstream.length, reading: "no DOM commit showed the band in `replaying`: the replay of the later rows ended before the band rendered a frame for it, so the progress is not judged" });
      if (reviewReady) {
        const seqBefore = newestEntrySeq(await allHistoryRows());
        const viaFinalize = await pressBand("finalize");
        const dialog = await waitUntil(finalizeDialog, (value) => value !== null, 15000);
        const offered = { overwrite: dialog.value?.overwrite?.text ?? null, newAlternative: dialog.value?.submit?.text ?? null, nameField: Boolean(dialog.value?.name), title: dialog.value?.title ?? null };
        if (dialog.ok) await dialogChoose("overwrite");
        const gone = await waitUntil(band, (state) => state === null, 30000);
        const edited = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => Number(row.key) > seqBefore && isHistoryEdit(row)) ?? null, (row) => row !== null, 20000, 800);
        verdict("u6-finalize-offers-overwrite-and-new-alternative-and-overwrites", dialog.ok && offered.overwrite !== null && offered.newAlternative !== null && gone.ok && edited.ok, { via: viaFinalize, offered, sessionClosed: gone.ok, row: edited.value?.label ?? null, reading: "Finalize opens the prompt with the destructive Overwrite AND New alternative (with its name field); Overwrite closes the session and History lists the history-edit row" });
      } else note("u6-NOT-REACHED-the-review-is-not-ready", { path, review: resolved?.review ?? null, text: resolved?.text?.slice(0, 180) ?? null });
    } else note("u4-NOT-REACHED-the-editor-did-not-open", { band: begun.band?.stage ?? null, via: begun.via });
  }
  if (await band()) await exitSession();
  await sleep(800);
  if (target && reviewReady) {
    const labelsBeforeAlternative = await stableDocumentLabels();
    const alternativesBefore = await readAlternatives();
    const seqBeforeAlternative = newestEntrySeq(await allHistoryRows());
    const begun = await beginEditOf(target.key);
    const draft = begun.band?.stage === "editing" ? await universalChangeOneInput() : { changed: null, attempts: [] };
    const name = `probe ${editor} ${currentLocale} ${stamp.slice(11, 19)}`;
    const path: string[] = [];
    let ready: Awaited<ReturnType<typeof band>> = null;
    if (draft.changed) {
      const via = await pressBand("accept");
      const review = await reviewed();
      path.push(`Accept (${via}) → ${review.value?.review ?? "no review"}`);
      ready = await resolveBlockers(review.value, path);
    }
    if (ready?.review === "ready") {
      const viaFinalize = await pressBand("finalize");
      const dialog = await waitUntil(finalizeDialog, value => value !== null, 15000);
      const nameValue = dialog.ok ? await dialogFillName(name) : null;
      const submitted = nameValue === name && await dialogChoose("submit");
      const gone = await waitUntil(band, state => state === null, 30000);
      const edited = await waitUntil(async () => documentEntries(await allHistoryRows()).find(row => Number(row.key) > seqBeforeAlternative && isHistoryEdit(row) && row.label.includes(name)) ?? null, row => row !== null, 20000, 800);
      const listed = await waitUntil(readAlternatives, rows => rows.some(row => row.text.includes(name) && isCurrentAlternative(row)) && rows.some(row => !row.text.includes(name)), 15000, 800);
      verdict("u6-second-edit-finalizes-as-a-named-new-alternative", dialog.ok && nameValue === name && submitted && gone.ok && edited.ok && listed.ok, { changed: draft.changed, attempts: draft.attempts, path, via: viaFinalize, name, nameValue, submitted, sessionClosed: gone.ok, row: edited.value?.label ?? null, alternatives: listed.value.map(row => ({ key: row.key, text: row.text, current: isCurrentAlternative(row) })), reading: "a second clean edit is accepted and finalized through New alternative; History records its name, the named alternative is current, and the previous head remains offered" });
      const previous = listed.value.find(row => !row.text.includes(name) && (alternativesBefore.some(before => before.key === row.key && isCurrentAlternative(before)) || alternativesBefore.length === 0));
      const mine = listed.value.find(row => row.text.includes(name));
      if (previous && mine) {
        const viaPrevious = await switchAlternative(previous);
        const restored = await waitUntil(stableDocumentLabels, labels => JSON.stringify(labels) === JSON.stringify(labelsBeforeAlternative), 15000, 800);
        const viaMine = await switchAlternative(mine);
        const current = await waitUntil(readAlternatives, rows => rows.some(row => row.key === mine.key && isCurrentAlternative(row)), 15000, 800);
        verdict("u6-new-alternative-preserves-and-switches-the-previous-head", restored.ok && current.ok, { viaPrevious, viaMine, restoredRows: restored.value.length, expectedRows: labelsBeforeAlternative.length, current: current.value.filter(isCurrentAlternative).map(row => row.text), reading: "switching to the prior head restores its exact history, then switching back selects the new named alternative" });
      }
    } else verdict("u6-second-edit-reaches-review-for-new-alternative", false, { begun: begun.band?.stage ?? null, changed: draft.changed, attempts: draft.attempts, path, review: ready?.review ?? null, reading: "after Overwrite the same mutation must remain editable so its second clean change can finalize as a named alternative" });
    if (await band()) await exitSession();
    await sleep(800);
  }
  if (conflict === null) {
    const journeyRows = documentEntries(await allHistoryRows()).filter((row) => !isHistoryEdit(row) && Number(row.key) > seqAtBoot).sort((left, right) => Number(left.key) - Number(right.key));
    const upstream = journeyRows[0] ?? null;
    const upstreamMutation = upstream?.expandable ? ((await expandEntry(upstream.id)).mutations[0] ?? null) : null;
    const offer = upstreamMutation ? await rowActionState(upstreamMutation.id, copy.withdraw, false) : null;
    if (journeyRows.length < 2 || !upstreamMutation || !offer?.present || offer.disabled === true) note("u-conflict-NOT-OFFERED-no-upstream-row-to-withdraw", { journeyRows: journeyRows.map((row) => row.label.slice(0, 60)), withdraw: offer ? { present: offer.present, disabled: offer.disabled, reason: offer.reason } : null });
    else {
      const labelsBeforeConflict = await stableDocumentLabels();
      const forWithdraw = await findMutationRow(upstreamMutation.key);
      const via = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
      let after = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
      if (after.value?.stage === "editing") {
        await pressBand("accept");
        after = await reviewed();
      }
      const path: string[] = [`Withdraw on "${upstreamMutation.label.slice(0, 60)}" (${via}) → ${after.value?.review ?? "no review"}${after.value?.outcome ? `, worst outcome ${after.value.outcome}` : ""}`];
      const resolved = after.value?.review === "blocked" ? await resolveBlockers(after.value, path) : after.value;
      if (after.value?.review === "blocked") {
        if (resolved?.review === "ready") conflict = { by: "withdrawing the upstream row", path };
        verdict("u-conflict-a-blocked-review-is-resolved-through-next-problem", resolved?.review === "ready", { by: "withdrawing the upstream row", path, text: resolved?.text?.slice(0, 180) ?? null, reading: "withdrawing the older of the two rows leaves the later row without what it depends on: the review blocks, Next problem names the blocker, withdrawing it reaches ready" });
      } else note("u-conflict-NOT-OFFERED-the-later-row-survives-the-withdraw", { path, review: after.value?.review ?? null, outcome: after.value?.outcome ?? null, text: after.value?.text?.slice(0, 180) ?? null, reading: "the later row does not depend on the withdrawn one strongly enough to fail: no blocked review to resolve on this editor's two rows" });
      if (await band()) await exitSession();
      await waitUntil(stableDocumentLabels, (labels) => JSON.stringify(labels) === JSON.stringify(labelsBeforeConflict), 10000, 1000);
    }
  } else verdict("u-conflict-a-blocked-review-is-resolved-through-next-problem", true, { by: conflict.by, path: conflict.path, reading: "the conflict clause was taken in u5: the edit made a row Fatal, the review blocked, Next problem named the blocker, withdrawing it reached ready" });
  const withdrawOffers: string[] = [];
  const withdrawable = async () => {
    const entries = documentEntries(await allHistoryRows()).filter((row) => !isHistoryEdit(row)).sort((left, right) => Number(right.key) - Number(left.key)).slice(0, 4);
    for (const entry of entries) {
      const mutations = entry.expandable ? (await expandEntry(entry.id)).mutations.slice(0, 6) : [];
      for (const mutation of mutations) {
        const offer = await rowActionState(mutation.id, copy.withdraw, false);
        withdrawOffers.push(`${entry.label.slice(0, 40)} › ${mutation.label.slice(0, 50)}: ${!offer.present ? "absent" : offer.disabled ? `disabled: ${offer.reason}` : "offered"}`);
        if (offer.present && offer.disabled !== true) return { key: mutation.key, label: mutation.label };
      }
    }
    return null;
  };
  let pull = await withdrawable();
  let rowMadeForIt: string | null = null;
  if (!pull && producer && invalidatedBy === null) {
    seq = Math.max(seq, newestEntrySeq(await allHistoryRows()));
    const again = await runVerb(producer, "a row for Withdraw → Restore");
    await closeRail();
    await openHistory();
    rowMadeForIt = again?.label ?? null;
    if (again) pull = await withdrawable();
  }
  const labelsBefore = await stableDocumentLabels();
  if (!pull) note("u7-NOT-OFFERED-no-row-offers-withdraw", { withdrawOffers: withdrawOffers.slice(0, 12), rowMadeForIt, documentRows: labelsBefore.length });
  else {
    const forWithdraw = await findMutationRow(pull.key);
    const viaWithdraw = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
    let withdrawn = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
    if (withdrawn.value?.stage === "editing") {
      await pressBand("accept");
      withdrawn = await reviewed();
    }
    let forRestore = await findMutationRow(pull.key);
    let restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
    let restoreWaitedMs = 0;
    for (let attempt = 0; attempt < 7 && !restoreOffer?.present; attempt++) {
      await sleep(800);
      restoreWaitedMs += 800;
      forRestore = await findMutationRow(pull.key);
      restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
    }
    const viaRestore = forRestore && restoreOffer?.present ? await pressRowAction(forRestore.id, copy.restore) : "absent";
    await sleep(1500);
    const afterRestore = await band();
    if (await band()) await exitSession();
    await sleep(800);
    const labelsAfter = await stableDocumentLabels();
    verdict("u7-withdraw-then-restore-then-exit-leaves-zero-trace", viaWithdraw === "action" && withdrawn.ok && viaRestore === "action" && (await band()) === null && JSON.stringify(labelsAfter) === JSON.stringify(labelsBefore), { mutation: pull.label, rowMadeForIt, withdraw: { via: viaWithdraw, review: withdrawn.value?.review ?? null }, restore: { offered: restoreOffer ? { present: restoreOffer.present, disabled: restoreOffer.disabled, reason: restoreOffer.reason } : null, offeredAfterMs: restoreWaitedMs, via: viaRestore, bandAfter: afterRestore?.stage ?? null }, rows: [labelsBefore.length, labelsAfter.length], reading: "Withdraw on a mutation row opens a session with that mutation withdrawn; Restore on the same row drops the draft; leaving the session leaves the document rows exactly as they were" });
  }
  const faults = { uncaught: pageErrors.filter((row) => row.locale === currentLocale).length, hard: hardFaults.filter((row) => row.locale === currentLocale).length };
  const rawLabels = [...verbs.filter((row) => row.label === "" || row.label === row.verb).map((row) => `rail verb ${row.verb}: "${row.label}"`), ...inventory.filter((row) => row.name === "" || row.name === row.pointer || row.name === row.pointer.split(".").pop()).map((row) => `control [${row.pointer}]: "${row.name}"`)];
  verdict("u8-no-faults-and-labels-in-the-locale", faults.uncaught === 0 && faults.hard === 0 && rawLabels.length === 0 && commandsLabel.toLowerCase().includes(copy.commands.toLowerCase()) && (target === null || editingWords), { ...faults, commandsSection: commandsLabel.slice(0, 40), expectedCommands: copy.commands, bandSpokeTheLocale: target === null ? "no session was opened" : editingWords, labelsThatAreRawKeys: rawLabels.slice(0, 10), firstErrors: pageErrors.slice(0, 3).map((row) => row.text.slice(0, 160)), reading: "no uncaught page error and no hard guest fault during the journey; the History section and the band speak the locale under test; no rail verb and no editor control is labelled by its raw key or left without a name" });
  const file = join(OUT, `${editor}-controls.json`);
  let record: { locales?: Record<string, unknown> } & Record<string, unknown> = {};
  try {
    record = JSON.parse(readFileSync(file, "utf8")) as typeof record;
  } catch {
    record = {};
  }
  writeFileSync(file, JSON.stringify({ ...record, editor, url: shell.url, title: shell.title, windows: shell.windows, locales: { ...(record.locales ?? {}), [currentLocale]: { at: new Date().toISOString(), route, secondRowBy: secondHow || null, editedMutation: target?.label ?? null, railVerbs: verbs.map((row) => ({ verb: row.verb, category: row.category, label: row.label })), controls: inventory.map((row) => ({ pointer: row.pointer, role: row.role, name: row.name, value: row.value, min: row.min, max: row.max, step: row.step, options: row.options, readOnly: row.disabled })), operated, conflict } } }, null, 1));
  note("u-journey-summary", { editor, url: shell.url, route, secondRowBy: secondHow || null, controls: inventory.length, rolesOperated: Object.keys(operated), reviewReady, conflictBy: conflict?.by ?? null, controlsFile: file, tookMs: Date.now() - started, verbs: verbs.length });
  await closePanels();
};
//#endregion 🔖️Steps
