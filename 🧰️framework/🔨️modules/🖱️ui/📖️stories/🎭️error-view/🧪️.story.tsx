import { ErrorView, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/ErrorView",
  component: ErrorView,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof ErrorView>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ retryable = false }: { retryable?: boolean }) {
  const message = useLabel("ui.sync.attachFailed");
  const retry = useLabel("ui.common.retry");
  const [attempts, setAttempts] = useState(0);
  return <div className="min-h-64"><ErrorView message={message} onRetry={retryable ? () => setAttempts((value) => value + 1) : undefined} /><output aria-live="polite">{attempts > 0 ? `${retry}: ${attempts}` : ""}</output></div>;
}

export const Default: Story = { args: { message: "" }, render: () => <Demo /> };
export const Retryable: Story = { args: { message: "" }, render: () => <Demo retryable /> };
