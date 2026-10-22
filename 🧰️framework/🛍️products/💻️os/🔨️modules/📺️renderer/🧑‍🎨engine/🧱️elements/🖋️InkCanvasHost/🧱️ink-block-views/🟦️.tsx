/** 🧱️ InkCanvasHost block views: the math renderer port with its KaTeX default and the text, table, image, math and stroke block views. */
import * as React from "../../⚛️react-port/🟦️.ts";
import { cn, useLabel } from "@semio-tech/ui-react";
import type { InkBounds, InkCamera, InkImageAsset, InkItem, InkResizeHandle, InkTextItem, InkTextRun } from "../🧾️ink-model/🟦️.ts";
import { inkItemBounds, inkPositiveMod, worldToScreen } from "../📐️ink-geometry/🟦️.ts";
import { inkImageAssetDataUrl } from "../🌳️ink-document-ops/🟦️.ts";

//#region 🔖️InkCanvasHost
//#region MathRenderer
export interface InkMathRenderer {
  render(tex: string, displayMode: boolean): string;
}

let inkMathRenderer: InkMathRenderer = {
  render(tex: string, displayMode: boolean) {
    return `<span class="ink-math-fallback">${displayMode ? `$$${tex}$$` : `$${tex}$`}</span>`;
  },
};

/** ∑ Sets the active ink math renderer adapter (defaults to a plain-text fallback until KaTeX loads). */
export function setInkMathRenderer(renderer: InkMathRenderer): void {
  inkMathRenderer = renderer;
}

async function ensureKatexMathRenderer(): Promise<void> {
  try {
    const katex = await import("katex");
    await import("katex/dist/katex.min.css");
    setInkMathRenderer({
      render(tex: string, displayMode: boolean) {
        return katex.default.renderToString(tex, { displayMode, throwOnError: false });
      },
    });
  } catch {}
}

if (typeof window !== "undefined") void ensureKatexMathRenderer();

//#endregion MathRenderer
//#region BlockViews
function inkTextRunStyle(run: InkTextRun): React.CSSProperties {
  return { fontWeight: run.bold ? "bold" : undefined, fontStyle: run.italic ? "italic" : undefined, textDecoration: run.underline ? "underline" : undefined };
}

function InkTextRunView({ run }: { readonly run: InkTextRun }) {
  if (run.link) {
    return (
      <a href={run.link} className="text-primary underline" style={inkTextRunStyle(run)} onPointerDown={(event) => event.stopPropagation()}>
        {run.text}
      </a>
    );
  }
  return <span style={inkTextRunStyle(run)}>{run.text}</span>;
}

function InkTextContentView({ block }: { readonly block: InkTextItem }) {
  return (
    <div className="text-foreground h-full w-full overflow-auto p-2 whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
      {block.paragraphs.map((paragraph, paragraphIndex) => (
        <div key={paragraphIndex}>
          {paragraph.runs.map((run, runIndex) => (
            <InkTextRunView key={runIndex} run={run} />
          ))}
        </div>
      ))}
    </div>
  );
}

export function InkItemView({
  block,
  assets,
  selected,
  hovered,
  hidden,
  onPointerDown,
}: {
  readonly block: InkItem;
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly hidden: boolean;
  readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
  const groupLabel = useLabel("ui.host.blockGroup");
  if (!block.visible) return null;
  const bounds = inkItemBounds(block);
  const common = {
    "data-ink-block-id": block.id,
    className: cn("bg-background/90 absolute overflow-hidden rounded border shadow-sm", selected && "ring-primary ring-2", hovered && !selected && "ring-primary/60 ring-1", block.locked && "opacity-70", hidden && "pointer-events-none opacity-0"),
    style: {
      left: bounds.x,
      top: bounds.y,
      width: Math.max(8, bounds.width),
      height: Math.max(8, bounds.height),
      transform: block.rotation ? `rotate(${block.rotation}deg)` : undefined,
    },
    onPointerDown: (event: React.PointerEvent) => onPointerDown(event, block.id),
  };
  if (block.kind === "text")
    return (
      <div {...common}>
        <InkTextContentView block={block} />
      </div>
    );
  if (block.kind === "math") {
    const html = inkMathRenderer.render(block.tex, block.displayMode);
    return (
      <div {...common}>
        <div className="flex h-full w-full items-center justify-center p-2">
          <div className="ink-math" dangerouslySetInnerHTML={{ __html: html }} />
        </div>
      </div>
    );
  }
  if (block.kind === "table") {
    return (
      <div {...common}>
        <table className="h-full w-full border-collapse text-sm">
          <thead>
            <tr>
              {block.columns.map((column) => (
                <th key={column} className="border-border border px-2 py-1 text-left font-medium">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {block.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-border border px-2 py-1 align-top">
                    {cell.content}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }
  if (block.kind === "image") {
    const asset = assets?.[block.imageKey];
    const src = asset ? inkImageAssetDataUrl(asset) : null;
    return (
      <div {...common}>
        {src ? <img src={src} alt={block.name} className="h-full w-full object-contain" draggable={false} /> : <div className="bg-muted text-muted-foreground flex h-full w-full items-center justify-center text-xs">{block.imageKey}</div>}
      </div>
    );
  }
  if (block.kind === "stroke") {
    if (block.points.length < 2) return null;
    const path = block.points.map((point, index) => `${index === 0 ? "M" : "L"} ${block.x + point[0]} ${block.y + point[1]}`).join(" ");
    const [r, g, b, a] = block.color;
    return (
      <svg className="pointer-events-none absolute inset-0 overflow-visible" style={{ width: "100%", height: "100%" }}>
        <path d={path} fill="none" stroke={`rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`} strokeWidth={block.strokeWidth} strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    );
  }
  if (block.kind === "group") {
    return (
      <div {...common}>
        <div className="text-muted-foreground p-1 text-xs">
          {groupLabel} · {block.children.length} children
        </div>
      </div>
    );
  }
  return null;
}

const INK_RESIZE_HANDLES: readonly InkResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const INK_RESIZE_CURSOR: Record<InkResizeHandle, string> = { nw: "nwse-resize", n: "ns-resize", ne: "nesw-resize", e: "ew-resize", se: "nwse-resize", s: "ns-resize", sw: "nesw-resize", w: "ew-resize" };

export function InkSelectionChrome({ camera, bounds, onResizePointerDown }: { readonly camera: InkCamera; readonly bounds: InkBounds; readonly onResizePointerDown: (handle: InkResizeHandle, event: React.PointerEvent) => void }) {
  const topLeft = worldToScreen(camera, bounds.x, bounds.y);
  const width = bounds.width * camera.zoom;
  const height = bounds.height * camera.zoom;
  return (
    <>
      <div className="border-primary pointer-events-none absolute z-20 border" style={{ left: topLeft.x, top: topLeft.y, width, height }} />
      {INK_RESIZE_HANDLES.map((handle) => {
        const left = handle.includes("w") ? topLeft.x - 4 : handle.includes("e") ? topLeft.x + width - 4 : topLeft.x + width / 2 - 4;
        const top = handle.includes("n") ? topLeft.y - 4 : handle.includes("s") ? topLeft.y + height - 4 : topLeft.y + height / 2 - 4;
        return <div key={handle} className="border-primary bg-background absolute z-30 h-2 w-2 rounded-sm border" style={{ left, top, cursor: INK_RESIZE_CURSOR[handle] }} onPointerDown={(event) => onResizePointerDown(handle, event)} />;
      })}
    </>
  );
}

export function InkViewportGrid({ camera, spacing, subdivisions, opacity, color }: { readonly camera: InkCamera; readonly spacing: number; readonly subdivisions: number; readonly opacity: number; readonly color: string }) {
  const majorPx = spacing * camera.zoom;
  const minorPx = majorPx / Math.max(1, subdivisions);
  const offsetX = inkPositiveMod(camera.x, majorPx);
  const offsetY = inkPositiveMod(camera.y, majorPx);
  const patternId = `ink-viewport-grid-${spacing}-${subdivisions}`;
  const minorLines: React.ReactNode[] = [];
  for (let index = 1; index < subdivisions; index += 1) {
    const position = index * minorPx;
    minorLines.push(
      <line key={`v-${index}`} x1={position} y1={0} x2={position} y2={majorPx} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
      <line key={`h-${index}`} x1={0} y1={position} x2={majorPx} y2={position} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
    );
  }
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden>
      <defs>
        <pattern id={patternId} width={majorPx} height={majorPx} patternUnits="userSpaceOnUse" x={offsetX} y={offsetY}>
          {minorLines}
          <path d={`M ${majorPx} 0 L 0 0 0 ${majorPx}`} fill="none" stroke={color} strokeWidth={1} opacity={opacity} />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill={`url(#${patternId})`} />
    </svg>
  );
}
//#endregion BlockViews
//#endregion 🔖️InkCanvasHost
