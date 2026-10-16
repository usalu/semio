/** 🖊 Dag marquee overlay and ink canvas host. */
import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { SelectionMarquee } from "@semio-tech/ui-react";
import { computeDagMarqueeOverlay, InkCanvasHost, inkItemBounds, eraseInkStrokePointsInItem, inkParagraphsToHtml, inkResizeBounds, inkScaleItemWithinGroup, inkClipboardPayload, inkItemsFromClipboardPayload, screenToWorld, worldToScreen, type InkDocument, type InkStrokeItem } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("dag marquee overlay", () => {
  it("computes a rect overlay with numeric bounds for the rectangle method", () => {
    const pointsJson = JSON.stringify([
      { x: 10, y: 20 },
      { x: 30, y: 50 },
    ]);
    const overlay = computeDagMarqueeOverlay(pointsJson, false, "rectangle");
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "full" });
  });

  // Regression: board rust publishes `[[x,y],…]` (not `{x,y}` objects). Object-only parsing yielded
  // NaN bounds so the live select marquee never painted even though selection itself worked.
  it("computes a rect overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
      "rectangle",
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("computes a lasso overlay carrying the raw points for the lasso method", () => {
    const points = [
      { x: 10, y: 20 },
      { x: 30, y: 50 },
      { x: 15, y: 40 },
    ];
    const overlay = computeDagMarqueeOverlay(JSON.stringify(points), true, "lasso");
    expect(overlay).toEqual({ kind: "lasso", points, coverage: "partial" });
  });

  it("computes a lasso overlay from the rust tuple-array wire format", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
      "lasso",
    );
    expect(overlay).toEqual({
      kind: "lasso",
      points: [
        { x: 10, y: 20 },
        { x: 30, y: 50 },
        { x: 15, y: 40 },
      ],
      coverage: "full",
    });
  });

  it("infers lasso from a non-rectangular path when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 50],
        [15, 40],
      ]),
      false,
    );
    expect(overlay?.kind).toBe("lasso");
  });

  it("infers rectangle from four axis-aligned corner points when method is omitted", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [10, 20],
        [30, 20],
        [30, 50],
        [10, 50],
      ]),
      true,
    );
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "partial" });
  });

  it("returns null for fewer than two points", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([[0, 0]]), false, "rectangle")).toBeNull();
  });

  it("returns null for malformed point entries", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([[10], [20, 30]]), false, "rectangle")).toBeNull();
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 10 }, { x: 20, y: 30 }]), false, "rectangle")).toBeNull();
  });

  // Regression: node-graph-host.tsx used to pass `shape={{ shape: "polygon", points }}` (a single
  // nested-object prop) instead of separate `shape`/`points` props, so `props.shape === "rect"` was
  // always false and the polygon branch read `props.points` as undefined — crashing on every marquee
  // drag and tripping the shell's render error boundary (visible as an interaction "reset").
  it("renders a rect overlay from a computeDagMarqueeOverlay rect result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
      ]),
      false,
      "rectangle",
    );
    if (!overlay || overlay.kind !== "rect") throw new Error("expected rect overlay");
    const markup = renderToStaticMarkup(
      createElement(SelectionMarquee, {
        coverage: overlay.coverage ?? "full",
        shape: "rect",
        rect: { x: overlay.x ?? 0, y: overlay.y ?? 0, width: overlay.width ?? 0, height: overlay.height ?? 0 },
      }),
    );
    expect(markup).toContain("<rect");
    expect(markup).toContain('width="40"');
    expect(markup).toContain('height="25"');
  });

  it("renders a polygon overlay from a computeDagMarqueeOverlay lasso result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        [0, 0],
        [40, 25],
        [5, 30],
      ]),
      false,
      "lasso",
    );
    if (!overlay || overlay.kind !== "lasso") throw new Error("expected lasso overlay");
    const markup = renderToStaticMarkup(createElement(SelectionMarquee, { coverage: overlay.coverage ?? "full", shape: "polygon", points: overlay.points ?? [] }));
    expect(markup).toContain("<polygon");
    expect(markup).toContain("0,0 40,25 5,30");
  });
});

describe("ink canvas host", () => {
  const semioInkDocument: InkDocument = {
    schema: "ink.document",
    id: "semio",
    title: "Semio Note",
    camera: { x: 0, y: 0, zoom: 1 },
    gridVisible: true,
    snapEnabled: false,
    pencilWidth: 3,
    eraserRadius: 12,
    blocks: [
      {
        kind: "text",
        id: "welcome-text",
        name: "Welcome",
        x: 80,
        y: 80,
        width: 360,
        height: 120,
        visible: true,
        locked: false,
        paragraphs: [{ runs: [{ text: "Welcome to Note — an infinite canvas for text, images, tables, math, and pencil ink." }] }],
        fontSize: 20,
        fontWeight: "normal",
        align: "left",
      },
      { kind: "math", id: "welcome-math", name: "Equation", x: 80, y: 240, width: 240, height: 80, visible: true, locked: false, tex: "E = mc^2", displayMode: true },
      {
        kind: "table",
        id: "welcome-table",
        name: "Blocks",
        x: 80,
        y: 360,
        width: 360,
        height: 140,
        visible: true,
        locked: false,
        columns: ["Block", "Description"],
        rows: [
          [{ content: "Text" }, { content: "Rich text blocks" }],
          [{ content: "Math" }, { content: "TeX equations" }],
          [{ content: "Ink" }, { content: "Freehand pencil strokes" }],
        ],
      },
    ],
  };

  it("renders the semio example composite scene with rich text, table, and math fallback", () => {
    const markup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: {
          type: "componentScene",
          surfaceId: "note.play.composite",
          controllerId: "note-play",
          componentKind: "ink-canvas",
          inkCanvas: {
            documentJson: JSON.stringify(semioInkDocument),
            selectionJson: "[]",
            activeUtility: "selectDirect",
            viewMode: "composite",
            interactive: true,
          },
        },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(markup).toContain("Welcome to Note");
    expect(markup).toContain("<table");
    expect(markup).toMatch(/\$\$E = mc\^2\$\$|annotation encoding="application\/x-tex">E = mc\^2</);
    expect(markup).toContain('data-surface-id="note.play.composite"');
  });

  it("shows the grid pattern in composite mode but not in navigator mode", () => {
    const baseNode = {
      type: "componentScene" as const,
      surfaceId: "note.play.composite",
      controllerId: "note-play",
      componentKind: "ink-canvas",
    };
    const compositeMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "composite", interactive: true } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(compositeMarkup).toContain("ink-viewport-grid");

    const navigatorMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "navigator", interactive: false } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(navigatorMarkup).not.toContain("ink-viewport-grid");
  });

  it("resizes with a minimum size and scales ink points when a group is resized", () => {
    const fromBounds = { x: 0, y: 0, width: 100, height: 100 };
    const shrunk = inkResizeBounds(fromBounds, "e", -1000, 0);
    expect(shrunk.width).toBe(8);

    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [100, 100],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const scaled = inkScaleItemWithinGroup(ink, { x: 0, y: 0, width: 100, height: 100 }, { x: 0, y: 0, width: 200, height: 50 });
    expect(scaled.kind).toBe("stroke");
    if (scaled.kind === "stroke")
      expect(scaled.points).toEqual([
        [0, 0],
        [200, 50],
      ]);
  });

  it("splits an ink stroke into fragments when erasing its middle point", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 80,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [40, 0],
        [80, 0],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const fragments = eraseInkStrokePointsInItem(ink, 40, 0, 5);
    expect(fragments).toHaveLength(0);
    const wideStroke: InkStrokeItem = {
      ...ink,
      points: [
        [0, 0],
        [10, 0],
        [40, 0],
        [70, 0],
        [80, 0],
      ],
    };
    const splitFragments = eraseInkStrokePointsInItem(wideStroke, 40, 0, 5);
    expect(splitFragments).toHaveLength(2);
  });

  it("round-trips bold and link marks between paragraphs and html", () => {
    const html = inkParagraphsToHtml([{ runs: [{ text: "hello", bold: true, link: "https://semio.tech" }] }]);
    expect(html).toContain("<strong>");
    expect(html).toContain('href="https://semio.tech"');
  });

  it("round-trips a clipboard payload of note blocks", () => {
    const payload = inkClipboardPayload([semioInkDocument.blocks[1]!]);
    const parsed = inkItemsFromClipboardPayload(payload);
    expect(parsed).toHaveLength(1);
    expect(parsed?.[0]?.kind).toBe("math");
  });

  it("computes ink block bounds from its local points", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 10,
      y: 10,
      width: 1,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [5, 5],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    expect(inkItemBounds(ink)).toEqual({ x: 10, y: 10, width: 5, height: 5 });
  });

  it("applies the canonical wheel-zoom camera formula symmetrically for screen<->world conversion", () => {
    const camera = { x: 50, y: 50, zoom: 2 };
    const world = screenToWorld(camera, 150, 150);
    expect(world).toEqual([50, 50]);
    expect(worldToScreen(camera, 50, 50)).toEqual({ x: 150, y: 150 });
  });
});
