// #region 🔌️Adapters
import { Form, SharedBorderRow, SharedBorderRowField } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";
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
    <Form onSubmit={(event) => event.preventDefault()}>
      <SharedBorderRow>
        <SharedBorderRowField label="Query">
          <input name="query" />
        </SharedBorderRowField>
        <button type="submit" data-shared-border-cell="" className="px-single text-xs">
          Submit
        </button>
      </SharedBorderRow>
    </Form>
  ),
};
// #endregion 🧾️FormMatrix
