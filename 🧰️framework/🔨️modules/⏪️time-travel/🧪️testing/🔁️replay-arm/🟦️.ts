/** ⏺️ The replay arm: an in-page recorder of the band while a replay runs. */
import { evalSafe } from "../📄️page/🟦️.ts";
import { band } from "../🎗️band/🟦️.ts";
import { renderer } from "../🧠️state/🟦️.ts";
import { BAND_WORDS, REVIEW_WORDS } from "../🧊️wgpu-mirror/🟦️.ts";

//#region 🔖️Band
//#region 🔖️ReplayArm

type ArmSample = { t: number; stage: string; review: string; done: number | null; total: number | null };

type ArmTarget = { t: number; stage: string; present: boolean; disabled: boolean | null; reason: string; how: string };

export type ArmLog = { samples: ArmSample[]; acted: { t: number; via: string; disabled: boolean | null; reason: string; stage: string; settledMs: number } | null; target: ArmTarget[] };

type ArmAction = { kind: "cancel" } | { kind: "edit"; rowId: string; source: string; flags: string };

/** 🧘️ How long after the band turned `replaying` the armed Edit press waits for the rows to follow it (one History body
 * refresh is ≥ 100 ms). */
export const EDIT_SETTLE_MS = 160;

/** 🪤️ Arms the page to act in the first frame the band shows `replaying` — a person pressing the moment the progress bar
 * appears — and samples every band state (stage, review, progress) with its in-page time until {@link disarmReplay}. `cancel`
 * presses the band's Cancel replay (React `[data-semio-time-travel-control=cancelReplay]`, wgpu `shell.time-travel.cancel-replay`);
 * `edit` presses Edit on the row `rowId` (its Edit row action, else the row's activation, which is the same verb) — as a person
 * does, on a SETTLED read: once the action reads disabled, else `EDIT_SETTLE_MS` after the band turned `replaying` (one body
 * refresh is ≥ 100 ms, so by then the rows have followed the band), with a timer beside the observer so a quiet DOM still ticks. */
export const armReplay = (action: ArmAction) =>
  evalSafe(
    (arg) => {
      const host = window as unknown as { __probeArm?: ArmLog; __probeArmObserver?: MutationObserver; __probeArmTimer?: number };
      host.__probeArmObserver?.disconnect();
      if (host.__probeArmTimer !== undefined) clearInterval(host.__probeArmTimer);
      const armedAt = performance.now();
      let replayingSince: number | null = null;
      const armLog: ArmLog = { samples: [], acted: null, target: [] };
      host.__probeArm = armLog;
      const read = (): Omit<ArmSample, "t"> => {
        if (arg.wgpu) {
          const status = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]');
          const text = status?.getAttribute("aria-label") ?? "";
          const stage = status ? (arg.stages.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "") : "none";
          const review = arg.reviews.find(([, source]) => new RegExp(source).test(text))?.[0] ?? "";
          const meter = document.querySelector('#semio-wgpu-accessibility [data-node-key="shell.time-travel.progress"]') ?? status;
          const now = meter?.getAttribute("aria-valuenow");
          const max = meter?.getAttribute("aria-valuemax");
          return { stage, review, done: now ? Number(now) : null, total: max ? Number(max) : null };
        }
        const band = document.querySelector("[data-semio-time-travel]");
        const progress = band?.querySelector("progress");
        return { stage: band?.getAttribute("data-semio-time-travel") ?? "none", review: band?.querySelector("[data-semio-time-travel-review]")?.getAttribute("data-semio-time-travel-review") ?? "", done: progress ? Number(progress.getAttribute("value")) : null, total: progress ? Number(progress.getAttribute("max")) : null };
      };
      const press = (): { via: string; disabled: boolean | null; reason: string } => {
        const disabledOf = (el: Element) => (el as HTMLButtonElement).disabled === true || el.getAttribute("aria-disabled") === "true";
        const reasonOf = (el: Element) => (el.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim();
        if (arg.action.kind === "cancel") {
          const control = document.querySelector<HTMLElement>(arg.wgpu ? '#semio-wgpu-accessibility [data-node-key="shell.time-travel.cancel-replay"]' : '[data-semio-time-travel-control="cancelReplay"]');
          if (!control) return { via: "absent", disabled: null, reason: "" };
          const disabled = disabledOf(control);
          control.click();
          return { via: "cancelReplay", disabled, reason: reasonOf(control) };
        }
        const pattern = new RegExp(arg.action.source, arg.action.flags);
        const rowId = arg.action.rowId;
        if (arg.wgpu) {
          const nodes = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]"));
          const button = nodes.find((node) => (node.dataset.nodeKey ?? "").startsWith(`${rowId}::row-action::`) && pattern.test((node.getAttribute("aria-label") ?? "").trim()));
          const target = button ?? nodes.find((node) => node.dataset.nodeKey === rowId);
          if (!target) return { via: "absent", disabled: null, reason: "" };
          const disabled = disabledOf(target);
          target.click();
          return { via: button ? "action" : "row", disabled, reason: reasonOf(target) };
        }
        const row = document.getElementById(rowId);
        if (!row) return { via: "absent", disabled: null, reason: "" };
        const button = Array.from(row.querySelectorAll<HTMLElement>('button, [role="button"]')).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
        const target = button ?? row.querySelector<HTMLElement>('[data-slot="tree-label"]') ?? row;
        const disabled = disabledOf(target) || row.getAttribute("aria-disabled") === "true";
        target.click();
        return { via: button ? "action" : "row", disabled, reason: reasonOf(target) };
      };
      const watch = (stage: string) => {
        if (arg.action.kind !== "edit") return;
        const pattern = new RegExp(arg.action.source, arg.action.flags);
        const rowId = arg.action.rowId;
        const button = arg.wgpu
          ? Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find((node) => (node.dataset.nodeKey ?? "").startsWith(`${rowId}::row-action::`) && pattern.test((node.getAttribute("aria-label") ?? "").trim()))
          : Array.from(document.getElementById(rowId)?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => pattern.test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
        const describedBy = (button?.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean);
        const how = button ? [(button as HTMLButtonElement).disabled === true ? "disabled attribute" : "", button.getAttribute("aria-disabled") === "true" ? "aria-disabled" : "", button.getAttribute("aria-busy") === "true" ? "aria-busy" : "", describedBy.length ? `describedby ${describedBy.length} id(s), ${describedBy.filter((id) => document.getElementById(id)).length} in the document` : "no aria-describedby", button.getAttribute("title") ? `title ${button.getAttribute("title")}` : ""].filter(Boolean).join("; ") : "";
        const state = { stage, present: Boolean(button), disabled: button ? (button as HTMLButtonElement).disabled === true || button.getAttribute("aria-disabled") === "true" : null, reason: button ? describedBy.map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim() : "", how };
        const last = armLog.target[armLog.target.length - 1];
        if (!last || last.stage !== state.stage || last.present !== state.present || last.disabled !== state.disabled || last.reason !== state.reason || last.how !== state.how) armLog.target.push({ t: Math.round(performance.now() - armedAt), ...state });
        if (armLog.target.length > 200) armLog.target.splice(1, 1);
      };
      const tick = () => {
        const state = read();
        watch(state.stage);
        const last = armLog.samples[armLog.samples.length - 1];
        if (!last || last.stage !== state.stage || last.review !== state.review || last.done !== state.done || last.total !== state.total) {
          armLog.samples.push({ t: Math.round(performance.now() - armedAt), ...state });
          if (armLog.samples.length > 800) armLog.samples.splice(1, 1);
        }
        if (state.stage === "replaying" && replayingSince === null) replayingSince = performance.now();
        const settledMs = replayingSince === null ? 0 : Math.round(performance.now() - replayingSince);
        const editReadsDisabled = armLog.target[armLog.target.length - 1]?.disabled === true;
        if (armLog.acted === null && state.stage === "replaying" && (arg.action.kind === "cancel" || editReadsDisabled || settledMs >= arg.settleMs)) armLog.acted = { t: Math.round(performance.now() - armedAt), stage: state.stage, settledMs, ...press() };
      };
      host.__probeArmObserver = new MutationObserver(tick);
      host.__probeArmObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
      host.__probeArmTimer = window.setInterval(tick, 40);
      tick();
      return true;
    },
    false,
    { action, settleMs: EDIT_SETTLE_MS, wgpu: renderer === "wgpu", stages: BAND_WORDS.map(([stage, pattern]) => [stage, pattern.source] as [string, string]), reviews: REVIEW_WORDS.map(([review, pattern]) => [review, pattern.source] as [string, string]) },
  );

/** 📼️ The armed page's log: every sampled band state and the press it made (null before any). */
const replayArmLog = () => evalSafe(() => (window as unknown as { __probeArm?: ArmLog }).__probeArm ?? null, null as ArmLog | null);

/** 🧯️ Stops the arm's observer and answers its final log. */
export const disarmReplay = async () => {
  const final = await replayArmLog();
  await evalSafe(() => {
    const host = window as unknown as { __probeArmObserver?: MutationObserver; __probeArmTimer?: number };
    host.__probeArmObserver?.disconnect();
    if (host.__probeArmTimer !== undefined) clearInterval(host.__probeArmTimer);
  }, undefined);
  return final;
};
//#endregion 🔖️ReplayArm
//#endregion 🔖️Band
