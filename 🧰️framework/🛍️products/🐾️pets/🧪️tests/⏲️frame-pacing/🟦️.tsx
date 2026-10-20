/** ⏲️ How the pet layer keeps time: animation-frame timestamps become whole ticks (sixty-four a second, at most eight
 * a frame), a calmer stage skips frames, a stage at rest asks for no frame at all and waits on a single timer, a hidden
 * document runs nothing and its absence is never caught up. A hand-driven clock comes in through the seams; the
 * browser's own frames and timers are stood in for by the fake clock of `@sinonjs/fake-timers` (vitest's).
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/⏲️pacing/🟦️.ts
 * @see https://gafferongames.com/post/fix_your_timestep/
 */
import { afterEach, describe, vi } from "vitest";
import { registerCountsSixtyFourTicksTests } from "./🔶️counts-sixty-four-ticks/🟦️.tsx";
import { registerCancelsFrameTimerWhileTests } from "./🔸️cancels-frame-timer-while/🟦️.tsx";

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("⏲️ frame pacing", () => {
  registerCountsSixtyFourTicksTests();
  registerCancelsFrameTimerWhileTests();
});
