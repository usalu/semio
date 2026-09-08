type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { CanvasEventBindingController } = dependencies;

  const { describe, expect, it } = vitest;

  describe("CanvasEventBindingController", () => {
    it("disposes registered listeners", () => {
      const ctrl = new CanvasEventBindingController();
      let count = 0;
      const target = {
        addEventListener: () => {
          count += 1;
        },
        removeEventListener: () => {
          count -= 1;
        },
      };
      ctrl.listen(target, "pointermove", () => {});
      expect(count).toBe(1);
      ctrl.dispose();
      expect(count).toBe(0);
    });
  });

}
