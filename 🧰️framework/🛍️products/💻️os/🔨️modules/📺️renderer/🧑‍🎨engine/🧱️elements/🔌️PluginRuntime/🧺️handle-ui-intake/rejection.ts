/** 🚫️ A guest UI patch the shell's retained-UI intake refused, kept as a typed, localized fault instead of a string the next refresh forgets.
 *
 * 🐛️ The PDF editor's thumbnail strip published a tree row carrying a record `Activate` binding; the intake refused it
 * (`Owned UI graph invariants violated (invalidRowTarget) node=1`), the refresh that carried it dropped the surface, and the
 * boot error the user saw was the NEXT refresh's `stopped without publishing requested UI surfaces (missing=["1:pdf.thumbnails"])`,
 * which names neither the contract rule nor the node (ticket 26/11/22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END, round 3, failure 5).
 * The rejection is now recorded per instance and surface, logged once as a console diagnostic, and any refresh that fails while
 * that surface is still unpublished fails with the rejection itself (code, contract rule, node, localized notice).
 *
 * @see ./rejection-cases.json
 * @see 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts (`graphViolationMessage`, the failure text parsed here)
 */

/** 🏷️ The fault code every refused guest UI patch carries. */
export const PLUGIN_UI_INTAKE_REJECTED_CODE = "plugin-ui.intake-rejected";

/** 🌐️ One notice text per supported UI language; no language is assumed. */
export type PluginUiIntakeNoticeV1 = Readonly<{ en: string; de: string }>;

/** 🧾️ What the intake refused: the surface, the intake phase, the contract rule and node the retained-UI contract named, and its raw text. */
export type PluginUiIntakeRejectionV1 = Readonly<{ surfaceId: string; phase: string; rule: string | null; node: number | null; failure: string }>;

/** 🔎️ Reads the contract rule and node off a retained-UI failure text (`… (invalidRowTarget) node=1`); both are `null` when the text names none. */
export function parsePluginUiIntakeFailureV1(failure: string): Readonly<{ rule: string | null; node: number | null }> {
  const rule = /\(([A-Za-z][A-Za-z0-9]*)\)/.exec(failure)?.[1] ?? null;
  const node = /\bnode=(\d+)\b/.exec(failure)?.[1];
  return { rule, node: node === undefined ? null : Number(node) };
}

/** 🪟️ The window kind a retained surface id (`<instance>:<window kind>`) addresses. */
function surfaceWindowKind(surfaceId: string): string {
  const separator = surfaceId.indexOf(":");
  return separator < 0 ? surfaceId : surfaceId.slice(separator + 1);
}

/** 🗣️ The localized notice of one rejection: which window cannot be shown and which contract rule at which node refused it. */
export function pluginUiIntakeRejectionNoticeV1(rejection: PluginUiIntakeRejectionV1): PluginUiIntakeNoticeV1 {
  const window = surfaceWindowKind(rejection.surfaceId);
  const rule = rejection.rule ?? rejection.phase;
  return {
    en: `The window “${window}” cannot be shown: the app published an interface the shell refuses (${rule}${rejection.node === null ? "" : `, node ${rejection.node}`}).`,
    de: `Das Fenster „${window}“ kann nicht angezeigt werden: Die App hat eine Oberfläche veröffentlicht, die die Shell ablehnt (${rule}${rejection.node === null ? "" : `, Knoten ${rejection.node}`}).`,
  };
}

/** 🚫️ The error a refused patch raises: `code`, `origin` (the surface) and the localized `notice` reach the shell's fault card through `windowFaultFromError`. */
export class PluginUiIntakeRejectedError extends Error {
  readonly code = PLUGIN_UI_INTAKE_REJECTED_CODE;
  readonly origin: string;
  readonly notice: PluginUiIntakeNoticeV1;
  constructor(readonly rejection: PluginUiIntakeRejectionV1, options?: ErrorOptions) {
    super(`${PLUGIN_UI_INTAKE_REJECTED_CODE}:${rejection.phase}:${rejection.failure} surface=${rejection.surfaceId}`, options);
    this.name = "PluginUiIntakeRejectedError";
    this.origin = rejection.surfaceId;
    this.notice = pluginUiIntakeRejectionNoticeV1(rejection);
  }
}

/** 🧾️ Builds the rejection of `surfaceId` from the intake's phase and failure text. */
export function pluginUiIntakeRejectionV1(surfaceId: string, phase: string, failure: string | null): PluginUiIntakeRejectionV1 {
  const text = failure ?? "unknown";
  return { surfaceId, phase, ...parsePluginUiIntakeFailureV1(text), failure: text };
}

/** 📒️ The rejections of one plugin handle, per instance and surface, until the surface installs or the instance closes. */
export type PluginUiIntakeRejectionLedgerV1 = Readonly<{
  record: (instanceId: number, rejection: PluginUiIntakeRejectionV1) => PluginUiIntakeRejectedError;
  clear: (instanceId: number, surfaceId: string) => void;
  forget: (instanceId: number) => void;
  explain: (instanceId: number, surfaceIds: Iterable<string>, error: unknown) => unknown;
}>;

/** 📒️ Creates the ledger. `explain` answers the error a failed refresh should raise: a rejection as is, else the recorded rejection of the
 * first requested surface (carrying the original failure as `cause`), else the original failure. `report` receives every recorded
 * rejection once, as the console diagnostic. */
export function createPluginUiIntakeRejectionLedgerV1(report: (error: PluginUiIntakeRejectedError) => void): PluginUiIntakeRejectionLedgerV1 {
  const byInstance = new Map<number, Map<string, PluginUiIntakeRejectionV1>>();
  return {
    record: (instanceId, rejection) => {
      const surfaces = byInstance.get(instanceId) ?? new Map<string, PluginUiIntakeRejectionV1>();
      surfaces.set(rejection.surfaceId, rejection);
      byInstance.set(instanceId, surfaces);
      const error = new PluginUiIntakeRejectedError(rejection);
      report(error);
      return error;
    },
    clear: (instanceId, surfaceId) => { byInstance.get(instanceId)?.delete(surfaceId); },
    forget: (instanceId) => { byInstance.delete(instanceId); },
    explain: (instanceId, surfaceIds, error) => {
      if (error instanceof PluginUiIntakeRejectedError) return error;
      const surfaces = byInstance.get(instanceId);
      if (!surfaces) return error;
      for (const surfaceId of surfaceIds) {
        const rejection = surfaces.get(surfaceId);
        if (rejection) return new PluginUiIntakeRejectedError(rejection, { cause: error });
      }
      return error;
    },
  };
}
