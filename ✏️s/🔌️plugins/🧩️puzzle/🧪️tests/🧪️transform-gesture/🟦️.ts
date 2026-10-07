/** 🧩️ Independent xstate oracle for the phase fixture consumed by both native puzzle transform charts. */
import { expect, test } from "vitest";
import { initialTransition, setup, transition } from "xstate";
import fixture from "../../🧫️fixtures/🛠️transform-gesture/🔣️.json";

const chart = setup({
  types: { events: {} as { type: string; offset: number[] } },
  guards: { moves: ({ event }) => event.offset.some(value => value !== 0) },
}).createMachine({
  id: "transform",
  initial: "idle",
  states: { idle: { on: { once: { target: "committed", guard: "moves" } } }, committed: { type: "final" } },
});
for (const row of fixture.cases) test(`transform phase ${row.phase}, offset ${row.offset}`, () => {
  const [initial] = initialTransition(chart);
  const [next] = transition(chart, initial, { type: row.phase, offset: row.offset });
  expect(next.status === "done" ? 1 : 0).toBe(row.committed);
});
