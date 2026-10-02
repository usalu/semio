/** ↩️ rewriting disconnect-working-edges/↩️inverse — mirror of the one-row exact undo. */
import type { DisconnectWorkingEdges } from "../🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(_payload: DisconnectWorkingEdges, baseBeforeFixtureJson: string): EditBeforeFixture[] {
  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
