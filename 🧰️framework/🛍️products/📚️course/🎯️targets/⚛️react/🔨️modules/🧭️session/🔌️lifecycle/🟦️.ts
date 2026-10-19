/** 🔌️ How long a session runs: the lifetime every background activity is bound to, the releases of what it started and whether the browser is online. */

import { stoppedLifetime } from "../📋️session-model/🟦️.ts";

//#region 🔌️Lifecycle
/** ⏳️ The lifetime of one session between {@link begin} and {@link abort}; before the first start it counts as ended. */
export class Lifecycle {
  private lifetime = stoppedLifetime();
  private online = true;
  private readonly teardown: (() => void)[] = [];

  /** 🚦️ Aborted once the session is stopped or asked the proctor again. */
  get signal(): AbortSignal {
    return this.lifetime.signal;
  }

  /** 📶️ Whether the browser says it is online. */
  isOnline(): boolean {
    return this.online;
  }

  /** ▶️ Opens a new lifetime. */
  begin(): void {
    this.lifetime = new AbortController();
  }

  /** 🧲️ Remembers `release` for {@link release}. */
  own(release: () => void): void {
    this.teardown.push(release);
  }

  /** 🌐️ Follows the browser's connectivity: `changed` is called with the new state after every change, the first state is read at once. Does nothing outside a browser. */
  watchBrowser(changed: (online: boolean) => void): void {
    if (typeof window === "undefined") return;
    const online = (): void => this.connectivity(true, changed);
    const offline = (): void => this.connectivity(false, changed);
    window.addEventListener("online", online);
    window.addEventListener("offline", offline);
    this.teardown.push(
      () => window.removeEventListener("online", online),
      () => window.removeEventListener("offline", offline),
    );
    this.online = typeof navigator === "undefined" || navigator.onLine !== false;
  }

  /** 🛑️ Ends the lifetime. */
  abort(): void {
    this.lifetime.abort();
  }

  /** 🧹️ Lets go of everything {@link own}ed or watched. */
  release(): void {
    for (const release of this.teardown.splice(0)) release();
  }

  /** 🔄️ Ends the lifetime and opens a new one; says `false` and does nothing while the session is stopped. */
  restart(): boolean {
    if (this.lifetime.signal.aborted) return false;
    this.lifetime.abort();
    this.lifetime = new AbortController();
    return true;
  }

  private connectivity(online: boolean, changed: (online: boolean) => void): void {
    this.online = online;
    changed(online);
  }
}
//#endregion 🔌️Lifecycle
