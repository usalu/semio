import { describe } from "vitest";
import { registerReplaysIdentityRemainsCloseTests } from "./🔶️replays-identity-remains-close/🟦️.tsx";
import { registerCursorizesLiveControlled20Tests } from "./🔸️cursorizes-live-controlled-20/🟦️.tsx";
import { registerEnqueuesCoalescesPinsUnpinsTests } from "./🟠️enqueues-coalesces-pins-unpins/🟦️.tsx";
import { registerResolvesExactCollidingSampleTests } from "./🟢️resolves-exact-colliding-sample/🟦️.tsx";

describe("owned Diagram force", () => {
  registerReplaysIdentityRemainsCloseTests();
  registerCursorizesLiveControlled20Tests();
  registerEnqueuesCoalescesPinsUnpinsTests();
  registerResolvesExactCollidingSampleTests();
});
