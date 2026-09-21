// @vitest-environment jsdom
/**
 * 🖱️ Browser proof that every admitted DOM wheel owns one Scroll dispatch. The shared schema and
 * fixture are also consumed by the native host test; this side uses jsdom's real EventTarget and
 * WheelEvent rather than mirroring a renderer accumulator.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");
const schemaPath = resolve(suiteRoot, "../../🧬️schema/🖱️wheel-application-point/🔣️.json");

type Modifiers = { readonly shift: boolean; readonly ctrl: boolean; readonly meta: boolean; readonly alt: boolean };
type ScrollRow = { readonly kind: "scroll"; readonly x: number; readonly y: number; readonly deltaX: number; readonly deltaY: number; readonly modifiers: Modifiers };
type PointerMoveRow = { readonly kind: "pointer-move"; readonly x: number; readonly y: number };
type InputRow = ScrollRow | PointerMoveRow;
type Application = { readonly x: number; readonly y: number; readonly deltaX: number; readonly delta: number; readonly modifiers: Modifiers };
type Case = {
  readonly name: string;
  readonly events: readonly InputRow[];
  readonly applications: readonly Application[];
  readonly supersededApplication?: Application;
};
type Fixture = {
  readonly rule: { readonly id: string; readonly statement: string };
  readonly cases: readonly Case[];
};

const fixtureValue: unknown = JSON.parse(readFileSync(fixturePath, "utf8"));
const fixture = fixtureValue as Fixture;
const schema = JSON.parse(readFileSync(schemaPath, "utf8")) as object;

/** 🖱️ Mounts a browser event receiver and records the owned value copied from every WheelEvent. */
const receive = (events: readonly InputRow[]): Application[] => {
  const receiver = document.createElement("canvas");
  const applications: Application[] = [];
  receiver.addEventListener("wheel", (event) => {
    applications.push({
      x: event.clientX,
      y: event.clientY,
      deltaX: event.deltaX,
      delta: event.deltaY,
      modifiers: { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey },
    });
  });
  document.body.append(receiver);
  for (const row of events) {
    if (row.kind === "pointer-move") {
      receiver.dispatchEvent(new MouseEvent("mousemove", { bubbles: true, clientX: row.x, clientY: row.y }));
      continue;
    }
    receiver.dispatchEvent(
      new WheelEvent("wheel", {
        bubbles: true,
        cancelable: true,
        clientX: row.x,
        clientY: row.y,
        deltaX: row.deltaX,
        deltaY: row.deltaY,
        shiftKey: row.modifiers.shift,
        ctrlKey: row.modifiers.ctrl,
        metaKey: row.modifiers.meta,
        altKey: row.modifiers.alt,
      }),
    );
  }
  receiver.remove();
  return applications;
};

describe("🖱️ owned Scroll dispatch", () => {
  it("validates the shared fixture against its strict schema", () => {
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixtureValue), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.rule.id).toBe("ownedScrollDispatch");
    expect(fixture.rule.statement).toContain("exactly one ordered owned dispatch");
  });

  for (const testCase of fixture.cases) {
    it(`copies every browser wheel sample into one owned dispatch — ${testCase.name}`, () => {
      const applications = receive(testCase.events);
      const scrollCount = testCase.events.filter((row) => row.kind === "scroll").length;
      expect(applications).toEqual(testCase.applications);
      expect(applications).toHaveLength(scrollCount);
      expect(new Set(applications).size).toBe(applications.length);

      if (testCase.supersededApplication) {
        const scrolls = testCase.events.filter((row): row is ScrollRow => row.kind === "scroll");
        const finalEvent = testCase.events.at(-1);
        const finalScroll = scrolls.at(-1);
        expect(finalEvent).toBeDefined();
        expect(finalScroll).toBeDefined();
        const superseded = {
          x: finalEvent!.x,
          y: finalEvent!.y,
          deltaX: scrolls.reduce((sum, row) => sum + row.deltaX, 0),
          delta: scrolls.reduce((sum, row) => sum + row.deltaY, 0),
          modifiers: finalScroll!.modifiers,
        };
        expect(superseded).toEqual(testCase.supersededApplication);
        expect(applications).not.toEqual([superseded]);
      }
    });
  }
});
