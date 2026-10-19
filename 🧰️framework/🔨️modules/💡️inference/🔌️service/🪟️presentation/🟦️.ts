import type { BrowserActorUiMountedV1 } from "../../../🛰️backbone/🎭️artifact-actor/🟦️.ts";
import type { UiNodeId, UiNodeRecord, UiRevision } from "../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";

/** 📖️ The acknowledged retained document a mounted service view exposes for reading: its revision, root and nodes, without any way to change them. */
export type MountedServiceUiStateV1 = Readonly<{ revision: UiRevision; root: UiNodeId | null; nodes: ReadonlyMap<UiNodeId, UiNodeRecord> }>;

/** 🔬️ Read-only port over the host's retained UI document; the host's store satisfies it structurally and the owner never receives dispatch authority. */
export interface MountedServiceUiReaderV1 {
  getState(): MountedServiceUiStateV1;
}

/** 🪟️ One acknowledged mounted view of an installed service as the host hands it to the owner's probe. */
export type ServiceMountedViewV1 = Omit<BrowserActorUiMountedV1, "kind" | "instanceId"> & Readonly<{ sessionInstanceId: number; windowKindId: string; store: MountedServiceUiReaderV1 }>;

/** 🎛️ One owner-defined action a presentation reports; the host forwards it without decoding the payload. */
export type InferencePortUiAction = Readonly<{ kind: string; payload?: unknown }>;

/** 🪟 Owner-supplied presentation; the host never decodes domain payloads. `Rendered` is the host's element type. */
export interface InstalledServicePresentationV1<Rendered = unknown> {
  readonly owner: string;
  readonly serviceId: string;
  probe?(source: ServiceMountedViewV1 | null): unknown;
  render(payload: unknown, locale: "en" | "de", onAction: (action: InferencePortUiAction) => void): Rendered;
}
