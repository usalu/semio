/** 🧩️ S deployment explicitly composes neutral registry, S guests and Hub catalog owners. */
import { composeRebuildPlansV1, readRebuildPlanV1, type RebuildPlanV1 } from "@semio-tech/plugin-registry/rebuild";
import { fileURLToPath } from "node:url";

/** 🧊️ S owns its guest checks and staging operations. */
export function readSGuestRebuildPlanV1(): RebuildPlanV1 {
  return readRebuildPlanV1(fileURLToPath(new URL("./🔣️.json", import.meta.url)));
}

/** 🏁️ The deployment caller supplies every concrete owner through the general contract. */
export function readSRebuildPlanV1(): RebuildPlanV1 {
  return composeRebuildPlansV1([readRebuildPlanV1(), readSGuestRebuildPlanV1(), readRebuildPlanV1(fileURLToPath(new URL("../../../🌎️hub/🔁️rebuild/🔣️.json", import.meta.url)))]);
}
