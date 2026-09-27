import { expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import { chromium } from "playwright";
import fixture from "../../../../../../../🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json";
import schema from "../../../../../../../🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧬️schema/🎞️animation/🔣️.json";
it("browser animation ownership survives discarded candidates and clears on accepted static output", async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    const observed = await page.evaluate(({ activity, primitives }) => {
      const target = document.createElement("div");
      document.body.append(target);
      const install = (active: boolean) => {
        for (const animation of target.getAnimations()) animation.cancel();
        if (active) target.animate([{ opacity: 0 }, { opacity: 1 }], { duration: 1600, iterations: Infinity });
      };
      const history = activity.map(step => {
        if (step.kind === "present") install(step.active === true);
        return target.getAnimations().some(animation => animation.playState === "running");
      });
      const packets = primitives.map(row => {
        for (const animation of target.getAnimations()) animation.cancel();
        for (const kind of [...row.draw, ...row.overlay]) {
          if ([6, 7, 9].includes(kind)) target.animate([{ opacity: 0 }, { opacity: 1 }], { duration: kind === 7 ? 3200 : 1600, iterations: Infinity });
        }
        return target.getAnimations().some(animation => animation.playState === "running");
      });
      return { history, packets };
    }, fixture);
    expect(observed.history).toEqual(fixture.activity.map(step => step.nextMs !== null));
    expect(observed.packets).toEqual(fixture.primitives.map(row => row.active));
    console.log("[DEBUG] browser animation ownership validates accepted and discarded frame sequence");
  } finally {
    await browser.close();
  }
});

it("validates the shared finite GPU animation clock contract", () => {
  const validate = new Ajv2020().compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

it("matches browser animation phase after long uptime and across the common wrap", async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    const observed = await page.evaluate(({ shaderPeriodsUs, samples }) => {
      const target = document.createElement("div");
      document.body.append(target);
      return shaderPeriodsUs.flatMap(periodUs => {
        const animation = target.animate([{ opacity: 0 }, { opacity: 1 }], { duration: periodUs / 1000, iterations: Infinity });
        animation.pause();
        const rows = samples.map(row => {
          animation.currentTime = (row.nowUs ?? 0) / 1000;
          return { id: row.id, periodUs, phaseUs: row.phaseUs, progress: animation.effect!.getComputedTiming().progress };
        });
        animation.cancel();
        return rows;
      });
    }, fixture);
    for (const row of observed) expect(row.progress, `${row.id}: ${row.periodUs}`).toBeCloseTo((row.phaseUs % row.periodUs) / row.periodUs, 6);
    console.log(`[DEBUG] browser animation clock validated ${observed.length} phases`);
  } finally {
    await browser.close();
  }
});
