/** 🔺️ block2d create-handle/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { CreateHandle } from "../🦠️mutation/🟦️component";
import type { Block2dDiff } from "../../../🔺️diff/🟦️component";
import type { Block2dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: CreateHandle, base: Block2dSnapshot): Block2dDiff;
