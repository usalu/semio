/** 🔄️ Deferred reprojection of the document: steps, loads and remote changes. */
import { COPY } from "../🌍️copy/🟦️.ts";
import { currentLocale, renderer } from "../🧠️state/🟦️.ts";
import { presentKey, textOfKey } from "../🧱️chrome/🟦️.ts";
import { mirrorFind, mirror } from "../🧊️wgpu-mirror/🟦️.ts";
import { evalSafe, sleep } from "../📄️page/🟦️.ts";
import { verdict, note } from "../🧾️report/🟦️.ts";

//#region 🔖️Band
//#region 🔖️Reprojection

type ReprojectionKind = "step" | "load" | "remote";

type ReprojectionRead = { title: string; status: string; kind: ReprojectionKind | null; done: number | null; total: number | null; paused: boolean; cancel: boolean; rerun: boolean };

/** 🎴️ Which history change a reprojection status line names, with its progress (`HistoryPatch.reprojection {kind, done,
 * total, paused}`, rendered by the runtime's `time_travel_reprojection_section` in the current locale). */
const reprojectionKindOf = (status: string): { kind: ReprojectionKind | null; done: number | null; total: number | null; paused: boolean } => {
  const copy = COPY[currentLocale];
  for (const [kind, pattern] of [["step", copy.stepProgress], ["load", copy.loadProgress], ["remote", copy.remoteProgress]] as const) {
    const match = pattern.exec(status);
    if (match) return { kind, done: Number(match[1]), total: Number(match[2]), paused: false };
  }
  return { kind: status.includes(copy.remotePaused) ? "remote" : null, done: null, total: null, paused: status.includes(copy.remotePaused) };
};

/** 📡️ The history body's `framework.history.reprojection` section (design §16.6, gap N17, §20.8): what replays — this
 * replica's own history step, a whole-document load or a remote change — its progress in words, paused, and its Cancel
 * replay / Replay again controls; null while nothing replays (or the History panel is closed). */
export const reprojection = async (): Promise<ReprojectionRead | null> => {
  const section = await presentKey("framework.history.reprojection");
  if (section === null) return null;
  const status = (await textOfKey("framework.history.reprojection.status")).replace(/\s+/g, " ").trim();
  const sectionText = renderer === "wgpu" ? (mirrorFind(await mirror(), "framework.history.reprojection")?.label ?? "") : await textOfKey("framework.history.reprojection");
  const title = (sectionText.split("\n").map((line) => line.trim()).find(Boolean) ?? "").slice(0, 80);
  const row = async (authored: string) => (await presentKey(authored)) !== null || (await presentKey(`${authored}.row`)) !== null;
  return { title, status, ...reprojectionKindOf(status), cancel: await row("framework.history.reprojection.cancelReplay"), rerun: await row("framework.history.reprojection.rerun") };
};

type ReprojectionSample = { t: number; text: string; title: string; shell: string; kind: string | null; phase: string | null; notice: string | null; role: string | null; live: string | null; control: string | null; session: boolean };

type ReprojectionPress = { t: number; via: string; disabled: boolean | null; notices: string[] };

type ReprojectionArmLog = { samples: ReprojectionSample[]; undo: ReprojectionPress | null; cancel: ReprojectionPress | null; ids: string[] | null };

type ReprojectionArmAction = "observe" | "cancel" | "undo-then-cancel";

/** 🕸️ Arms the page to react to a replaying history change the frame it is announced — a person hearing "History step: Replaying
 * history…" or "Document load: Loading document…" — and samples, with its in-page time, both of its surfaces until
 * {@link disarmReprojection}: the shell's polite status outside the History panel (React `[data-semio-history-reprojection]`
 * with its `-phase`, `data-notice-code`, `role=status` announcement and `-control`; wgpu the mirror node
 * `shell.history.reprojection`) and the History body's `framework.history.reprojection` section, plus whether a session band
 * is open. `undo-then-cancel` first presses the body's Undo (`framework.history.undo.run`, a history step that must be refused
 * while one replays or a document loads), then — once the refusal's notice showed or 1.5 s passed with the change still up —
 * Cancel replay (the shell's `cancelReplay` control, else the body's); `cancel` presses Cancel replay at once; `observe` only
 * samples. */
export const armReprojection = (action: ReprojectionArmAction) =>
  evalSafe(
    (arg) => {
      const host = window as unknown as { __probeReprojection?: ReprojectionArmLog; __probeReprojectionObserver?: MutationObserver; __probeNotices?: { code: string }[] };
      host.__probeReprojectionObserver?.disconnect();
      const armedAt = performance.now();
      const armLog: ReprojectionArmLog = { samples: [], undo: null, cancel: null, ids: null };
      host.__probeReprojection = armLog;
      const find = (authored: string) => document.querySelector<HTMLElement>(arg.wgpu ? `#semio-wgpu-accessibility [data-node-key="${authored}"], #semio-wgpu-accessibility [data-node-key$="/${authored}"]` : `[id="${authored}"], [id$="/${authored}"]`);
      const textOf = (el: HTMLElement | null) => (el === null ? "" : arg.wgpu ? `${el.getAttribute("aria-label") ?? ""}` : el.innerText).replace(/\s+/g, " ").trim();
      const titleOf = (el: HTMLElement | null) => (el === null ? "" : arg.wgpu ? (el.getAttribute("aria-label") ?? "") : (el.innerText.split("\n").map((line) => line.trim()).find(Boolean) ?? "")).slice(0, 80);
      const notices = () => (host.__probeNotices ?? []).map((row) => row.code);
      const click = (el: HTMLElement | null, via: string): ReprojectionPress => {
        const button = el === null ? null : arg.wgpu || el.matches("button") ? el : (el.querySelector<HTMLElement>("button") ?? el);
        const disabled = button === null ? null : (button as HTMLButtonElement).disabled === true || button.getAttribute("aria-disabled") === "true";
        button?.click();
        return { t: Math.round(performance.now() - armedAt), via: button === null ? "absent" : via, disabled, notices: notices() };
      };
      const shell = () => {
        if (arg.wgpu) {
          const node = document.querySelector<HTMLElement>('#semio-wgpu-accessibility [data-node-key="shell.history.reprojection"]');
          return { shell: (node?.getAttribute("aria-label") ?? "").trim(), kind: null, phase: null, notice: null, role: node?.getAttribute("role") ?? null, live: node?.getAttribute("aria-live") ?? null, control: null };
        }
        const el = document.querySelector<HTMLElement>("[data-semio-history-reprojection]");
        const announcement = el?.querySelector<HTMLElement>("[data-semio-history-reprojection-announcement]") ?? null;
        return { shell: (announcement?.textContent ?? "").replace(/\s+/g, " ").trim(), kind: el?.getAttribute("data-semio-history-reprojection") ?? null, phase: el?.getAttribute("data-semio-history-reprojection-phase") ?? null, notice: el?.getAttribute("data-notice-code") ?? null, role: announcement?.getAttribute("role") ?? null, live: announcement?.getAttribute("aria-live") ?? null, control: el?.querySelector("[data-semio-history-reprojection-control]")?.getAttribute("data-semio-history-reprojection-control") ?? null };
      };
      const tick = () => {
        const status = textOf(find("framework.history.reprojection.status"));
        const sample = { text: status, title: status === "" ? "" : titleOf(find("framework.history.reprojection")), ...shell(), session: document.querySelector(arg.wgpu ? '#semio-wgpu-accessibility [data-node-key="shell.time-travel.status"]' : "[data-semio-time-travel]") !== null };
        const last = armLog.samples[armLog.samples.length - 1];
        if (!last || last.text !== sample.text || last.shell !== sample.shell || last.phase !== sample.phase || last.control !== sample.control || last.session !== sample.session) {
          armLog.samples.push({ t: Math.round(performance.now() - armedAt), ...sample });
          if (armLog.samples.length > 600) armLog.samples.splice(1, 1);
        }
        const live = sample.text !== "" || sample.shell !== "";
        if (live && armLog.ids === null) {
          const section = find("framework.history.reprojection");
          const scope = arg.wgpu ? Array.from(document.querySelectorAll<HTMLElement>('#semio-wgpu-accessibility [data-node-key*="history.reprojection"]')).map((node) => `${node.dataset.nodeKey ?? ""}[${node.getAttribute("role") ?? ""}]`) : Array.from((section?.closest('[data-slot="tree-section-wrapper"]') ?? section)?.querySelectorAll<HTMLElement>("[id]") ?? []).filter((node) => /framework\.history/.test(node.id)).map((node) => `${node.id.replace(/^.*framework\.history\./, "")}[${node.getAttribute("data-slot") ?? node.getAttribute("role") ?? ""}]`);
          armLog.ids = scope.slice(0, 16);
        }
        if (!live || arg.action === "observe") return;
        const cancel = () => (arg.wgpu ? null : document.querySelector<HTMLElement>('[data-semio-history-reprojection-control="cancelReplay"]')) ?? find("framework.history.reprojection.cancelReplay") ?? find("framework.history.reprojection.cancelReplay.row");
        if (arg.action === "cancel" && armLog.cancel === null) {
          armLog.cancel = click(cancel(), "cancelReplay");
          return;
        }
        if (armLog.undo === null) {
          armLog.undo = click(find("framework.history.undo.run") ?? find("framework.history.undo"), "framework.history.undo");
          setTimeout(tick, 1600);
          return;
        }
        if (armLog.cancel === null && (notices().some((code) => code === "history.replaying" || code === "document.loading") || performance.now() - armedAt - armLog.undo.t >= 1500)) armLog.cancel = click(cancel(), "cancelReplay");
      };
      host.__probeReprojectionObserver = new MutationObserver(tick);
      host.__probeReprojectionObserver.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
      tick();
      return true;
    },
    false,
    { action, wgpu: renderer === "wgpu" },
  );

/** 🛑️ Stops the reprojection arm and answers its log: every status it sampled and the presses it made. */
export const disarmReprojection = async () => {
  const final = await evalSafe(() => (window as unknown as { __probeReprojection?: ReprojectionArmLog }).__probeReprojection ?? null, null as ReprojectionArmLog | null);
  await evalSafe(() => (window as unknown as { __probeReprojectionObserver?: MutationObserver }).__probeReprojectionObserver?.disconnect(), undefined);
  return final;
};

/** 📉️ An armed reprojection log summarised: the statuses it saw on either surface (de-duplicated), their kinds and largest
 * progress total, when the change first showed and when it went away, the shell samples, and the presses. */
export const reprojectionSummary = (armLog: ReprojectionArmLog | null) => {
  const samples = armLog?.samples ?? [];
  const shown = samples.filter((sample) => sample.text !== "" || sample.shell !== "");
  const lines = shown.flatMap((sample) => [sample.text, sample.shell]).filter(Boolean);
  const kinds = [...new Set([...lines.map((line) => reprojectionKindOf(line).kind), ...shown.map((sample) => sample.kind)].filter((kind): kind is ReprojectionKind => kind === "step" || kind === "load" || kind === "remote"))];
  const totals = lines.map((line) => reprojectionKindOf(line).total ?? 0);
  const firstAt = shown[0]?.t ?? null;
  const goneAt = firstAt === null ? null : (samples.find((sample) => sample.t > firstAt && sample.text === "" && sample.shell === "")?.t ?? null);
  return { seen: shown.length > 0, kinds, total: totals.length ? Math.max(...totals) : null, titles: [...new Set(shown.map((sample) => sample.title).filter(Boolean))].slice(0, 4), texts: [...new Set(lines)].slice(0, 6), shell: shown.filter((sample) => sample.shell !== "").map(({ t, shell, kind, phase, notice, role, live, control, session }) => ({ t, shell, kind, phase, notice, role, live, control, session })).slice(0, 12), firstAt, goneAt, undo: armLog?.undo ?? null, cancel: armLog?.cancel ?? null, ids: armLog?.ids ?? null };
};

/** 📢️ The shell-status verdicts of one observed history change (audit W1E-3, contract of S4-UI): it is announced outside the
 * History panel — React `[data-semio-history-reprojection=<kind>]` with its `-phase`, a `role=status` `aria-live=polite`
 * announcement reading the kernel copy "<title>: <text>"; wgpu the polite (or progressbar) mirror node
 * `shell.history.reprojection` named by the kernel line —, its Cancel replay / Replay again controls show only while no
 * session is open, and a refused change names its reason in words (its code only as `data-notice-code`). `suffix` tells two
 * observations of one step apart; nothing is judged when the shell never showed the change. */
export const reprojectionShellVerdicts = (summary: ReturnType<typeof reprojectionSummary>, suffix = "") => {
  const copy = COPY[currentLocale];
  const titles: Record<ReprojectionKind, string> = { step: copy.stepTitle, load: copy.loadTitle, remote: copy.remoteTitle };
  const shell = summary.shell;
  if (!shell.length) {
    const bodyShownMs = summary.firstAt === null ? 0 : summary.goneAt === null ? Number.POSITIVE_INFINITY : summary.goneAt - summary.firstAt;
    if (bodyShownMs >= 1000) verdict(`reprojection-status-is-announced-outside-the-history-panel${suffix}`, false, { bodyShownMs: Number.isFinite(bodyShownMs) ? bodyShownMs : "until the observation ended", titles: summary.titles, texts: summary.texts, shell: [], reading: renderer === "wgpu" ? "the History body showed the change for longer than a second while the mirror never projected `shell.history.reprojection`" : "the History body showed the change for longer than a second while no shell status `[data-semio-history-reprojection]` was ever rendered — a person with the History panel closed is told nothing (W1E-3)" });
    else note(`reprojection-shell-status-not-seen${suffix}`, { summary, reading: "the change came and went without a shell frame showing it (progress refreshes ≥ 100 ms)" });
    return;
  }
  const announced = (sample: (typeof shell)[number]) => {
    const kind = (sample.kind as ReprojectionKind | null) ?? reprojectionKindOf(sample.shell).kind;
    const line = reprojectionKindOf(sample.shell);
    const words = line.kind !== null || line.paused || /:\s*\S/.test(sample.shell);
    if (renderer === "wgpu") return words && (sample.live === "polite" || sample.role === "progressbar");
    return kind !== null && sample.shell.startsWith(`${titles[kind]}: `) && words && sample.role === "status" && sample.live === "polite" && ["progress", "paused", "refused"].includes(sample.phase ?? "");
  };
  verdict(`reprojection-status-is-announced-outside-the-history-panel${suffix}`, shell.every(announced), { shell, reading: renderer === "wgpu" ? "W1E-3 on wgpu: the mirror node `shell.history.reprojection` is a polite status (a progressbar while it replays) named by the kernel's status line" : "W1E-3: `[data-semio-history-reprojection=<kind>]` + `-phase=progress|paused|refused`, a `role=status` `aria-live=polite` announcement reading the kernel copy \"<title>: <text>\" (`historyReprojectionStatus`)" });
  if (renderer === "wgpu") note(`reprojection-controls-live-in-the-history-body${suffix}`, { reading: "the wgpu shell status dispatches nothing; Cancel replay / Replay again are the History body's `framework.history.reprojection` rows" });
  else verdict(`reprojection-controls-show-only-while-no-session-is-open${suffix}`, shell.every((sample) => (sample.session ? sample.control === null : sample.phase === "refused" || sample.control !== null)), { shell, reading: "`historyReprojectionControlV1`: Cancel replay (Replay again while a remote change is paused) only while no history-edit session is open — the session's band owns both while it is" });
  const refused = shell.filter((sample) => sample.phase === "refused");
  if (refused.length) verdict(`a-refused-history-change-names-its-reason-in-words${suffix}`, refused.every((sample) => sample.notice !== null && !sample.shell.includes(sample.notice)), { refused, reading: "a refused adoption reads `<kind>.refused` with the notice's words; its code is only `data-notice-code`" });
};

/** ⌛️ Waits until neither the shell status nor the history body shows a replaying change for `quietMs` in a row (the step
 * adopted, the load finished or a cancel dropped it), at most `timeoutMs`. */
export const reprojectionSettled = async (timeoutMs = 120000, quietMs = 1500) => {
  const start = Date.now();
  let quietSince: number | null = null;
  const shellLive = () => (renderer === "wgpu" ? mirror().then((nodes) => nodes.some((node) => node.key === "shell.history.reprojection")) : evalSafe(() => document.querySelector("[data-semio-history-reprojection]") !== null, false));
  while (Date.now() - start < timeoutMs) {
    const live = (await reprojection()) !== null || (await shellLive());
    quietSince = !live ? (quietSince ?? Date.now()) : null;
    if (quietSince !== null && Date.now() - quietSince >= quietMs) return { ok: true, waitedMs: Date.now() - start };
    await sleep(250);
  }
  return { ok: false, waitedMs: Date.now() - start };
};
//#endregion 🔖️Reprojection
//#endregion 🔖️Band
