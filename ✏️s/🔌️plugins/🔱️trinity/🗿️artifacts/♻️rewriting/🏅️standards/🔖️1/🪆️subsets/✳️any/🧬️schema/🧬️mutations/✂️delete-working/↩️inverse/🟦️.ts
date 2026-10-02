/** ↩️ rewriting delete-working-nodes/↩️inverse — mirror of the one-row exact undo. */
import type { DeleteWorkingNodes } from "../🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(_payload: DeleteWorkingNodes, baseBeforeFixtureJson: string): EditBeforeFixture[] {
  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
