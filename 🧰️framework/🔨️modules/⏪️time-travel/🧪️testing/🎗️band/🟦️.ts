/** 🎗️ The time-travel band, its trace, notices, chords and the finalize dialog. */
import { renderer, t0, currentLocale, chordLocales, page } from "../🧠️state/🟦️.ts";
import type { Locale } from "../🧠️state/🟦️.ts";
import { wgpuBand, BAND_WORDS, REVIEW_WORDS, WGPU_BAND_CONTROLS, wgpuPress, mirror, mirrorLocator, mirrorActivate } from "../🧊️wgpu-mirror/🟦️.ts";
import { evalSafe } from "../🔬️probe/🟦️.ts";
import { log, note, verdict } from "../🧾️report/🟦️.ts";
import { openPanelTabIds, HISTORY_TAB, inside } from "../🧱️chrome/🟦️.ts";
import type { Band } from "./📋️readout/🟦️.ts";
export type { Band };

//#region 🔖️Band


/** ⏱️ The time-travel band, or null when no session is live: React's `[data-semio-time-travel]`, wgpu's announced
 * `shell.time-travel.status` with its mirrored controls. */
export const band = (): Promise<Band | null> => (renderer === "wgpu" ? wgpuBand() : reactBand());

const reactBand = () =>
  evalSafe(
    () => {
      const el = document.querySelector("[data-semio-time-travel]");
      if (!el) return null;
      const attr = (selector: string, name: string) => el.querySelector(selector)?.getAttribute(name) ?? null;
      const progress = el.querySelector("progress");
      return {
        stage: el.getAttribute("data-semio-time-travel") ?? "",
        role: el.getAttribute("role"),
        live: el.getAttribute("aria-live"),
        generation: el.getAttribute("data-time-travel-generation"),
        text: (el as HTMLElement).innerText.replace(/\s+/g, " ").trim(),
        review: attr("[data-semio-time-travel-review]", "data-semio-time-travel-review"),
        outcome: attr("[data-semio-time-travel-outcome]", "data-semio-time-travel-outcome"),
        fault: attr("[data-semio-time-travel-fault]", "data-semio-time-travel-fault"),
        target: (el.querySelector("[data-semio-time-travel-target]") as HTMLElement | null)?.innerText ?? null,
        progress: progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : null,
        controls: Array.from(el.querySelectorAll("[data-semio-time-travel-control]")).map((button) => {
          const reason = (button.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)).filter((hint): hint is HTMLElement => hint !== null && hint.getAttribute("data-slot") === "row-action-reason").map((hint) => (hint.textContent ?? "").trim()).join(" ");
          return {
            control: button.getAttribute("data-semio-time-travel-control") ?? "",
            disabled: button.getAttribute("aria-disabled") === "true" || (button as HTMLButtonElement).disabled === true,
            title: reason || button.getAttribute("title"),
            keys: button.getAttribute("aria-keyshortcuts"),
            text: (button as HTMLElement).innerText.trim(),
            native: (button as HTMLButtonElement).disabled === true,
            id: button.id,
          };
        }),
      };
    },
    null as Band | null,
  );

/** 🎞️ Installs an in-page MutationObserver that records every distinct band state (stage | review | progress) with
 * its time, so a stage React rendered for one frame is still seen, and — for the page's life — every fault line the band shows
 * (React: `data-semio-time-travel-fault` code + its words; wgpu: a status line carrying a code-like token). Re-install after
 * every reload. */
export const installBandTrace = () =>
  renderer === "wgpu"
    ? evalSafe(
        (words) => {
          const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver; __probeBandFaults?: { code: string | null; text: string }[] };
          host.__probeBandTrace = [];
          host.__probeBandFaults ??= [];
          host.__probeBandObserver?.disconnect();
          let last = "";
          const read = () => {
            const el = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
            const text = el?.getAttribute("aria-label") ?? "";
            const code = /\b(?:timeTravel|vcs|history|plugin|document|module)\.[\w-]+/u.exec(text)?.[0] ?? null;
            if (code && !host.__probeBandFaults!.some((row) => row.text === text)) host.__probeBandFaults!.push({ code, text: text.slice(0, 300) });
            const stage = words.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const review = words.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
            const meter = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.progress"]') ?? el;
            const progress = meter?.getAttribute("aria-valuenow") !== null && meter?.getAttribute("aria-valuenow") !== undefined ? `${meter?.getAttribute("aria-valuenow")}/${meter?.getAttribute("aria-valuemax")}` : "";
            const key = el ? `${stage}|${review}|${progress}` : "none";
            if (key === last) return;
            last = key;
            host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
            if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
          };
          const root = document.getElementById("semio-wgpu-accessibility") ?? document.body;
          host.__probeBandObserver = new MutationObserver(read);
          host.__probeBandObserver.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-label", "aria-valuenow", "aria-valuemax"] });
          read();
          return true;
        },
        false,
        { stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
      )
    : reactInstallBandTrace();

const reactInstallBandTrace = () =>
  evalSafe(() => {
    const host = window as unknown as { __probeBandTrace?: { t: number; key: string }[]; __probeBandObserver?: MutationObserver; __probeBandFaults?: { code: string | null; text: string }[] };
    host.__probeBandTrace = [];
    host.__probeBandFaults ??= [];
    host.__probeBandObserver?.disconnect();
    let last = "";
    const read = () => {
      const el = document.querySelector("[data-semio-time-travel]");
      const fault = el?.querySelector<HTMLElement>("[data-semio-time-travel-fault]");
      const code = fault?.getAttribute("data-semio-time-travel-fault") ?? null;
      const words = (fault?.textContent ?? "").replace(/\s+/g, " ").trim();
      if (fault && !host.__probeBandFaults!.some((row) => row.code === code && row.text === words)) host.__probeBandFaults!.push({ code, text: words.slice(0, 300) });
      const progress = el?.querySelector("progress");
      const key = el ? `${el.getAttribute("data-semio-time-travel")}|${el.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? ""}|${progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : ""}` : "none";
      if (key === last) return;
      last = key;
      host.__probeBandTrace!.push({ t: Math.round(performance.now()), key });
      if (host.__probeBandTrace!.length > 400) host.__probeBandTrace!.shift();
    };
    host.__probeBandObserver = new MutationObserver(read);
    host.__probeBandObserver.observe(document.body, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-semio-time-travel", "data-semio-time-travel-review", "value", "max"] });
    read();
    return true;
  }, false);

type ShownNotice = { code: string; text: string; t?: number };

/** 🔔️ Records every transient notice the shell shows for the page's life, by code with its words — React's
 * `[data-notice-code]`, wgpu's polite `shell.notice` mirror node named by the message and described by the code
 * (`🧯️wgpu-transient-notice` corpus) — and, under the probe's own code `shell.document-restore`, React's document-restore
 * alert (`[data-semio-bootstrap-status][role=alert]`: "Document restore failed: …", `🪪️host-bootstrap`); re-install after a
 * reload. */
export const installNoticeTrace = () =>
  evalSafe((wgpu) => {
    const host = window as unknown as { __probeNotices?: { code: string; text: string; at: number }[]; __probeNoticeObserver?: MutationObserver };
    host.__probeNotices ??= [];
    host.__probeNoticeObserver?.disconnect();
    const keep = (code: string | null | undefined, text: string) => {
      const words = text.replace(/\s+/g, " ").trim().slice(0, 300);
      if (code && !host.__probeNotices!.some((row) => row.code === code && row.text === words)) host.__probeNotices!.push({ code, text: words, at: performance.now() });
    };
    const read = () => {
      if (wgpu) {
        const node = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.notice"]');
        const describedBy = node?.getAttribute("aria-describedby");
        keep(describedBy ? document.getElementById(describedBy)?.textContent?.trim() : null, node?.getAttribute("aria-label") ?? "");
        return;
      }
      for (const el of Array.from(document.querySelectorAll<HTMLElement>("[data-notice-code]"))) keep(el.getAttribute("data-notice-code"), el.textContent ?? "");
      for (const el of Array.from(document.querySelectorAll<HTMLElement>('[data-semio-bootstrap-status][role="alert"]'))) keep("shell.document-restore", el.textContent ?? "");
    };
    host.__probeNoticeObserver = new MutationObserver(read);
    host.__probeNoticeObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true, attributeFilter: wgpu ? undefined : ["data-notice-code"] });
    read();
    return true;
  }, false, renderer === "wgpu");

export const pageNotices = async (): Promise<ShownNotice[]> => {
  const rows = await evalSafe(() => ((window as unknown as { __probeNotices?: { code: string; text: string; at: number }[] }).__probeNotices ?? []).map((row) => ({ code: row.code, text: row.text, agoMs: performance.now() - row.at })), [] as { code: string; text: string; agoMs: number }[]);
  const clock = (Date.now() - t0) / 1000;
  return rows.map((row) => ({ code: row.code, text: row.text, t: Number((clock - row.agoMs / 1000).toFixed(1)) }));
};

export const harvestedNotices: ({ locale: Locale } & ShownNotice)[] = [];

/** 🧺️ Keeps the current page's notices for the locale before its page goes away (a reload, a closed fresh context). */
export const harvestNotices = async () => {
  for (const row of await pageNotices()) if (!harvestedNotices.some((kept) => kept.locale === currentLocale && kept.code === row.code && kept.text === row.text)) harvestedNotices.push({ locale: currentLocale, ...row });
  for (const row of await pageBandFaults()) if (!harvestedBandFaults.some((kept) => kept.locale === currentLocale && kept.code === row.code && kept.text === row.text)) harvestedBandFaults.push({ locale: currentLocale, ...row });
};

type BandFault = { code: string | null; text: string };

const pageBandFaults = () => evalSafe(() => ((window as unknown as { __probeBandFaults?: BandFault[] }).__probeBandFaults ?? []).slice(), [] as BandFault[]);

export const harvestedBandFaults: ({ locale: Locale } & BandFault)[] = [];

/** 🧨️ Every fault line the locale's bands showed so far (harvested pages and the current one). */
export const shownBandFaults = async (): Promise<BandFault[]> => {
  const rows = [...harvestedBandFaults.filter((row) => row.locale === currentLocale).map(({ code, text }) => ({ code, text })), ...(await pageBandFaults())];
  return rows.filter((row, index) => rows.findIndex((other) => other.code === row.code && other.text === row.text) === index);
};

/** 🗣️ Every notice the locale's pages showed so far (code and words): the harvested ones and the current page's. */
export const shownNotices = async (): Promise<ShownNotice[]> => {
  const rows: ShownNotice[] = [...harvestedNotices.filter((row) => row.locale === currentLocale).map(({ code, text, t }) => ({ code, text, t })), ...(await pageNotices())];
  return rows.filter((row, index) => rows.findIndex((other) => other.code === row.code && other.text === row.text) === index);
};

/** 📻️ Every notice code the locale's pages showed so far. */
export const shownNoticeCodes = async () => [...new Set((await shownNotices()).map((row) => row.code))];

export const bandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: { t: number; key: string }[] }).__probeBandTrace ?? []).slice(), [] as { t: number; key: string }[]);

export const clearBandTrace = () => evalSafe(() => ((window as unknown as { __probeBandTrace?: unknown[] }).__probeBandTrace = []).length, 0);

/** ⌨️ Puts keyboard focus where the shell admits a chord: React listens on the document, so the active field is blurred;
 * the wgpu wire admits keys only from its canvas or a mirror element, so the largest focusable canvas takes focus. */
export const prepareChord = async () => {
  if (renderer !== "wgpu") {
    await evalSafe(() => (document.activeElement as HTMLElement | null)?.blur?.(), undefined);
    return "blurred";
  }
  return evalSafe(() => {
    const canvases = Array.from(document.querySelectorAll("canvas")).sort((left, right) => right.width * right.height - left.width * left.height);
    for (const canvas of canvases) {
      canvas.focus();
      if (document.activeElement === canvas) return "canvas";
    }
    const status = document.querySelector<HTMLElement>('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
    status?.focus();
    return document.activeElement === status ? "status" : "none";
  }, "none");
};

type BandWay = "auto" | "chord" | "button";

/** ↔️ The way a chord locale does NOT take by default — the one press per locale that keeps both ways driven. */
export const otherWay = (): BandWay => (chordLocales.has(currentLocale) ? "button" : "chord");

/** 🎹️ Presses one band control by its button, or by its chord (`accept`, `discard`, `exit`): in a chord locale by default,
 * always when `way` is `chord`, never when it is `button`. */
export const pressBand = async (control: string, way: BandWay = "auto") => {
  const chords: Record<string, string> = { accept: "Alt+Enter", discard: "Alt+Backspace", exit: "Alt+Shift+Backspace" };
  if (chords[control] && (way === "chord" || (way === "auto" && chordLocales.has(currentLocale)))) {
    await prepareChord();
    await page.keyboard.press(chords[control]).catch(() => {});
    return `chord ${chords[control]}`;
  }
  if (renderer === "wgpu") {
    const id = WGPU_BAND_CONTROLS.find(([name]) => name === control)?.[1];
    return (id && (await wgpuPress(id))) ?? "absent";
  }
  await page.locator(`[data-semio-time-travel-control="${control}"]`).first().click({ timeout: 4000 }).catch((error) => log(`band ${control} click failed ${String(error).split("\n")[0]}`));
  return "button";
};

/** 🧱️ Every text of another element the band's own text lies on (React; null on wgpu, whose band is painted in the canvas):
 * sampled at three points of each text box of the band, a hit is an element outside the band — neither its ancestor nor its
 * descendant — one of whose own text boxes holds that point. An in-flow band with an opaque surface has none. Also answers
 * where the band sits (`[data-slot=layout-subfooter] > [data-semio-bottom-bands]`, the bands row under the footer). */
export const bandOverlaps = () =>
  renderer === "wgpu"
    ? Promise.resolve(null)
    : evalSafe(
        () => {
          const strip = document.querySelector<HTMLElement>("[data-semio-time-travel]");
          if (!strip) return null;
          const textBoxes = (el: Element) =>
            Array.from(el.childNodes)
              .filter((node) => node.nodeType === Node.TEXT_NODE && (node.textContent ?? "").trim().length > 0)
              .flatMap((node) => {
                const range = document.createRange();
                range.selectNodeContents(node);
                return Array.from(range.getClientRects()).filter((rect) => rect.width >= 2 && rect.height >= 2);
              });
          const points: { x: number; y: number }[] = [];
          for (const el of [strip, ...Array.from(strip.querySelectorAll<HTMLElement>("*"))]) for (const rect of textBoxes(el)) for (const share of [0.15, 0.5, 0.85]) points.push({ x: rect.left + rect.width * share, y: rect.top + rect.height / 2 });
          const hits = new Map<Element, string>();
          const covers = new Map<Element, string>();
          const name = (el: Element) => `${el.tagName.toLowerCase()}#${(el.closest("[id]") as HTMLElement | null)?.id.slice(-60) ?? ""}[${el.getAttribute("data-slot") ?? ""}] "${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}"`;
          const targets = Array.from(strip.querySelectorAll<HTMLElement>("[data-semio-time-travel-control]")).map((control) => control.getBoundingClientRect()).flatMap((rect) => [0.2, 0.5, 0.8].map((share) => ({ x: rect.left + rect.width * share, y: rect.top + rect.height / 2 })));
          for (const point of [...points, ...targets]) {
            if (point.x < 0 || point.y < 0 || point.x > innerWidth || point.y > innerHeight) continue;
            const top = document.elementFromPoint(point.x, point.y);
            if (top && !strip.contains(top) && !covers.has(top)) covers.set(top, name(top));
          }
          for (const point of points) {
            for (const el of document.elementsFromPoint(point.x, point.y)) {
              if (hits.has(el) || strip.contains(el) || el.contains(strip)) continue;
              const style = getComputedStyle(el);
              if (style.visibility === "hidden" || style.display === "none" || Number(style.opacity) === 0) continue;
              if (textBoxes(el).some((rect) => point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom)) hits.set(el, `${el.tagName.toLowerCase()}#${(el.closest("[id]") as HTMLElement | null)?.id.slice(-60) ?? ""}[${el.getAttribute("data-slot") ?? ""}] "${(el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}"`);
            }
          }
          const bands = strip.closest("[data-semio-bottom-bands]");
          const rect = strip.getBoundingClientRect();
          return { points: points.length + targets.length, overlaps: [...hits.values()].slice(0, 12), coveredBy: [...covers.values()].slice(0, 12), inBottomBands: bands !== null, inSubfooter: bands?.parentElement?.getAttribute("data-slot") === "layout-subfooter", slot: bands?.parentElement?.getAttribute("data-slot") ?? null, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)] };
        },
        null as { points: number; overlaps: string[]; coveredBy: string[]; inBottomBands: boolean; inSubfooter: boolean; slot: string | null; rect: number[] } | null,
      );

/** 🪩️ Judges {@link bandOverlaps} under `name` (a note on wgpu). */
export const bandOverlapVerdict = async (name: string) => {
  const read = await bandOverlaps();
  if (read === null) return note(`${name}-not-observable`, { reading: renderer === "wgpu" ? "the wgpu band is painted in the canvas; its layout is the wgpu shell law's" : "no band is open" });
  verdict(name, read.points > 0 && read.overlaps.length === 0 && read.coveredBy.length === 0, { ...read, reading: "the band's text lies on no other element's text and nothing is painted over its text or its controls (band rect ∩ footer / panel tab bars / panel rows = ∅; every control is the top element at its own points): an in-flow strip with its own surface, never an overlay and never under one" });
};

/** 🫧️ Whether the mutation row `key` is revealed WITHOUT the probe scrolling (React: History panel open, the row inside the
 * viewport and the top element at its start; wgpu: projected by the mirror, which holds only what is on screen). */
export const mutationRowRevealed = async (key: string) => {
  const open = (await openPanelTabIds()).includes(HISTORY_TAB);
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => !other.key.includes("::") && other.key.endsWith(`framework.history.mutation.${key}`));
    return { open, revealed: open && Boolean(node), row: node?.key ?? null };
  }
  return evalSafe(
    (arg) => {
      const row = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id.endsWith(`framework.history.mutation.${arg.key}`));
      if (!row) return { open: arg.open, revealed: false, row: null as string | null };
      const rect = row.getBoundingClientRect();
      const inside = rect.width > 0 && rect.height > 0 && rect.top >= 0 && rect.bottom <= innerHeight && rect.left >= 0 && rect.right <= innerWidth;
      const top = inside ? document.elementFromPoint(rect.left + Math.min(rect.width / 2, 60), rect.top + rect.height / 2) : null;
      return { open: arg.open, revealed: arg.open && inside && Boolean(top && (row.contains(top) || top.contains(row))), row: row.id as string | null };
    },
    { open, revealed: false, row: null as string | null },
    { key, open },
  );
};

type DialogButton = { text: string; destructive: string | null; tone: string | null; disabled: boolean | null; description: string | null };

type FinalizeDialog = { title: string; text: string; overwrite: DialogButton | null; submit: DialogButton | null; cancel: { text: string } | null; name: { id: string; value: string } | null };

/** 🗨️ The wgpu finalize dialog through the mirror: `shell.dialog.<id>` (title) with `.choice.overwrite`, `.confirm` (New
 * alternative), `.cancel` and `.field.name`; the chrome projects only the dialog's nodes while it is open. */
const wgpuFinalizeDialog = async (): Promise<FinalizeDialog | null> => {
  const nodes = await mirror();
  const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
  if (!root) return null;
  const button = (suffix: string): DialogButton | null => {
    const node = nodes.find((other) => other.key === `${root.key}.${suffix}`);
    return node ? { text: node.label, destructive: null, tone: null, disabled: node.disabled, description: node.description || null } : null;
  };
  const name = nodes.find((node) => node.key === `${root.key}.field.name`);
  return { title: root.label, text: nodes.filter((node) => node.key.startsWith(root.key)).map((node) => node.label).join(" ").slice(0, 500), overwrite: button("choice.overwrite"), submit: button("confirm"), cancel: button("cancel"), name: name ? { id: name.key, value: name.value ?? name.label } : null };
};

/** ✍️ Types the alternative name into the open finalize dialog; answers what the field then holds. */
export const dialogFillName = async (value: string) => {
  if (renderer === "wgpu") {
    const dialog = await wgpuFinalizeDialog();
    const field = dialog?.name ? await mirrorLocator(dialog.name.id) : null;
    if (!field) return null;
    await field.fill(value, { force: true, timeout: 4000 }).catch(() => {});
    return field.inputValue({ timeout: 2000 }).catch(() => null);
  }
  const field = page.locator('[role="dialog"] input[id="name"], [role="dialog"] input[type="text"]').first();
  await field.fill(value, { timeout: 4000 }).catch(() => {});
  return field.inputValue({ timeout: 2000 }).catch(() => null);
};

/** 🗳️ Presses the finalize dialog's destructive Overwrite choice or its New alternative submit. */
export const dialogChoose = async (choice: "overwrite" | "submit") => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const root = nodes.find((node) => /^shell\.dialog\.[^.]+$/.test(node.key) && nodes.some((other) => other.key === `${node.key}.confirm`));
    return root ? mirrorActivate(`${root.key}.${choice === "overwrite" ? "choice.overwrite" : "confirm"}`) : false;
  }
  return page.locator(`[id="${choice === "overwrite" ? "ui.dialog.choice.overwrite" : "ui.dialog.submit"}"]`).first().click({ timeout: 4000 }).then(() => true).catch((error) => {
    log(`dialog ${choice} click failed ${String(error).split("\n")[0]}`);
    return false;
  });
};

/** 💬️ The finalize dialog (`finalizeHistoryEdit`): title, the destructive Overwrite choice, the name field, submit. */
export const finalizeDialog = (): Promise<FinalizeDialog | null> => (renderer === "wgpu" ? wgpuFinalizeDialog() : reactFinalizeDialog());

const reactFinalizeDialog = () =>
  evalSafe(
    () => {
      const dialog = Array.from(document.querySelectorAll('[role="dialog"]')).find((el) => el.querySelector('[id="ui.dialog.submit"]')) as HTMLElement | undefined;
      if (!dialog) return null;
      const button = (id: string) => {
        const el = dialog.querySelector(`[id="${id}"]`);
        const control = (el?.matches("button") ? el : el?.querySelector("button")) as HTMLButtonElement | null;
        const describedBy = (control ?? el)?.getAttribute("aria-describedby");
        return el ? { text: (el as HTMLElement).innerText.trim(), destructive: el.getAttribute("data-destructive") ?? control?.getAttribute("data-destructive") ?? null, tone: el.getAttribute("data-tone") ?? control?.getAttribute("data-tone") ?? null, disabled: control?.disabled ?? null, description: describedBy ? (document.getElementById(describedBy)?.textContent ?? null) : null } : null;
      };
      const name = (dialog.querySelector('input[id="name"]') ?? dialog.querySelector('input[type="text"]')) as HTMLInputElement | null;
      return {
        title: (dialog.querySelector("h2") as HTMLElement | null)?.innerText.trim() ?? dialog.innerText.split("\n",1).join(""),
        text: dialog.innerText.replace(/\s+/g, " ").slice(0, 500),
        overwrite: button("ui.dialog.choice.overwrite"),
        submit: button("ui.dialog.submit"),
        cancel: button("ui.dialog.cancel"),
        name: name ? { id: name.id, value: name.value } : null,
      };
    },
    null as FinalizeDialog | null,
  );
//#endregion 🔖️Band
