import { configureHostPorts, type IconRenderRequest } from "@semio-tech/ui-react";
import { expect, it } from "vitest";
import Ajv from "ajv";
import schema from "../../🧬️schema/📤️export-request/🔣️.json";
import { renderIconRequest } from "../../🚚️request/🟦️.ts";
import fixture from "../🧫️fixtures/🚚️request/🔣️.json";

it("resolves preview and export asset transport without changing the shot", async () => {
  for (const sample of fixture.cases) {
    const received: IconRenderRequest[] = [];
    const restore = configureHostPorts({ iconRender: { render: async (request) => { received.push(request); return { dataUrl: "data:image/svg+xml,%3Csvg/%3E" }; } } });
    const request: IconRenderRequest = { assetUrl: sample.url, camera: { position: [0, -10, 0], target: [0, 0, 0], zoom: 1 }, lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" }, width: 37, height: 19, format: "svg", shape: "ellipse" };
    try {
      await renderIconRequest(request);
      expect(received).toEqual([{ ...request, assetUrl: sample.transport }]);
      expect(new URL(received[0]!.assetUrl, "https://host.example").href).toBe(new URL(sample.transport, "https://host.example").href);
      expect(request.assetUrl).toBe(sample.url);
    } finally { restore(); }
  }
});

it("admits drawing controls through the shared preview and export request using the schema", async () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  const delivered: IconRenderRequest[] = [];
  const restore = configureHostPorts({ iconRender: { render: async (request) => {
    delivered.push(request);
    return { dataUrl: "data:image/svg+xml,%3Csvg/%3E" };
  } } });
  try {
    for (const entry of fixture.drawingCases) {
      const request: IconRenderRequest = {
        assetUrl: fixture.cases[0]!.url, format: "svg", width: 37, height: 19,
        camera: { position: [0, -10, 0], target: [0, 0, 0], zoom: 1 },
        lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" },
        drawing: entry.drawing as IconRenderRequest["drawing"],
      };
      expect(validate(request)).toBe(entry.accept);
      if (entry.accept) await expect(renderIconRequest(request)).resolves.toHaveProperty("dataUrl");
      else await expect(renderIconRequest(request)).rejects.toThrow("export schema");
    }
    expect(delivered).toHaveLength(fixture.drawingCases.filter((entry) => entry.accept).length);
  } finally {
    restore();
  }
});
