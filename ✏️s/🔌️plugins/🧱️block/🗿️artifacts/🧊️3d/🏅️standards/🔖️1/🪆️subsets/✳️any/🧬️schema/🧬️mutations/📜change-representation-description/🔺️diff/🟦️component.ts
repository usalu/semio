/** 🔺️ block3d change-representation-description/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { ChangeRepresentationDescription } from "../🦠️mutation/🟦️component";
import type { Block3dDiff } from "../../../🔺️diff/🟦️component";
import type { Block3dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: ChangeRepresentationDescription, base: Block3dSnapshot): Block3dDiff;
