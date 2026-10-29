/** 🛡️ Keeps a failing viewer from taking its page down: the error is shown with a way to try again.
 * @see ../../🖼️previews/🟦️.tsx — `FigureBoundary` */
import { useState, type ReactElement, type ReactNode } from "react";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Button, Notice } from "../../🪟️chrome/🟦️.tsx";
import { FigureBoundary } from "../../🖼️previews/🟦️.tsx";

/** 🛡️ Wraps the viewer. */
export function ViewerBoundary(props: { readonly resetKey?: unknown; readonly children: ReactNode }): ReactElement {
  const { t } = useArchive();
  const [attempt, setAttempt] = useState(0);
  return (
    <FigureBoundary resetKey={`${String(attempt)}`} fallback={(message) => <Notice kind="error" title={t("viewer.failed")} actions={<Button icon="rotate-cw" onClick={() => setAttempt(attempt + 1)}>{t("action.retry")}</Button>}>{message}</Notice>}>
      <div key={attempt}>{props.children}</div>
    </FigureBoundary>
  );
}
