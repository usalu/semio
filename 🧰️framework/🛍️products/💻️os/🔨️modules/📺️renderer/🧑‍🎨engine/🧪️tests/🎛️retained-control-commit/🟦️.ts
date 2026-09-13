/**
 * 🎛️ The TypeScript twin of `🖱️ui/🧪️tests/🎛️retained-control-commit/🦀️.rs`. Both read the SAME
 * neutral fixture (`🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json`): the Rust law drives the live
 * wgpu `EventRouter` — real hit-testing, real capture, real focus — and asserts the `UiCommand::App`
 * it produced; this one re-derives every expected dispatch a SECOND, independent time from React's
 * own two payload rules, so a divergence between the two targets shows up as a fixture failure
 * rather than as a silent renderer difference.
 *
 * The rules re-derived here are React's, at their source:
 *  - `🛠️ShellHelpers/🟦️.tsx`'s `uiInputField` — a scalar payload is named by its TRIGGER (`delta`
 *    for `Trigger::Delta`, `value` for every other).
 *  - `🛠️ShellHelpers/🟦️.tsx`'s `uiIntentPayload` — the named payload is MERGED OVER the node's
 *    authored args (`{...args, ...named}`), never substituted for them.
 *  - `📃️UiDocumentStore/🟦️.tsx`'s `emitIntent` — a node with no binding for the trigger dispatches
 *    nothing at all.
 *
 * The defect both sides pin: the wgpu retained router named committing an edited value through
 * `on_change` as "a documented gap for a later milestone", so typing into a generation's Form field
 * moved a caret and reached no guest (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const uiRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui");

type Binding = { readonly trigger: string; readonly action: string; readonly args?: Record<string, unknown> };
type FixtureNode = {
  readonly kind: string;
  readonly id: string;
  readonly inputKind?: string;
  readonly value?: string | number;
  readonly commit?: string | null;
  readonly on?: boolean;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly uniform?: boolean;
  readonly t?: number;
  readonly orbId?: string;
  readonly classifierKind?: string;
};
type Gesture = { readonly kind: string; readonly at: readonly [number, number]; readonly to?: readonly [number, number]; readonly text?: string };
type FixtureCase = {
  readonly name: string;
  readonly node: FixtureNode;
  readonly binding: Binding | null;
  readonly deltaBinding?: Binding | null;
  readonly bounds: readonly [number, number, number, number];
  readonly gesture: Gesture;
  readonly expected: { readonly action: string; readonly args: Record<string, unknown> } | null;
};

const law = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🎛️retained-control-commit/🔣️.json"), "utf8")) as { readonly cases: readonly FixtureCase[] };

/** 🏷️ `uiInputField`: a scalar payload is named by its trigger, never by the control that fired it. */
const inputField = (trigger: string): string => (trigger === "delta" ? "delta" : "value");

/** 🎬️ `uiIntentPayload`: the named scalar merged OVER the node's authored args. `undefined` when the node declares no binding for that trigger (`emitIntent`). */
const dispatch = (binding: Binding | null | undefined, value: unknown): { action: string; args: Record<string, unknown> } | undefined => {
  if (!binding) return undefined;
  return { action: binding.action, args: { ...(binding.args ?? {}), [inputField(binding.trigger)]: value } };
};

/** ✍️ The buffer an editable control holds after the gesture's text is typed at the caret, which focus seeds at the END of the declarative value. */
const typedBuffer = (node: FixtureNode, gesture: Gesture): string => `${node.value ?? ""}${gesture.text ?? ""}`;

/** ⏎️ React's `InputView`: `commit === "blur"` commits through `Trigger::Commit` (Enter / blur), everything else through `Trigger::Change` (every keystroke). */
const inputCommitsOnBlur = (node: FixtureNode): boolean => node.kind === "input" && node.commit === "blur";

/** 🎚️ The slider value a press at `x` reports: the full-width track, snapped onto `step`, clamped into min..=max. */
const sliderValueAt = (bounds: readonly [number, number, number, number], x: number, min: number, max: number, step: number): number => {
  const span = max - min;
  if (!(span > 0)) return min;
  const ratio = bounds[2] > 0 ? Math.min(1, Math.max(0, (x - bounds[0]) / bounds[2])) : 0;
  const raw = min + ratio * span;
  const snapped = step > 0 ? min + Math.round((raw - min) / step) * step : raw;
  return Math.min(max, Math.max(min, snapped));
};

/** 💍️ The ring `t` a press reports: the turn fraction of the angle from the centre of its own bounds, normalised into [0, 1). */
const ringTAt = (bounds: readonly [number, number, number, number], x: number, y: number): number => {
  const turns = Math.atan2(y - (bounds[1] + bounds[3] * 0.5), x - (bounds[0] + bounds[2] * 0.5)) / (Math.PI * 2);
  return turns - Math.floor(turns);
};

/** ➖️➕️ Which of a stepper's three equal segments a point lands in: -1 decrement, 1 increment, 0 the value readout. */
const stepperSegmentSign = (bounds: readonly [number, number, number, number], x: number): number => {
  const segment = bounds[2] / 3;
  if (x < bounds[0] + segment) return -1;
  if (x >= bounds[0] + segment * 2) return 1;
  return 0;
};

const expectedDispatch = (testCase: FixtureCase): { action: string; args: Record<string, unknown> } | undefined => {
  const { node, bounds, gesture } = testCase;
  // 🎚️ A drag ends where `to` says; every other gesture acts at `at`.
  const point = gesture.to ?? gesture.at;
  const x = bounds[0] + bounds[2] * point[0];
  const y = bounds[1] + bounds[3] * point[1];
  switch (node.kind) {
    case "input": {
      const buffer = typedBuffer(node, gesture);
      const value = node.inputKind === "number" ? Number(buffer) : buffer;
      if (inputCommitsOnBlur(node)) return gesture.kind === "type" ? undefined : dispatch(testCase.binding, value);
      return dispatch(testCase.binding, value);
    }
    case "iconSelect":
      return dispatch(testCase.binding, typedBuffer(node, gesture));
    case "toggle":
      return dispatch(testCase.binding, !node.on);
    case "slider":
      return dispatch(testCase.binding, sliderValueAt(bounds, x, node.min ?? 0, node.max ?? 0, node.step ?? 0));
    case "ring":
      return dispatch(testCase.binding, ringTAt(bounds, x, y));
    case "numberStepper": {
      const sign = stepperSegmentSign(bounds, x);
      if (sign === 0) return undefined;
      const step = node.step ?? 0;
      return dispatch(testCase.deltaBinding, sign * step) ?? dispatch(testCase.binding, (node.value as number) + sign * step);
    }
    default:
      throw new Error(`fixture node kind ${node.kind}`);
  }
};

describe("retained control commit", () => {
  it("covers every value-carrying component kind", () => {
    const kinds = new Set(law.cases.map((testCase) => testCase.node.kind));
    expect([...kinds].sort()).toEqual(["iconSelect", "input", "numberStepper", "ring", "slider", "toggle"]);
  });

  for (const testCase of law.cases) {
    it(`${testCase.name} dispatches what React's own payload rules dispatch`, () => {
      const derived = expectedDispatch(testCase);
      if (testCase.expected === null) {
        expect(derived).toBeUndefined();
        return;
      }
      expect(derived).toBeDefined();
      expect(derived!.action).toBe(testCase.expected.action);
      // 🔢️ Numbers compare by value, not by float identity — the fixture carries the decimal the
      // guest receives, and both targets round the same gesture to it.
      expect(Object.keys(derived!.args).sort()).toEqual(Object.keys(testCase.expected.args).sort());
      for (const [key, value] of Object.entries(testCase.expected.args)) {
        if (typeof value === "number") expect(derived!.args[key] as number).toBeCloseTo(value, 9);
        else expect(derived!.args[key]).toEqual(value);
      }
    });
  }
});
