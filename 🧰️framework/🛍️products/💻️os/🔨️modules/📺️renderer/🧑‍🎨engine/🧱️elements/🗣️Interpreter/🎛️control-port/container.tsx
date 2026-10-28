/** 🧱️ Container presentation borrows a record and rendered children without owning their source. */
import type { MouseEventHandler, ReactNode } from "react";
import { Field, Section, borderElementClass, cn } from "@semio-tech/ui-react";
import type { UiNodeRecord, Component } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import { accessibilityAriaProps, layoutSpecStyle, styleSpecDataAttributes } from "../📐️layout-accessibility/🟦️.tsx";
import { activityBorderClass } from "../🔤️display-views/🟦️.tsx";
import { wireLabel } from "../🌉️scene-host-contract/🟦️.ts";

export interface UiContainerFrameProps {
  readonly record: UiNodeRecord;
  readonly id: string;
  readonly children: ReactNode;
  readonly controlId?: string | null;
  readonly selected?: boolean;
  readonly hovered?: boolean;
  readonly onActivate?: MouseEventHandler<HTMLDivElement>;
}

export function UiContainerFrame({ record, id, children, controlId, selected, hovered, onActivate }: UiContainerFrameProps) {
  const component = record.component as Extract<Component, { type: "container" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, id);
  const style = layoutSpecStyle(record.layout);
  const dataAttrs = styleSpecDataAttributes(record.style);
  if (component.role === "section" || component.role === "group") return <Section id={id} title={component.label ? wireLabel(component.label) : undefined} className={cn(selected && "ring-primary ring-1")}>{describedBy}{children}</Section>;
  if (component.role === "field") return <Field {...dataAttrs} {...aria} style={style} id={id} controlId={controlId} label={component.label ?? ""} description={component.description ?? undefined} required={component.required ?? undefined} error={component.error ?? undefined}>{describedBy}{children}</Field>;
  return <div style={style} {...dataAttrs} {...aria} aria-label={record.accessibility.label ?? (component.role === "form" ? component.label ?? undefined : undefined)} role={onActivate ? "button" : component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined} id={id} data-ui-node-id={record.id} data-ui-node-key={record.key} data-activity={record.activity} className={cn(activityBorderClass(record), onActivate && cn(borderElementClass, "border cursor-pointer rounded-md"), selected && "ring-primary ring-1", hovered && "outline-primary/50 outline-1")} aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined} onClick={onActivate}>{describedBy}{children}</div>;
}
