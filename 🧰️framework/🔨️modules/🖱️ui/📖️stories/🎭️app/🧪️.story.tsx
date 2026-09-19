// #region 🧲️Header
// 🧪️ 🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️app/🧪️.story.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { App, Mode, reactHostPort, uiDataLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
// #endregion 🔌️Adapters

const meta = {
  title: "🖱️ui⚛️react/App",
  component: App,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { modes: [], activeModeId: "design" },
} satisfies Meta<typeof App>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeModeId, setActiveModeId] = reactHostPort.useState("design");
    return (
      <div className="h-[400px] w-full">
        <App
          modes={[
            {
              id: "design",
              label: uiDataLabel("Design"),
              children: (
                <Mode
                  windows={[
                    { id: "left", iconId: "app-window", children: <div className="flex h-full items-center justify-center">Design Left</div> },
                    { id: "right", iconId: "app-window", children: <div className="flex h-full items-center justify-center">Design Right</div> },
                  ]}
                  activeWindowId="left"
                />
              ),
            },
            {
              id: "review",
              label: uiDataLabel("Review"),
              children: <Mode windows={[{ id: "preview", iconId: "app-window", children: <div className="flex h-full items-center justify-center">Review Preview</div> }]} activeWindowId="preview" />,
            },
          ]}
          activeModeId={activeModeId}
          onActiveModeChange={setActiveModeId}
        />
      </div>
    );
  },
};
