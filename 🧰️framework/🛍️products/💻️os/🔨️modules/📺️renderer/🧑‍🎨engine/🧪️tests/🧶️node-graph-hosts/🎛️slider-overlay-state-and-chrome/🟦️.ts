/** 🎛️ The slider overlay state parsing and track-only chrome zoom tests. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { expect, it } from "vitest";
import { parseDagSliderOverlays, GraphSliderOverlays } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔗 Registers the slider overlay state json, track-only overlay and zoom scaled chrome tests. */
export function registerSliderOverlayStateAndChromeTests(): void {
  it("parses slider overlay state json for flow graph hosts", () => {
    const sliders = parseDagSliderOverlays(
      JSON.stringify({
        camera: { x: 0, y: 0, zoom: 1 },
        sliders: [
          {
            widgetId: "slider_2",
            label: "Radius",
            value: 2.2,
            min: 0,
            max: 10,
            step: 0.1,
            x: 100,
            y: 50,
            w: 120,
            h: 8,
          },
        ],
      }),
    );
    expect(sliders).toHaveLength(1);
    expect(sliders[0]?.widgetId).toBe("slider_2");
    expect(sliders[0]?.value).toBe(2.2);
  });
  it("renders graph slider overlays as track-only controls without a nested value readout", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "track-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 1 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-slot="slider"');
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).not.toContain('data-slot="slider-value"');
    expect(markup).not.toContain('data-slot="slider-row"');
  });
  it("scales graph slider overlay chrome with canvas zoom so the knob matches other elements", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "zoom-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 2 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-graph-slider-zoom="2"');
    expect(markup).toContain("translate(-50%, -50%) scale(2)");
    expect(markup).toContain("width:120px");
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).toContain("size-tiny");
    expect(markup).not.toMatch(/data-slot="slider-thumb"[^>]*size-small/);
  });
}
