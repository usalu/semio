/** 📜️ The history panel: windowed rows, entries, row actions and the inventory. */
import { renderer, page } from "../🧠️state/🟦️.ts";
import { mirror, chromeHits, mirrorActivate, mirrorFind, mirrorFocus } from "../🧊️wgpu-mirror/🟦️.ts";
import { evalSafe, markToken, sleep, waitUntil } from "../🔬️probe/🟦️.ts";
import { openTab, HISTORY_TAB, presentKey, byId, resolveDomId } from "../🧱️chrome/🟦️.ts";
import { note, log } from "../🧾️report/🟦️.ts";

//#region 🔖️History

export type HistoryRow = { id: string; kind: "entry" | "mutation"; key: string; label: string; text: string; expandable: boolean; expanded: boolean; parent: string | null; index: number; icon: string };

/** 📃️ Every materialised history row (`framework.history.entry.<seq>` and `framework.history.mutation.<id>`) in reading
 * order; a mutation row belongs to the entry row above it. wgpu answers from the ARIA mirror (label plus description). */
export const readHistory = async (): Promise<HistoryRow[]> => {
  if (renderer !== "wgpu") return reactReadHistory();
  let parent: string | null = null;
  return (await mirror())
    .filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.(entry\.\d+|mutation\.[^\/␟\u001f]+)$/.test(node.key))
    .map((node, index): HistoryRow => {
      const match=node.key.match(/(?:^|[\/␟\u001f])framework\.history\.(entry|mutation)\.([^\/␟\u001f]+)$/),kind=match?.[1],key=match?.[2];
      if((kind!=="entry"&&kind!=="mutation")||key===undefined)throw Error("Actual history mirror identity required");
      if (kind === "entry") parent = node.key;
      return { id: node.key, kind, key, label: node.label.replace(/\s+/g, " ").trim(), text: `${node.label} ${node.description}`.replace(/\s+/g, " ").trim().slice(0, 400), expandable: node.expanded !== null, expanded: node.expanded === "true", parent: kind === "mutation" ? parent : null, index, icon: "" };
    });
};

const reactReadHistory = () =>
  evalSafe(
    () => {
      const rows = Array.from(document.querySelectorAll("[id]")).filter((el) => /(^|[\/␟])framework\.history\.(entry\.\d+|mutation\.[^\/␟]+)$/.test(el.id));
      let parent: string | null = null;
      return rows.map((el, index): HistoryRow => {
        const match=el.id.match(/(?:^|[\/␟])framework\.history\.(entry|mutation)\.([^\/␟]+)$/),kind=match?.[1],key=match?.[2];
        if((kind!=="entry"&&kind!=="mutation")||key===undefined)throw Error("Actual history DOM identity required");
        const labelEl = el.querySelector('[data-slot="tree-label"]') as HTMLElement | null;
        const first = labelEl?.querySelector("span") as HTMLElement | null;
        if (kind === "entry") parent = el.id;
        return {
          id: el.id,
          kind,
          key,
          label: (first?.innerText ?? labelEl?.innerText ?? "").replace(/\s+/g, " ").trim(),
          text: [(el as HTMLElement).innerText, el.getAttribute("title") ?? "", ...Array.from(el.querySelectorAll("[title]")).map((node) => node.getAttribute("title") ?? "")].join(" ").replace(/\s+/g, " ").trim().slice(0, 400),
          expandable: el.hasAttribute("aria-expanded"),
          expanded: el.getAttribute("aria-expanded") === "true",
          parent: kind === "mutation" ? parent : null,
          index,
          icon: el.querySelector('[data-slot="tree-icon"] [data-icon]')?.getAttribute("data-icon") ?? "",
        };
      });
    },
    [] as HistoryRow[],
  );

export type TreeWindowRead = { key: string; path: string | null; total: number; offset: number; length: number };

/** 🏢️ The tree window whose authored key is `key` (exact, or `/`-namespaced) as React stamps it on its branch container:
 * `data-tree-window-total` (every row the window holds, materialised or not), `-offset` and `-length` (the slice it
 * materialises now). Null while the container is not mounted, and always on wgpu, whose ARIA mirror publishes no window size.
 * @see ../../../../🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx */
export const treeWindow = (key: string): Promise<TreeWindowRead | null> =>
  renderer === "wgpu"
    ? Promise.resolve(null)
    : evalSafe(
        (target) => {
          const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
            const authored = node.getAttribute("data-tree-window-key") ?? "";
            return authored === target || authored.endsWith(`/${target}`);
          });
          if (!el) return null;
          const number = (name: string) => Number(el.getAttribute(name) ?? "NaN");
          return { key: el.getAttribute("data-tree-window-key") ?? target, path: el.getAttribute("data-tree-window-path"), total: number("data-tree-window-total"), offset: number("data-tree-window-offset"), length: number("data-tree-window-length") };
        },
        null as TreeWindowRead | null,
        key,
      );

/** 📶️ How many rows the history's Commands section holds — its window's `total`, never the rows the DOM holds (W1E-6). */
export const historyCommandsTotal = async () => {
  if (renderer === "wgpu") {
    const sizes = (await mirror()).filter((node) => !node.key.includes("::") && /(^|[\/␟\u001f])framework\.history\.entry\.\d+$/.test(node.key)).map((node) => node.setSize).filter((size): size is number => size !== null && Number.isFinite(size));
    return sizes.length ? Math.max(...sizes) : null;
  }
  const read = await treeWindow("framework.history.commands");
  return read && Number.isFinite(read.total) ? read.total : null;
};

/** ⏬️ Scrolls the tree window `key` until it materialises its last row (`offset + length = total`), the way a person scrolls a
 * long list to its end; answers the final window and how many scrolls it took (React only; null on wgpu). */
export const reachWindowEnd = async (key: string) => {
  if (renderer === "wgpu") return null;
  let read = await treeWindow(key);
  let scrolls = 0;
  for (; read && read.offset + read.length < read.total && scrolls < 60; scrolls++) {
    await evalSafe(
      (target) => {
        const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
          const authored = node.getAttribute("data-tree-window-key") ?? "";
          return authored === target || authored.endsWith(`/${target}`);
        });
        (el?.lastElementChild as HTMLElement | null)?.scrollIntoView({ block: "end" });
        return Boolean(el);
      },
      false,
      key,
    );
    await sleep(500);
    read = await treeWindow(key);
  }
  return read ? { ...read, scrolls, atEnd: read.offset + read.length >= read.total } : null;
};

/** ⏫️ Scrolls the history panel's scroll container to its start or end (the Commands section is windowed). wgpu: a wheel
 * over the panel's registered scroll region. */
export const scrollHistory = async (where: "start" | "end") => {
  if (renderer !== "wgpu") return reactScrollHistory(where);
  const region = ((await chromeHits()).hits ?? []).find((hit) => /scroll/i.test(hit.kind) && /history/i.test(`${hit.controlId} ${hit.windowId ?? ""}`));
  if (!region) return false;
  await page.mouse.move(region.rect[0] + region.rect[2] / 2, region.rect[1] + region.rect[3] / 2);
  await page.mouse.wheel(0, where === "end" ? 20000 : -20000);
  return true;
};

const reactScrollHistory = (where: "start" | "end") =>
  evalSafe(
    (edge) => {
      const row = Array.from(document.querySelectorAll("[id]")).find((el) => /framework\.history\.(entry\.\d+|commands)$/.test(el.id));
      let node = row?.parentElement ?? null;
      while (node) {
        const style = getComputedStyle(node);
        if ((style.overflowY === "auto" || style.overflowY === "scroll") && node.scrollHeight > node.clientHeight + 4) {
          node.scrollTop = edge === "end" ? node.scrollHeight : 0;
          return true;
        }
        node = node.parentElement;
      }
      return false;
    },
    false,
    where,
  );

/** 🗄️ Opens the History panel and waits for its body (`framework.history.actions`). */
export const openHistory = async () => {
  const opened = await openTab(HISTORY_TAB);
  const body = await waitUntil(() => presentKey("framework.history.actions"), (id) => id !== null, 15000);
  if (renderer !== "wgpu" && body.ok) await waitUntil(() => treeWindow("framework.history.commands"), (read) => read !== null && (read.total === 0 || read.length > 0), 8000, 150);
  if (renderer === "wgpu" && body.ok) {
    let last = -1;
    await waitUntil(async () => (await readHistory()).length, (count) => {
      const settled = count > 0 && count === last;
      last = count;
      return settled;
    }, 12000, 800);
  }
  return opened && body.ok;
};

/** 🛞️ Scrolls the History body by `fraction` of its viewport (negative = up); answers false once it cannot move that way.
 * wgpu: a wheel of that share of the panel's registered scroll region. */
export const scrollHistoryBy = async (fraction: number) => {
  if (renderer === "wgpu") {
    const region = ((await chromeHits()).hits ?? []).find((hit) => /scroll/i.test(hit.kind) && /history/i.test(`${hit.controlId} ${hit.windowId ?? ""}`));
    if (!region) return false;
    await page.mouse.move(region.rect[0] + region.rect[2] / 2, region.rect[1] + region.rect[3] / 2);
    await page.mouse.wheel(0, region.rect[3] * fraction);
    return true;
  }
  return evalSafe(
    (share) => {
      const row = Array.from(document.querySelectorAll("[id]")).find((el) => /framework\.history\.(entry\.\d+|commands)$/.test(el.id));
      let node = row?.parentElement ?? null;
      while (node) {
        const style = getComputedStyle(node);
        if ((style.overflowY === "auto" || style.overflowY === "scroll") && node.scrollHeight > node.clientHeight + 4) {
          const before = node.scrollTop;
          node.scrollTop = before + node.clientHeight * share;
          return Math.abs(node.scrollTop - before) > 1;
        }
        node = node.parentElement;
      }
      return false;
    },
    false,
    fraction,
  );
};

/** 📖️ Pages through the windowed History body from its start (`down`) or its end (`up`), calling `visit` with the rows each
 * page materialises until it answers true (found), every row the Commands window counts (`total`) was seen (unless `deep`: a
 * search for a mutation row goes on through an expanded row's own window, which holds hundreds of rows under one entry), or the
 * body stops moving / shows nothing new (two pages without a window total — wgpu —, four while the total says rows remain). */
export const pageHistory = async (direction: "down" | "up", visit: (rows: HistoryRow[]) => Promise<boolean>, deep = false) => {
  await openHistory();
  const whole = renderer === "wgpu" && !deep ? await readHistory() : [];
  const wholeTotal = whole.length ? await historyCommandsTotal() : null;
  const scrolled = wholeTotal !== null && whole.filter((row) => row.kind === "entry").length >= wholeTotal ? false : await scrollHistory(direction === "down" ? "start" : "end");
  if (scrolled) await sleep(600);
  await waitUntil(readHistory, (rows) => rows.filter((row) => row.kind === "entry" && row.expanded).every((entry) => rows.some((child) => child.parent === entry.id)), 2500, 150);
  const total = await historyCommandsTotal();
  const seen = new Set<string>();
  let idle = 0;
  for (let index = 0; index < 80; index++) {
    const rows = await readHistory();
    const fresh = rows.filter((row) => !seen.has(row.id)).length;
    for (const row of rows) seen.add(row.id);
    if (await visit(rows)) return true;
    idle = fresh ? 0 : idle + 1;
    if (!deep && total !== null && [...seen].filter((id) => /framework\.history\.entry\.\d+$/.test(id)).length >= total) break;
    if (!scrolled || idle >= (total === null ? 2 : 4) || !(await scrollHistoryBy(direction === "down" ? 0.8 : -0.8))) break;
    await sleep(350);
  }
  return false;
};

/** 🧵️ Every history row of the windowed body, paged from its start to its end (it ends scrolled to the newest rows), merged by id. */
export const allHistoryRows = async () => {
  const merged = new Map<string, HistoryRow>();
  for (let attempt = 0; attempt < 3 && ![...merged.values()].some((row) => row.kind === "entry"); attempt++) {
    if (attempt) await sleep(700);
    await pageHistory("down", async (rows) => {
      for (const row of rows) merged.set(row.id, row);
      return false;
    });
  }
  return [...merged.values()];
};

/** 🪬️ Whether a Commands row is a shell chrome command — a panel toggle or tab switch, which the probe itself causes every time it
 * opens or closes a panel to read the body (React: the row's `monitor` icon; wgpu: its label, the mirror carries no icon). */
export const isShellChromeRow = (row: HistoryRow) => row.kind === "entry" && !row.expandable && (row.icon === "monitor" || /^(Toggle Panel|Switch Panel Tab|Panel umschalten|Panel-Tab wechseln)$/.test(row.label));

/** 📑️ The entry rows that are document or history-edit rows: expandable rows (they carry mutation children) plus
 * rows labelled as a history edit. Chrome rows (panel toggles, tool arming) are neither. */
export const documentEntries = (rows: HistoryRow[]) => rows.filter((row) => row.kind === "entry" && (row.expandable || /History edit|Verlauf bearbeitet|Verlaufsbearbeitung/.test(row.label)));

/** 🆔️ The edit ids the document rows carry: each expandable row is expanded on the page that shows it and its mutation
 * children's keys (`<editId>#<op>`) read — the label-independent identity of what history holds. */
export const documentEditIds = async (_rows: HistoryRow[]) => {
  const ids = new Set<string>();
  const expanded = new Set<string>();
  await pageHistory("down", async (rows) => {
    for (const row of rows) if (row.kind === "mutation") ids.add(row.key.replace(/#\d+$/, ""));
    for (const entry of documentEntries(rows).filter((row) => row.expandable && !expanded.has(row.id)).slice(0, 12)) {
      expanded.add(entry.id);
      for (const mutation of (await expandEntry(entry.id)).mutations) ids.add(mutation.key.replace(/#\d+$/, ""));
    }
    return expanded.size >= 48;
  });
  return [...ids].sort();
};

/** 🔤️ Whether a row label is the locale-invariant op text a row falls back to without a label (`<mutation-kind> node { id=… }`). */
export const readsAsOpText = (label: string) => /^[a-z]+(?:-[a-z]+)+\s/.test(label) || /\{\s*\w+=/.test(label);

/** 🕵️ The mutation row `key` names (`framework.history.mutation.<key>`): paged from the newest rows upwards, expanding each
 * collapsed document row a page shows until the row is materialised (a row lists its mutations only while expanded). */
export const findMutationRow = async (key: string) => {
  let found: HistoryRow | null = null;
  const expanded = new Set<string>();
  await pageHistory("up", async (rows) => {
    found = rows.find((row) => row.kind === "mutation" && row.key === key) ?? null;
    for (const entry of [...documentEntries(rows)].reverse().filter((row) => row.expandable && !expanded.has(row.id) && (!row.expanded || !rows.some((child) => child.parent === row.id)))) {
      if (found) break;
      expanded.add(entry.id);
      found = (await expandEntry(entry.id)).mutations.find((row) => row.key === key) ?? null;
    }
    return found !== null;
  }, true);
  return found as HistoryRow | null;
};

/** 🆕️ The first mutation row of an expandable document row newer than `newestBefore` that `matches`, with its entry row. */
export const newMutation = async (newestBefore: number, matches: (row: HistoryRow) => boolean, timeoutMs = 30000) => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    for (const entry of documentEntries(await allHistoryRows()).filter((row) => row.expandable && Number(row.key) > newestBefore)) {
      const mutation = (await expandEntry(entry.id)).mutations.find(matches);
      if (mutation) return { entry: entry as HistoryRow | null, mutation: mutation as HistoryRow | null, waitedMs: Date.now() - start };
    }
    await sleep(1000);
  }
  const rows = await allHistoryRows();
  note("history-rows-at-the-missed-mutation", { newestBefore, entries: rows.filter((row) => row.kind === "entry" && Number(row.key) > newestBefore).map((row) => `${row.key}=${row.label}${row.expandable ? (row.expanded ? " (open)" : " (expandable)") : ""}`).slice(0, 14), mutations: rows.filter((row) => row.kind === "mutation").map((row) => `${row.parent ?? "?"} › ${row.label}`).slice(0, 8), reading: "what the History body showed when no new mutation row matched within the wait" });
  return { entry: null as HistoryRow | null, mutation: null as HistoryRow | null, waitedMs: Date.now() - start };
};

/** 🔝️ The newest entry sequence among `rows` (entry rows are keyed by their command-log seq). The Commands list is
 * windowed, so "new rows" are the ones NEWER than this, never the ones missing from an earlier read. */
export const newestEntrySeq = (rows: HistoryRow[]) => Math.max(-1, ...rows.filter((row) => row.kind === "entry").map((row) => Number(row.key)).filter(Number.isFinite));

/** 🌳️ Expands one entry row through its disclosure button (wgpu: its activation, else focus + ArrowRight) and waits for
 * its mutation children. */
export const expandEntry = async (entryId: string) => {
  if (renderer === "wgpu") {
    const row = (await mirror()).find((node) => node.key === entryId);
    let state = !row ? "absent" : row.expanded === "true" ? "open" : "pointer on the chevron";
    if (state === "pointer on the chevron") {
      const hits = (await chromeHits()).hits ?? [];
      const chevron = hits.find((hit) => hit.controlId === `tree.chevron.${entryId}`);
      const label = hits.find((hit) => hit.controlId === `tree.label.${entryId}`);
      if (chevron) await page.mouse.click(chevron.rect[0] + chevron.rect[2] / 2, chevron.rect[1] + chevron.rect[3] / 2);
      else if (label) {
        state = "pointer on the row's right end";
        await page.mouse.click(label.rect[0] + label.rect[2] - 7, label.rect[1] + label.rect[3] / 2);
      } else state = "no hit for the row";
    }
    const children = await waitUntil(readHistory, (rows) => rows.some((entry) => entry.parent === entryId), 14000, 400);
    if (!children.ok) {
      const hits = ((await chromeHits()).hits ?? []).filter((hit) => hit.controlId.includes(entryId)).map((hit) => `${hit.controlId} [${hit.rect.map((value) => Math.round(value)).join(",")}]`);
      log(`expand ${entryId}: ${state}, no child after ${children.waitedMs} ms; row now expanded=${(await mirror()).find((node) => node.key === entryId)?.expanded ?? "absent"}; hits ${JSON.stringify(hits)}; mutation rows in the mirror ${JSON.stringify(children.value.filter((entry) => entry.kind === "mutation").map((entry) => `${entry.parent} › ${entry.label}`).slice(0, 4))}`);
    }
    return { state, mutations: children.value.filter((entry) => entry.parent === entryId) };
  }
  const state = await evalSafe(
    (id) => {
      const row = document.getElementById(id);
      if (!row) return "absent";
      if (row.getAttribute("aria-expanded") === "true") return "open";
      const chevron = row.querySelector("button[aria-expanded]") as HTMLButtonElement | null;
      if (chevron) chevron.click();
      else (row as HTMLElement).click();
      return chevron ? "chevron" : "row";
    },
    "absent",
    entryId,
  );
  const children = await waitUntil(readHistory, (rows) => rows.some((row) => row.parent === entryId), 10000);
  return { state, mutations: children.value.filter((row) => row.parent === entryId) };
};

/** 🕹️ Presses the row action of `rowId` whose accessible name matches `name` (hovering first, the action strip is a
 * reveal region); falls back to the row's own label, which is the row's activation. */
export const pressRowAction = async (rowId: string, name: RegExp) => {
  if (renderer === "wgpu") {
    const action = (await mirror()).find((node) => node.key.startsWith(`${rowId}::row-action::`) && name.test(node.label.trim()));
    if (action) return (await mirrorActivate(action.key, action.window)) ? "action" : "absent";
    return (await mirrorActivate(rowId)) ? "label" : "absent";
  }
  const token = markToken();
  await byId(rowId).hover({ timeout: 3000 }).catch(() => {});
  const via = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      if (!row) return "absent";
      const pattern = new RegExp(arg.source, arg.flags);
      const candidates = Array.from(row.querySelectorAll('button, [role="button"], [data-slot="action"]'));
      const hit = candidates.find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? (el as HTMLElement).innerText ?? "").trim()));
      if (hit) {
        hit.setAttribute("data-probe-target", arg.token);
        return "action";
      }
      const label = row.querySelector('[data-slot="tree-label"]');
      (label ?? row).setAttribute("data-probe-target", arg.token);
      return "label";
    },
    "absent",
    { rowId, source: name.source, flags: name.flags, token },
  );
  if (via !== "absent") await page.locator(`[data-probe-target="${token}"]`).first().click({ force: true, timeout: 4000 }).catch((error) => log(`pressRowAction ${rowId} click failed ${String(error).split("\n")[0]}`));
  return via;
};

type ReasonReveal = { hover: boolean; leave: boolean; focus: boolean; escape: boolean; click: boolean; blur: boolean; describedBy: boolean };

type RowActionState = { present: boolean; disabled: boolean | null; reason: string; label: string; shown: string | null; reveal: ReasonReveal | null; via: string; native?: boolean; busy?: boolean };

/** 🚫️ The row action of `rowId` named by `name` as a person perceives it (gap N15, W1E-1): whether it is disabled, the reason
 * it names to assistive technology (always the button's `aria-describedby`; wgpu the mirror button's description) and, on
 * React unless `reveal` is false, the reason a sighted person sees — `[data-slot=row-action-reason][data-revealed]` carrying
 * the reason under the same id: revealed after a hover (≈ 400 ms), on keyboard focus and on a press, concealed again on
 * leave, Escape and blur. wgpu's painted tooltip is outside the DOM, so its mirror description stands for it. */
export const rowActionState = async (rowId: string, name: RegExp, reveal = true): Promise<RowActionState> => {
  if (renderer === "wgpu") {
    const action = (await mirror()).find((node) => node.key.startsWith(`${rowId}::row-action::`) && name.test(node.label.trim()));
    return action ? { present: true, disabled: action.disabled, reason: action.description, label: action.label, shown: action.description || null, reveal: null, via: action.key, native: false, busy: action.busy } : { present: false, disabled: null, reason: "", label: "", shown: null, reveal: null, via: "absent" };
  }
  const token = markToken();
  const read = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      const pattern = new RegExp(arg.source, arg.flags);
      const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
      if (!button) return null;
      button.setAttribute("data-probe-target", arg.token);
      const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
      return { disabled: button.getAttribute("aria-disabled") === "true" || (button as HTMLButtonElement).disabled, reason, label: (button.getAttribute("aria-label") ?? button.innerText ?? "").trim(), native: (button as HTMLButtonElement).disabled === true, busy: button.getAttribute("aria-busy") === "true" };
    },
    null as { disabled: boolean; reason: string; label: string; native: boolean; busy: boolean } | null,
    { rowId, source: name.source, flags: name.flags, token },
  );
  if (!read) return { present: false, disabled: null, reason: "", label: "", shown: null, reveal: null, via: "absent" };
  if (!reveal || !read.reason) return { present: true, ...read, shown: null, reveal: null, via: "action" };
  const target = page.locator(`[data-probe-target="${token}"]`).first();
  const revealed = () =>
    evalSafe(
      (arg) => {
        const button = document.querySelector(`[data-probe-target="${arg.token}"]`);
        const ids = (button?.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean);
        const hint = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="row-action-reason"][data-revealed]')).find((el) => (el.textContent ?? "").includes(arg.reason) && el.getBoundingClientRect().width > 2 && getComputedStyle(el).visibility !== "hidden");
        return { shown: hint ? (hint.textContent ?? "").trim().slice(0, 160) : null, describedBy: Boolean(hint && ids.includes(hint.id)) };
      },
      { shown: null as string | null, describedBy: false },
      { token, reason: read.reason },
    );
  await target.hover({ timeout: 3000, force: true }).catch(() => {});
  await sleep(900);
  const hover = await revealed();
  await page.mouse.move(1, 1).catch(() => {});
  await sleep(400);
  const leave = await revealed();
  await target.focus().catch(() => {});
  await sleep(300);
  const focus = await revealed();
  await page.keyboard.press("Escape").catch(() => {});
  await sleep(300);
  const escape = await revealed();
  await target.click({ force: true, timeout: 3000 }).catch(() => {});
  await sleep(300);
  const click = await revealed();
  await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
  await page.mouse.move(1, 1).catch(() => {});
  await sleep(300);
  const blur = await revealed();
  const sequence: ReasonReveal = { hover: hover.shown !== null, leave: leave.shown === null, focus: focus.shown !== null, escape: escape.shown === null, click: click.shown !== null, blur: blur.shown === null, describedBy: hover.describedBy || focus.describedBy || click.describedBy };
  const shown = hover.shown ?? focus.shown ?? click.shown;
  return { present: true, ...read, shown: Object.values(sequence).every(Boolean) ? shown : null, reveal: sequence, via: "action" };
};

/** 👇️ Taps the row action of `rowId` named by `name` with a touch (a phone or tablet person) and answers whether the reason of
 * a disabled one then shows as `[data-slot=row-action-reason][data-revealed]` (React; null on wgpu, whose tooltip is painted). */
export const tapRevealsReason = async (rowId: string, name: RegExp) => {
  if (renderer === "wgpu") return null;
  const target = await evalSafe(
    (arg) => {
      const row = document.getElementById(arg.rowId);
      const pattern = new RegExp(arg.source, arg.flags);
      const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
      if (!button) return null;
      button.scrollIntoView({ block: "center" });
      const rect = button.getBoundingClientRect();
      const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
      return { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2, disabled: button.getAttribute("aria-disabled") === "true", reason };
    },
    null as { x: number; y: number; disabled: boolean; reason: string } | null,
    { rowId, source: name.source, flags: name.flags },
  );
  if (!target) return { tapped: false, disabled: null as boolean | null, reason: "", shown: null as string | null };
  await page.touchscreen.tap(target.x, target.y);
  await sleep(500);
  const shown = await evalSafe((reason) => (reason ? (Array.from(document.querySelectorAll<HTMLElement>('[data-slot="row-action-reason"][data-revealed]')).find((el) => (el.textContent ?? "").includes(reason) && el.getBoundingClientRect().width > 2)?.textContent?.trim() ?? null) : null), null as string | null, target.reason);
  return { tapped: true, disabled: target.disabled as boolean | null, reason: target.reason, shown };
};

/** 🔘️ Clicks a panel-body button by its authored id (`framework.history.editor.withdraw`, …), first scrolling its section and
 * its row into view unless `reveal` is false (a deep row of a windowed section the caller already scrolled to). */
export const pressAuthored = async (authored: string, reveal = true) => {
  const section = authored.match(/framework\.history\.(editor|timeTravel)/)?.[0];
  if (section && reveal) {
    await revealHistory(section);
    await revealHistory(`${authored}.row`);
  }
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = mirrorFind(nodes, authored) ?? mirrorFind(nodes, `${authored}.row`);
    if (!node) return { present: false, disabled: null as boolean | null, id: null as string | null };
    await mirrorActivate(node.key, node.window);
    return { present: true, disabled: node.disabled as boolean | null, id: node.key as string | null };
  }
  const id = (await resolveDomId(authored)) ?? (await resolveDomId(`${authored}.row`));
  if (!id) return { present: false, disabled: null as boolean | null, id: null as string | null };
  const disabled = await evalSafe((target) => {
    const el = document.getElementById(target);
    const button = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
    if (button) return button.disabled;
    const flag = el?.getAttribute("aria-disabled") ?? el?.getAttribute("data-disabled");
    return flag === null || flag === undefined ? null : flag === "true" || flag === "";
  }, null as boolean | null, id);
  const target = page.locator(`[id="${id}"]`).first();
  const tag = await target.evaluate((el) => el.tagName.toLowerCase()).catch(() => "");
  const inner = tag === "button" ? 1 : await target.locator("button").count().catch(() => 0);
  await (tag === "button" ? target : inner ? target.locator("button").first() : target.locator('[data-slot="tree-label"]').first()).click({ force: true, timeout: 4000 }).catch(() => target.click({ force: true, timeout: 4000 }).catch(() => {}));
  return { present: true, disabled, id };
};

/** 📇️ The history-panel inventory for diagnosis: every id under `framework.history` (wgpu: every mirrored history, band,
 * dialog and indicator node), with role and state. */
export const historyInventory = async (): Promise<Record<string, unknown>[]> =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.history|shell\.time-travel|shell\.dialog|ui\.timeTravel|timeTravel\.indicator|framework\.sync|s-presence-peers|shell\.panel\.tab/.test(node.key)).slice(0, 400).map((node) => ({ id: node.key, window: node.window, role: node.role, expanded: node.expanded, disabled: node.disabled, aria: node.label, title: node.description, value: node.value ?? node.valueNow, text: node.label }))
    : reactHistoryInventory();

const reactHistoryInventory = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll("[id]"))
        .filter((el) => /framework\.history|framework\.panel\.history|ui\.dialog/.test(el.id))
        .slice(0, 400)
        .map((el) => ({
          id: el.id,
          tag: el.tagName.toLowerCase(),
          slot: el.getAttribute("data-slot"),
          role: el.getAttribute("role"),
          expanded: el.getAttribute("aria-expanded"),
          disabled: (el as HTMLButtonElement).disabled ?? null,
          aria: el.getAttribute("aria-label"),
          title: el.getAttribute("title"),
          value: (el as HTMLInputElement).value ?? null,
          text: (el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 140) ?? "",
        })),
    [] as Record<string, unknown>[],
  );
//#endregion 🔖️History

/** 🪜️ Scrolls the history body so the section (or row) `authored` names sits at the top of the panel — what a user does to
 * reach it; the body is windowed, so rows below the viewport are not in the document until they scroll in. */
export const revealHistory = async (authored: string) => {
  if (renderer === "wgpu") {
    let node = mirrorFind(await mirror(), authored);
    if (!node && (await scrollHistory("end"))) {
      await sleep(700);
      node = mirrorFind(await mirror(), authored);
    }
    if (node) await mirrorFocus(node.key);
    await sleep(node ? 500 : 0);
    return Boolean(node);
  }
  const found = await evalSafe(
    (target) => {
      const el = Array.from(document.querySelectorAll("[id]")).find((node) => node.id === target || node.id.endsWith(`/${target}`) || node.id.endsWith(`␟${target}`)) as HTMLElement | undefined;
      el?.scrollIntoView({ block: "start" });
      return Boolean(el);
    },
    false,
    authored,
  );
  await sleep(found ? 500 : 0);
  return found;
};
