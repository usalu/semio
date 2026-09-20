// #region 🧲️Header
// 🧪️ 🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️mode/🧪️.story.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Mode, createEvenWindowLayout, reactHostPort, uiDataLabel, type WindowLayoutAxisNode, type WindowLayoutStackNode } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import dockAxisGeometry from "../../🧫️fixtures/📐️dock-axis-geometry/🔣️.json";
import { contentThroughGlassPlay, maximizeStackPlay, quadLayoutPlay } from "../../🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/🟦️.ts";
// #endregion 🔌️Adapters

// `Mode` mounts each `windows[].children` inside a `mode-dock-stack-body` (level="base") wrapping
// a `<Window>` (level="window"), both of which already fill `ui-surface`/`ui-glass` — this stays
// bg-transparent so it doesn't double-tint either ancestor fill.
const Pane = ({ label }: { label: string }) => (
  <div className="flex h-full items-center justify-center bg-transparent">
    <span className="text-lg font-semibold">{label}</span>
  </div>
);

type GeometryNode = { kind: "stack"; id: string } | { kind: "row" | "column"; children: Array<{ weight: number; node: GeometryNode }> };

function geometryModeNode(node: GeometryNode, size?: number): WindowLayoutAxisNode | WindowLayoutStackNode {
  if (node.kind === "stack") return { kind: "stack", size, activeId: node.id, children: [{ kind: "window", id: node.id }] };
  return { kind: node.kind, size, children: node.children.map((child) => geometryModeNode(child.node, child.weight)) };
}

// #region 🧪️SilhouetteVisualFixture
const SilhouetteVisualFixture = () => (
  <div data-testid="silhouette-visual-content" data-window-content-layout="edgeless" aria-label="Continuous content / Fortlaufender Inhalt" className="relative h-full min-h-0 overflow-hidden bg-transparent">
    <div
      aria-hidden
      className="pointer-events-none absolute inset-x-0 bottom-0"
      style={{
        top: -48,
        backgroundColor: "#0d2742",
        backgroundImage:
          "linear-gradient(90deg, transparent 0 47px, #ffd43b 47px 55px, transparent 55px 103px, #52e5ff 103px 111px, transparent 111px), linear-gradient(0deg, rgb(255 255 255 / 12%) 1px, transparent 1px)",
        backgroundSize: "160px 100%, 100% 24px",
      }}
    />
    <div aria-hidden className="pointer-events-none absolute bottom-0 w-2 bg-[#ff4d6d]" style={{ insetInlineStart: 23, top: -48 }} />
    <div data-testid="silhouette-visual-label" className="absolute text-sm font-semibold text-white" style={{ insetInlineStart: 72, top: 16 }}>
      0123456789 · Text · Szene · Scene
    </div>
  </div>
);
// #endregion 🧪️SilhouetteVisualFixture

const meta = {
  title: "🖱️ui⚛️react/Mode",
  component: Mode,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { windows: [], activeWindowId: null },
} satisfies Meta<typeof Mode>;

export default meta;

type Story = StoryObj<typeof meta>;

export const QuadLayout: Story = {
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("overview");
    return (
      <div className="h-[500px] w-full p-single">
        <Mode
          windows={[
            { id: "overview", title: uiDataLabel("Overview"), iconId: "app-window", children: <Pane label="Overview" /> },
            { id: "detail", title: uiDataLabel("Detail"), iconId: "app-window", children: <Pane label="Detail" /> },
            { id: "selection", title: uiDataLabel("Selection"), iconId: "app-window", children: <Pane label="Selection" /> },
            { id: "context", title: uiDataLabel("Context"), iconId: "app-window", children: <Pane label="Context" /> },
          ]}
          layout={{
            kind: "row",
            children: [
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "overview" }], activeId: "overview" },
                  { kind: "stack", children: [{ kind: "window", id: "detail" }], activeId: "detail" },
                ],
              },
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "selection" }], activeId: "selection" },
                  { kind: "stack", children: [{ kind: "window", id: "context" }], activeId: "context" },
                ],
              },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
  play: quadLayoutPlay,
};

export const TabStack: Story = {
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("design");
    return (
      <div className="h-[400px] w-full p-single">
        <Mode
          windows={[
            { id: "design", title: uiDataLabel("Design"), iconId: "app-window", children: <Pane label="Design Pane" /> },
            { id: "review", title: uiDataLabel("Review"), iconId: "app-window", children: <Pane label="Review Pane" /> },
            { id: "notes", title: uiDataLabel("Notes"), iconId: "app-window", children: <Pane label="Notes Pane" /> },
          ]}
          layout={{
            kind: "stack",
            activeId: "design",
            children: [
              { kind: "window", id: "design" },
              { kind: "window", id: "review" },
              { kind: "window", id: "notes" },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
};

export const MaximizeStack: Story = {
  render: () => (
    <div className="h-[400px] w-full p-single">
      <Mode
        windows={[
          { id: "a", title: uiDataLabel("Alpha"), iconId: "app-window", children: <Pane label="Alpha" /> },
          { id: "b", title: uiDataLabel("Beta"), iconId: "app-window", children: <Pane label="Beta" /> },
        ]}
        layout={{
          kind: "row",
          children: [
            { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
            { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
          ],
        }}
        activeWindowId="a"
      />
    </div>
  ),
  play: maximizeStackPlay,
};

export const EvenSplit: Story = {
  render: () => (
    <div className="h-[400px] w-full p-single">
      <Mode
        windows={[
          { id: "a", title: uiDataLabel("A"), iconId: "app-window", children: <Pane label="A" /> },
          { id: "b", title: uiDataLabel("B"), iconId: "app-window", children: <Pane label="B" /> },
        ]}
        layout={createEvenWindowLayout(["a", "b"])}
        activeWindowId="a"
      />
    </div>
  ),
};

export const GeometryOracle: Story = {
  render: () => {
    const { viewport, layout } = dockAxisGeometry;
    return (
      <div data-testid="dock-axis-geometry-oracle" style={{ width: viewport.width, height: viewport.height }}>
        <Mode
          windows={[
            { id: "left", title: uiDataLabel("Left / Links"), iconId: "app-window", children: <Pane label="Left / Links" /> },
            { id: "right-top", title: uiDataLabel("Top / Oben"), iconId: "app-window", children: <Pane label="Top / Oben" /> },
            { id: "right-bottom", title: uiDataLabel("Bottom / Unten"), iconId: "app-window", children: <Pane label="Bottom / Unten" /> },
          ]}
          layout={geometryModeNode(layout as GeometryNode)}
          activeWindowId="left"
        />
      </div>
    );
  },
};

export const ContentThroughGlass: Story = {
  args: { windows: [], activeWindowId: "flow" },
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("flow");
    return (
      <div
        data-testid="silhouette-visual-floor"
        className="h-[420px] w-full p-single"
        style={{
          backgroundColor: "#641f45",
          backgroundImage: "linear-gradient(135deg, rgb(255 255 255 / 10%) 25%, transparent 25% 50%, rgb(255 255 255 / 10%) 50% 75%, transparent 75%)",
          backgroundSize: "32px 32px",
        }}
      >
        <Mode
          windows={[
            { id: "flow", title: uiDataLabel("Flow / Fluss"), iconId: "flow", children: <SilhouetteVisualFixture /> },
            { id: "reference", title: uiDataLabel("Reference / Referenz"), iconId: "app-window", children: <Pane label="Reference / Referenz" /> },
          ]}
          layout={{
            kind: "stack",
            activeId: "flow",
            children: [
              { kind: "window", id: "flow" },
              { kind: "window", id: "reference" },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
  play: contentThroughGlassPlay,
};
