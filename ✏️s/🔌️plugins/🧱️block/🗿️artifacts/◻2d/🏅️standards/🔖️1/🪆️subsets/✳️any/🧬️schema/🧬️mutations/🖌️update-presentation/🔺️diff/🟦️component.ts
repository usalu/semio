/** 🔺️ block2d update-presentation/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { UpdatePresentation } from "../🦠️mutation/🟦️component";
import type { Block2dDiff } from "../../../🔺️diff/🟦️component";
import type { Block2dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: UpdatePresentation, base: Block2dSnapshot): Block2dDiff;
