import { EmptyState, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/EmptyState",
  component: EmptyState,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof EmptyState>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ actionable = false }: { actionable?: boolean }) {
  const title = useLabel("ui.common.noData");
  const description = useLabel("ui.common.noFileSystemNodes");
  const label = useLabel("ui.common.importFile");
  const [requested, setRequested] = useState(false);
  return <div className="min-h-64"><EmptyState title={title} description={description} action={actionable ? { label, onClick: () => setRequested(true) } : undefined} /><output aria-live="polite">{requested ? label : ""}</output></div>;
}

export const Default: Story = { args: { title: "" }, render: () => <Demo /> };
export const WithAction: Story = { args: { title: "" }, render: () => <Demo actionable /> };
