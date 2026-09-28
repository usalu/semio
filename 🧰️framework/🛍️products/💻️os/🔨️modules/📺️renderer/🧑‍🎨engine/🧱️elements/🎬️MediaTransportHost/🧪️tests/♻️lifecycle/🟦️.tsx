type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { MediaTransportHost, MediaTransportOwnerContext } = dependencies;
  const { afterEach, describe, expect, it, vi } = vitest;
  const [{ default: Ajv2020 }, React, testing, { chromium }, { dirname, resolve }, { fileURLToPath }, { mediaTransportLabels }, { collectMediaTransportBytes }, { SEGMENTED_DOWNLOAD_CONTRACT }] = await Promise.all([
    import("ajv/dist/2020.js"),
    import("react"),
    import("@semio-tech/ui-react/test"),
    import("playwright"),
    import("node:path"),
    import("node:url"),
    import("../../../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts"),
    import("../../🚚️lifecycle/🟦️.ts"),
    import("../../../📤️SegmentedDownload/🟦️.ts"),
  ]);
  const { createElement } = React;
  const { cleanup, fireEvent, render, waitFor } = testing;
  const [{ default: fixture }, { default: schema }] = await Promise.all([
    import("../../🧫️fixtures/♻️lifecycle/🔣️.json"),
    import("../../🧬️schema/♻️lifecycle/🔣️.json"),
  ]);
  void source;

  const handle = {
    app_instance_id: fixture.identity.instanceId,
    parent_document_id: fixture.identity.parentDocumentId,
    operation_id: 9n,
    base_revision: BigInt(fixture.identity.revision),
    generation: BigInt(fixture.identity.generation),
  };
  const props = (revision = fixture.identity.revision) => ({
    schemaVersion: 1,
    kind: fixture.media.kind,
    mediaType: fixture.media.mediaType,
    revision,
    durationMs: fixture.media.durationMs,
    positionMs: fixture.media.positionMs,
    selectionStartMs: fixture.media.selection[0],
    selectionEndMs: fixture.media.selection[1],
    locale: "en",
    labels: mediaTransportLabels("en"),
    resource: {
      kind: "artifact-media-export",
      controllerId: fixture.identity.controllerId,
      appInstanceId: fixture.identity.instanceId,
      parentDocumentId: fixture.identity.parentDocumentId,
      outputPort: fixture.identity.outputPort,
      revision,
      generation: fixture.identity.generation,
    },
    capability: { status: "ready", reason: null },
    hostContentHeight: 240,
  });
  const status = (state: "running" | "complete" | "cancelled" | "failed", progress = 0n) => ({ handle, state, applied_progress: progress, checkpoint_available: false, mime_type: fixture.media.mediaType, total_bytes: BigInt(fixture.output.totalBytes), detail: state });
  const makePort = () => {
    const polls = [status("running", 1n), status("running", 2n), status("complete", 2n)];
    const chunks = fixture.output.chunks.map((data: number[], index: number) => ({ handle, data: Uint8Array.from(data), terminal: index === fixture.output.chunks.length - 1 }));
    return {
      submitMediaExport: vi.fn(async () => handle),
      pollMediaExport: vi.fn(async () => polls.shift() ?? status("complete", 2n)),
      cancelMediaExport: vi.fn(async () => {}),
      takeMediaExportChunk: vi.fn(async () => chunks.shift() ?? { handle, data: new Uint8Array(), terminal: true }),
    };
  };
  const mount = (port: ReturnType<typeof makePort>, value: unknown = props()) => render(
    createElement(MediaTransportOwnerContext.Provider, { value: { instanceId: fixture.identity.instanceId, controllerId: fixture.identity.controllerId, windowId: fixture.identity.windowId, port } },
      createElement(MediaTransportHost, { value, nodeId: 7, nodeKey: "media.transport" })),
  );

  describe("🎬️ MediaTransportHost lifecycle", () => {
    afterEach(() => {
      cleanup();
      vi.restoreAllMocks();
    });

    it("validates the language-neutral bounded lifecycle", () => {
      const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.output.chunks.reduce((total: number, chunk: number[]) => total + chunk.length, 0)).toBe(fixture.output.totalBytes);
      expect(fixture.requirements.maximumTotalBytes).toBe(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes);
      expect(fixture.requirements.maximumPages).toBe(SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks);
    });

    it("drains completed output before reporting a MIME refusal and never cancels an already-complete owner", async () => {
      const port = makePort();
      port.pollMediaExport.mockReset();
      port.pollMediaExport.mockResolvedValue({ ...status("complete"), mime_type: "application/schema" });
      await expect(collectMediaTransportBytes({ instanceId: fixture.identity.instanceId, parentDocumentId: fixture.identity.parentDocumentId, revision: BigInt(fixture.identity.revision), generation: BigInt(fixture.identity.generation), mediaType: fixture.media.mediaType, port, cancelled: () => false, progress: () => {} })).rejects.toThrow("media-export.media-type");
      expect(port.takeMediaExportChunk).toHaveBeenCalledTimes(fixture.output.chunks.length);
      expect(port.cancelMediaExport).not.toHaveBeenCalled();
    });

    it("accepts an exact zero-byte terminal and drains a nonterminal empty page before refusing it", async () => {
      const zeroPort = makePort();
      zeroPort.pollMediaExport.mockReset();
      zeroPort.pollMediaExport.mockResolvedValue({ ...status("complete"), total_bytes: 0n });
      zeroPort.takeMediaExportChunk.mockReset();
      zeroPort.takeMediaExportChunk.mockResolvedValue({ handle, data: new Uint8Array(), terminal: true });
      await expect(collectMediaTransportBytes({ instanceId: fixture.identity.instanceId, parentDocumentId: fixture.identity.parentDocumentId, revision: BigInt(fixture.identity.revision), generation: BigInt(fixture.identity.generation), mediaType: fixture.media.mediaType, port: zeroPort, cancelled: () => false, progress: () => {} })).resolves.toEqual({ chunks: [], mediaType: fixture.media.mediaType });
      const emptyPort = makePort();
      emptyPort.pollMediaExport.mockReset();
      emptyPort.pollMediaExport.mockResolvedValue({ ...status("complete"), total_bytes: 0n });
      emptyPort.takeMediaExportChunk.mockReset();
      emptyPort.takeMediaExportChunk.mockResolvedValueOnce({ handle, data: new Uint8Array(), terminal: false }).mockResolvedValueOnce({ handle, data: new Uint8Array(), terminal: true });
      await expect(collectMediaTransportBytes({ instanceId: fixture.identity.instanceId, parentDocumentId: fixture.identity.parentDocumentId, revision: BigInt(fixture.identity.revision), generation: BigInt(fixture.identity.generation), mediaType: fixture.media.mediaType, port: emptyPort, cancelled: () => false, progress: () => {} })).rejects.toThrow("media-export.empty-page");
      expect(emptyPort.takeMediaExportChunk).toHaveBeenCalledTimes(2);
    });

    it("submits the exact document revision, advances one bounded turn, drains bytes and never autoplays", async () => {
      vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
      vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
      const play = vi.spyOn(HTMLMediaElement.prototype, "play").mockResolvedValue();
      const createUrl = vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:media-42");
      vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => {});
      const port = makePort();
      const view = mount(port);
      await waitFor(() => expect(view.container.querySelector('[data-media-state="ready"]')).not.toBeNull());
      expect(port.submitMediaExport).toHaveBeenCalledWith(fixture.identity.instanceId, fixture.identity.outputPort, fixture.identity.parentDocumentId, BigInt(fixture.identity.revision));
      expect(port.pollMediaExport).toHaveBeenCalledTimes(fixture.running.progress.length + 1);
      expect(port.takeMediaExportChunk).toHaveBeenCalledTimes(fixture.output.chunks.length);
      expect(createUrl).toHaveBeenCalledTimes(1);
      const media = view.container.querySelector("audio")!;
      expect(media.autoplay).toBe(fixture.requirements.autoplay);
      expect(play).not.toHaveBeenCalled();
      expect(view.getByRole("button", { name: "Play" })).not.toBeNull();
      expect(view.getByRole("slider", { name: "Seek" })).not.toBeNull();
      expect(view.getByRole("slider", { name: "Selection start" })).not.toBeNull();
      expect(view.getByRole("slider", { name: "Selection end" })).not.toBeNull();
      fireEvent.click(view.getByRole("button", { name: "Play" }));
      expect(play).toHaveBeenCalledTimes(1);
    });

    it("cancels the exact generation and resets source ownership when its revision changes", async () => {
      vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
      vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
      vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:media-42");
      const revoke = vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => {});
      const port = makePort();
      const view = mount(port);
      await waitFor(() => expect(view.container.querySelector('[data-media-state="ready"]')).not.toBeNull());
      const replacement = { ...props("43"), resource: { ...props("43").resource, generation: "4" } };
      view.rerender(createElement(MediaTransportOwnerContext.Provider, { value: { instanceId: fixture.identity.instanceId, controllerId: fixture.identity.controllerId, windowId: fixture.identity.windowId, port } }, createElement(MediaTransportHost, { value: replacement, nodeId: 7, nodeKey: "media.transport" })));
      await waitFor(() => expect(revoke).toHaveBeenCalledWith("blob:media-42"));
      expect(view.container.querySelector("audio")).toBeNull();
    });

    it("serializes unmount cancellation after the in-flight poll and cancels the exact handle", async () => {
      vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
      vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
      let releasePoll: ((value: ReturnType<typeof status>) => void) | null = null;
      const port = makePort();
      port.pollMediaExport.mockImplementationOnce(() => new Promise((resolve) => { releasePoll = resolve; }));
      const view = mount(port);
      await waitFor(() => expect(port.pollMediaExport).toHaveBeenCalledWith(fixture.identity.instanceId, handle));
      view.unmount();
      releasePoll!(status("running", 1n));
      await waitFor(() => expect(port.cancelMediaExport).toHaveBeenCalledWith(fixture.identity.instanceId, handle));
      expect(port.cancelMediaExport).toHaveBeenCalledTimes(1);
    });

    it("refuses a stale app/controller owner before any export bytes are requested", async () => {
      const port = makePort();
      const invalid = { ...props(), resource: { ...props().resource, appInstanceId: fixture.identity.instanceId + 1 } };
      const view = mount(port, invalid);
      await waitFor(() => expect(view.container.querySelector('[data-media-state="failed"]')?.textContent).toContain("owner-mismatch"));
      expect(port.submitMediaExport).not.toHaveBeenCalled();
    });

    it("exposes the mounted native media and two-ended selection controls in Chromium", async () => {
      vi.stubGlobal("TextEncoder", class {
        encode(value = ""): Uint8Array { return Uint8Array.from(Buffer.from(value)); }
      });
      const { build } = await import("esbuild");
      const bundle = await build({ entryPoints: [resolve(dirname(fileURLToPath(source.url)), "🌐️browser.tsx")], bundle: true, format: "iife", platform: "browser", target: "chrome120", write: false, logLevel: "silent", define: { "import.meta.vitest": "undefined" } });
      const browser = await chromium.launch({ headless: true });
      try {
        const page = await browser.newPage();
        await page.setContent('<div id="root"></div>');
        await page.addScriptTag({ content: bundle.outputFiles[0]!.text });
        await page.evaluate((input) => window.mountMediaTransportOracle(input), fixture);
        const observed = await page.locator('[data-media-state="ready"]').evaluate((host) => {
          const media = host.querySelector("audio,video") as HTMLMediaElement | null;
          const sliders = [...host.querySelectorAll<HTMLInputElement>('input[type="range"]')];
          return {
            tag: media?.tagName,
            autoplay: media?.autoplay,
            paused: media?.paused,
            labels: sliders.map((slider) => slider.getAttribute("aria-label")),
            ranges: sliders.map((slider) => [slider.min, slider.max, slider.value]),
            window: host.getAttribute("data-media-window"),
          };
        });
        expect(observed).toEqual({ tag: "AUDIO", autoplay: false, paused: true, labels: ["Seek", "Selection start", "Selection end"], ranges: [["0", "1000", "250"], ["0", "899", "100"], ["101", "1000", "900"]], window: fixture.identity.windowId });
        expect(await page.evaluate(() => window.mediaTransportCalls)).toEqual([
          `submit:${fixture.identity.instanceId}:${fixture.identity.outputPort}:${fixture.identity.parentDocumentId}:${fixture.identity.revision}`,
          "poll:0", "poll:1", "poll:2", "chunk:0", "chunk:1",
        ]);
      } finally {
        await browser.close();
      }
    }, 60_000);
  });
}

if (import.meta.vitest) {
  const dependencies = await import("../../🟦️.tsx");
  await registerTests1(import.meta.vitest, dependencies, { directory: import.meta.dir, url: import.meta.url });
}
