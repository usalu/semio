/** 🟡️ The slices across frames tests of the 📡️ surface survey suite: a survey is a job that runs in slices of a time budget and adds up to the very survey of one go. */
import { expect, it } from "vitest";
import { surveyJob, survey } from "@semio-tech/pets-react";
import { build, card, optionsOf, place, scenes, viewport, type Box } from "../🟣️harness/🟦️.tsx";

/** 🕰️ A clock that moves by `step` milliseconds every time somebody asks it. */
function ticking(step: number): () => number {
  let time = 0;
  return () => (time += step);
}

/** 🔷 Registers the slices across frames tests of the 📡️ surface survey suite. */
export function registerSlicesAcrossFramesTests(): void {
  for (const scene of scenes) {
    it(`adds the slices up to the survey of one go: ${scene.name}`, () => {
      viewport(scene.viewport.width, scene.viewport.height);
      build(scene.nodes, document.body, new Map());
      const frame = scene.frame === undefined ? undefined : place(document.createElement("div"), scene.frame);
      if (frame !== undefined) document.body.append(frame);
      const options = optionsOf(scene, frame);
      const whole = survey(document, options);
      const job = surveyJob(document, options, ticking(1));
      let runs = 0;
      let sliced = job.run(0);
      for (runs = 1; sliced === null && runs < 10_000; runs++) sliced = job.run(0);
      expect(sliced).toEqual(whole);
    });
  }
  it("reads one element per slice when the budget is spent, and leaves the page untouched between slices", () => {
    viewport(1000, 700);
    let reads = 0;
    const boxes: Box[] = Array.from({ length: 12 }, (_, index) => ({ left: 10 + index * 80, top: 100 + (index % 3) * 150, width: 70, height: 40 }));
    for (const box of boxes) {
      const element = card(box);
      const read = element.getBoundingClientRect;
      element.getBoundingClientRect = () => {
        reads += 1;
        return read.call(element);
      };
    }
    const before = document.body.outerHTML;
    const job = surveyJob(document, {}, ticking(1));
    const perSlice: number[] = [];
    let seen = null;
    while (seen === null && perSlice.length < 1000) {
      const start = reads;
      seen = job.run(0);
      perSlice.push(reads - start);
    }
    expect(seen).toEqual(survey(document));
    expect(perSlice.length).toBeGreaterThanOrEqual(boxes.length);
    expect(Math.max(...perSlice)).toBeLessThanOrEqual(1);
    expect(document.body.outerHTML).toBe(before);
  });
  it("finishes in one slice when the budget is not spent, and keeps answering the same survey once done", () => {
    viewport(1000, 700);
    card({ left: 100, top: 300, width: 200, height: 100 });
    const job = surveyJob(document, {}, () => 0);
    const done = job.run(1000);
    expect(done).toEqual(survey(document));
    expect(job.run(1000)).toBe(done);
  });
}
