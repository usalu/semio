/** 🟣️ The shared declarations of the Select suite tests. */
import * as React from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../🟦️.tsx";

export function BasicSelect(props: Omit<React.ComponentProps<typeof Select>, "id"> = {}): React.ReactElement {
  return (
    <Select id="select-component-test-1" {...props}>
      <SelectTrigger aria-label="Mode">
        <SelectValue placeholder="Choose" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="alpha">Alpha</SelectItem>
        <SelectItem value="beta" disabled>
          Beta
        </SelectItem>
        <SelectItem value="gamma">Gamma</SelectItem>
      </SelectContent>
    </Select>
  );
}
