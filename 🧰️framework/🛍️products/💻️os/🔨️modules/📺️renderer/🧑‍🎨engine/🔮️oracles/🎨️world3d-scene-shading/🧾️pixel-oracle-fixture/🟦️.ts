/** 🧾️ The pixel oracle fixture type shared by the reference visual and grid visual oracles. */
declare const THREE: any;
declare const worldSurfaceMaterial: (name: string, material: any, vertexColors: boolean) => any;

/** 🌐️ The names the browser bundle's own prelude supplies to the oracle bodies below. Each
 * `run*Oracle` stringifies one of these functions (`Function.prototype.toString`) and prepends the
 * matching `import` lines before Bun bundles the pair for the page, so these bindings exist in the
 * BROWSER and are never resolved in this driver process — exactly like `THREE` above. */
declare const React: typeof import("react");
declare const createRoot: typeof import("react-dom/client").createRoot;
declare const Canvas: React.ComponentType<Record<string, unknown>>;
declare const Grid: React.ComponentType<Record<string, unknown>>;

/** 🖼️ The slice of a pixel-oracle fixture a run reads back: where its schema lives and the rows a
 * recorded run is compared against. The actual pixel readback is compared against these rows. */
export type PixelOracleFixture = {
    readonly pixelOracle: {
    readonly status: string;
    readonly maximumRgbError: number;
    readonly rows: readonly { readonly id: string; readonly rgba8: readonly number[] }[];
  };
};
