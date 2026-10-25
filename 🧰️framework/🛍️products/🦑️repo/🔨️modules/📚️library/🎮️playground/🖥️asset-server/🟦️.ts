/** 🔌️ Local dev-time asset server port for wgpu Trunk/native playgrounds (Trunk forwards
 * route-scoped requests — e.g. `/osm`, `/vt`, `/dem` — here; driven by each playground's declared
 * `[[package.metadata.semio.assets]]` rows, not any one app's routes). */
export const SEMIO_ASSET_SERVER_PORT = 6141;

/** 🌐️ Process env for the absolute asset server URL base (native-bin wgpu route-relative fetches). */
export const SEMIO_ASSET_BASE_URL_ENV = "SEMIO_ASSET_BASE_URL";
