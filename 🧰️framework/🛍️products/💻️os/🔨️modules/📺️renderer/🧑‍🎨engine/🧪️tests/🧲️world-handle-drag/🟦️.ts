/** 🧲️ The handle drag parameter, pick and event sequences reproduce the language-neutral vectors, which an independent numpy solve also derives. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🧲️handle-drag.json";
import { world3dHandleDragParameterV1, world3dHandleDragStepV1, world3dHandlePickV1, type WorldHandleDragEvent, type WorldHandleDragState, type WorldVec3 } from "../../🧱️elements/🌐️World3dHost/🧲️handle-drag/🟦️.ts";

describe("world handle drag", () => {
  for (const row of fixture.parameter) {
    it(`parameter: ${row.name}`, () => {
      const s = world3dHandleDragParameterV1(row.rayOrigin as WorldVec3, row.rayDirection as WorldVec3, row.axisOrigin as WorldVec3, row.axis as WorldVec3);
      if (row.s === null) expect(s).toBeNull();
      else expect(s).toBeCloseTo(row.s, 9);
    });
  }
  for (const row of fixture.pick) {
    it(`pick: ${row.name}`, () => expect(world3dHandlePickV1(row.handles, row.x, row.y, row.radiusPx)).toBe(row.expected));
  }
  for (const row of fixture.sequences) {
    it(`sequence: ${row.name}`, () => {
      let state: WorldHandleDragState = null;
      const sent = row.events.map((event) => {
        const step = world3dHandleDragStepV1(state, event as WorldHandleDragEvent);
        state = step.state;
        return step.dispatch;
      });
      expect(sent).toEqual(row.dispatches);
      expect(state).toBeNull();
    });
  }
});
