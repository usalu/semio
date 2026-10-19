/** 🧲️ The catalogue drag payload, ghost descriptor and suggestion ranking tests. */
import { expect, it } from "vitest";
import { pendingPanelUiNode } from "@semio-tech/framework";
import { catalogueGhostDescriptorJson, flowCatalogueItemDescriptor, flowRankCatalogueSuggestions, flowSpotlightSuggestionListScrollClass, parseCatalogueAppDragPayload, uiNodeToTreePanelConfig } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🔎 Registers the catalogue drag payload, ghost neuron, addWidget descriptor, suggestion ranking and spotlight overflow tests. */
export function registerCatalogueDragAndSuggestionTests(): void {
  it("parses a catalogue app drag payload, ignoring extra keys", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ pluginId: "s.system", appId: "draw", label: "Draw", extra: "x" }))).toEqual({
      pluginId: "s.system",
      appId: "draw",
      label: "Draw",
    });
  });
  it("rejects catalogue app drag payloads missing pluginId/appId, and garbage", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ appId: "draw" }))).toBeNull();
    expect(parseCatalogueAppDragPayload(JSON.stringify({ kind: "neuron" }))).toBeNull();
    expect(parseCatalogueAppDragPayload("not json")).toBeNull();
  });
  it("builds a ghost neuron descriptor, preferring label over appId", () => {
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw", label: "Draw" }))).toEqual({ kind: "neuron", neuronKind: "Draw" });
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw" }))).toEqual({ kind: "neuron", neuronKind: "draw" });
  });
  it("builds addWidget descriptors from catalogue items", () => {
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" }))).toEqual({
      kind: "neuron",
      neuronKind: "math.add",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "outputExport", format: "svg", name: "Export SVG", abbreviation: "SVG", icon: "emoji:📤️", summary: "" }))).toEqual({
      kind: "outputExport",
      format: "svg",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" }))).toEqual({ kind: "inputSlider", label: "Slider" });
  });
  it("ranks catalogue suggestions by exact/prefix match with neurons first", () => {
    const sections = [
      {
        id: "inputs",
        title: "Inputs",
        items: [
          { kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" },
          { kind: "inputNote", name: "Note", abbreviation: "Note", icon: "emoji:📝️", summary: "" },
        ],
      },
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" },
          { kind: "neuron", neuronKind: "math.subtract", name: "Subtract", abbreviation: "Sub", icon: "emoji:➖️", summary: "" },
        ],
      },
    ];
    expect(flowRankCatalogueSuggestions(sections, "add").map((item) => item.neuronKind ?? item.kind)).toEqual(["math.add"]);
    expect(flowRankCatalogueSuggestions(sections, "sl").map((item) => item.kind)).toEqual(["inputSlider"]);
    const brepSections = [
      {
        id: "brep",
        title: "Brep",
        items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Axis-aligned box" }],
      },
    ];
    expect(flowRankCatalogueSuggestions(brepSections, "brep").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    expect(flowRankCatalogueSuggestions(brepSections, "box").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    const empty = flowRankCatalogueSuggestions(sections, "");
    expect(empty[0]?.kind).toBe("neuron");
    expect(empty.some((item) => item.kind === "inputSlider")).toBe(true);
  });
  it("returns every catalogue item for empty query without a 20-item cap", () => {
    const items = Array.from({ length: 25 }, (_, index) => ({
      kind: "neuron",
      neuronKind: `math.operation${index}`,
      name: `Operation ${index}`,
      abbreviation: `Operation${index}`,
      icon: "emoji:➕️",
      summary: "",
    }));
    const sections = [{ id: "math", title: "Math", items }];
    expect(flowRankCatalogueSuggestions(sections, "")).toHaveLength(25);
  });
  it("enables overflow scrolling only when spotlight suggestions are expanded", () => {
    expect(flowSpotlightSuggestionListScrollClass(false)).toContain("overflow-hidden");
    expect(flowSpotlightSuggestionListScrollClass(false)).not.toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("max-h-[min(24rem,70vh)]");
  });
  it("hosts semantic panel bodies full-width via tree emptyState, not a property-layout control wrapper", () => {
    const config = uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection");
    expect(config.sections).toEqual([]);
    expect(config.emptyState).toBeTruthy();
    expect(config.className).toContain("w-full");
  });
}
