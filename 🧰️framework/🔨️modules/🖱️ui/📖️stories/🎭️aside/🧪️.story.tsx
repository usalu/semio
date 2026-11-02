import { Aside, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/Aside",
  component: Aside,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof Aside>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ tones = false }: { tones?: boolean }) {
  const title = useLabel("ui.common.example");
  const content = useLabel("ui.common.noData");
  return <div className="max-w-xl">{(tones ? ["note", "tip", "caution", "danger"] as const : ["note"] as const).map((kind) => <Aside key={kind} kind={kind} title={title}>{content}</Aside>)}</div>;
}

export const Default: Story = { args: { children: null }, render: () => <Demo /> };
export const Tones: Story = { args: { children: null }, render: () => <Demo tones /> };
