/** 🔺️ block3d delete-representation/🔺️diff facade — sparse delta builder signature (real, non-stub). */
import type { DeleteRepresentation } from "../🦠️mutation/🟦️component";
import type { Block3dDiff } from "../../../🔺️diff/🟦️component";
import type { Block3dSnapshot } from "../../../📸️snapshot/🟦️component";

export declare function diff(payload: DeleteRepresentation, base: Block3dSnapshot): Block3dDiff;
