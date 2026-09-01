/** 🔺️ rewrite edit-before-fixture/🔺️diff — mirror of the single-field diff builder. */
import type { EditBeforeFixture } from "../🟦️component.ts";

export function diff(payload: EditBeforeFixture): { beforeFixtureJson: string } {
  return { beforeFixtureJson: payload.newBeforeFixtureJson };
}
