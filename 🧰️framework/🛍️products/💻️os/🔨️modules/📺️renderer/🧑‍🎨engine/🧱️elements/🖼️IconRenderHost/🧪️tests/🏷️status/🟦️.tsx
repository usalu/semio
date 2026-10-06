// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, render, waitFor } from "@semio-tech/ui-react/test";
import { configureHostPorts, uiI18n } from "@semio-tech/ui-react";
import { afterEach, expect, it } from "vitest";
import { IconRenderHost } from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🏷️status/🔣️.json";

afterEach(cleanup);

it("validates the neutral icon status contract", () => {
});

it("renders actual icon host states in each selected language", async () => {
  for (const pack of fixture.locales) {
    if (pack.locale !== "en" && pack.locale !== "de") throw new Error("Invalid fixture locale");
    await uiI18n.changeLanguage(pack.locale);
    for (const sample of fixture.cases) {
      const restore = configureHostPorts({ iconRender: { render: () => sample.resident
        ? Promise.resolve({ dataUrl: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'/%3E" })
        : sample.faulted || sample.assetMiss ? Promise.reject(new Error(fixture.diagnostic)) : new Promise(() => {}) } });
      try {
        const request = { assetUrl: "mesh://status", camera: { position: [0, -10, 0], target: [0, 0, 0], zoom: 1 }, width: 256, height: 128, format: "png" };
        const view = render(<IconRenderHost node={{ type: "componentScene", surfaceId: "status-test", controllerId: "status-test", componentKind: "icon-render", ...(sample.request ? { iconRender: { requestJson: JSON.stringify(request) } } : {}) }} onAction={() => {}} />);
        if (sample.state === "ready") {
          await waitFor(() => expect(view.container.querySelector("img")).not.toBeNull());
          for (const label of Object.values(pack.labels).filter(Boolean)) expect(view.container.textContent).not.toContain(label!);
        } else {
          await waitFor(() => expect(view.getByText(pack.labels[sample.state as "empty" | "rendering" | "failed"])).toBeTruthy());
          expect(view.container.querySelector("img")).toBeNull();
          if (sample.state === "failed") expect(view.getByText(fixture.diagnostic)).toBeTruthy();
        }
      } finally {
        cleanup();
        restore();
      }
    }
  }
});
