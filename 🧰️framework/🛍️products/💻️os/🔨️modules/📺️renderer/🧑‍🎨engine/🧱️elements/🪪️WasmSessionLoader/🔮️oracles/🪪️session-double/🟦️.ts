import { vi } from "vitest";
import * as flowSessionLoader from "../../🟦️.tsx";

export function boardTestSession(): flowSessionLoader.Board2dWasmSession {
  return {
    attach_canvas: vi.fn(async () => {}),
    setSize: vi.fn(),
    renderFrame: vi.fn(),
    parseFixtureJson: () => true,
    syncDescriptorJson: vi.fn(),
    setKindCatalogsJson: vi.fn(),
    setCamera: vi.fn(),
    setSelectionIdsJson: vi.fn(),
    setCanvasThemeJson: vi.fn(),
    pointerDownScreen: vi.fn(),
    pointerMoveScreen: vi.fn(),
    pointerUpScreen: vi.fn(),
    pointerCancelScreen: vi.fn(),
    wheelScreen: vi.fn(),
    drainEventsJson: vi.fn(() => "[]"),
    cameraJson: () => '{"x":0,"y":0,"zoom":1}',
    gpuReady: () => true,
    free: vi.fn(),
    setSelectionIdsJsonSilent: vi.fn(),
    setFixtureDropPreviewJson: vi.fn(),
  };
}
