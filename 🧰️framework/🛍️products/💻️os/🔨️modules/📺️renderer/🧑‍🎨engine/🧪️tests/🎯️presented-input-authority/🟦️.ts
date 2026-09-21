// @vitest-environment jsdom
/**
 * 🎯️ Neutral oracle for pointer authority following accepted pixels.
 *
 * The model separates a completed chrome candidate from the presented registry. A DOM successor
 * cannot receive pointer input before its commit mounts it, and an abandoned render leaves the
 * mounted predecessor interactive.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🎯️presented-input-authority/🔣️.json");

type Control = { readonly id: string; readonly rect: readonly [number, number, number, number]; readonly binding?: string };
type Expectation = { readonly routed: readonly string[]; readonly refused: readonly string[]; readonly presentedGeneration: number };
type Row = { readonly id: string; readonly operations: readonly string[]; readonly expect: Expectation };
type RetainedControl = Control & { readonly key: string; readonly value?: string | number; readonly step?: number; readonly items?: readonly string[]; readonly role?: string; readonly label?: string };
type Fixture = {
  readonly controls: { readonly presented: Control; readonly candidate: Control };
  readonly replacement: { readonly presented: Control; readonly moved: Control; readonly overlap: Control };
  readonly retainedControls: {
    readonly select: { readonly presented: RetainedControl; readonly candidate: RetainedControl };
    readonly numberStepper: { readonly presented: RetainedControl; readonly candidate: RetainedControl };
    readonly keyboard: { readonly presented: RetainedControl; readonly candidate: RetainedControl };
    readonly accessibility: { readonly presented: RetainedControl; readonly candidate: RetainedControl };
  };
  readonly rows: readonly Row[];
  readonly laws: readonly string[];
};

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

class PresentedInputAuthority {
  private presented: readonly Control[] = [];
  private candidate: readonly Control[] = [];
  private generation = 0;

  presentInitial(...controls: readonly Control[]): void {
    this.presented = controls;
    this.generation = 1;
  }

  completeCandidate(...controls: readonly Control[]): void {
    this.candidate = controls;
  }

  acceptCandidate(): void {
    if (this.candidate.length === 0) throw new Error("candidate input registry missing");
    this.presented = this.candidate;
    this.candidate = [];
    this.generation += 1;
  }

  abortCandidate(): void {
    this.candidate = [];
  }

  hitAt(x: number, y: number): Control | undefined {
    return [...this.presented].reverse().find(({ rect }) => x >= rect[0] && y >= rect[1] && x <= rect[0] + rect[2] && y <= rect[1] + rect[3]);
  }

  presentedGeneration(): number {
    return this.generation;
  }
}


class PresentedRevision<T> {
  private value: T;
  private candidate: { readonly value: T; readonly baseMutationEpoch: number } | undefined;
  private mutationEpoch = 0;

  constructor(initial: T) {
    this.value = initial;
  }

  prepare(value: T): void {
    this.candidate = { value, baseMutationEpoch: this.mutationEpoch };
  }

  interact(mutator?: (value: T) => T): T {
    if (mutator) this.value = mutator(this.value);
    this.mutationEpoch += 1;
    return this.value;
  }

  accept(): boolean {
    if (!this.candidate || this.candidate.baseMutationEpoch !== this.mutationEpoch) return false;
    this.value = this.candidate.value;
    this.candidate = undefined;
    return true;
  }

  current(): T {
    return this.value;
  }
}

function replay(row: Row): { routed: string[]; refused: string[]; presentedGeneration: number } {
  const authority = new PresentedInputAuthority();
  const routed: string[] = [];
  const refused: string[] = [];
  for (const operation of row.operations) {
    switch (operation) {
      case "presentInitial":
        authority.presentInitial(fixture.controls.presented);
        break;
      case "completeCandidateChrome":
        authority.completeCandidate(fixture.controls.candidate);
        break;
      case "acceptCandidate":
        authority.acceptCandidate();
        break;
      case "abortCandidate":
        authority.abortCandidate();
        break;
      case "pressPresented":
      case "pressCandidate": {
        const control = operation === "pressPresented" ? fixture.controls.presented : fixture.controls.candidate;
        (authority.hitAt(control.rect[0] + control.rect[2] / 2, control.rect[1] + control.rect[3] / 2)?.id === control.id ? routed : refused).push(control.id);
        break;
      }
      default:
        throw new Error(`${row.id}: unknown operation ${operation}`);
    }
  }
  return { routed, refused, presentedGeneration: authority.presentedGeneration() };
}

describe("presented input authority", () => {
  afterEach(() => cleanup());

  it("declares the closed presentation laws", () => {
    expect(fixture.laws).toEqual([
      "a-completed-chrome-registry-is-not-pointer-authority-before-its-pixels-are-presented",
      "presentation-acceptance-atomically-replaces-hits-and-their-owner-map",
      "presentation-abort-never-clears-or-replaces-the-last-presented-input-authority",
      "a-moved-same-key-control-keeps-its-presented-position-and-binding-until-acceptance",
      "successive-presentations-at-the-same-input-generation-receive-distinct-candidate-witnesses",
      "a-pending-select-successor-cannot-open-or-dispatch-from-candidate-semantics",
      "a-pending-number-stepper-successor-cannot-use-candidate-value-step-or-binding",
      "keyboard-focus-and-tab-stay-on-the-presented-retained-revision",
      "accessibility-projection-and-actions-stay-on-the-presented-retained-revision",
      "presented-input-mutation-invalidates-a-candidate-built-from-an-older-interaction-epoch",
    ]);
  });

  it("keeps the old position and binding until the moved same-key successor is accepted", () => {
    const authority = new PresentedInputAuthority();
    const { presented, moved, overlap } = fixture.replacement;
    authority.presentInitial(presented);
    authority.completeCandidate(moved, overlap);
    expect(authority.hitAt(45, 5)).toEqual(presented);
    expect(authority.hitAt(45, 5)?.binding).toBe("presented-binding");
    expect(authority.hitAt(65, 5)).toBeUndefined();
    authority.acceptCandidate();
    expect(authority.hitAt(45, 5)).toEqual(overlap);
    expect(authority.hitAt(45, 5)?.binding).toBe("overlap-binding");
    expect(authority.hitAt(65, 5)).toEqual(moved);
    expect(authority.hitAt(65, 5)?.binding).toBe("moved-binding");
  });

  it("keeps Select and NumberStepper semantics on the accepted revision", () => {
    const select = new PresentedRevision(fixture.retainedControls.select.presented);
    select.prepare(fixture.retainedControls.select.candidate);
    expect(select.current()).toMatchObject({ value: "system", binding: "setAppearance.presented", items: ["system", "dark"] });
    expect(select.accept()).toBe(true);
    expect(select.current()).toMatchObject({ value: "dark", binding: "setAppearance.candidate", items: ["dark", "light"] });

    const stepper = new PresentedRevision(fixture.retainedControls.numberStepper.presented);
    stepper.prepare(fixture.retainedControls.numberStepper.candidate);
    const displayed = stepper.current();
    expect({ next: Number(displayed.value) + Number(displayed.step), binding: displayed.binding }).toEqual({ next: 3, binding: "setScale.presented" });
    expect(stepper.accept()).toBe(true);
    const accepted = stepper.current();
    expect({ next: Number(accepted.value) + Number(accepted.step), binding: accepted.binding }).toEqual({ next: 15, binding: "setScale.candidate" });
  });

  it("keeps keyboard and accessibility authority on the accepted revision and rejects a stale candidate", () => {
    const keyboard = new PresentedRevision(fixture.retainedControls.keyboard.presented);
    keyboard.prepare(fixture.retainedControls.keyboard.candidate);
    expect(keyboard.interact()).toMatchObject({ key: "settings.presented.focus", binding: "commit.presented" });
    expect(keyboard.accept()).toBe(false);
    expect(keyboard.current()).toMatchObject({ key: "settings.presented.focus", binding: "commit.presented" });

    const accessibility = new PresentedRevision(fixture.retainedControls.accessibility.presented);
    accessibility.prepare(fixture.retainedControls.accessibility.candidate);
    expect(accessibility.current()).toMatchObject({ role: "combobox", label: "Theme", value: "system", binding: "setAppearance.presented" });
  });

  it("matches the mounted React DOM revision until the successor commit", () => {
    const actions: string[] = [];
    const mounted = (control: RetainedControl) => createElement("button", { "data-testid": control.id, onClick: () => actions.push(control.binding ?? "") }, `${control.label ?? control.key}:${control.value ?? ""}`);
    const presented = fixture.retainedControls.accessibility.presented;
    const candidate = fixture.retainedControls.accessibility.candidate;
    const view = render(mounted(presented));
    const byId = (id: string) => view.container.querySelector(`[data-testid="${id}"]`);
    expect(byId(presented.id)?.textContent).toBe("Theme:system");
    fireEvent.click(byId(presented.id) ?? document.body);
    expect(actions).toEqual(["setAppearance.presented"]);
    expect(byId(candidate.id)).toBeNull();
    view.rerender(mounted(candidate));
    fireEvent.click(byId(candidate.id) ?? document.body);
    expect(actions).toEqual(["setAppearance.presented", "setAppearance.candidate"]);
  });

  for (const row of fixture.rows) {
    it(row.id, () => {
      expect(replay(row)).toEqual(row.expect);
    });
  }
});
