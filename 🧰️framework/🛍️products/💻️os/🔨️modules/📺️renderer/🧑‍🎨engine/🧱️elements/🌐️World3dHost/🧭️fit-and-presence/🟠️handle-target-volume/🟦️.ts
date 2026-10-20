/** 🟠️ The handle target volume phase of useWorldFitAndPresence. */
import { useCallback } from "react";
import { type WorldVolumeRelocatePayload } from "@semio-tech/framework/infinite/world/r3f";
import { world3dMarkerInteractionTarget } from "../../🔗️instance-selection/🟦️.ts";
import { world3dSelectionActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { useAttractionsState } from "../🔶️attractions/🟦️.ts";
import type { useMeshesRefState } from "../🔸️meshes-ref/🟦️.ts";

/** 🟡 Builds handleTargetVolumeSelect, handleTargetVolumeRelocate of useWorldFitAndPresence. */
export function useHandleTargetVolumeState({ targetVolumes, interactionDomainId, dispatch, selectionMode }: Pick<ReturnType<typeof useAttractionsState>, "targetVolumes" | "interactionDomainId"> & Pick<ReturnType<typeof useMeshesRefState>, "dispatch" | "selectionMode">) {
  const handleTargetVolumeSelect = useCallback(
    (id: string) => {
      const volume = targetVolumes.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("targetVolume", id, volume);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, volume?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (volume?.locked) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge: "replace" });
        return;
      }
      dispatch("setSelection", {
        selection: { objectIds: [], vortexIds: [], attractionIds: [], targetVolumeIds: [id], referenceIds: [] },
      });
    },
    [dispatch, interactionDomainId, selectionMode, targetVolumes],
  );

  const handleTargetVolumeRelocate = useCallback(
    (payload: WorldVolumeRelocatePayload) => {
      dispatch("relocateTargetVolume", {
        volumeId: payload.volumeId,
        mode: payload.mode,
        before: payload.before,
        after: payload.after,
      });
    },
    [dispatch],
  );
  return { handleTargetVolumeRelocate, handleTargetVolumeSelect };
}
