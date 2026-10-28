/** 🎛️ Ordinary controls rendered against a document or an original owned read. */
import type { UiNodeRecord } from "@semio-tech/framework";
import type { UiControlContext } from "./🟦️.ts";
import { ButtonView } from "../🔤️display-views/🟦️.tsx";
import { IconSelectView, InputView, NumberStepperView, RingView, SelectView, SliderView, ToggleView } from "../✍️input-views/🟦️.tsx";

export function OrdinaryControlView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiControlContext }) {
  switch (record.component.type) {
    case "button": return <ButtonView record={record} context={context} />;
    case "input": return <InputView record={record} context={context} />;
    case "select": return <SelectView record={record} context={context} />;
    case "toggle": return <ToggleView record={record} context={context} />;
    case "slider": return <SliderView record={record} context={context} />;
    case "numberStepper": return <NumberStepperView record={record} context={context} />;
    case "ring": return <RingView record={record} context={context} />;
    case "iconSelect": return <IconSelectView record={record} context={context} />;
    default: return null;
  }
}
