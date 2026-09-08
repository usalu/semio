type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { PUZZLE_BOARD_SESSION_FACTORIES, appId, appRole, boot, brand, defaults, locks, pluginFilter, renderer } = dependencies;

  const plugins = boot.plugins;
  if (renderer !== "wgpu") {
    const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
    void bootFrameworkOs({ plugin: pluginFilter, plugins, surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES, appId, appRole, locks, defaults, brand }).catch((error) => {
      console.error("[DEBUG] os-dev react boot failed", error);
    });
  }

}
