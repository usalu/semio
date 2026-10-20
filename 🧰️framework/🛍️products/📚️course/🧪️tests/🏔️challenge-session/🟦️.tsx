/** ⛰️ The challenges through the session: the sheet of each, switching voids the open run, the clock of an expert run, the hints of an easy run, points by challenge and stored runs restored only in the shape of the challenges.
 *
 * @see ../🧰️learner-journey-kit/🟦️.tsx — the shared quiz, proctor double and builders
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the client under test
 */
import { describe } from "vitest";
import { registerStartsRunChallengeSheetTests } from "./🔶️starts-run-challenge-sheet/🟦️.tsx";
import { registerNeverLetsDeputyOpenTests } from "./🔸️never-lets-deputy-open/🟦️.tsx";

describe("⛰️ the challenges through the session", () => {
  registerStartsRunChallengeSheetTests();
  registerNeverLetsDeputyOpenTests();
});
