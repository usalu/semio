/** 🧩️ `<staffage-figure>`: the framework-neutral custom element of the staffage renderer; it renders SVG with the library writer and needs no React at the use site.
 *
 * Attributes: `asset` (catalog id), `view` (one of the seven representations), `viewpoint` (required: `elevation` side elevation, `front` front elevation, `plan` top view, or `all` for the aligned three-view sheet; missing or unknown shows an explicit problem), `scale` (denominator), `lang` (`en`|`de`, inherited from an ancestor when absent; nothing renders without one), `pose` | `clip` + `time`, `playing`, `rate`, `loop`,
 * `zoom`, `fit`, `theme` (`light`|`dark`, analytic views) (`paper`|`contain`), `transparent`, `no-patterns` (hatch patterns are the colour-independent reading and are on by default), `analytic` (adds the table alternative), `thermal-quantity`, `force-quantity`, `live`,
 * and `no-dimensions` / `no-scale-bar` / `no-legend` / `no-labels` / `no-ground` to switch annotations off. Every change re-renders.
 * Properties: `.state` (last evaluated frame or `null`), `.clock` (assign a shared `StaffageClock`). Events: `staffage-evaluated`, `staffage-inspect`.
 *
 * @see ../🧍️figure/🟦️.tsx — the React twin with the same semantics
 * @see ../🧮️pipeline/🟦️.ts — the shared evaluator
 */
import { LANGUAGES, THERMAL_QUANTITIES, FORCE_QUANTITIES, VIEWPOINTS } from "@teaching/staffage";
import type { Asset, DepictionOptions, EvaluationQuery, ForceQuantity, ItemValue, Language, Text, ThermalQuantity, Viewpoint } from "@teaching/staffage";
import { StaffageClock } from "../⏱️clock/🧱️core/🟦️.ts";
import { describeScene, isView, valueText, valuedItems, word } from "../🗣️labels/🟦️.ts";
import { defaultQuery, figureFrame, inlineSvg, queryAt } from "../🧮️pipeline/🟦️.ts";
import type { DepictionRequest, StaffageFrame } from "../🧮️pipeline/🟦️.ts";
import { SHEET_ORDER, VIEWPOINT_NAMES, VIEWPOINT_PHRASES, sheetOf, viewpointWord } from "../🔭️viewpoints/🟦️.ts";

/** 📚️ Where the element finds assets: a list, a map or a lookup. */
export type StaffageCatalog = readonly Asset[] | ReadonlyMap<string, Asset> | ((id: string) => Asset | undefined);

/** 🧾️ What `.state` exposes. */
export type StaffageElementState = { readonly asset: Asset; readonly query: EvaluationQuery; readonly frame: StaffageFrame; readonly frames: Readonly<Partial<Record<Viewpoint, StaffageFrame>>>; readonly language: Language };

/** 🧩️ The observed attributes of the element. */
export const STAFFAGE_ATTRIBUTES = ["asset", "view", "scale", "lang", "pose", "clip", "time", "playing", "rate", "loop", "zoom", "fit", "viewpoint", "theme", "transparent", "no-patterns", "analytic", "thermal-quantity", "force-quantity", "live", "no-dimensions", "no-scale-bar", "no-legend", "no-labels", "no-ground"] as const;

const HIDDEN = "position:absolute;width:1px;height:1px;margin:-1px;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap;border:0";

const pick = <T extends string>(options: readonly T[], value: string | null): T | undefined => options.find((option) => option === value);

function lookup(catalog: StaffageCatalog, id: string): Asset | undefined {
  if (typeof catalog === "function") return catalog(id);
  return "get" in catalog ? catalog.get(id) : catalog.find((asset) => asset.id === id);
}

function escapeHtml(text: string): string {
  return text.replace(/[&<>"]/gu, (c) => `&#${c.charCodeAt(0)};`);
}

/** 🧩️ Registers `<staffage-figure>` (or `name`) over `catalog` and returns the element class. */
export function defineStaffageElement(catalog: StaffageCatalog, name = "staffage-figure"): CustomElementConstructor {
  const existing = customElements.get(name);
  if (existing !== undefined) return existing;

  class StaffageFigureElement extends HTMLElement {
    static readonly observedAttributes = STAFFAGE_ATTRIBUTES;
    #state: StaffageElementState | null = null;
    #clock: StaffageClock | null = null;
    #shared: StaffageClock | null = null;
    #unsubscribe: (() => void) | null = null;
    #observer: IntersectionObserver | null = null;
    #visible = true;
    #scheduled = false;
    #lastKey = "";
    #focused = -1;
    readonly #root: ShadowRoot = this.attachShadow({ mode: "open" });

    get state(): StaffageElementState | null {
      return this.#state;
    }

    get clock(): StaffageClock | null {
      return this.#shared ?? this.#clock;
    }

    set clock(clock: StaffageClock | null) {
      this.#shared = clock;
      this.#bindClock();
      this.#schedule();
    }

    connectedCallback(): void {
      if (typeof IntersectionObserver === "function") {
        this.#observer = new IntersectionObserver((entries) => {
          this.#visible = this.hasAttribute("live") || (entries.at(-1)?.isIntersecting ?? true);
          if (this.#visible) this.#schedule();
        });
        this.#observer.observe(this);
      }
      this.#bindClock();
      this.#schedule();
    }

    disconnectedCallback(): void {
      this.#observer?.disconnect();
      this.#unsubscribe?.();
      this.#unsubscribe = null;
      this.#clock?.dispose();
      this.#clock = null;
    }

    attributeChangedCallback(attribute: string): void {
      if (["playing", "rate", "loop", "time"].includes(attribute)) this.#bindClock();
      this.#schedule();
    }

    #bindClock(): void {
      this.#unsubscribe?.();
      this.#unsubscribe = null;
      if (!this.isConnected) return;
      const rate = Number(this.getAttribute("rate") ?? 1) || 1;
      const time = Number(this.getAttribute("time") ?? 0) || 0;
      if (this.#shared === null) {
        this.#clock ??= new StaffageClock({ playing: false, reducedMotion: false });
        this.#clock.setRate(rate);
        this.#clock.setLoop(this.getAttribute("loop") !== "false");
        if (!this.hasAttribute("playing")) this.#clock.seek(time);
        if (this.hasAttribute("playing")) this.#clock.play();
        else this.#clock.pause();
      }
      const clock = this.clock;
      this.#unsubscribe = clock?.subscribe(() => this.#schedule()) ?? null;
    }

    #schedule(): void {
      if (this.#scheduled) return;
      this.#scheduled = true;
      queueMicrotask(() => {
        this.#scheduled = false;
        if (this.isConnected) this.#render();
      });
    }

    #language(): Language | undefined {
      const own = this.closest("[lang]")?.getAttribute("lang")?.slice(0, 2).toLowerCase();
      return pick(LANGUAGES, this.getAttribute("lang")?.slice(0, 2).toLowerCase() ?? own ?? null);
    }

    #render(): void {
      const language = this.#language();
      const id = this.getAttribute("asset");
      const asset = id === null ? undefined : lookup(catalog, id);
      const view = this.getAttribute("view");
      const denominator = Number(this.getAttribute("scale"));
      const choice = this.getAttribute("viewpoint");
      const viewpoint = pick(VIEWPOINTS, choice) as Viewpoint | undefined;
      const sheeted = choice === "all";
      if (language === undefined) {
        this.#state = null;
        this.#lastKey = "";
        this.#root.replaceChildren();
        return;
      }
      const problem: Text | null =
        asset === undefined
          ? { en: `Unknown asset "${id ?? ""}".`, de: `Unbekanntes Asset "${id ?? ""}".` }
          : !isView(view)
            ? { en: `Attribute view must be one of the seven representations, not "${view ?? ""}".`, de: `Das Attribut view muss eine der sieben Darstellungen sein, nicht "${view ?? ""}".` }
            : !(denominator > 0)
              ? { en: "Attribute scale must be a positive denominator.", de: "Das Attribut scale muss ein positiver Nenner sein." }
              : viewpoint === undefined && !sheeted
                ? { en: `Attribute viewpoint is required: elevation, front, plan or all, not "${choice ?? ""}".`, de: `Das Attribut viewpoint ist erforderlich: elevation, front, plan oder all, nicht "${choice ?? ""}".` }
                : null;
      if (problem !== null || asset === undefined || !isView(view)) {
        const message = problem ?? { en: "Unknown problem.", de: "Unbekanntes Problem." };
        const problemKey = JSON.stringify(["problem", message, language]);
        if (problemKey !== this.#lastKey) {
          this.#lastKey = problemKey;
          this.#state = null;
          this.#root.innerHTML = `<div role="alert" part="problem" lang="${language}" style="border:1px dashed currentColor;padding:0.5em">${escapeHtml(message[language])}</div>`;
        }
        return;
      }
      const clock = this.clock;
      const animated = this.hasAttribute("playing") || this.#shared !== null || this.hasAttribute("time");
      const timeS = clock === null || !animated ? null : clock.state.frame / clock.state.fps;
      if (!this.#visible && this.#state !== null) return;
      const pose = this.getAttribute("pose");
      const clip = this.getAttribute("clip");
      const base: EvaluationQuery = pose !== null ? { kind: "pose", pose } : clip !== null ? { kind: "clip", clip, timeS: timeS ?? 0 } : defaultQuery(asset, timeS);
      const query = queryAt(base, timeS);
      const theme = pick(["light", "dark"] as const, this.getAttribute("theme"));
      const thermalQuantity = pick(THERMAL_QUANTITIES, this.getAttribute("thermal-quantity")) as ThermalQuantity | undefined;
      const forceQuantity = pick(FORCE_QUANTITIES, this.getAttribute("force-quantity")) as ForceQuantity | undefined;
      const annotations: Partial<DepictionOptions["annotations"]> = {
        ...(this.hasAttribute("no-dimensions") ? { dimensions: false } : {}),
        ...(this.hasAttribute("no-scale-bar") ? { scaleBar: false } : {}),
        ...(this.hasAttribute("no-legend") ? { legend: false } : {}),
        ...(this.hasAttribute("no-labels") ? { labels: false } : {}),
        ...(this.hasAttribute("no-ground") ? { ground: false } : {}),
      };
      const request = {
        view,
        scale: { denominator },
        language,
        annotations,
        ...(theme === undefined ? {} : { theme }),
        ...(this.hasAttribute("transparent") ? { transparent: true } : {}),
        patterns: !this.hasAttribute("no-patterns"),
        ...(thermalQuantity === undefined ? {} : { thermalQuantity }),
        ...(forceQuantity === undefined ? {} : { forceQuantity }),
      };
      const fit = this.getAttribute("fit") === "contain" ? "contain" : "paper";
      const zoom = Number(this.getAttribute("zoom") ?? 1) || 1;
      const analytic = this.hasAttribute("analytic");
      const key = JSON.stringify([asset.id, query, request, choice, fit, zoom, analytic]);
      if (key === this.#lastKey) return;
      this.#lastKey = key;
      if (sheeted) return this.#renderSheet(asset, query, request, language, zoom);
      const frame = figureFrame(asset, query, { ...request, viewpoint: viewpoint! });
      const { scene } = frame;
      const size = fit === "paper" ? `width:${scene.widthMm * zoom}mm;height:${scene.heightMm * zoom}mm` : `width:100%;height:100%;aspect-ratio:${scene.widthMm} / ${scene.heightMm}`;
      const items = valuedItems(scene);
      const table =
        analytic && items.length > 0
          ? `<table id="values" lang="${language}" style="${HIDDEN}"><caption>${word("values", language)}</caption><thead><tr><th scope="col">${word("item", language)}</th><th scope="col">${word("quantity", language)}</th><th scope="col">${word("value", language)}</th></tr></thead><tbody>${items.map((row) => `<tr><th scope="row">${escapeHtml(row.value.label[language])}</th><td>${escapeHtml(row.value.quantity)}</td><td>${escapeHtml(valueText(row.value, language))}</td></tr>`).join("")}</tbody></table>`
          : "";
      this.#root.innerHTML = `<style>:host{display:block;max-width:100%}.figure{position:relative}.figure:focus-visible{outline:2px solid currentColor;outline-offset:2px}</style><div class="figure" part="figure" data-viewpoint="${viewpoint}" style="${size}" role="img" tabindex="0" lang="${language}" aria-label="${escapeHtml(describeScene(scene, language))}" data-fidelity="${scene.metadata.fidelity}"${table === "" && scene.metadata.fidelity !== "conventional-symbol" ? "" : ` aria-describedby="${[table === "" ? "" : "values", scene.metadata.fidelity === "conventional-symbol" ? "fidelity" : ""].filter(Boolean).join(" ")}"`}>${inlineSvg(scene)}${scene.metadata.fidelity === "conventional-symbol" ? `<span id="fidelity" data-fidelity-note="" style="position:absolute;left:2px;bottom:2px;font-size:10px">${escapeHtml(word("conventionalSymbol", language))}</span>` : ""}</div>${table}`;
      this.#wireInspect(items);
      this.#state = { asset, query, frame, frames: { [viewpoint!]: frame }, language };
      this.dispatchEvent(new CustomEvent("staffage-evaluated", { detail: { state: frame.state, scene: frame.scene, query }, bubbles: true, composed: true }));
    }

    #renderSheet(asset: Asset, query: EvaluationQuery, request: Omit<DepictionRequest, "viewpoint">, language: Language, zoom: number): void {
      const sheet = sheetOf(asset, [query], { view: request.view, denominator: request.scale.denominator, language, ...(request.theme === undefined ? {} : { theme: request.theme }) });
      const mm = (value: number): string => `${value * zoom}mm`;
      const frames: Partial<Record<Viewpoint, StaffageFrame>> = {};
      const cells = SHEET_ORDER.map((at) => {
        const cell = sheet.layout.cells[at];
        const resolution = sheet.resolutions[at];
        const phrase = VIEWPOINT_PHRASES[at][language];
        const place = `grid-column:${cell.column};grid-row:${cell.row};width:${mm(cell.widthMm)};height:${mm(cell.heightMm)}`;
        if (!resolution.available)
          return `<figure data-viewpoint="${at}" data-available="false" aria-label="${escapeHtml(VIEWPOINT_NAMES[at][language])}" style="margin:0;${place}"><div data-notice="unavailable" style="border:1px dashed currentColor;padding:0.5em;height:100%;box-sizing:border-box;overflow:auto"><p style="margin:0"><b>${escapeHtml(viewpointWord("viewpoints.unavailable", language, { view: phrase }))}</b></p><p style="margin:0">${escapeHtml(resolution.message[language])}</p>${resolution.details === undefined ? "" : `<details><summary>${escapeHtml(viewpointWord("viewpoints.failedDetails", language))}</summary><code>${escapeHtml(resolution.details)}</code></details>`}<p style="margin:0">${escapeHtml(viewpointWord("viewpoints.nothingInstead", language))}</p></div></figure>`;
        const frame = figureFrame(asset, query, { ...request, viewpoint: at });
        frames[at] = frame;
        return `<figure data-viewpoint="${at}" data-available="true" role="img" lang="${language}" aria-label="${escapeHtml(describeScene(frame.scene, language))}" style="position:relative;margin:0;overflow:hidden;${place}">${inlineSvg(frame.scene, cell.frame)}</figure>`;
      });
      const grid = `display:grid;grid-template-columns:${mm(sheet.layout.cells.elevation.widthMm)} ${mm(sheet.layout.cells.front.widthMm)};grid-template-rows:${mm(sheet.layout.cells.plan.heightMm)} ${mm(sheet.layout.cells.elevation.heightMm)};gap:${mm(sheet.layout.gapMm)}`;
      this.#root.innerHTML = `<style>:host{display:block;max-width:100%}</style><div part="figure" role="group" lang="${language}" aria-label="${escapeHtml(viewpointWord("viewpoints.sheet", language, { title: asset.title[language] }))}" data-three-views="" style="${grid}">${cells.join("")}</div>`;
      const first = SHEET_ORDER.find((at) => frames[at] !== undefined);
      if (first === undefined) {
        this.#state = null;
        return;
      }
      this.#state = { asset, query, frame: frames[first]!, frames, language };
      this.dispatchEvent(new CustomEvent("staffage-evaluated", { detail: { state: frames[first]!.state, scene: frames[first]!.scene, query, viewpoints: Object.keys(frames) }, bubbles: true, composed: true }));
    }

    #wireInspect(items: ReturnType<typeof valuedItems>): void {
      const figure = this.#root.querySelector(".figure");
      if (figure === null) return;
      const emit = (index: number): void => {
        const item = items[index];
        this.#focused = index;
        const detail: { value: ItemValue | null; item: { bone?: string; region?: string } | null } = { value: item?.value ?? null, item: item === undefined ? null : { ...(item.bone === undefined ? {} : { bone: item.bone }), ...(item.region === undefined ? {} : { region: item.region }) } };
        this.dispatchEvent(new CustomEvent("staffage-inspect", { detail, bubbles: true, composed: true }));
      };
      const ids = new Map(items.map((item, index) => [item.id.replace(/[^A-Za-z0-9_.-]/gu, "-"), index]));
      figure.addEventListener("pointermove", (event) => {
        for (let node = event.target instanceof Element ? event.target : null; node !== null && node !== figure; node = node.parentElement) {
          const index = ids.get(node.getAttribute("id") ?? "");
          if (index !== undefined) return emit(index);
        }
        emit(-1);
      });
      figure.addEventListener("pointerleave", () => emit(-1));
      figure.addEventListener("keydown", (event) => {
        const key = (event as KeyboardEvent).key;
        const step = key === "ArrowRight" || key === "ArrowDown" ? 1 : key === "ArrowLeft" || key === "ArrowUp" ? -1 : 0;
        if (step !== 0 && items.length > 0) {
          event.preventDefault();
          emit((this.#focused + step + items.length) % items.length);
        } else if (key === "Escape") emit(-1);
      });
    }
  }

  customElements.define(name, StaffageFigureElement);
  return StaffageFigureElement;
}
