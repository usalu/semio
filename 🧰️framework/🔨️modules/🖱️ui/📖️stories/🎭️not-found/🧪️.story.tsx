import { NotFound, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/NotFound",
  component: NotFound,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof NotFound>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ description = false, navigation = false }: { description?: boolean; navigation?: boolean }) {
  const title = useLabel("ui.common.noData");
  const detail = useLabel("ui.common.noFileSystemNodes");
  const back = useLabel("ui.nav.back");
  const example = useLabel("ui.common.example");
  const reset = useLabel("ui.common.reset");
  const [route, setRoute] = useState("missing");
  return <div className="min-h-64">
    {route === "missing" ? <div onClickCapture={navigation ? (event) => {
      if (!(event.target instanceof Element) || !event.target.closest("button")) return;
      event.preventDefault();
      event.stopPropagation();
      setRoute("parent");
    } : undefined}>
      <NotFound title={title} description={description ? detail : undefined} parentPath={navigation ? "/story-parent" : undefined} parentLabel={back} />
    </div> : <section className="p-double"><h1>{example}</h1><button type="button" onClick={() => setRoute("missing")}>{reset}</button></section>}
    <output aria-live="polite">{route === "parent" ? back : ""}</output>
  </div>;
}

export const Default: Story = { args: { title: "" }, render: () => <Demo /> };
export const WithDescription: Story = { args: { title: "" }, render: () => <Demo description /> };
export const LocalBackNavigation: Story = { args: { title: "" }, render: () => <Demo description navigation /> };
