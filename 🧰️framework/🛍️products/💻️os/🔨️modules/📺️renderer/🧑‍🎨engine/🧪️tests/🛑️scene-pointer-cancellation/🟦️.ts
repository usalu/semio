import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import boardSource from "../../🧱️elements/🖥️Board2dHost/🟦️.tsx?raw";
import inkSource from "../../🧱️elements/🖋️InkCanvasHost/🟦️.tsx?raw";
import nodeGraphSource from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx?raw";
import paintSource from "../../🧱️elements/🖌️Paint2dHost/🟦️.tsx?raw";
import textEditorSource from "../../🧱️elements/✏️TextEditor/🟦️.tsx?raw";
import tiledMapSource from "../../🧱️elements/🧭️TiledMapHost/🟦️.tsx?raw";
import engineCanvasSource from "../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs?raw";
import scenesSource from "../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs?raw";
import interpreterSource from "../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs?raw";
import shellSource from "../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs?raw";
import rendererSource from "../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs?raw";
import winitSource from "../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs?raw";
import fixture from "../../🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json";
import schema from "../../🧬️schema/🛑️scene-pointer-cancellation/🔣️.json";

type Owner = Readonly<{ surfaceId: string; generation: number; pointerId: number }>;

const sameOwner = (left: Owner, right: Owner): boolean => left.surfaceId === right.surfaceId && left.generation === right.generation && left.pointerId === right.pointerId;
const cancelOwner = (active: Owner | null, signal: Owner): Owner | null => (active && sameOwner(active, signal) ? null : active);

describe("scene pointer cancellation contract", () => {
  it("validates the six-family language-neutral contract with the independent JSON Schema oracle", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, cases: fixture.cases.slice(1) })).toBe(false);
    expect(validate({ ...fixture, cases: fixture.cases.map((entry) => (entry.family === "ink" ? { ...entry, reactStatus: "invented" } : entry)) })).toBe(false);
  });

  it("pins discard versus TiledMap navigation commit without inventing React support", () => {
    expect(fixture.cases.map(({ family }) => family)).toEqual(["nodeGraph", "board2d", "ink", "tiledMap", "paint2d", "textEditor"]);
    const tiledMap = fixture.cases.find(({ family }) => family === "tiledMap")!;
    expect(tiledMap).toMatchObject({ reactStatus: "reference", terminal: "commit-navigation", invokeNormalPointerUp: true, publishOnCancel: ["camera"] });
    for (const entry of fixture.cases.filter(({ family }) => family !== "tiledMap")) {
      expect(entry.terminal, entry.family).toBe("discard");
      expect(entry.invokeNormalPointerUp, entry.family).toBe(false);
      expect(entry.publishOnCancel, entry.family).toEqual([]);
      expect(entry.preservePublished, entry.family).toEqual(expect.arrayContaining(["selection", "camera"]));
    }
  });

  it("pins TiledMap cancel and hover retirement to one exact retained owner", () => {
    expect(new Set(Object.values(fixture.tiledMap.owner)).size).toBe(3);
    expect(fixture.tiledMap.cancel).toEqual([
      expect.objectContaining({ mode: "pan", button: 1, cameraPublications: 1, selectionPublications: 0, clearsDrag: true }),
      expect.objectContaining({ mode: "marquee", button: 0, cameraPublications: 0, selectionPublications: 0, clearsDrag: true }),
      expect.objectContaining({ mode: "none", button: 0, cameraPublications: 0, selectionPublications: 0, clearsDrag: false }),
    ]);
    for (const row of fixture.tiledMap.cancel) {
      expect(row).toMatchObject({ nextDown: "accepted", duplicateCancel: "inert", outsideUp: "inert" });
    }
    expect(fixture.tiledMap.hoverLeave).toEqual({ prior: "position:position.upper", emptyTargets: "[]", publications: 1, repeatPublications: 0, peerPublications: 0, gestureFanout: [] });
  });

  it("addresses cancellation by surface, generation, and pointer before admitting the next generation", () => {
    const active = { surfaceId: "pane.scene", generation: 41, pointerId: 7 };
    expect(cancelOwner(active, { ...active, surfaceId: "pane.sibling" })).toEqual(active);
    expect(cancelOwner(active, { ...active, generation: 40 })).toEqual(active);
    expect(cancelOwner(active, { ...active, pointerId: 8 })).toEqual(active);
    expect(cancelOwner(active, active)).toBeNull();
    const replacement = { ...active, generation: 42, pointerId: 9 };
    expect(cancelOwner(replacement, active)).toEqual(replacement);
    expect(fixture.owner).toEqual({
      identity: ["surfaceId", "generation", "pointerId"],
      match: "all-fields",
      staleGeneration: "ignore",
      zeroGeneration: "invalid",
      replacement: "cancel-old-before-reuse",
      afterCancel: "next-down-accepted",
    });
  });

  it("records the current React producer boundary exactly", () => {
    const explicit = (source: string): boolean => source.includes("const onPointerCancel") || source.includes("onPointerCancel={");
    const graphPointerSurface = nodeGraphSource.slice(nodeGraphSource.indexOf('className="absolute inset-0 z-30 touch-none"'), nodeGraphSource.indexOf("onWheel=", nodeGraphSource.indexOf('className="absolute inset-0 z-30 touch-none"')));
    const actual = {
      nodeGraph: explicit(graphPointerSurface) ? "reference" : "missing",
      board2d: explicit(boardSource) ? "reference" : "missing",
      ink: explicit(inkSource) ? "reference" : "missing",
      tiledMap: explicit(tiledMapSource) ? "reference" : "missing",
      paint2d: explicit(paintSource) ? "reference" : "missing",
      textEditor: explicit(textEditorSource) ? "reference" : "missing",
    };
    expect(actual).toEqual(Object.fromEntries(fixture.cases.map(({ family, reactStatus }) => [family, reactStatus])));
    expect(boardSource.match(/const onPointerCancel[\s\S]*?\n    };/)?.[0]).not.toContain("pointerUpScreen");
    expect(tiledMapSource.match(/const onPointerCancel[\s\S]*?\n    };/)?.[0]).toContain("session.pointerUpScreen");
  });

  it("preserves the physical pointer and surface generation through the native cancellation owner", () => {
    expect(winitSource).toContain("DispatchEvent::PointerCancel { pointer } => app.handle_pointer_cancel(pointer.id)");
    expect(rendererSource).toContain("handle_pointer_cancel(&mut self, pointer_id: ui_render::PointerId)");
    expect(rendererSource).toContain("claim_scene_pointer_owner(target.clone(), pointer_id)");
    expect(interpreterSource).toContain("surface_generation: u64");
    expect(interpreterSource).toContain("pointer_id: Option<ui_render::PointerId>");
    expect(interpreterSource).toContain("engine.surface_generation(&target.window_id) != Some(target.window_generation)");
    expect(interpreterSource).toContain("job.matches_pointer_owner(slot.surface_generation, pointer_id)");
    expect(shellSource).toContain("handle_pointer_cancel_for(&mut self, pointer_id");
    expect(shellSource).toContain("SurfaceKind::TiledMap");
    const cancelRoute = shellSource.slice(shellSource.indexOf("pub fn handle_pointer_cancel_for"), shellSource.indexOf("crate::scenes::cancel_canvas_interactions", shellSource.indexOf("pub fn handle_pointer_cancel_for")));
    expect(cancelRoute).toContain("tiled_map_pointer_cancel_into");
    expect(cancelRoute).not.toContain("tiled_map_pointer_up_into");
    expect(scenesSource).toContain("tiled_map_pointer_leave_into");
    expect(engineCanvasSource).toContain("node_graph_pointer_cancel_into");
    expect(engineCanvasSource).toContain("puzzle_board_pointer_cancel_into");
    expect(engineCanvasSource).toContain("paint2d_pointer_cancel_into");
    expect(engineCanvasSource).toContain("text_editor_pointer_cancel_into");
  });
});
