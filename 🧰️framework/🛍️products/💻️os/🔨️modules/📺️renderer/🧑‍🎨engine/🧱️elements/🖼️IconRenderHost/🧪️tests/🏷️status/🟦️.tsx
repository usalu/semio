// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, render, waitFor } from "@semio-tech/ui-react/test";
import { configureHostPorts, uiI18n, type IconRenderRequest } from "@semio-tech/ui-react";
import { afterEach, expect, it } from "vitest";
import { IconRenderHost } from "../../🟦️.tsx";
import fixture from "../🧫️fixtures/🏷️status/🔣️.json";
import drawingFixture from "../🧫️fixtures/📤️svg-export/🔣️.json";

afterEach(cleanup);

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

it("replaces a pending preview and paints the SVG background only once", async () => {
  await uiI18n.changeLanguage("en");
  const translucent = drawingFixture.drawingCases.find(sample => sample.backgroundOpacity);
  if (!translucent?.background) throw new Error("Missing translucent background fixture");
  const before: IconRenderRequest = {
    assetUrl: "mesh://status", camera: { position: [0, -10, 0], target: [0, 0, 0], zoom: 1 },
    width: 256, height: 128, format: "svg", background: translucent.background,
    lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" },
  };
  const after: IconRenderRequest = { ...before, drawing: { background: true, fill: false, mergeLines: true } };
  const hidden: IconRenderRequest = { ...after, drawing: { ...after.drawing, background: false } };
  const calls: { request: IconRenderRequest; signal?: AbortSignal }[] = [];
  const restore = configureHostPorts({ iconRender: {
    render: (request, signal) => {
      calls.push({ request, signal });
      return request.drawing ? Promise.resolve({ dataUrl: "data:image/svg+xml,%3Csvg/%3E" }) : new Promise(() => {});
    },
  } });
  const preview = (request: IconRenderRequest) => (
    <IconRenderHost node={{ type: "componentScene", surfaceId: "draw-preview", controllerId: "draw-preview", componentKind: "icon-render", iconRender: { requestJson: JSON.stringify(request) } }} onAction={() => {}} />
  );
  try {
    const view = render(preview(before));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.signal?.aborted).toBe(false);
    const background = () => view.container.querySelector<HTMLElement>("[data-icon-shot-frame]")?.style.background;
    expect(background()).toMatch(/^rgba?\(/);
    view.rerender(preview(after));
    await waitFor(() => expect(view.container.querySelector("img")).not.toBeNull());
    expect(calls[0]?.signal?.aborted).toBe(true);
    expect(calls[1]?.request.drawing).toEqual(after.drawing);
    expect(background()).toBe("transparent");
    view.rerender(preview(hidden));
    await waitFor(() => expect(calls).toHaveLength(3));
    expect(background()).toBe("transparent");
  } finally {
    cleanup();
    restore();
  }
});
