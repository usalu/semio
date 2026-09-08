/** 🪟️ Ephemeral host view for one live document opening, independent of document config. */
import { parseResolvedPluginViewState, type ResolvedPluginViewState } from "../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import type { DocumentScope } from "../../📇️directory/🧬️schema/🟦️.ts";

export type BrowserActorViewStateRequest = {
  readonly kind: "browser-actor-view-state";
  readonly clientInstanceId: string;
  readonly scope: DocumentScope;
  readonly viewState: ResolvedPluginViewState;
};

/** 📥️ Validates opening identity and resolved host context before worker retention. */
export function parseBrowserActorViewStateRequest(value: unknown): BrowserActorViewStateRequest {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("browser actor view: expected object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "clientInstanceId,kind,scope,viewState" || row.kind !== "browser-actor-view-state" || typeof row.clientInstanceId !== "string" || !/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(row.clientInstanceId)) throw new Error("browser actor view: invalid opening identity");
  if (row.scope === null || typeof row.scope !== "object" || Array.isArray(row.scope)) throw new Error("browser actor view: invalid scope");
  const scope = row.scope as Record<string, unknown>;
  if (Object.keys(scope).sort().join(",") !== "documentId,spaceId" || Object.values(scope).some((id) => typeof id !== "string" || id.length === 0 || Array.from(id).length > 256 || /[\u0000-\u001f\u007f]/u.test(id))) throw new Error("browser actor view: invalid scope");
  return { kind: "browser-actor-view-state", clientInstanceId: row.clientInstanceId, scope: { spaceId: scope.spaceId as string, documentId: scope.documentId as string }, viewState: parseResolvedPluginViewState(row.viewState) };
}
