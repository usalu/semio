/** 🧱️ Container presentation borrows a record and rendered children without owning their source. */
import type { MouseEventHandler, ReactNode } from "react";
import { CardStack, Field, OverviewCard, SharedBorderRow, SharedBorderRowCell, borderElementClass, cn, overviewCardChipClass } from "@semio-tech/ui-react";
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
  readonly row?: boolean;
}

/** 🪢️ A container whose children are bordered controls or fields is one shared-border row: no gap, no padding, one hairline between children. */
export function UiContainerFrame({ record, id, children, controlId, selected, hovered, onActivate, row }: UiContainerFrameProps) {
  const component = record.component as Extract<Component, { type: "container" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, id);
  const style = layoutSpecStyle(record.layout);
  const dataAttrs = styleSpecDataAttributes(record.style);
  const rowStyle = { ...style, gap: 0, padding: 0, flexWrap: "nowrap" as const, alignItems: "stretch" };
  const orientation = record.layout.kind === "stack" && record.layout.axis === "horizontal" ? "horizontal" : "vertical";
  if (component.role === "section" || component.role === "group") {
    const title = component.label ? wireLabel(component.label) : undefined;
    return (
      <SharedBorderRow as="section" id={id} orientation="vertical" aria-labelledby={title ? `${id}.title` : undefined} className={cn(selected && "ring-primary ring-1")}>
        {title ? <SharedBorderRowCell><h2 className="text-base font-semibold" id={`${id}.title`}>{title}</h2></SharedBorderRowCell> : null}
        {describedBy}
        {children}
      </SharedBorderRow>
    );
  }
  if (component.role === "cardStack") return <CardStack {...dataAttrs} {...aria} id={id} data-ui-node-id={record.id} data-ui-node-key={record.key}>{describedBy}{children}</CardStack>;
  if (component.role === "card") {
    const title = component.label ? wireLabel(component.label) : "";
    const footer = component.description ? wireLabel(component.description) : null;
    return <OverviewCard as="section" slot="ui-card" headingId={`${id}.title`} icon={null} title={title} footerRight={footer ? <span className={overviewCardChipClass}>{footer}</span> : undefined} flush data={{ "data-ui-node-id": String(record.id), "data-ui-node-key": record.key }}>{describedBy}{children}</OverviewCard>;
  }
  if (component.role === "field") return <Field {...dataAttrs} {...aria} data-ui-node-id={record.id} data-ui-node-key={record.key} style={rowStyle} id={id} controlId={controlId} label={component.label ?? ""} description={component.description ?? undefined} required={component.required ?? undefined} error={component.error ?? undefined}>{describedBy}{children}</Field>;
  if (row && !onActivate) return <SharedBorderRow style={rowStyle} orientation={orientation} {...dataAttrs} {...aria} aria-label={record.accessibility.label ?? (component.role === "form" ? component.label ?? undefined : undefined)} role={component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined} id={id} data-ui-node-id={record.id} data-ui-node-key={record.key} data-activity={record.activity} className={cn(activityBorderClass(record), selected && "ring-primary ring-1", hovered && "outline-primary/50 outline-1")} aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}>{describedBy}{children}</SharedBorderRow>;
  return <div style={style} {...dataAttrs} {...aria} aria-label={record.accessibility.label ?? (component.role === "form" ? component.label ?? undefined : undefined)} role={onActivate ? "button" : component.role === "form" ? "form" : component.role === "toolbar" ? "toolbar" : undefined} id={id} data-ui-node-id={record.id} data-ui-node-key={record.key} data-activity={record.activity} className={cn(activityBorderClass(record), onActivate && cn(borderElementClass, "border cursor-pointer rounded-md"), selected && "ring-primary ring-1", hovered && "outline-primary/50 outline-1")} aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined} onClick={onActivate}>{describedBy}{children}</div>;
}
