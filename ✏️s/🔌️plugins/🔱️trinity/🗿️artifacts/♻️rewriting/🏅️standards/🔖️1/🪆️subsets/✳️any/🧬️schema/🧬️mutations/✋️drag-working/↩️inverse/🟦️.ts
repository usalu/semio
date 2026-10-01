/** ↩️ rewriting drag-working-nodes/↩️inverse — mirror of the one-row exact undo. */
import type { DragWorkingNodes } from "../🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(_payload: DragWorkingNodes, baseBeforeFixtureJson: string): EditBeforeFixture[] {
  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
