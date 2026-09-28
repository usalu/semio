import { configureHostPorts, type IconRenderRequest } from "@semio-tech/ui-react";
import { expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import { renderIconRequest } from "../../🚚️request/🟦️.ts";
import fixture from "../../🧫️fixtures/🚚️request/🔣️.json";
import schema from "../../🧬️schema/🚚️request/🔣️.json";

it("validates the neutral icon request transport fixture", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

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
