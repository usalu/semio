/** 🧯️ Scoped React fault boundary of the shell: logs the fault and renders a localized fallback with optional retry. Leaf module below the shell composition root, so hosts can import it without importing the root. */
import React, { Component, type ReactNode } from "react";
import { Button, type UiLabel } from "@semio-tech/ui-react";
import { SemioFaultError, type Fault } from "@semio-tech/framework";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

//#region ErrorBoundary
type ShellFaultBoundaryProps = {
  readonly boundaryId: string;
  readonly fallbackLabel: UiLabel;
  readonly children: ReactNode;
  readonly onFault?: (fault: Fault | null, error: Error, boundaryId: string) => void;
  readonly retry?: () => void;
};

type ShellFaultBoundaryState = { readonly hasFault: boolean; readonly message: string; readonly fault: Fault | null };

/** 🧯️ Scoped React fault boundary — logs `[TRACE] shell fault` and renders a localized fallback with optional retry. */
export class ShellFaultBoundary extends Component<ShellFaultBoundaryProps, ShellFaultBoundaryState> {
  constructor(props: ShellFaultBoundaryProps) {
    super(props);
    this.state = { hasFault: false, message: "", fault: null };
  }

  static getDerivedStateFromError(error: Error): ShellFaultBoundaryState {
    const fault = error instanceof SemioFaultError ? error.fault : null;
    return { hasFault: true, message: fault?.message ?? error.message, fault };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    this.props.onFault?.(error instanceof SemioFaultError ? error.fault : null, error, this.props.boundaryId);
  }

  private readonly retry = () => {
    this.setState({ hasFault: false, message: "", fault: null });
    this.props.retry?.();
  };

  render() {
    if (this.state.hasFault) {
      return (
        <div className="p-double" role="alert" data-shell-fault-boundary={this.props.boundaryId}>
          <p className="text-sm text-destructive">
            {this.props.fallbackLabel}: {this.state.message}
          </p>
          {this.props.retry ? <Button className="mt-single" icon="rotate-ccw" text={shellLabel("ui.common.retry")} onClick={this.retry} /> : null}
        </div>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary
