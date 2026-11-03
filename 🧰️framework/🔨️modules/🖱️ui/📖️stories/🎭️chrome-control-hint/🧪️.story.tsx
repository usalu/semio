import { ChromeControlHint } from "@semio-tech/ui/elements/ChromeControlHint";
import { useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/ChromeControlHint",
  component: ChromeControlHint,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
  argTypes: { always: { control: "boolean" } },
} satisfies Meta<typeof ChromeControlHint>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ always }: { always?: boolean }) {
  const text = useLabel("ui.common.save");
  const saved = useLabel("ui.sync.saved");
  const [pressed, setPressed] = useState(false);
  return <div className="flex flex-col gap-double p-double"><ChromeControlHint id="ui.common.save" text={text} always={always}><button type="button" onClick={() => setPressed(true)}>{text}</button></ChromeControlHint><output aria-live="polite">{pressed ? saved : ""}</output></div>;
}

export const Default: Story = { args: { children: <button type="button" /> }, render: (args) => <Demo always={args.always} /> };
export const Always: Story = { ...Default, args: { children: <button type="button" />, always: true } };
