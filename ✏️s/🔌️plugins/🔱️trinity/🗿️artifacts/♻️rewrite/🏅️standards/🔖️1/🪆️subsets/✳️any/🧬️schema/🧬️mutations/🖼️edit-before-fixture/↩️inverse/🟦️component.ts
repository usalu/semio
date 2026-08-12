/** ↩️ rewrite edit-before-fixture/↩️inverse — mirror of the BASE-lookup old-body inverse builder. */
import type { EditBeforeFixture } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: EditBeforeFixture, baseBeforeFixtureJson: string): EditBeforeFixture[] {
  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
