import type { WasmBuildProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { buildRepositoryWasmWebV1 } from "../🔣️policy/🟦️.ts";

/** 🔌️ Implements the framework WASM build port with the repository compiler store, dependency custody and pinned toolchain. */
export const provider: WasmBuildProviderV1 = { buildWeb: buildRepositoryWasmWebV1 };
