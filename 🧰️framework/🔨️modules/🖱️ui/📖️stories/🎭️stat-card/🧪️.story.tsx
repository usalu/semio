import { StatCard, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/StatCard",
  component: StatCard,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof StatCard>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo(args: React.ComponentProps<typeof StatCard>) {
  const label = useLabel("ui.panel.artifact");
  return <StatCard {...args} label={label} />;
}

export const Default: Story = { args: { label: "", value: "128" }, render: (args) => <Demo {...args} /> };
export const PositiveDelta: Story = { ...Default, args: { label: "", value: "128", delta: { value: "+12", tone: "success" }, icon: <span aria-hidden>↑</span> } };
export const NegativeDelta: Story = { ...Default, args: { label: "", value: "96", delta: { value: "−20", tone: "danger" } } };
