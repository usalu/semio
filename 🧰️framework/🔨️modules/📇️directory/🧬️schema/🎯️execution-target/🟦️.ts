/** 🎯️ Execution target lease: the verified component, descriptor, lease fields, surface kind admission, status text and progress stages. */

import { type DocumentExecutionTargetBrowserActorV1 } from "../🌐️browser-actor/🟦️.ts";
import { type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceV1 } from "../📂️document-open/🟦️.ts";

//#region 🔖️Views
//#region 🪪️ExecutionTargetLease
/** 🧱️ Exact byte identity of the verified component, duplicated from {@link DocumentOpenPackageV1}
 * so a server byte response can never silently answer with a different object. */
export interface DocumentExecutionTargetComponentV1 {
  sha256: string;
  blake3: string;
  byteLength: number;
}

/** 📜️ Exact byte identity of the verified raw package descriptor. */
export interface DocumentExecutionTargetDescriptorV1 {
  sha256: string;
  byteLength: number;
}

/** 🪪️ Receipt-free public fields of one document execution-target lease. It carries no plan
 * receipt, socket grant, session token, hub origin, raw path or module URL: those are never lease
 * fields and never lease constructors. */
export interface DocumentExecutionTargetLeaseFieldsV1 {
  schema: "semio.os.document-execution-target-lease/v1";
  version: 1;
  scope: DocumentScope;
  descriptorDigestV1: string;
  catalog: DocumentOpenCatalogV1;
  package: DocumentOpenPackageV1;
  component: DocumentExecutionTargetComponentV1;
  descriptor: DocumentExecutionTargetDescriptorV1;
  browserActor: DocumentExecutionTargetBrowserActorV1;
  artifact: DocumentOpenArtifactV1;
  parentDialect: DocumentOpenParentDialectV1;
  surface: DocumentOpenSurfaceV1;
  grant: DocumentOpenGrantV1;
  checkpoint: DocumentOpenCheckpointV1;
  revalidation: DocumentOpenRevalidationV1;
}

/** 🗂️ One artifact kind a descriptor declares, reduced to the pair a document open is admitted on. */
export type SurfaceArtifactKindV1 = { readonly id: string; readonly schema: string };

/** 🎯️ One surface app of a verified descriptor as the pairing rule reads it: its role, its full dialect coordinate, the
 * artifact kinds it declares itself and the kind its io presents (`null` when it presents none). */
export type SurfaceKindAppV1 = {
  readonly role: "editor" | "viewer";
  readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string };
  readonly artifactKinds: readonly SurfaceArtifactKindV1[];
  readonly presents: SurfaceArtifactKindV1 | null;
};

/** 🗂️ The one surface ↔ artifact-kind pairing rule, the browser twin of the hub's `app_opens_kind`
 * (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`) that publishes and verifies every open target: a plugin-level
 * kind (`PluginBuilder::artifact_kind`) is opened only by the surfaces whose own dialect names it, even when a sibling app
 * lists it as an input; a HOSTED kind (`hostedArtifactKinds`) by the surfaces whose dialect names it or whose io presents it
 * and by the viewers of a presenting editor's exact dialect; any other kind is opened by the editor that declares it itself
 * (a plugin on the declaration tree
 * stitches its spec onto the app) and by the viewers of that editor's dialect — the read-only surface of the same
 * documents. Replayed from `🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json` and from the hub's own
 * `🔏️trusted-catalog/🧫️fixtures/🎯️descriptor-open-targets/🔣️.json`.
 *
 * 🧯️ The browser used to admit on the plugin-level list alone, so every document of a plugin migrated onto the
 * declaration tree (note, draw, writer, puzzle, …: `manifest.artifactKinds = []`) was refused after its
 * verified download with "The document component could not be verified" (measured inside `s` against hub 7800,
 * ticket 26/09/23 S15). It then still refused every viewer the hub issued (a viewer declares no kinds of its own), so a
 * Spectator's open ended in "The document target changed" and an unmounted viewer (ticket 26/09/23 C13, row 3.4). */
export function surfaceOpensArtifactKindV1(
  pluginArtifactKinds: readonly SurfaceArtifactKindV1[],
  hostedArtifactKinds: readonly SurfaceArtifactKindV1[],
  apps: readonly SurfaceKindAppV1[],
  app: SurfaceKindAppV1,
  artifact: { readonly kind: string; readonly schema: string },
): boolean {
  const declares = (kinds: readonly SurfaceArtifactKindV1[]): boolean => kinds.some((kind) => kind.id === artifact.kind && kind.schema === artifact.schema);
  if (declares(pluginArtifactKinds)) return app.dialect.artifactKind === artifact.kind;
  if (declares(hostedArtifactKinds)) {
    const presents = (candidate: SurfaceKindAppV1): boolean => candidate.presents !== null && candidate.presents.id === artifact.kind && candidate.presents.schema === artifact.schema;
    return (
      app.dialect.artifactKind === artifact.kind ||
      presents(app) ||
      (app.role === "viewer" &&
        apps.some(
          (editor) =>
            editor.role === "editor" &&
            editor.dialect.artifactKind === app.dialect.artifactKind &&
            editor.dialect.standard === app.dialect.standard &&
            editor.dialect.subset === app.dialect.subset &&
            presents(editor),
        ))
    );
  }
  return (
    declares(app.artifactKinds) ||
    (app.role === "viewer" &&
      apps.some(
        (editor) =>
          editor.role === "editor" &&
          editor.dialect.artifactKind === app.dialect.artifactKind &&
          editor.dialect.standard === app.dialect.standard &&
          editor.dialect.subset === app.dialect.subset &&
          declares(editor.artifactKinds),
      ))
  );
}

/** 🌐️ Complete localized execution-target status vocabulary. No code carries an origin, URL, path,
 * receipt, grant, digest or user identity; EN and DE are both explicit with no default language. */
export type DocumentExecutionTargetStatusCodeV1 = "verifying" | "retrying" | "catching-up" | "integrity-failed" | "stale" | "cancelled" | "renderer-unavailable" | "link-expired" | "access-revoked";

export const DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1: Readonly<Record<DocumentExecutionTargetStatusCodeV1, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  verifying: Object.freeze({ en: "Verifying document component…", de: "Dokumentkomponente wird überprüft…" }),
  retrying: Object.freeze({ en: "The hub is busy. Asking again for the document component…", de: "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…" }),
  "catching-up": Object.freeze({ en: "Catching up with the hub…", de: "Gleiche mit dem Hub ab…" }),
  "integrity-failed": Object.freeze({ en: "The document component could not be verified. Reopen the document.", de: "Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut." }),
  stale: Object.freeze({ en: "The document target changed. Reopen the document.", de: "Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut." }),
  cancelled: Object.freeze({ en: "Opening the document was cancelled.", de: "Das Öffnen des Dokuments wurde abgebrochen." }),
  "renderer-unavailable": Object.freeze({ en: "The verified document component is ready, but this renderer is unavailable.", de: "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar." }),
  "link-expired": Object.freeze({ en: "The connection was lost for too long. Reconnect to keep editing this document.", de: "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten." }),
  "access-revoked": Object.freeze({ en: "Your access to this document was removed.", de: "Ihr Zugriff auf dieses Dokument wurde entfernt." }),
});

/** 🔊️ ARIA live-region politeness for one execution-target status: progress and a retry of a declared transient answer
 * announce (the opening still runs and can be cancelled), every terminal integrity/stale/renderer outcome asserts. */
export function documentExecutionTargetStatusRoleV1(code: DocumentExecutionTargetStatusCodeV1): "status" | "alert" {
  return code === "verifying" || code === "retrying" || code === "catching-up" ? "status" : "alert";
}

/** 🪜️ Every stage one execution-target install passes through. The first five are the owner's own
 * fetch and verification; the `actor-` stages are the browser-actor child's, relayed verbatim from
 * `BROWSER_ACTOR_CHILD_LOAD_STAGES` so the load phase — which used to report nothing at all between
 * the last fetched byte and either a live component or a silent stall — names the stage it is in. */
export type DocumentExecutionTargetProgressStageV1 =
  | "manifest"
  | "component"
  | "descriptor"
  | "browser-actor"
  | "verify"
  | "actor-transfer"
  | "actor-received"
  | "actor-verified"
  | "actor-importing"
  | "actor-imported"
  | "actor-decode"
  | "actor-compile"
  | "actor-instantiate"
  | "actor-activating"
  | "actor-active"
  | "actor-describe"
  | "actor-verify"
  | "actor-open"
  | "actor-cold"
  | "actor-view"
  | "actor-ready"
  | "canonical-pair"
  | "catch-up";

/** 📈️ Bounded install progress. It never carries bytes, paths, receipts or full digests. For the `catch-up` stage the two counts
 * are hub tail messages (delivered / retained), not bytes. */
export interface DocumentExecutionTargetProgressV1 {
  stage: DocumentExecutionTargetProgressStageV1;
  completedBytes: number;
  totalBytes: number;
}
//#endregion 🪪️ExecutionTargetLease
//#endregion 🔖️Views
