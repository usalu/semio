/** 🖱️ A press between two chips of a frame lands on content, so it activates the frame like any other press inside it; there is no cutout that counts as canvas. */
import { fireEvent, render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
import { setSurfaceActiveRoot, useSurfaceActive } from "./../🟦️.tsx";

const Frame: React.FC = () => {
  const ref = React.useRef<HTMLDivElement>(null);
  const [active, bind] = useSurfaceActive(ref);
  return (
    <div ref={ref} data-testid="frame" data-active={active ? "true" : "false"} data-window-frame="" {...bind}>
      <div data-window-frame-content="" data-testid="content" />
      <div data-window-frame-chip="" data-dock="top" />
    </div>
  );
};

describe("frame press", () => {
  it("activates the frame on a press that reaches content between chips", () => {
    setSurfaceActiveRoot(null);
    const view = render(<Frame />);
    expect(view.getByTestId("frame").dataset.active).toBe("false");
    fireEvent.pointerDown(view.getByTestId("content"));
    expect(view.getByTestId("frame").dataset.active).toBe("true");
  });
});
