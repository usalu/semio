import { WindowMeasuresTree, WindowMeasureTreeGroup, WindowMeasureTreeLeaf } from "@semio-tech/ui/elements/Tree/window-measures";
import { useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";
import { expect, userEvent, within } from "storybook/test";

const meta = {
  title: "🖱️ui⚛️react/WindowMeasuresTree",
  component: WindowMeasuresTree,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof WindowMeasuresTree>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ pending = false, direction = "down" }: { pending?: boolean; direction?: "up" | "down" }) {
  const title = useLabel("ui.common.example");
  const loading = useLabel("ui.common.loading");
  const noData = useLabel("ui.common.noData");
  const reset = useLabel("ui.common.reset");
  const [value, setValue] = useState(42);
  return <section className="max-w-md" aria-label={title}>
    <WindowMeasuresTree direction={direction}>
      <WindowMeasureTreeGroup id={`story-measures-${direction}-${pending}`} label={title} defaultOpen>
        <WindowMeasureTreeLeaf label={title}>
          <input type="number" aria-label={title} className="w-full" value={value} onChange={(event) => setValue(event.currentTarget.valueAsNumber || 0)} />
        </WindowMeasureTreeLeaf>
        <WindowMeasureTreeLeaf label={pending ? loading : noData} loading={pending}><output aria-live="polite">{pending ? loading : value}</output></WindowMeasureTreeLeaf>
        <WindowMeasureTreeLeaf label={noData} waiting={pending}><output>{noData}</output></WindowMeasureTreeLeaf>
        <WindowMeasureTreeLeaf fullWidth><button type="button" onClick={() => setValue(42)}>{reset}</button></WindowMeasureTreeLeaf>
      </WindowMeasureTreeGroup>
    </WindowMeasuresTree>
  </section>;
}

export const Default: Story = {
  args: { children: null }, render: () => <Demo />,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const toggle = canvas.getByRole("button", { expanded: true });
    expect(toggle).toHaveAccessibleName();
    await userEvent.click(toggle);
    expect(canvas.getByRole("button", { expanded: false })).toBe(toggle);
    expect(canvas.queryByRole("spinbutton")).toBeNull();
    await userEvent.click(toggle);
    expect(canvas.getByRole("spinbutton")).toHaveValue(42);
  },
};
export const LoadingAndWaiting: Story = { args: { children: null }, render: () => <Demo pending /> };
export const Upward: Story = { args: { children: null }, render: () => <Demo direction="up" /> };
