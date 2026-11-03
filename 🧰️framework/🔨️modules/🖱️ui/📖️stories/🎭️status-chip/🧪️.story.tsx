import { StatusChip, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/StatusChip",
  component: StatusChip,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
  argTypes: { status: { control: "select", options: ["ok", "busy", "error", "offline"] } },
} satisfies Meta<typeof StatusChip>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ status }: React.ComponentProps<typeof StatusChip>) {
  const labels = {
    ok: useLabel("ui.sync.online"),
    busy: useLabel("ui.sync.connecting"),
    error: useLabel("ui.common.error"),
    offline: useLabel("ui.sync.offline"),
  };
  return <StatusChip status={status} label={labels[status]} />;
}

export const Default: Story = { args: { status: "ok", label: "" }, render: (args) => <Demo {...args} /> };
export const States: Story = {
  args: { status: "ok", label: "" },
  render: (args) => <div className="flex flex-wrap gap-double">{(["ok", "busy", "error", "offline"] as const).map((status) => <Demo {...args} key={status} status={status} />)}</div>,
};
