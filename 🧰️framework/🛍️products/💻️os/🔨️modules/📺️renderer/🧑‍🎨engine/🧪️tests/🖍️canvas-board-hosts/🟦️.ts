/** 🖍 Canvas 2d host, puzzle 2d board host, board event coalescing, gumball and peer presence. */
import { describe } from "vitest";
import { registerRendersCanvas2dHostTests } from "./🔶️renders-canvas-2d-host/🟦️.ts";
import { registerCoalescesPuzzle2dBoardTests } from "./🔸️coalesces-puzzle-2d-board/🟦️.ts";
import { registerCollectsLiveMirrorMutationsTests } from "./🟠️collects-live-mirror-mutations/🟦️.ts";

describe("framework renderer hosts", () => {
  registerRendersCanvas2dHostTests();
  registerCoalescesPuzzle2dBoardTests();
  registerCollectsLiveMirrorMutationsTests();
});
