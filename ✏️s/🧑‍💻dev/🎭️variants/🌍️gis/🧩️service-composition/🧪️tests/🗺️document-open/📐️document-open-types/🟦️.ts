/** 📐️ The type aliases the GIS document open tests share across their phases: the framework types they reach through dynamic module paths. */
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";
export type ActorInstanceLifetime = import("@semio-tech/framework-actor/lifetime").ActorInstanceLifetime;
export type ArtifactEvent = import("@semio-tech/framework-os").ArtifactEvent;
export type ArtifactState = import("@semio-tech/framework-os/modules/store/worker").ArtifactState;
export type BackboneWorkerResponse = import("@semio-tech/framework-os").BackboneWorkerResponse;
export type BrowserActorChildValue = import("@semio-tech/framework/browser-bundle/child/schema").BrowserActorChildValue;
export type BrowserActorUiPatchOfferV1 = import("@semio-tech/framework/browser-bundle/patch-handoff").BrowserActorUiPatchOfferV1;
export type DocumentBrowserActorChild = import("@semio-tech/framework-os/modules/store/worker").DocumentBrowserActorChild;
export type MutationEnvelope = import("@semio-tech/framework-replication").MutationEnvelope;
export type ResolvedPluginViewState = import("@semio-tech/framework/manifest").ResolvedPluginViewState;
export type WireArtifactBootstrap = import("@semio-tech/framework-replication").WireArtifactBootstrap;
export type Vitest = NonNullable<ImportMeta["vitest"]>;

/** 🧮️ The mutable observations of the cold pair transfer, shared by its fake worker and its phases. */
export type ColdPairProbe = { renderWakes: number; activeLifetime: ActorInstanceLifetime | null; panelRefreshes: number; stagedCommandSequence: number | null; stagedColdPairIngress: Record<string, any>; lifecycleAcknowledged: boolean; patchAcknowledged: boolean; patchRejected: boolean; actionPatchAcknowledged: boolean; heldActionPatch: BrowserActorUiPatchOfferV1 | null; loaded: number; described: number; terminated: number; owner: InstanceType<ServiceCompositionTestDependenciesV1["VerifiedColdDocumentPair"]> | null; reservation: Awaited<ReturnType<ServiceCompositionTestDependenciesV1["reserveDocumentBrowserActorChild"]>> };
