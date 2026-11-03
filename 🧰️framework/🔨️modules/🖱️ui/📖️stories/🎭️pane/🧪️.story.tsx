import { Pane, PaneHost } from "@semio-tech/ui/elements/WindowPane";
import type { Anchor } from "@semio-tech/ui/modules/panel-anchor";
import { useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/Pane",
  component: Pane,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof Pane>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ initiallyFolded = true, expanded = false }: { initiallyFolded?: boolean; expanded?: boolean }) {
  const title = useLabel("ui.common.example");
  const content = useLabel("ui.common.noData");
  const reset = useLabel("ui.common.reset");
  const [folded, setFolded] = useState(initiallyFolded);
  const [anchor, setAnchor] = useState<Anchor>("top-left");
  const [size, setSize] = useState(300);
  return <div className="flex flex-col gap-double">
    <div className="h-96 border"><PaneHost>
      <Pane id="story.pane" icon="settings" label={title} anchor={anchor} onAnchorChange={setAnchor} folded={folded} onFoldToggle={() => setFolded((value) => !value)} size={size} onSizeChange={setSize} expanded={expanded}>
        <p>{content}</p>
      </Pane>
    </PaneHost></div>
    <button type="button" onClick={() => { setFolded(initiallyFolded); setAnchor("top-left"); setSize(300); }}>{reset}</button>
  </div>;
}

export const Default: Story = { args: { id: "story.pane", icon: "settings", anchor: "top-left" }, render: () => <Demo /> };
export const Open: Story = { args: { id: "story.pane", icon: "settings", anchor: "top-left" }, render: () => <Demo initiallyFolded={false} /> };
export const Expanded: Story = { args: { id: "story.pane", icon: "settings", anchor: "top-left" }, render: () => <Demo initiallyFolded={false} expanded /> };
