import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Ring } from "../../🧱️elements/⭕️Ring/🟦️.tsx";

const ui = resolve(import.meta.dir, "../..");
type Law = { radius: number; knobs: { id: string; t: number; offset: [number, number] }[]; drags: { id: string; offset: [number, number]; t: number }[] };
const law = JSON.parse(readFileSync(join(ui, "🧪️tests/🧫️fixtures/⭕️ring-angle/🔣️.json"), "utf8")) as Law;

for (const knob of law.knobs) test("the actual React Ring draws t=" + knob.t + " at " + knob.id, () => {
  const html = renderToStaticMarkup(React.createElement(Ring, { id: "ring", orbs: [{ id: "sun", t: knob.t, selected: true }] }));
  const orb = /data-slot="orb"[^>]*?cx="([^"]+)"[^>]*?cy="([^"]+)"/u.exec(html);
  expect(orb, html).not.toBeNull();
  expect(Number(orb![1])).toBeCloseTo(knob.offset[0], 4);
  expect(Number(orb![2])).toBeCloseTo(knob.offset[1], 4);
});

for (const drag of law.drags) test("React's drag formula reads " + drag.id + " as t=" + drag.t, () => {
  let angle = Math.atan2(drag.offset[1], drag.offset[0]) + Math.PI / 2;
  if (angle < 0) angle += 2 * Math.PI;
  expect((angle / (2 * Math.PI)) % 1).toBeCloseTo(drag.t, 6);
});
