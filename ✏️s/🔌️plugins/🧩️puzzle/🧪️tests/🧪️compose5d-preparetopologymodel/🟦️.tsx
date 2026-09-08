type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { PUZZLE_5D_TOPOLOGY_ICON_WIDTH, compose5d, prepareTopologyModel } = dependencies;

  const { describe, it, expect } = vitest;

  describe("compose5d + prepareTopologyModel", () => {
    it("flattens a fixed root and derived child with fastener x/y", () => {
      const flat = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          {
            id: "root",
            anchor: "fixed",
            x: 1,
            y: 2,
            handles: [{ id: "root:conn-a", handleKind: "compose.connector", angle: 0, t: 0 }],
          },
          { id: "child", anchor: "derived", x: 0, y: 0, handles: [{ id: "child:conn-a", handleKind: "compose.connector", angle: 0, t: 0 }] },
        ],
        edges: [
          {
            id: "link-1",
            source: "root:conn-a",
            target: "child:conn-a",
            x: 0.5,
            y: 0.25,
            gap: 0,
            shift: 0,
            rise: 0,
            rotation: 0,
            turn: 0,
            tilt: 0,
          },
        ],
      };
      const volume = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [8, 8, 8], target: [0, 0, 0], zoom: 1 },
        objects: [
          {
            id: "root",
            anchor: "fixed",
            origin: [1, 2, 3],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "root:conn-a", position: [0, 0, 0], direction: [0, 0, 1], vortexKind: "compose.connector" }],
          },
          {
            id: "child",
            anchor: "derived",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "child:conn-a", position: [0, 0, 0], direction: [0, 0, 1], vortexKind: "compose.connector" }],
          },
        ],
        attractions: [
          {
            id: "link-1",
            attracting: "root:conn-a",
            attracted: "child:conn-a",
            x: 0.5,
            y: 0.25,
          },
        ],
      };
      const prepared = prepareTopologyModel(compose5d(flat, volume));
      const child = prepared.parts.find((part) => part.id === "child");
      expect(child?.["3d"]?.origin).toBeDefined();
      expect((child?.["3d"]?.origin as number[])[0]).toBeCloseTo(1, 2);
      expect(child?.["2d"]?.x).toBeCloseTo(1.5 * PUZZLE_5D_TOPOLOGY_ICON_WIDTH, 2);
    });
  });

}
