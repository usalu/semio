/** 🎮️ Shell plumbing, leftover interaction views: the interaction view, state and world gumball pose a program leaves behind in its output. */
import type { SelectionMode } from "@semio-tech/framework";

//#region ShellHelpers
export type LeftoverInteractionViewV1 = {
  readonly selectedIds: readonly string[];
  readonly hoverTarget: { readonly domain: string; readonly channel: string; readonly id: string } | null;
  readonly locked: Readonly<Record<string, boolean>>;
  readonly gumballActive: boolean;
  readonly gumballAnchorId: string | null;
  readonly selection: Readonly<Record<string, { readonly granularity: string; readonly ids: readonly string[]; readonly anchorId?: string }>>;
  readonly hover: Readonly<Record<string, { readonly channel: string; readonly ids: readonly string[] }>>;
  /** 🧭️ `SelectionMode`, not `string`: this view is built from — and handed straight back to — the
   * shell's own `InteractionState`, whose `activeMode` the interaction module declares as that union. */
  readonly activeMode: Readonly<Record<string, SelectionMode>>;
  readonly activeGranularity: Readonly<Record<string, string>>;
  readonly activeUtility?: string | null;
  /** 🪟️ The window INSTANCE the action that produced this leftover addressed, straight from the
   * guest's own encode (`leftover_interaction_view_from`, `🔌️plugin/🦀️.rs`). `null` is a windowless,
   * document-scoped action — never a synthetic window surface (wave B56). */
  readonly windowId: string | null;
  readonly selectionCleared?: boolean;
};

/** 🧭️ The guest encodes `activeMode` as free text; only the two the interaction module declares are a
 * `SelectionMode`, and anything else is dropped rather than smuggled into the shell's own state. */
function leftoverSelectionModeRecord(value: unknown): Record<string, SelectionMode> {
  const isSelectionMode = (mode: unknown): mode is SelectionMode => mode === "single" || mode === "multiple";
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, SelectionMode] => isSelectionMode(entry[1])));
}

function leftoverStringRecord(value: unknown): Record<string, string> {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, string] => typeof entry[1] === "string"));
}

function leftoverLockedRecord(value: unknown): Record<string, boolean> {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).filter((entry): entry is [string, boolean] => typeof entry[1] === "boolean"));
}

function leftoverSelectionRecord(value: unknown): LeftoverInteractionViewV1["selection"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domain, raw] of Object.entries(value as Record<string, unknown>)) {
    if (!raw || typeof raw !== "object" || Array.isArray(raw)) continue;
    const record = raw as { granularity?: unknown; ids?: unknown; anchorId?: unknown };
    const ids = Array.isArray(record.ids) ? record.ids.filter((id): id is string => typeof id === "string") : [];
    selection[domain] = { granularity: typeof record.granularity === "string" ? record.granularity : "", ids, ...(typeof record.anchorId === "string" ? { anchorId: record.anchorId } : {}) };
  }
  return selection;
}

function leftoverHoverRecord(value: unknown): LeftoverInteractionViewV1["hover"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  const hover: Record<string, { channel: string; ids: string[] }> = {};
  for (const [domain, raw] of Object.entries(value as Record<string, unknown>)) {
    if (!raw || typeof raw !== "object" || Array.isArray(raw)) continue;
    const record = raw as { channel?: unknown; ids?: unknown };
    hover[domain] = { channel: typeof record.channel === "string" ? record.channel : "pointer", ids: Array.isArray(record.ids) ? record.ids.filter((id): id is string => typeof id === "string") : [] };
  }
  return hover;
}

/** 🕹️ Peels leftover `Invocation.output.interactionView` — same leftover lane as history_patch.
 * Gumball visibility follows `gumball.active` only (transform utility), not bare `selectedIds`. */
export function interactionViewFromLeftoverOutput(output: unknown): LeftoverInteractionViewV1 | null {
  if (!output || typeof output !== "object" || Array.isArray(output)) return null;
  const envelope = output as { interactionView?: unknown };
  const raw = envelope.interactionView;
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return null;
  const view = raw as Record<string, unknown>;
  const selectedIds = Array.isArray(view.selectedIds) ? view.selectedIds.filter((id): id is string => typeof id === "string") : [];
  const hoverRaw = view.hoverTarget;
  const hoverTarget =
    hoverRaw && typeof hoverRaw === "object" && !Array.isArray(hoverRaw) && typeof (hoverRaw as { id?: unknown }).id === "string"
      ? {
          domain: typeof (hoverRaw as { domain?: unknown }).domain === "string" ? (hoverRaw as { domain: string }).domain : "",
          channel: typeof (hoverRaw as { channel?: unknown }).channel === "string" ? (hoverRaw as { channel: string }).channel : "pointer",
          id: (hoverRaw as { id: string }).id,
        }
      : null;
  const gumball = view.gumball && typeof view.gumball === "object" && !Array.isArray(view.gumball) ? (view.gumball as { active?: unknown; anchorId?: unknown }) : {};
  return {
    selectedIds,
    hoverTarget,
    locked: leftoverLockedRecord(view.locked),
    gumballActive: gumball.active === true,
    gumballAnchorId: typeof gumball.anchorId === "string" ? gumball.anchorId : gumball.active === true ? selectedIds[0] ?? null : null,
    windowId: typeof view.windowId === "string" && view.windowId.length > 0 ? view.windowId : null,
    selection: leftoverSelectionRecord(view.selection),
    hover: leftoverHoverRecord(view.hover),
    activeMode: leftoverSelectionModeRecord(view.activeMode),
    activeGranularity: leftoverStringRecord(view.activeGranularity),
    ...(typeof view.activeUtility === "string" ? { activeUtility: view.activeUtility } : {}),
    ...(view.selectionCleared === true ? { selectionCleared: true } : {}),
  };
}

/** 🕹️ Host InteractionState from leftover publication — Inspection/clipboard/lock/gumball consumers. */
export function leftoverInteractionStateV1(view: LeftoverInteractionViewV1): {
  readonly selection: LeftoverInteractionViewV1["selection"];
  readonly hover: LeftoverInteractionViewV1["hover"];
  readonly activeMode: LeftoverInteractionViewV1["activeMode"];
  readonly activeGranularity: LeftoverInteractionViewV1["activeGranularity"];
} {
  const vortex = view.selection.vortex;
  const vortexEmpty = !vortex || vortex.ids.length === 0;
  const selection =
    !vortexEmpty || view.selectedIds.length === 0
      ? view.selection
      : { ...view.selection, vortex: { granularity: vortex?.granularity || view.activeGranularity.vortex || "object", ids: [...view.selectedIds], ...(vortex?.anchorId ? { anchorId: vortex.anchorId } : {}) } };
  return { selection, hover: view.hover, activeMode: view.activeMode, activeGranularity: view.activeGranularity };
}

/** 🕹️ Leftover gumball pose for World3d — transformMode + target from the selected instance. */
export function leftoverWorldGumballPoseV1(
  leftover: { readonly gumballActive: boolean; readonly gumballAnchorId: string | null; readonly ids: readonly string[] },
  instances: readonly { readonly id: string; readonly interactionId?: string; readonly position?: readonly [number, number, number]; readonly x?: number; readonly y?: number; readonly z?: number }[],
): { readonly transformMode: "move" | "transform" | undefined; readonly gumballTarget: readonly [number, number, number] | undefined } {
  const id = leftover.gumballAnchorId ?? leftover.ids[0];
  const inst = id ? instances.find((row) => row.id === id || row.interactionId === id) : undefined;
  const gumballTarget =
    inst?.position ??
    (inst && inst.x != null && inst.y != null && inst.z != null ? ([inst.x, inst.y, inst.z] as const) : undefined);
  return {
    transformMode: leftover.gumballActive ? "transform" : undefined,
    gumballTarget,
  };
}
//#endregion ShellHelpers
