/** 🚪️ The gate to the 3D code: the viewer, the model pipeline and the exporters are separate chunks that load the first time something asks for them, so a page that only draws never downloads them. Everything else in the archive reaches the viewer through here and the exporters through a dynamic import in the downloads, never by a static import.
 *
 * `SpatialFigure` shows the interactive 3D model; while its code loads it says so in words, and if the code cannot be loaded it says that in the reader's language and offers to try again.
 *
 * @see ../⚛️figure/🟦️.tsx — what is loaded
 * @see ../../📤️downloads/🟦️.ts — loads the 3D exporter the same way
 */
import { Component, Suspense, lazy, type ReactElement, type ReactNode } from "react";
import { translator } from "../../🌐️i18n/🟦️.ts";
import type { ModelFigureProps } from "../⚛️figure/🟦️.tsx";

const ModelFigure = lazy(() => import("../⚛️figure/🟦️.tsx").then((module) => ({ default: module.ModelFigure })));

class Loading extends Component<{ readonly language: ModelFigureProps["language"]; readonly children: ReactNode }, { readonly failed: boolean; readonly attempt: number }> {
  override state = { failed: false, attempt: 0 };

  static getDerivedStateFromError(): { failed: true } {
    return { failed: true };
  }

  override render(): ReactNode {
    const t = translator(this.props.language);
    if (this.state.failed)
      return (
        <div role="alert" lang={this.props.language} data-model-state="error" style={{ border: "1px dashed currentColor", padding: "0.5em" }}>
          <b>{t("model.error")}</b>{" "}
          <button type="button" onClick={() => this.setState((previous) => ({ failed: false, attempt: previous.attempt + 1 }))}>
            {t("model.retry")}
          </button>
        </div>
      );
    return <div key={this.state.attempt} style={{ inlineSize: "100%", blockSize: "100%" }}>{this.props.children}</div>;
  }
}

/** 🧊️ The interactive 3D model, loaded on demand. */
export function SpatialFigure(props: ModelFigureProps): ReactElement {
  const t = translator(props.language);
  return (
    <Loading language={props.language}>
      <Suspense fallback={<div role="status" lang={props.language} data-model-state="loading" style={{ padding: "0.5em" }}>{t("model.progress")}</div>}>
        <ModelFigure {...props} />
      </Suspense>
    </Loading>
  );
}
