import { Catalogue } from "@semio-tech/ui/elements/Tree/catalogue";
import { createIconComponent, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

const meta = {
  title: "🖱️ui⚛️react/Catalogue",
  component: Catalogue,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof Catalogue>;
export default meta;
type Story = StoryObj<typeof meta>;
const FileIcon = createIconComponent("file-text");
const FolderIcon = createIconComponent("folder");

function Demo({ empty = false, palette = false }: { empty?: boolean; palette?: boolean }) {
  const title = useLabel("ui.common.example");
  const file = useLabel("ui.common.importFile");
  const noData = useLabel("ui.common.noData");
  const items = empty ? [] : [
    { id: "file", label: file, icon: palette ? <FileIcon /> : undefined, payload: { kind: "file", id: "file" } },
    { id: "example", label: title, icon: palette ? <FolderIcon /> : undefined, payload: { kind: "folder", id: "example" } },
  ];
  return <section className="max-w-md" aria-label={title}><Catalogue title={title} items={items} />{empty && <p role="status">{noData}</p>}</section>;
}

export const Default: Story = { args: { title: "", items: [] }, render: () => <Demo /> };
export const Empty: Story = { args: { title: "", items: [] }, render: () => <Demo empty /> };
export const Palette: Story = { args: { title: "", items: [] }, render: () => <Demo palette /> };
