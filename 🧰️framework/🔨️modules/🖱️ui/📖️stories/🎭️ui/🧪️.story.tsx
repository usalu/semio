// #region 🧲️Header
// 🧪️ 🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️ui/🧪️.story.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { App, Mode, reactHostPort, Ui, uiDataLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
// #endregion 🔌️Adapters

// `Mode` mounts each `windows[].children` inside a `mode-dock-stack-body` (level="base") wrapping
// a `<Window>` (level="window"), both of which already fill `ui-surface`/`ui-glass` — this stays
// bg-transparent so it doesn't double-tint either ancestor fill.
const Pane = ({ title }: { title: string }) => (
  <div className="flex h-full items-center justify-center bg-transparent">
    <h2 className="text-xl font-bold">{title}</h2>
  </div>
);

const meta = {
  title: "🖱️ui⚛️react/Ui",
  component: Ui,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { apps: [], activeAppId: "editor" },
} satisfies Meta<typeof Ui>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeAppId, setActiveAppId] = reactHostPort.useState("editor");
    return (
      <div className="h-[480px] w-full">
        <Ui
          apps={[
            {
              id: "editor",
              label: uiDataLabel("Editor"),
              children: (
                <App
                  modes={[
                    {
                      id: "design",
                      label: uiDataLabel("Design"),
                      children: (
                        <Mode
                          windows={[
                            { id: "scene", iconId: "app-window", children: <Pane title="Scene" /> },
                            { id: "tree", iconId: "app-window", children: <Pane title="Tree" /> },
                          ]}
                          activeWindowId="scene"
                          onActiveWindowChange={() => {}}
                        />
                      ),
                    },
                    {
                      id: "review",
                      label: uiDataLabel("Review"),
                      children: <Mode windows={[{ id: "preview", iconId: "app-window", children: <Pane title="Preview" /> }]} activeWindowId="preview" />,
                    },
                  ]}
                  activeModeId="design"
                  onActiveModeChange={() => {}}
                />
              ),
            },
            {
              id: "dashboard",
              label: uiDataLabel("Dashboard"),
              children: <Mode windows={[{ id: "stats", iconId: "app-window", children: <Pane title="Statistics" /> }]} activeWindowId="stats" />,
            },
          ]}
          activeAppId={activeAppId}
          onActiveAppChange={setActiveAppId}
        />
      </div>
    );
  },
};
