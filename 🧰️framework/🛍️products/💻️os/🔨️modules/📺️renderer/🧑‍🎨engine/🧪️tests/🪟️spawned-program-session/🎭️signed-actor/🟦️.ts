import Ajv from "ajv";
import { describe, expect, it } from "bun:test";
import { browserActorProgramIsFocusedV1, browserActorProgramIsPublishedV1, browserActorWorkflowNodeIsMountedV1, browserDocumentActorIdentityV1 } from "../../../🧱️elements/🏛️ShellHost/🛫️spawned-session/🟢️request-service-operation/🪪️owner/🟦️.ts";
import type { ActiveSession } from "../../../🧱️elements/🐚️Shell/📦️program-sessions/🟦️.ts";
import type { OpenDocumentSession, RetainedBrowserActorUiV1 } from "../../../🧱️elements/🏛️ShellHost/🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import fixture from "../🧫️fixtures/🎭️signed-actor/🔣️.json";

function session(owner: typeof fixture.primary, panelJson?: string): ActiveSession {
  return {
    pluginId: owner.pluginId,
    instanceId: owner.instanceId,
    app: { id: owner.appId, role: owner.role },
    viewState: { panelJson },
  } as ActiveSession;
}

const validateActorTuple = new Ajv({ $data: true }).compile({
  type: "object",
  required: ["authority", "claim"],
  properties: {
    authority: { type: "object" },
    claim: {
      type: "object",
      required: ["spaceId", "documentId", "verifiedSurfaceId", "clientInstanceId", "activationGeneration"],
      properties: Object.fromEntries(
        ["spaceId", "documentId", "verifiedSurfaceId", "clientInstanceId", "activationGeneration"]
          .map(field => [field, { const: { $data: `/authority/${field}` } }]),
      ),
    },
  },
});

describe("signed browser actor ownership", () => {
  for (const example of fixture.cases) {
    it(example.title, () => {
      const owners = { primary: fixture.primary, editor: fixture.editor, viewer: fixture.viewer };
      const owner = owners[example.target as keyof typeof owners];
      const spawned = example.panel === null ? null : owners[example.panel as "editor" | "viewer"];
      const row = spawned === null ? null : {
        pluginId: spawned.pluginId, instanceId: spawned.instanceId, appId: spawned.appId,
        id: "node-8", label: "Terrain", breadcrumb: [],
      };
      const primary = session(owners.primary, JSON.stringify({
        activePanelTab: "catalogue",
        spawnedApps: row === null ? [] : [row],
        ...(example.focus ? { activeSpawnedId: "node-8" } : {}),
      }));
      const target = session(owner);
      const entry = {
        scope: fixture.scope, documentId: fixture.scope.documentId,
        clientInstanceId: fixture.actor.clientInstanceId,
        session: target,
      } as OpenDocumentSession;
      const identity = { ...fixture.actor, verifiedSurfaceId: owner.appId, scope: { ...fixture.scope } };
      if (example.change === "spaceId" || example.change === "documentId") identity.scope[example.change] = "unexpected";
      else if (example.change === "clientInstanceId" || example.change === "activationGeneration" || example.change === "verifiedSurfaceId") identity[example.change] = "unexpected";
      const retained = {
        ...fixture.actor, scope: fixture.scope, verifiedSurfaceId: owner.appId, sessionInstanceId: owner.instanceId,
        identity: example.change === "pending" ? null : identity,
      } as RetainedBrowserActorUiV1;

      expect(browserActorProgramIsPublishedV1(primary, target)).toBe(example.published);
      expect(browserActorProgramIsFocusedV1(primary, target)).toBe(example.focused);
      const actorIsAttached = browserDocumentActorIdentityV1(entry, retained) === identity;
      expect(actorIsAttached).toBe(example.attached);
      expect(actorIsAttached).toBe(validateActorTuple({
        authority: { ...fixture.scope, ...fixture.actor, verifiedSurfaceId: owner.appId },
        claim: example.change === "pending" ? null : { ...identity, ...identity.scope },
      }));
      const retired = new WeakSet<OpenDocumentSession>();
      const signed = { spaceId: fixture.scope.spaceId, workflowArtifactId: "workflow-88", documents: new Map([["runtime", entry]]), actors: new Map([["runtime", retained]]), retired };
      expect(browserActorWorkflowNodeIsMountedV1(primary, "node-8", signed)).toBe(example.focus && example.focused && example.attached);
      retired.add(entry);
      expect(browserActorWorkflowNodeIsMountedV1(primary, "node-8", signed)).toBe(false);
    });
  }
});
