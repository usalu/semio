import { TutorialGhostPointer, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/TutorialGhostPointer",
  component: TutorialGhostPointer,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  argTypes: { progress: { control: { type: "range", min: 0, max: 1, step: 0.05 } } },
} satisfies Meta<typeof TutorialGhostPointer>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo(args: React.ComponentProps<typeof TutorialGhostPointer>) {
  const label = useLabel("ui.common.selectTarget");
  return <div className="min-h-64 p-double"><p>{label}</p><TutorialGhostPointer {...args} /></div>;
}

const from = { kind: "screenNormalized", x: 0.25, y: 0.5 } as const;
const to = { kind: "screenNormalized", x: 0.75, y: 0.5 } as const;
export const Default: Story = { args: { progress: 0.5, cue: { at: 0, durationMs: 1000, gesture: { kind: "leftClick", at: from } } }, render: (args) => <Demo {...args} /> };
export const Drag: Story = { ...Default, args: { progress: 0.5, cue: { at: 0, durationMs: 1000, gesture: { kind: "drag", from, to, button: "left", modifiers: [] } } } };
export const Orbit: Story = { ...Default, args: { progress: 0.5, cue: { at: 0, durationMs: 1000, gesture: { kind: "orbit", from, to, button: "right", modifiers: [] } } } };
export const NoCue: Story = { ...Default, args: { progress: 0, cue: null } };
