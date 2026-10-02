/** ↩️ rewriting connect-working-ports/↩️inverse — mirror of the one-row exact undo. */
import type { ConnectWorkingPorts } from "../🟦️.ts";
import type { EditBeforeFixture } from "../../🖼️edit-before-fixture/🟦️.ts";

export function inverse(_payload: ConnectWorkingPorts, baseBeforeFixtureJson: string): EditBeforeFixture[] {
  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];
}
