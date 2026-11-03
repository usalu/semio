import { DesktopTitlebar, useLabel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/DesktopTitlebar",
  component: DesktopTitlebar,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta<typeof DesktopTitlebar>;
export default meta;
type Story = StoryObj<typeof meta>;

function Demo({ controls = false }: { controls?: boolean }) {
  const title = useLabel("ui.common.example");
  const minimize = useLabel("ui.common.minimize");
  const maximize = useLabel("ui.common.maximize");
  const close = useLabel("ui.common.close");
  const reset = useLabel("ui.common.reset");
  const [state, setState] = useState("open");
  return <div className="flex flex-col gap-double">
    {state !== "closed" && <div className={state === "maximized" ? "w-full border" : "max-w-lg border"}>
      <DesktopTitlebar title={title} controls={controls ? { minimize: () => setState("minimized"), maximize: () => setState("maximized"), close: () => setState("closed") } : undefined} />
      {state !== "minimized" && <div className="p-double">{title}</div>}
    </div>}
    {controls && <><output aria-live="polite">{state === "minimized" ? minimize : state === "maximized" ? maximize : state === "closed" ? close : ""}</output><button type="button" onClick={() => setState("open")}>{reset}</button></>}
  </div>;
}

export const Default: Story = { args: { title: "" }, render: () => <Demo /> };
export const WithControls: Story = { args: { title: "" }, render: () => <Demo controls /> };
