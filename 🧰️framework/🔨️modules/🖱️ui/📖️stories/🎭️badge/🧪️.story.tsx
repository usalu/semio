import { Badge, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/Badge",
  component: Badge,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
  argTypes: { tone: { control: "select", options: ["neutral", "info", "success", "warning", "danger"] } },
} satisfies Meta<typeof Badge>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo(args: React.ComponentProps<typeof Badge>) {
  const text = useLabel("ui.sync.saved");
  return <Badge {...args} text={text} />;
}

export const Default: Story = { args: { text: "", tone: "neutral" }, render: (args) => <Demo {...args} /> };
export const Tones: Story = {
  args: { text: "" },
  render: (args) => <div className="flex flex-wrap gap-single">{(["neutral", "info", "success", "warning", "danger"] as const).map((tone) => <Demo {...args} key={tone} tone={tone} />)}</div>,
};
export const WithIcon: Story = { ...Default, args: { text: "", tone: "success", icon: <span aria-hidden>✓</span> } };
