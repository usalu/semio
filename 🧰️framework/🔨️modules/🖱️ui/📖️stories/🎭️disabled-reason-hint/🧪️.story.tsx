import { DisabledReasonHint } from "@semio-tech/ui/elements/ChromeControlHint";
import { useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/DisabledReasonHint",
  component: DisabledReasonHint,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof DisabledReasonHint>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ initiallyRunnable = false }: { initiallyRunnable?: boolean }) {
  const reason = useLabel("ui.conflict.local.readOnly");
  const action = useLabel("ui.common.save");
  const saved = useLabel("ui.sync.saved");
  const [runnable, setRunnable] = useState(initiallyRunnable);
  const [pressed, setPressed] = useState(false);
  const id = "story-save-refusal";
  return <div className="flex flex-col gap-double p-double">
    <label><input type="checkbox" checked={!runnable} onChange={(event) => setRunnable(!event.target.checked)} /> {reason}</label>
    <DisabledReasonHint id={id} reason={runnable ? undefined : reason}><button type="button" aria-disabled={!runnable} aria-describedby={runnable ? undefined : id} onClick={() => { if (runnable) setPressed(true); }}>{action}</button></DisabledReasonHint>
    <output aria-live="polite">{pressed ? saved : ""}</output>
  </div>;
}

export const Default: Story = { args: { id: "story-save-refusal", reason: undefined, children: <button type="button" /> }, render: () => <Demo /> };
export const Runnable: Story = { ...Default, render: () => <Demo initiallyRunnable /> };
