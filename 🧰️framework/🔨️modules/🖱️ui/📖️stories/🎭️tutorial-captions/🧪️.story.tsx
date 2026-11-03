import { TutorialCaptions, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/TutorialCaptions",
  component: TutorialCaptions,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  argTypes: { visible: { control: "boolean" } },
} satisfies Meta<typeof TutorialCaptions>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo(args: React.ComponentProps<typeof TutorialCaptions>) {
  const text = useLabel("ui.common.selectTarget");
  return <div className="relative min-h-64"><TutorialCaptions {...args} text={args.text === null ? null : text} /></div>;
}

export const Default: Story = { args: { text: "", visible: true }, render: (args) => <Demo {...args} /> };
export const Hidden: Story = { ...Default, args: { text: "", visible: false } };
export const NoCue: Story = { ...Default, args: { text: null, visible: true } };
