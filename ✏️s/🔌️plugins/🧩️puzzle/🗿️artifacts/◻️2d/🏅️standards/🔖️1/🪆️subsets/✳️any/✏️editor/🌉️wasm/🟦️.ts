//#region 🌉️PuzzleBoardSession
import { createWasmModuleLoader, type AppSurfaceSessionFactory, type Board2dWasmSession } from "@semio-tech/framework-renderer-react";
import { installPageRuntimeAssets } from "../../../../../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🟦️.ts";

type PuzzleSessionModule = typeof import("@semio-tech/puzzle-wasm");
const loadModule = createWasmModuleLoader<PuzzleSessionModule>(async () => {
  const module = await import("@semio-tech/puzzle-wasm");
  await module.default();
  await installPageRuntimeAssets(module);
  return module;
});

/** 🧩️ Constructs a fresh puzzle-owned Board session from its actual wasm-bindgen package. */
export async function createPuzzleBoardSession(): Promise<Board2dWasmSession> {
  const module = await loadModule();
  return new module.BoardSession();
}

export const PUZZLE_BOARD_SESSION_FACTORIES: readonly AppSurfaceSessionFactory[] = [
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#editor", create: createPuzzleBoardSession },
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#viewer", create: createPuzzleBoardSession },
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle5d@1/*#editor", create: createPuzzleBoardSession },
  { kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle5d@1/*#viewer", create: createPuzzleBoardSession },
];
//#endregion 🌉️PuzzleBoardSession
