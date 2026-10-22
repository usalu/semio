/** 🧊️ Play URL of the wgpu dev server on `port` for a plugin filter. */
export function wgpuDevPlayUrl(host: string, port: number, plugin: string): string {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  return `http://${probeHost}:${port}/?plugin=${encodeURIComponent(plugin)}`;
}
