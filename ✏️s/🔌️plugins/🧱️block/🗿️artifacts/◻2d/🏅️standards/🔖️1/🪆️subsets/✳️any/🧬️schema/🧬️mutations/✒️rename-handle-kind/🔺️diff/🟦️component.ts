/** 🔺️ block2d rename-handle-kind/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { RenameHandleKind } from "../🦠️mutation/🟦️component";
import type { Block2dDiff } from "../../../🔺️diff/🟦️component";
import type { Block2dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: RenameHandleKind, base: Block2dSnapshot): Block2dDiff;
