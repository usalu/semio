type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { iconIdToRustVariant } = dependencies;

  const { describe, expect, it } = vitest;
  describe("metabolism icon codegen", () => {
    it("maps metabolism stems to Rust variants", () => {
      expect(iconIdToRustVariant("capsule_J")).toBe("CapsuleJ");
      expect(iconIdToRustVariant("cylindric-tambour_first-storey")).toBe("CylindricTambourFirstStorey");
    });
  });

}
