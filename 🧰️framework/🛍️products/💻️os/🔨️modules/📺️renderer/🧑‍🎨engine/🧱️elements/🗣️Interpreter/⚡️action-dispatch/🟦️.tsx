/** ⚡️ How a retained view dispatches a trigger: the control icon node, the trigger dispatch, the continuous trigger lane and the wire value conversion.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { useEffect, useRef, type ReactElement } from "../../⚛️react-port/🟦️.ts";
import { Icon, type IconName } from "@semio-tech/ui-react";
import { type ContinuousGestureLane, continuousPressIdentity, createContinuousGestureLane } from "@semio-tech/framework";
import { emitIntent } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type UiNodeRecord, type UiTrigger, type UiValue } from "@semio-tech/framework";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { dispatchControlTrigger, type UiControlContext } from "../🎛️control-port/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ActionDispatch
export function resolveControlIconNode(iconId: string, size: number | "tiny" | "small" | "base" | "large" = "small"): ReactElement {
  return <Icon icon={iconId as IconName} size={size} />;
}

export function dispatchTrigger(context: UiInterpreterContext, record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): ReturnType<UiInterpreterContext["onIntent"]> {
  const intent = emitIntent(context.store, record, trigger, input);
  return intent ? context.onIntent(intent) : undefined;
}

//#region 🎚️ContinuousControlLane
/** 🎚️ The coalescing lane a CONTINUOUS control's `change` trigger rides, one per mounted control.
 *
 * A slider dragged at 60 Hz, or a spinner held down, emits a value every frame. Dispatching each one
 * is one retained command, one document edit, one history entry and one preview re-evaluation EACH:
 * measured on 6018, a one-second drag of the Inspection panel's number field cost 29
 * `patchFlowWidgets`, 24 `toolRunStart`s and 29 history entries, and the mesh arrived 3.0 s behind the
 * value (`📓️slider-preview-update-2026-09-15.md`). The lane keeps ONE value in flight and ONE owed —
 * always the newest — and always sends the release.
 *
 * `context` and `record` are read through a ref because both identities change on every render while
 * the lane must outlive them: a lane recreated per render is not a lane. Press identities come from
 * `continuousPressIdentity`, unique per page even within one millisecond. */
export function useContinuousTriggerLane(context: UiControlContext, record: UiNodeRecord): ContinuousGestureLane<UiValue> {
  const bindingRef = useRef({ context, record });
  bindingRef.current = { context, record };
  const gestureRef = useRef<string | null>(null);
  const laneRef = useRef<ContinuousGestureLane<UiValue> | null>(null);
  if (laneRef.current === null) {
    laneRef.current = createContinuousGestureLane<UiValue>({
      send: (value, phase) => {
        if (gestureRef.current === null) gestureRef.current = continuousPressIdentity(bindingRef.current.record.key);
        const gesture = gestureRef.current;
        if (phase === "commit") gestureRef.current = null;
        return dispatchControlTrigger(bindingRef.current.context, bindingRef.current.record, "change", { value, gesture, commit: phase === "commit" } as UiValue)?.then(() => undefined);
      },
      abort: (reason) => {
        const gesture = gestureRef.current;
        gestureRef.current = null;
        if (gesture === null) return undefined;
        return dispatchControlTrigger(bindingRef.current.context, bindingRef.current.record, "change", { gesture, abort: reason } as UiValue)?.then(() => undefined);
      },
      onFault: (error) => undefined,
    });
  }
  const lane = laneRef.current;
  useEffect(() => () => lane.abort("retired"), [lane]);
  return lane;
}
//#endregion 🎚️ContinuousControlLane

/** 🧬️ Widens a primitive into the untagged `UiValue` union — every `Change`/`Delta` trigger's own
 * payload is always one of these three JS-native shapes, never a nested list/map, at this call site. */
export function toUiValue(value: string | number | boolean): UiValue {
  return value as UiValue;
}
//#endregion ActionDispatch
//#endregion 🔖️UiInterpreter
