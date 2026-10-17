/** 🔌️ App channel lifecycle: the two one-way flags that say whether one client still admits commands (`retired`) and whether its outcome subscription may still end (`disposed`). */

//#region 🔖️AppChannelClient
/** 🚦️ Owns the retire and dispose flags of one {@link AppChannelClient}. Both only ever go from `false` to `true`; every other collaborator reads them and none writes them. */
export class ChannelLifecycle {
  private retiredFlag = false;
  private disposedFlag = false;

  /** 🪦️ `true` once the command queue is closed: no command is admitted any more. */
  get retired(): boolean {
    return this.retiredFlag;
  }

  /** 🔚️ `true` once the background outcome subscription has been asked to end. */
  get disposed(): boolean {
    return this.disposedFlag;
  }

  /** 🚫️ `true` while a new command, query or subscription callback must be refused. */
  get closed(): boolean {
    return this.disposedFlag || this.retiredFlag;
  }

  /** 🔐️ Marks the command queue closed; `false` when it already was, so a second retire does nothing. */
  retire(): boolean {
    if (this.retiredFlag) return false;
    this.retiredFlag = true;
    return true;
  }

  /** 🔕️ Marks the outcome subscription as ended by its owner. */
  dispose(): void {
    this.disposedFlag = true;
  }
}
//#endregion 🔖️AppChannelClient
