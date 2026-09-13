/** 🧩️ Semantic parity probe owner. */

import { ParityDump, ParityNode, ParityRect, ParityRenderer, StructuralResult, compareParityStructural, dumpReactStructure, dumpWgpuStructure, parityNormalizeText } from "../🏗️structure/🟦️.ts";



//#endregion 🔖️Triage

//#region 🔖️ProbeCatalog
/** 🎬️Behavioral probe system — drives semantically-identical interactions on the react and wgpu
 * pages in lockstep (same click/type/key/drag/wheel sequence on both, each side resolving its OWN
 * click/drag/wheel coordinates from its OWN structural dump so the sequence stays semantically
 * identical even when pixel layout differs slightly) and diffs a fresh `compareParityStructural`
 * after every step. Complements `StructuralCompare`/`PixelCompare` (static end-state) and `Triage`
 * (boot) — this is the only sub-region that actually DRIVES interaction, closing the gap this
 * ticket's `verifyParityVariant` had: it previously only ever checked static boot state. */

type ProbeKeyCombo = string;

// 🎹️ Playwright key-combo syntax, e.g. `"Control+p"`, `"Escape"`.

/** 🔎️`exists`/`absent`/`focus`/`text` match a node whose `path` equals OR case-insensitively
 * *contains* the given string (also checked against `kind`) — a probe author usually only knows the
 * semantic identifier ("search"), not the full generated structural path, and loose matching keeps
 * the DSL usable without every catalog entry hardcoding brittle exact paths. */
type ProbeExpectPredicate =
  | { readonly kind: "exists"; readonly path: string }
  | { readonly kind: "absent"; readonly path: string }
  | { readonly kind: "focus"; readonly path: string }
  | { readonly kind: "text"; readonly path: string; readonly equals: string }
  | { readonly kind: "custom"; readonly name: string; readonly check: (dump: ParityDump) => boolean };

type ProbeStep =
  | { readonly kind: "click"; readonly path: string }
  | { readonly kind: "type"; readonly text: string }
  | { readonly kind: "key"; readonly combo: ProbeKeyCombo }
  | { readonly kind: "dragTo"; readonly fromPath: string; readonly toPath: string }
  | { readonly kind: "wheel"; readonly path: string; readonly deltaY: number }
  | { readonly kind: "settle"; readonly ms: number }
  | { readonly kind: "stateTransition" }
  | { readonly kind: "expect"; readonly predicate: ProbeExpectPredicate };

type ProbeStateSnapshot = {
  readonly digest: string;
  readonly nodeCount: number;
};

type ProbeStateEvidence = {
  readonly actionPath: string;
  readonly actionKind: string;
  readonly react: { readonly before: ProbeStateSnapshot; readonly after: ProbeStateSnapshot; readonly changedPaths: readonly string[] };
  readonly wgpu: { readonly before: ProbeStateSnapshot; readonly after: ProbeStateSnapshot; readonly changedPaths: readonly string[] };
};

type ProbeStepStatus = "PASS" | "FAIL" | "SKIP";

type ProbeStepResult = {
  readonly index: number;
  readonly step: ProbeStep;
  readonly status: ProbeStepStatus;
  readonly structural?: StructuralResult;
  readonly state?: ProbeStateEvidence;
  readonly detail?: string;
};

type ProbeRunResult = { readonly status: ProbeStepStatus; readonly steps: readonly ProbeStepResult[] };

type ParityProbeSuite = { readonly name: string; readonly steps: readonly ProbeStep[] };

function parityRectCenter(rect: ParityRect): readonly [number, number] {
  const [x, y, w, h] = rect;
  return [x + w / 2, y + h / 2];
}

async function parityDumpFor(page: import("playwright").Page, renderer: ParityRenderer): Promise<ParityDump> {
  return renderer === "react" ? dumpReactStructure(page) : dumpWgpuStructure(page);
}

function parityFindNodeExact(dump: ParityDump, path: string): ParityNode | null {
  return dump.nodes.find((n) => n.path === path) ?? null;
}

function parityNodeMatches(dump: ParityDump, needle: string): readonly ParityNode[] {
  const lower = needle.toLowerCase();
  return dump.nodes.filter((n) => n.path === needle || n.path.toLowerCase().includes(lower) || n.kind.toLowerCase().includes(lower));
}

//#region 🔖️StateTransitionProbe
const STATE_PROBE_KIND_PRIORITY = ["toggle", "select", "slider", "button", "stack"] as const;

const STATE_PROBE_MAX_CANDIDATES = 12;

type StateProbeCandidate = { readonly path: string; readonly kind: string };

function stateProbeCandidates(reactDump: ParityDump, wgpuDump: ParityDump): StateProbeCandidate[] {
  const wgpuByPath = new Map(wgpuDump.nodes.map((node) => [node.path, node]));
  const priority = new Map<string, number>(STATE_PROBE_KIND_PRIORITY.map((kind, index) => [kind, index]));
  return reactDump.nodes
    .filter((node) => {
      const peer = wgpuByPath.get(node.path);
      return Boolean(
        peer &&
        node.path.includes("#") &&
        priority.has(node.kind) &&
        peer.kind === node.kind &&
        node.visible &&
        peer.visible &&
        !node.state.disabled &&
        !peer.state.disabled &&
        node.rect[2] > 0 &&
        node.rect[3] > 0 &&
        peer.rect[2] > 0 &&
        peer.rect[3] > 0,
      );
    })
    .map((node) => ({ path: node.path, kind: node.kind }))
    .sort((a, b) => (priority.get(a.kind) ?? 99) - (priority.get(b.kind) ?? 99) || a.path.localeCompare(b.path))
    .slice(0, STATE_PROBE_MAX_CANDIDATES);
}

function stateProbeNodeValue(node: ParityNode): string {
  return JSON.stringify({ path: node.path, kind: node.kind, text: parityNormalizeText(node.text), visible: node.visible, disabled: node.state.disabled, selected: node.state.selected });
}

function stateProbeSnapshot(dump: ParityDump): ProbeStateSnapshot {
  const serialized = dump.nodes.map(stateProbeNodeValue).sort().join("\n");
  return { digest: Bun.hash(serialized).toString(16), nodeCount: dump.nodes.length };
}

function stateProbeChangedPaths(before: ParityDump, after: ParityDump): string[] {
  const beforeByPath = new Map(before.nodes.map((node) => [node.path, stateProbeNodeValue(node)]));
  const afterByPath = new Map(after.nodes.map((node) => [node.path, stateProbeNodeValue(node)]));
  const paths = new Set([...beforeByPath.keys(), ...afterByPath.keys()]);
  return [...paths].filter((path) => beforeByPath.get(path) !== afterByPath.get(path)).sort();
}

async function executeStateProbeCandidate(page: import("playwright").Page, renderer: ParityRenderer, candidate: StateProbeCandidate): Promise<{ readonly ok: boolean; readonly detail?: string }> {
  const click = await executeParityStep(page, renderer, { kind: "click", path: candidate.path });
  if (!click.ok) return click;
  if (candidate.kind === "select") {
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
  } else if (candidate.kind === "slider") {
    await page.keyboard.press("ArrowRight");
  }
  return { ok: true };
}

/** 🧭️ Drives an app-declared interactive `UiNode` shared by both renderers and proves that each
 * renderer observes a semantic state change (topology, text, visibility, disabled, or selected).
 * Framework chrome is excluded because only interpreter-owned `data-ui-path`/wgpu paths participate;
 * `#id` further requires an explicit app declaration rather than an anonymous layout node. */
async function runStateTransitionProbe(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page): Promise<{ readonly status: ProbeStepStatus; readonly state?: ProbeStateEvidence; readonly detail?: string }> {
  let reactBefore = await dumpReactStructure(reactPage);
  let wgpuBefore = await dumpWgpuStructure(wgpuPage);
  const candidates = stateProbeCandidates(reactBefore, wgpuBefore);
  if (candidates.length === 0) return { status: "SKIP", detail: "no common enabled app-declared toggle/select/slider/button/activatable-stack node" };

  const attempted: string[] = [];
  for (const candidate of candidates) {
    const [reactAction, wgpuAction] = await Promise.all([executeStateProbeCandidate(reactPage, "react", candidate), executeStateProbeCandidate(wgpuPage, "wgpu", candidate)]);
    if (!reactAction.ok || !wgpuAction.ok) {
      attempted.push(`${candidate.path}: ${reactAction.detail ?? "react ok"}; ${wgpuAction.detail ?? "wgpu ok"}`);
      continue;
    }
    await Promise.all([reactPage.waitForTimeout(350), wgpuPage.waitForTimeout(350)]);
    const [reactAfter, wgpuAfter] = await Promise.all([dumpReactStructure(reactPage), dumpWgpuStructure(wgpuPage)]);
    const reactChangedPaths = stateProbeChangedPaths(reactBefore, reactAfter);
    const wgpuChangedPaths = stateProbeChangedPaths(wgpuBefore, wgpuAfter);
    const evidence: ProbeStateEvidence = {
      actionPath: candidate.path,
      actionKind: candidate.kind,
      react: { before: stateProbeSnapshot(reactBefore), after: stateProbeSnapshot(reactAfter), changedPaths: reactChangedPaths },
      wgpu: { before: stateProbeSnapshot(wgpuBefore), after: stateProbeSnapshot(wgpuAfter), changedPaths: wgpuChangedPaths },
    };
    if (reactChangedPaths.length > 0 && wgpuChangedPaths.length > 0) return { status: "PASS", state: evidence };
    attempted.push(`${candidate.path}: react changed=${reactChangedPaths.length}, wgpu changed=${wgpuChangedPaths.length}`);
    reactBefore = reactAfter;
    wgpuBefore = wgpuAfter;
  }
  return { status: "FAIL", detail: `no candidate produced observable state on both renderers (${attempted.join(" | ")})` };
}

//#endregion 🔖️StateTransitionProbe

/** 🕹️Executes one non-`expect` step against a single page, resolving click/drag/wheel targets from
 * a dump pulled from THAT SAME page immediately beforehand — never the other renderer's dump, and
 * never a stale one — so react/wgpu layout drift never desyncs which element gets hit. */
async function executeParityStep(page: import("playwright").Page, renderer: ParityRenderer, step: Exclude<ProbeStep, { readonly kind: "expect" } | { readonly kind: "stateTransition" }>): Promise<{ readonly ok: boolean; readonly detail?: string }> {
  switch (step.kind) {
    case "click": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `click target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.click(cx, cy);
      return { ok: true };
    }
    case "type":
      await page.keyboard.type(step.text);
      return { ok: true };
    case "key":
      await page.keyboard.press(step.combo);
      return { ok: true };
    case "dragTo": {
      const dump = await parityDumpFor(page, renderer);
      const from = parityFindNodeExact(dump, step.fromPath);
      const to = parityFindNodeExact(dump, step.toPath);
      if (!from || !to) return { ok: false, detail: `dragTo target not found: ${!from ? step.fromPath : step.toPath}` };
      const [fx, fy] = parityRectCenter(from.rect);
      const [tx, ty] = parityRectCenter(to.rect);
      await page.mouse.move(fx, fy);
      await page.mouse.down();
      await page.mouse.move(tx, ty, { steps: 8 });
      await page.mouse.up();
      return { ok: true };
    }
    case "wheel": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `wheel target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.move(cx, cy);
      await page.mouse.wheel(0, step.deltaY);
      return { ok: true };
    }
    case "settle":
      await page.waitForTimeout(step.ms);
      return { ok: true };
  }
}

/** ✅️Evaluates one `expect` predicate against BOTH sides' freshly-pulled dumps — a predicate only
 * passes the step when it holds on react AND wgpu, since the point is cross-renderer parity, not
 * either renderer in isolation. */
function evaluateParityExpect(predicate: ProbeExpectPredicate, reactDump: ParityDump, wgpuDump: ParityDump): { readonly ok: boolean; readonly detail?: string } {
  const checkOne = (dump: ParityDump): { readonly ok: boolean; readonly detail?: string } => {
    switch (predicate.kind) {
      case "exists": {
        const ok = parityNodeMatches(dump, predicate.path).length > 0;
        return { ok, detail: ok ? undefined : `no node matching "${predicate.path}"` };
      }
      case "absent": {
        const ok = parityNodeMatches(dump, predicate.path).length === 0;
        return { ok, detail: ok ? undefined : `node still present matching "${predicate.path}"` };
      }
      case "focus": {
        const focus = dump.focusPath;
        const ok = focus !== null && (focus === predicate.path || focus.toLowerCase().includes(predicate.path.toLowerCase()));
        return { ok, detail: ok ? undefined : `focusPath "${focus ?? "null"}" does not match "${predicate.path}"` };
      }
      case "text": {
        const node = parityNodeMatches(dump, predicate.path)[0];
        const ok = node !== undefined && parityNormalizeText(node.text) === parityNormalizeText(predicate.equals);
        return { ok, detail: ok ? undefined : `text at "${predicate.path}" is "${node?.text ?? "<missing>"}", expected "${predicate.equals}"` };
      }
      case "custom": {
        const ok = predicate.check(dump);
        return { ok, detail: ok ? undefined : `custom predicate "${predicate.name}" failed` };
      }
    }
  };
  const react = checkOne(reactDump);
  const wgpu = checkOne(wgpuDump);
  const ok = react.ok && wgpu.ok;
  return { ok, detail: ok ? undefined : `react: ${react.detail ?? "ok"} | wgpu: ${wgpu.detail ?? "ok"}` };
}

/** 🏃️Runs `steps` on `reactPage`/`wgpuPage` in lockstep — never advances to the next step on either
 * page until the current one finished on both. Non-`expect` steps execute identically on both pages
 * then get a fresh `compareParityStructural` diff; `expect` steps take no page action and just
 * evaluate their predicate against fresh dumps from both. The first `FAIL` halts the run (remaining
 * steps marked `SKIP`) — steps are an ORDERED scenario, not a bag of independent assertions, so a
 * downstream step referencing state a failed step never reached would only add noise. Returns the
 * FULL step trail (not just a final boolean) so a failure is diagnosable by (which step, which axis)
 * — see `ParityMismatchAxis` for the axis vocabulary reused from `StructuralCompare`. */
async function runParityProbe(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, steps: readonly ProbeStep[]): Promise<ProbeRunResult> {
  const results: ProbeStepResult[] = [];
  let halted = false;
  for (let index = 0; index < steps.length; index++) {
    const step = steps[index];
    if (halted) {
      results.push({ index, step, status: "SKIP" });
      continue;
    }
    if (step.kind === "expect") {
      const reactDump = await dumpReactStructure(reactPage);
      const wgpuDump = await dumpWgpuStructure(wgpuPage);
      const outcome = evaluateParityExpect(step.predicate, reactDump, wgpuDump);
      results.push({ index, step, status: outcome.ok ? "PASS" : "FAIL", detail: outcome.detail });
      if (!outcome.ok) halted = true;
      continue;
    }
    if (step.kind === "stateTransition") {
      const outcome = await runStateTransitionProbe(reactPage, wgpuPage);
      results.push({ index, step, status: outcome.status, state: outcome.state, detail: outcome.detail });
      if (outcome.status === "FAIL") halted = true;
      continue;
    }
    const [reactOutcome, wgpuOutcome] = await Promise.all([executeParityStep(reactPage, "react", step), executeParityStep(wgpuPage, "wgpu", step)]);
    if (!reactOutcome.ok || !wgpuOutcome.ok) {
      results.push({ index, step, status: "FAIL", detail: [reactOutcome.detail, wgpuOutcome.detail].filter(Boolean).join(" | ") });
      halted = true;
      continue;
    }
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    results.push({ index, step, status: structural.status, structural });
    if (structural.status === "FAIL") halted = true;
  }
  const status = results.some((r) => r.status === "FAIL") ? "FAIL" : results.some((r) => r.status === "PASS") ? "PASS" : "SKIP";
  return { status, steps: results };
}

async function runParityProbeSuite(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, suite: ParityProbeSuite): Promise<{ readonly name: string } & ProbeRunResult> {
  const result = await runParityProbe(reactPage, wgpuPage, suite.steps);
  return { name: suite.name, ...result };
}

/** 🐚️Minimal cross-playground smoke suite — command palette open/close is the one interaction every
 * catalog playground exposes IDENTICALLY, via `useActionHotkey("mod+p", ...)` in
 * `framework/os/renderer/js/react/index.tsx` (`mod` accepts `ctrlKey || metaKey`, so `"Control+p"` works
 * regardless of host OS — no need to special-case macOS `"Meta+p"`).
 *
 * KNOWN LIMITATION (confirmed by reading `openStudioE2eCommandPalette` in `🔖️StudioE2eVerify` above,
 * and `UISearch` in `framework/os/renderer/js/react/index.tsx`): the palette is FRAMEWORK CHROME, not
 * `UiNode`-declared app content — React renders it through the owned Command facade (`[role='dialog'] [data-slot=
 * 'command-input']`), which never carries `data-ui-path`, so `REACT_DOM_DUMP_SCRIPT` (see
 * `🔖️StructuralDump`) cannot see it at all. The `exists`/`absent` checks below are therefore
 * expected to be unreliable (likely FAIL on the react side) until the structural dump is extended to
 * also tag framework-chrome overlays — a real, scoped follow-up (would also need mirroring into
 * `framework/os/renderer/wgpu/rs/lib.rs`'s `🔬️Introspection` walk, which is a different file, out of
 * reach from this one). Flagging rather than silently "fixing" by touching either renderer's core
 * dump mechanism unverified, per this pass's own constraint of no live browser run to confirm
 * against. */
const PARITY_SHELL_PROBE_SUITE: ParityProbeSuite = {
  name: "shell",
  steps: [
    { kind: "key", combo: "Control+p" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "exists", path: "search" } },
    { kind: "key", combo: "Escape" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "absent", path: "search" } },
  ],
};

/** 🧭️Default catalog-wide state-management probe. Unlike `shell`, this drives an explicitly-id'd
 * app surface node and records renderer-specific before/after digests plus every changed path. */
const PARITY_STATE_PROBE_SUITE: ParityProbeSuite = {
  name: "state",
  steps: [{ kind: "stateTransition" }],
};

/** 🗂️Starter catalog — keyed by suite name so `ParityProbeScript`/`verifyParityVariant` can look one
 * up by string. A per-playground text/dnd/scene suite (dragging dock panels, typing into a text
 * editor host, orbiting a 3d scene) is a natural follow-up once `shell` is confirmed working
 * end-to-end against a real live boot — out of scope for this pass per the ticket's own brief. */
const PARITY_PROBE_CATALOG: Readonly<Record<string, ParityProbeSuite>> = {
  state: PARITY_STATE_PROBE_SUITE,
  shell: PARITY_SHELL_PROBE_SUITE,
};

export { PARITY_PROBE_CATALOG, PARITY_SHELL_PROBE_SUITE, PARITY_STATE_PROBE_SUITE, ParityProbeSuite, ProbeExpectPredicate, ProbeKeyCombo, ProbeRunResult, ProbeStateEvidence, ProbeStateSnapshot, ProbeStep, ProbeStepResult, ProbeStepStatus, STATE_PROBE_KIND_PRIORITY, STATE_PROBE_MAX_CANDIDATES, StateProbeCandidate, evaluateParityExpect, executeParityStep, executeStateProbeCandidate, parityDumpFor, parityFindNodeExact, parityNodeMatches, parityRectCenter, runParityProbe, runParityProbeSuite, runStateTransitionProbe, stateProbeCandidates, stateProbeChangedPaths, stateProbeNodeValue, stateProbeSnapshot };
