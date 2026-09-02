// #region 🔌️Adapters
import { Form } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story";
// #endregion 🔌️Adapters

// #region 🧾️FormMatrix
const meta = {
  title: "🖱️ui⚛️react/Form",
  component: Form,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof Form>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NativeSubmission: Story = {
  render: () => (
    <Form className="flex items-center gap-single" onSubmit={(event) => event.preventDefault()}>
      <label className="flex flex-col gap-half text-xs">
        Query
        <input name="query" className="rounded-sm border px-single py-half" />
      </label>
      <button type="submit" className="rounded-sm border px-single py-half text-xs">
        Submit
      </button>
    </Form>
  ),
};
// #endregion 🧾️FormMatrix
