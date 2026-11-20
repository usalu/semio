// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/IconRenderHost/component.tsx
/** 🖼️ `IconRenderHost` — renders an icon-render scene: offscreen GLB shot preview inside a shot
 * frame, see https://threejs.org/docs/#examples/en/renderers/SVGRenderer. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useMemo, useState } from "../⚛️react-port/🟦️.ts";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { IconShotFrame, useLabel, type IconRenderRequest } from "@semio-tech/ui-react";
import { renderIconRequest } from "./🚚️request/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️IconRenderHost
//#region IconRenderHost
export function IconRenderHost({ node }: ComponentSceneHostProps) {
  const scene = node.iconRender;
  const requestJson = scene?.requestJson;
  const request = useMemo<IconRenderRequest | null>(() => {
    if (!requestJson) return null;
    try {
      return JSON.parse(requestJson) as IconRenderRequest;
    } catch {
      return null;
    }
  }, [requestJson]);
  const [preview, setPreview] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const iconShotLabel = useLabel("ui.host.iconShot");
  const renderingLabel = useLabel("ui.host.rendering");
  const failedLabel = useLabel("ui.host.iconRenderFailed");
  useEffect(() => {
    setPreview(null);
    setError(null);
    if (!request) return;
    const controller = new AbortController();
    void renderIconRequest(request, controller.signal)
      .then((result) => {
        if (!controller.signal.aborted) setPreview(result.dataUrl);
      })
      .catch((renderError: unknown) => {
        if (!controller.signal.aborted) setError(renderError instanceof Error ? renderError.message : String(renderError));
      });
    return () => controller.abort();
  }, [request]);
  if (!scene || !request) {
    return <div className="flex h-full items-center justify-center text-sm opacity-60">{emptySceneLabel}</div>;
  }
  const frameBackground = request.format === "svg" && (preview !== null || request.drawing?.background === false) ? "transparent" : request.background;
  const content = error ? (
    <div className="flex h-full flex-col items-center justify-center p-4 text-center text-sm text-destructive"><span>{failedLabel}</span><span>{error}</span></div>
  ) : preview ? (
    <img alt={scene.footer ?? iconShotLabel} className="block h-full w-full" src={preview} />
  ) : (
    <div className="flex h-full items-center justify-center text-sm opacity-60">{renderingLabel}</div>
  );
  return (
    <div className="semio-icon-render-host absolute inset-0 flex flex-col" data-surface-id={node.surfaceId}>
      <div className="relative min-h-0 flex-1">
        <IconShotFrame background={frameBackground} height={request.height} shape={request.shape ?? "rectangle"} width={request.width}>
          {content}
        </IconShotFrame>
      </div>
      {scene.footer ? <div className="shrink-0 px-3 pb-2 text-center text-xs opacity-60">{scene.footer}</div> : null}
    </div>
  );
}
//#endregion IconRenderHost
//#endregion 🔖️IconRenderHost
