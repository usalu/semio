/** 🪟️ How the screens show the others (design §17): card activity, cursors and focus frames on their anchors, name labels and colours, the learner card, the quiz cards and the leaderboard, inside the pages behind the cards too; the activity vectors are shared JSON.
 *
 * @see ../🧰️presence-client-kit/🟦️.tsx — the sockets and fixtures
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */
import { describe } from "vitest";
import { registerPlacesObservedCardCursorsTests } from "./🔶️places-observed-card-cursors/🟦️.tsx";
import { registerGathersOthersWatchedRoomTests } from "./🔸️gathers-others-watched-room/🟦️.tsx";

describe("👁️ what the screens show", () => {
  registerPlacesObservedCardCursorsTests();
  registerGathersOthersWatchedRoomTests();
});
