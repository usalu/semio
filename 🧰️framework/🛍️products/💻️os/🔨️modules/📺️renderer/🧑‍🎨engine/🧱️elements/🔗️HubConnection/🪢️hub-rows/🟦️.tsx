import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, SharedBorderRow, SharedBorderRowLabel, type SharedBorderRowProps } from "@semio-tech/ui-react";
import type { ReactElement, ReactNode } from "react";

/** 🪢️ One labelled field of a hub form: a horizontal shared-border row holding the label as a plain cell and the control (`Input`, `SelectTrigger`) as a direct cell, so the pair shares one hairline instead of drawing a box per control. */
export function HubFieldRow({ htmlFor, label, children }: { readonly htmlFor: string; readonly label: string; readonly children: ReactNode }): ReactElement {
  return (
    <SharedBorderRow data-slot="hub-field-row">
      <SharedBorderRowLabel htmlFor={htmlFor}>{label}</SharedBorderRowLabel>
      {children}
    </SharedBorderRow>
  );
}

/** 🪢️ A list of rows sharing one frame: `ul` column whose items ({@link HubRowItem}) are cells separated by single hairlines, never a gap or a border per row. */
export function HubRowList(props: Omit<SharedBorderRowProps, "orientation" | "as">): ReactElement {
  return <SharedBorderRow as="ul" orientation="vertical" {...props} />;
}

/** 🪢️ An item of a {@link HubRowList}: a nested horizontal row (itself a cell of the list) whose direct children, buttons and plain cells, are its cells. */
export function HubRowItem(props: Omit<SharedBorderRowProps, "as" | "framed">): ReactElement {
  return <SharedBorderRow as="li" {...props} />;
}

/** 🪢️ A labelled choice field of a hub form: {@link HubFieldRow} around the framework `Select`, whose trigger is a cell of the row; `children` are further cells (a forget button). */
export function HubSelectField({ id, label, value, options, disabled, alias, onValueChange, children }: {
  readonly id: string;
  readonly label: string;
  readonly value: string;
  readonly options: readonly { readonly value: string; readonly label: string }[];
  readonly disabled?: boolean;
  readonly alias?: string;
  readonly onValueChange: (value: string) => void;
  readonly children?: ReactNode;
}): ReactElement {
  return (
    <HubFieldRow htmlFor={id} label={label}>
      <Select id={`${id}-select`} value={value} disabled={disabled} onValueChange={onValueChange}>
        <SelectTrigger id={id} data-element-alias={alias} size="sm" className="flex-1 min-w-0 text-sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {options.map((option) => <SelectItem key={option.value} value={option.value}>{option.label}</SelectItem>)}
        </SelectContent>
      </Select>
      {children}
    </HubFieldRow>
  );
}
