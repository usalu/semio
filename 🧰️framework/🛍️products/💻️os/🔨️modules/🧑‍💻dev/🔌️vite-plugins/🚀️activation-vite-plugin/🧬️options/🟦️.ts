/** 🧩️ One watched component; `installDirectory` overrides `<installRoot>/<directoryName>` when a host serves extensions from several activation lanes. */
export type ActivationComponentSpec = Readonly<{ pluginId: string; directoryName: string; role: "plugin" | "extension"; sourceRoot: string; installDirectory?: string; cratePath?: string }>;

/** ⚙️ Where the activation plugin observes receipts, which modules it serves and which components it watches. */
export type ActivationVitePluginOptions = { readonly receiptDirectory: string; readonly moduleRoot: string; readonly installRoot: string; readonly freshnessRoot?: string; readonly components: readonly ActivationComponentSpec[] };
