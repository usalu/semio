/** 🔺️ block2d move-handle/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { MoveHandle } from "../🦠️mutation/🟦️component";
import type { Block2dDiff } from "../../../🔺️diff/🟦️component";
import type { Block2dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: MoveHandle, base: Block2dSnapshot): Block2dDiff;
