/** ✏️ Text peer carets (collab STEP 14, writer): from the shared `✏️text-carets` fixture, `TextPeerCaretsOverlayV1`
 * paints exactly one caret per OTHER actor that published a `space: "text"` view of this window, at its WORLD caret projected
 * through the viewer's own camera, with the localized accessible name (en + de); its own caret, another window and a
 * canvas-space view paint nothing. */
import { afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { TextPeerCaretsOverlayV1 } from "../../🟦️.tsx";
import { clearArtifactPresenceRosterV1, publishArtifactPresenceRosterV1, publishLocalPresenceActorV1 } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/✏️text-carets/🔣️.json" with { type: "json" };

afterEach(() => {
  clearArtifactPresenceRosterV1("local");
  cleanup();
});

describe("TextPeerCaretsOverlayV1", () => {
  for (const locale of ["en", "de"] as const) {
    it(`paints every other actor's caret at its projected text position (${locale})`, () => {
      publishLocalPresenceActorV1("local", fixture.myActor);
      publishArtifactPresenceRosterV1("local", fixture.roster as never);
      const view = render(createElement(TextPeerCaretsOverlayV1, { windowId: fixture.windowId, project: ([x, y]: readonly [number, number]) => ({ x, y: y - fixture.cameraY }), frame: 0, lineHeightPx: fixture.lineHeightPx, locale }));
      const carets = [...view.container.querySelectorAll<HTMLElement>("[data-peer-caret]")].map((element) => ({
        actor: element.getAttribute("data-peer-actor"),
        x: Number.parseFloat(element.style.left),
        y: Number.parseFloat(element.style.top),
        height: Number.parseFloat(element.style.height),
        label: element.getAttribute("aria-label"),
        cursor: element.hasAttribute("data-peer-cursor"),
      }));
      expect(carets).toEqual(fixture.expected.map((row) => ({ actor: row.actor, x: row.x, y: row.y, height: fixture.lineHeightPx, label: row.label[locale], cursor: true })));
    });
  }

  for (const locale of ["en", "de"] as const) {
    it(`shows every other actor's pending typing run on this window beside its caret (${locale})`, () => {
      publishLocalPresenceActorV1("local", fixture.myActor);
      publishArtifactPresenceRosterV1("local", fixture.roster as never);
      const view = render(createElement(TextPeerCaretsOverlayV1, { windowId: fixture.windowId, project: ([x, y]: readonly [number, number]) => ({ x, y: y - fixture.cameraY }), frame: 0, lineHeightPx: fixture.lineHeightPx, locale }));
      const typing = [...view.container.querySelectorAll<HTMLElement>("[data-peer-caret]")].flatMap((caret) => {
        const preview = caret.querySelector<HTMLElement>("[data-peer-typing]");
        return preview === null ? [] : [{ actor: caret.getAttribute("data-peer-actor"), text: preview.textContent, deleted: preview.querySelector("s")?.textContent ?? "", label: preview.getAttribute("aria-label") }];
      });
      expect(typing).toEqual(fixture.expectedTyping.map((row) => ({ actor: row.actor, text: row.text, deleted: row.deleted, label: row.label[locale] })));
    });
  }

  it("paints nothing without a caret projection", () => {
    publishLocalPresenceActorV1("local", fixture.myActor);
    publishArtifactPresenceRosterV1("local", fixture.roster as never);
    const view = render(createElement(TextPeerCaretsOverlayV1, { windowId: fixture.windowId, project: () => null, frame: 0, lineHeightPx: fixture.lineHeightPx }));
    expect(view.container.querySelector("[data-peer-caret]")).toBeNull();
  });
});
