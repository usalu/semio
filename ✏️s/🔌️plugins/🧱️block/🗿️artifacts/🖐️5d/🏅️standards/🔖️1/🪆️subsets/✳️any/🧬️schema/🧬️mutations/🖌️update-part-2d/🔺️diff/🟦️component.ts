/** 🔺️ block5d update-part-2d/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { UpdatePart2d } from "../🦠️mutation/🟦️component";
import type { Block5dDiff } from "../../../🔺️diff/🟦️component";
import type { Block5dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: UpdatePart2d, base: Block5dSnapshot): Block5dDiff;
