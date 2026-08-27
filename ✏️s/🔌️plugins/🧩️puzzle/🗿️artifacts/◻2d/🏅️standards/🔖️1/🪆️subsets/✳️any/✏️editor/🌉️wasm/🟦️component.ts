//#region 🌉️PuzzleBoardSession
import { createWasmModuleLoader, type AppSurfaceSessionFactory, type Board2dWasmSession } from "@semio-tech/framework-renderer-react";

type PuzzleSessionModule = typeof import("../../../../../../../../📦️packages/🦀️rust/pkg/semio_puzzle.js");
const loadModule = createWasmModuleLoader<PuzzleSessionModule>(async () => {
  const module = await import("../../../../../../../../📦️packages/🦀️rust/pkg/semio_puzzle.js");
  await module.default();
  return module;
});

/** 🧩️ Constructs a fresh puzzle-owned Board session from its actual wasm-bindgen package. */
export async function createPuzzleBoardSession(): Promise<Board2dWasmSession> {
  const module = await loadModule();
  return new module.BoardSession();
}

export const PUZZLE_BOARD_SESSION_FACTORIES: readonly AppSurfaceSessionFactory[] = [
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle2d@1/*#editor", create: createPuzzleBoardSession },
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle2d@1/*#viewer", create: createPuzzleBoardSession },
];
//#endregion 🌉️PuzzleBoardSession
