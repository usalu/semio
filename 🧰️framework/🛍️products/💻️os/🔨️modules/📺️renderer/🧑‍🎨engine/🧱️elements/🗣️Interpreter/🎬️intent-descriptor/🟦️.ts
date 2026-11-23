/** 🎬️ Bridges one semantic UI intent onto the plugin action address the shell dispatches.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; a leaf with no renderer imports, so the interpreter's own tests and the shell helpers above it share one definition without a cycle.
 */

import type { ActionDescriptor, UiIntent } from "@semio-tech/framework";

/** 🏷️ The argument FIELD a trigger's scalar payload travels under. An action reads named arguments —
 * `value` for an absolute edit (`engagement_input`, `engagement_control_select`,
 * `puzzle3d_absolute_or_delta`'s absolute half) and `delta` for a relative bump (its relative half) —
 * so the name belongs to the TRIGGER, not to the control that fired it. */
function uiInputField(trigger: UiIntent["trigger"]): string {
  return trigger === "delta" ? "delta" : "value";
}

/** 🎬️ The arguments one intent dispatches: the node's AUTHORED args with the gesture's own payload
 * merged over them.
 *
 * 🧯️ A scalar payload used to REPLACE the authored args wholesale — a `NumberStepper`'s `onChange`
 * reports the number `10.5`, so the action received the bare scalar `10.5` and every guest, which reads
 * `args.get("value")`, saw nothing at all while the authored `{windowId}` was thrown away with it. The
 * whole Settings panel was mute for exactly that reason: browser-measured on `:6013`, `setGridSpacing`
 * reaching the guest and settling with `historyUpserts: 0, effects: 0` while the stepper rendered its
 * optimistic 10.5 and both windows' rails stayed at 10 for 30 s (ticket 26/09/02/PUZZLE-3D-END-TO-END
 * wave B47 §2). A scalar is NAMED by its trigger and merged; a map payload merges as it always did. */
function uiIntentPayload(intent: UiIntent): unknown {
  if (intent.input === null) return intent.args ?? undefined;
  const scalar = typeof intent.input === "string" || typeof intent.input === "number" || typeof intent.input === "boolean" || typeof intent.input === "bigint";
  const named = scalar ? { [uiInputField(intent.trigger)]: intent.input } : intent.input;
  if (intent.args === null) return named;
  if (typeof intent.args === "object" && !Array.isArray(intent.args) && typeof named === "object" && !Array.isArray(named)) return { ...intent.args, ...named };
  return named;
}

/** 🌉️ Bridges one semantic UI intent onto the existing plugin action address. Version one is the direct `ActionFactory` mapping; later versions stay explicit in the action name until the host wire owns a version field. */
export function uiIntentToActionDescriptor(intent: UiIntent): ActionDescriptor {
  const payload = uiIntentPayload(intent);
  return { ...actionBindingToActionDescriptor({ action: intent.action, args: null }), ...(payload === undefined ? {} : { args: payload }) };
}

/** 🌉️ The plugin action address an authored binding fires — the mapping {@link uiIntentToActionDescriptor} applies
 * to a fired intent, for a host control that re-offers a guest's own binding (the Tasks window pausing a tool run with
 * the run's own ToolRun-panel Pause). */
export function actionBindingToActionDescriptor(binding: { readonly action: UiIntent["action"]; readonly args: unknown }): ActionDescriptor {
  return {
    controllerId: binding.action.scope,
    action: binding.action.version === 1 ? binding.action.name : `${binding.action.name}@${binding.action.version}`,
    ...(binding.args === null || binding.args === undefined ? {} : { args: binding.args }),
  };
}
