// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { TableAvatar } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 📻️TableAvatarMatrix
describe("TableAvatar", () => {
  it("shows a semantic fallback until the current image loads and restores it on error", () => {
    const { container, rerender } = render(<TableAvatar name="Ada Lovelace" icon="/ada.png" />);
    const image = container.querySelector('[data-slot="avatar-image"]') as HTMLImageElement;
    const fallback = container.querySelector('[data-slot="avatar-fallback"]') as HTMLSpanElement;

    expect(image.alt).toBe("Ada Lovelace");
    expect(image.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);
    expect(fallback.textContent).toBe("AL");
    expect(fallback.getAttribute("aria-label")).toBe("Ada Lovelace");

    fireEvent.load(image);
    expect(image.hidden).toBe(false);
    expect(fallback.hidden).toBe(true);

    rerender(<TableAvatar name="Ada Lovelace" icon="/ada-new.png" />);
    const newImage = container.querySelector('[data-slot="avatar-image"]') as HTMLImageElement;
    expect(newImage.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);

    fireEvent.error(newImage);
    expect(newImage.hidden).toBe(true);
    expect(fallback.hidden).toBe(false);
  });

  it("forwards the root ref, class and style while retaining selected and hovered rings", () => {
    const ref = React.createRef<HTMLSpanElement>();
    const { getByTestId } = render(
      <>
        <TableAvatar ref={ref} data-testid="selected" name="Selected Person" className="custom-avatar" style={{ width: 24 }} fallbackStyle={{ color: "red" }} isSelected />
        <TableAvatar data-testid="hovered" name="Hovered Person" isHovered />
      </>,
    );
    const selected = getByTestId("selected");
    const hovered = getByTestId("hovered");

    expect(ref.current).toBe(selected);
    expect(selected.className).toContain("custom-avatar");
    expect(selected.className).toContain("ring-[color:var(--active-base)]");
    expect(selected.style.width).toBe("24px");
    expect(selected.querySelector<HTMLElement>('[data-slot="avatar-fallback"]')?.style.color).toBe("red");
    expect(hovered.className).toContain("ring-[color:var(--hover-base)]");
  });
});
// #endregion 📻️TableAvatarMatrix
