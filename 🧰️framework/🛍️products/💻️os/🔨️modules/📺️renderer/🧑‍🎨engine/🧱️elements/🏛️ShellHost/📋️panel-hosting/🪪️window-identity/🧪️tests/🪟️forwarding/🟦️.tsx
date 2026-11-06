import { expect, it } from "vitest";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { ChromeAwareWindowScrollSurface } from "../../../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx";
import { windowDocumentScopeV1, windowIdentityAttributesV1 } from "../../🟦️.ts";

it("forwards each owning session and distinct document on real window surfaces", () => {
  const sessions = [1, 2].map(instanceId => ({ session: { pluginId: "plugin", app: { id: "editor" }, instanceId }, scope: { spaceId: "space", documentId: `artifact-${instanceId}` } }));
  const html = sessions.map(({ session }) => renderToStaticMarkup(createElement(ChromeAwareWindowScrollSurface, windowIdentityAttributesV1({ app: session.app.id, role: "editor", instance: session.instanceId, window: `window-${session.instanceId}`, kind: "model", document: windowDocumentScopeV1(sessions, { pluginId: session.pluginId, appId: session.app.id, instanceId: session.instanceId }) }))));
  expect(html[0]).toContain('data-surface-document-id="artifact-1"');
  expect(html[1]).toContain('data-surface-document-id="artifact-2"');
  expect(html[0]).toContain('data-surface-window-id="window-1"');
  expect(html[1]).toContain('data-surface-window-id="window-2"');
  for (const body of html) expect(body).toContain('data-surface-window-kind-id="model"');
});

it("refuses missing or ambiguous document ownership", () => {
  const session = { pluginId: "plugin", app: { id: "editor" }, instanceId: 1 };
  const entry = { session, scope: { spaceId: "space", documentId: "artifact" } };
  const owner = { pluginId: "plugin", appId: "editor", instanceId: 1 };
  expect(windowDocumentScopeV1([], owner)).toBeNull();
  expect(windowDocumentScopeV1([entry, entry], owner)).toBeNull();
  expect(windowDocumentScopeV1([{ session }], owner)).toBeNull();
});
