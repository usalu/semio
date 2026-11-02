import { TutorialVideoOverlay, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/TutorialVideoOverlay",
  component: TutorialVideoOverlay,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  argTypes: { localTimeMs: { control: { type: "range", min: 0, max: 3000, step: 100 } }, rate: { control: "select", options: [0.5, 1, 1.5, 2] } },
} satisfies Meta<typeof TutorialVideoOverlay>;
export default meta;
type Story = StoryObj<typeof meta>;

const src = new URL("../../../🖼️assets/🪧️logos/🎞️animation/🎬️animation.mp4", import.meta.url).href;
function Demo(args: React.ComponentProps<typeof TutorialVideoOverlay>) {
  const [playing, setPlaying] = useState(args.playing);
  const play = useLabel("tutorial.play");
  const pause = useLabel("tutorial.pause");
  const description = useLabel("ui.host.preview");
  return <div className="min-h-64 p-double"><p>{description}</p><button type="button" aria-pressed={playing} onClick={() => setPlaying((value) => !value)}>{playing ? pause : play}</button><TutorialVideoOverlay {...args} playing={playing} /></div>;
}

export const Default: Story = { args: { src, rect: { x: 0.35, y: 0.3, width: 0.5, height: 0.5 }, muted: true, playing: false, rate: 1, localTimeMs: 0 }, render: (args) => <Demo key={String(args.playing)} {...args} /> };
export const Playing: Story = { ...Default, args: { ...Default.args, playing: true } };
export const NoSource: Story = { ...Default, args: { ...Default.args, src: null } };
