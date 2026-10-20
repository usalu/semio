/** 🟢️ The shared declarations of the ⚙️ General Settings Tree and bottom-panel flow suite tests. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement as h, useState } from "react";
import { Button } from "@semio-tech/ui-react";
import { Input } from "@semio-tech/ui-react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@semio-tech/ui-react";

export const engineRoot = join(dirname(dirname(fileURLToPath(import.meta.url))), "..", "..");

export const frameworkRoot = join(engineRoot, "..", "..", "..", "..", "..");

export const fixture = JSON.parse(readFileSync(join(engineRoot, "🧪️tests/🧫️fixtures", "⚙️settings-general-layout", "🔣️.json"), "utf8"));

export const stepperEditingFixture = JSON.parse(readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🧪️tests/🧫️fixtures", "⌨️number-stepper-editing", "🔣️.json"), "utf8"));

export const sliderEditingFixture = JSON.parse(readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🧪️tests/🧫️fixtures", "⌨️slider-readout-editing", "🔣️.json"), "utf8"));

export const bottomGeometry = ({ panel, tabRows, rowHeight, contentInset }: typeof fixture.bottomPanel) => {
  const barHeight = tabRows * rowHeight;
  return {
    content: { x: panel.x + contentInset, y: panel.y + contentInset, w: panel.w - contentInset * 2, h: panel.h - barHeight - contentInset * 2 },
    rootTabY: panel.y + panel.h - rowHeight,
    leafTabY: panel.y + panel.h - tabRows * rowHeight,
    dividerY: panel.y + panel.h - barHeight,
  };
};

export const upFlowHeaderRect = ({ sectionRect, headerHeight }: typeof fixture.upFlowDisclosure) => ({
  x: sectionRect.x,
  y: sectionRect.y + sectionRect.h - headerHeight,
  w: sectionRect.w,
  h: Math.min(headerHeight, sectionRect.h),
});

export function DriverPublicationHarness() {
  const [driver, setDriver] = useState("compact");
  const [labels, setLabels] = useState("icons");
  const [saveLabel, setSaveLabel] = useState("");
  const [custom, setCustom] = useState(false);
  return h(
    "div",
    null,
    h(Select, { id: "framework.settings.driver", value: driver, onValueChange: setDriver }, h(SelectTrigger, { "aria-label": "framework.settings.driver" }, h(SelectValue)), h(SelectContent, null, h(SelectItem, { value: "default" }, "default"), h(SelectItem, { value: "compact" }, "compact"), h(SelectItem, { value: "custom.studio-driver" }, "custom.studio-driver"))),
    h(Select, { id: "framework.settings.driver.labels", value: labels, onValueChange: setLabels }, h(SelectTrigger, { "aria-label": "framework.settings.driver.labels" }, h(SelectValue)), h(SelectContent, null, h(SelectItem, { value: "icons" }, "icons"), h(SelectItem, { value: "full" }, "full"))),
    h(Input, { id: "framework.settings.driver.saveLabel", "aria-label": "framework.settings.driver.saveLabel", value: saveLabel, onChange: (event) => setSaveLabel(event.currentTarget.value) }),
    h(Button, { "aria-label": "framework.settings.driver.save", icon: "save", disabled: saveLabel.trim() === "", text: "save", onClick: () => { setDriver("custom.studio-driver"); setSaveLabel(""); setCustom(true); } }),
    custom ? h(Button, { "aria-label": "framework.settings.driver.delete", icon: "trash-2", text: "delete", onClick: () => { setDriver("default"); setCustom(false); } }) : null,
  );
}
