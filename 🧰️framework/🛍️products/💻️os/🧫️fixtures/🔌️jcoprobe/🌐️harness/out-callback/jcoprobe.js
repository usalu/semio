"use components";
import { fetchBody, slowEcho } from './host-shim.js';
import { environment, exit as exit$1, stderr, stdin, stdout, terminalInput, terminalOutput, terminalStderr, terminalStdin, terminalStdout } from './preview2-shim/cli.js';
import { monotonicClock } from './preview2-shim/clocks.js';
import { error, poll as poll$2, streams } from './preview2-shim/io.js';
const { getEnvironment } = environment;

if (getEnvironment=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getEnvironment', was 'getEnvironment' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { exit } = exit$1;

if (exit=== undefined) {
  const err = new Error("unexpectedly undefined local import 'exit', was 'exit' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStderr } = stderr;

if (getStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStderr', was 'getStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdin } = stdin;

if (getStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdin', was 'getStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdout } = stdout;

if (getStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdout', was 'getStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalInput } = terminalInput;

if (TerminalInput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalInput', was 'TerminalInput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalOutput } = terminalOutput;

if (TerminalOutput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalOutput', was 'TerminalOutput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStderr } = terminalStderr;

if (getTerminalStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStderr', was 'getTerminalStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdin } = terminalStdin;

if (getTerminalStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdin', was 'getTerminalStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdout } = terminalStdout;

if (getTerminalStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdout', was 'getTerminalStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { subscribeDuration } = monotonicClock;

if (subscribeDuration=== undefined) {
  const err = new Error("unexpectedly undefined local import 'subscribeDuration', was 'subscribeDuration' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Error: Error$1 } = error;

if (Error$1=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Error$1', was 'Error' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Pollable,
  poll } = poll$2;

if (Pollable=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Pollable', was 'Pollable' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (poll=== undefined) {
  const err = new Error("unexpectedly undefined local import 'poll', was 'poll' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { InputStream,
  OutputStream } = streams;

if (InputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'InputStream', was 'InputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (OutputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'OutputStream', was 'OutputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


function promiseWithResolvers() {
  if (Promise.withResolvers) {
    return Promise.withResolvers();
  } else {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }
}
const symbolDispose = Symbol.dispose || Symbol.for('dispose');
const symbolAsyncIterator = Symbol.asyncIterator;
const symbolIterator = Symbol.iterator;

const _debugLog = (...args) => {
  if (!globalThis?.process?.env?.JCO_DEBUG) { return; }
  console.debug(...args);
};
const ASYNC_DETERMINISM = 'random';
const GLOBAL_COMPONENT_MEMORY_MAP = new Map();
const CURRENT_TASK_META = {};

function _getGlobalCurrentTaskMeta(componentIdx) {
  if (componentIdx === null || componentIdx === undefined) {
    throw new Error("missing/invalid component idx");
  }
  const v = CURRENT_TASK_META[componentIdx];
  if (v === undefined || v === null) {
    return undefined;
  }
  return { ...v };
}


function _setGlobalCurrentTaskMeta(args) {
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  const { taskID, componentIdx } = args;
  return CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
}


function _withGlobalCurrentTaskMeta(args) {
  _debugLog('[_withGlobalCurrentTaskMeta()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (!args.fn) { throw new TypeError('missing fn'); }
  const { taskID, componentIdx, fn } = args;
  
  try {
    CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
    return fn();
  } catch (err) {
    _debugLog("error while executing sync callee/callback", {
      ...args,
      err,
    });
    throw err;
  } finally {
    CURRENT_TASK_META[componentIdx] = null;
  }
}

async function _withGlobalCurrentTaskMetaAsync(args) {
  _debugLog('[_withGlobalCurrentTaskMetaAsync()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (!args.fn) { throw new TypeError('missing fn'); }
  
  const { taskID, componentIdx, fn } = args;
  
  try {
    CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
    return await fn();
  } catch (err) {
    _debugLog("error while executing async callee/callback", {
      ...args,
      err,
    });
    throw err;
  } finally {
    CURRENT_TASK_META[componentIdx] = null;
  }
}

async function _clearCurrentTask(args) {
  _debugLog('[_clearCurrentTask()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  const { taskID, componentIdx } = args;
  
  const meta = CURRENT_TASK_META[componentIdx];
  if (!meta) { throw new Error(`missing current task meta for component idx [${componentIdx}]`); }
  
  if (meta.taskID !== taskID) {
    throw new Error(`task ID [${meta.taskID}] != requested ID [${taskID}]`);
  }
  if (meta.componentIdx !== componentIdx) {
    throw new Error(`component idx [${meta.componentIdx}] != requested idx [${componentIdx}]`);
  }
  
  CURRENT_TASK_META[componentIdx] = null;
}

function lookupMemoriesForComponent(args) {
  const { componentIdx } = args ?? {};
  if (args.componentIdx === undefined) { throw new TypeError("missing component idx"); }
  
  const metas = GLOBAL_COMPONENT_MEMORY_MAP.get(componentIdx);
  if (!metas) { return []; }
  
  if (args.memoryIdx === undefined) {
    return Object.values(metas);
  }
  
  const meta = metas[args.memoryIdx];
  return meta?.memory;
}

function registerGlobalMemoryForComponent(args) {
  const { componentIdx, memory, memoryIdx } = args ?? {};
  if (componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (memory === undefined && memoryIdx === undefined) { throw new TypeError('missing both memory & memory idx'); }
  let inner = GLOBAL_COMPONENT_MEMORY_MAP.get(componentIdx);
  if (!inner) {
    inner = {};
    GLOBAL_COMPONENT_MEMORY_MAP.set(componentIdx, inner);
  }
  
  inner[memoryIdx] = { memory, memoryIdx, componentIdx };
}

class RepTable {
  #data = [0, null];
  #size = 0;
  #target;
  
  constructor(args) {
    this.target = args?.target;
  }
  
  data() { return this.#data; }
  
  insert(val) {
    _debugLog('[RepTable#insert()] args', { val, target: this.target });
    const freeIdx = this.#data[0];
    if (freeIdx === 0) {
      this.#data.push(val);
      this.#data.push(null);
      const rep = (this.#data.length >> 1) - 1;
      _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep });
      this.#size += 1;
      return rep;
    }
    this.#data[0] = this.#data[freeIdx << 1];
    const placementIdx = freeIdx << 1;
    this.#data[placementIdx] = val;
    this.#data[placementIdx + 1] = null;
    _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep: freeIdx });
    this.#size += 1;
    return freeIdx;
  }
  
  get(rep) {
    _debugLog('[RepTable#get()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during get, (cannot be 0)'); }
    
    const baseIdx = rep << 1;
    const val = this.#data[baseIdx];
    return val;
  }
  
  contains(rep) {
    _debugLog('[RepTable#contains()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during contains, (cannot be 0)'); }
    
    const baseIdx = rep << 1;
    return !!this.#data[baseIdx];
  }
  
  remove(rep) {
    _debugLog('[RepTable#remove()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during remove, (cannot be 0)'); }
    if (this.#data.length === 2) { throw new Error('invalid'); }
    
    const baseIdx = rep << 1;
    const val = this.#data[baseIdx];
    
    this.#data[baseIdx] = this.#data[0];
    this.#data[0] = rep;
    this.#size -= 1;
    
    return val;
  }
  
  size() { return this.#size; }
  
  clear() {
    _debugLog('[RepTable#clear()] args', { rep, target: this.target });
    this.#data = [0, null];
  }
}
const _coinFlip = () => { return Math.random() > 0.5; };
let SCOPE_ID = 0;
const I32_MIN = -2_147_483_648;

const I32_MAX= 2_147_483_647;


function _isValidNumericPrimitive(ty, v) {
  if (v === undefined || v === null) { return false; }
  switch (ty) {
    case 'bool':
    return v === 0 || v === 1;
    break;
    case 'u8':
    return v >= 0 && v <= 255;
    break;
    case 's8':
    return v >= -128 && v <= 127;
    break;
    case 'u16':
    return v >= 0 && v <= 65535;
    break;
    case 's16':
    return v >= -32768 && v <= 32767;
    case 'u32':
    return v >= 0 && v <= 4_294_967_295;
    case 's32':
    return v >= -2_147_483_648 && v <= 2_147_483_647;
    case 'u64':
    return typeof v === 'bigint' && v >= 0 && v <= 18_446_744_073_709_551_615n;
    case 's64':
    return typeof v === 'bigint' && v >= -9223372036854775808n && v <= 9223372036854775807n;
    break;
    case 'f32':
    case 'f64': return typeof v === 'number';
    default:
    return false;
  }
  return true;
}

function _requireValidNumericPrimitive(ty, v) {
  if (v === undefined  || v === null || !_isValidNumericPrimitive(ty, v)) {
    throw new TypeError(`invalid ${ty} value [${v}]`);
  }
  return true;
}

const _typeCheckValidI32 = (n) => typeof n === 'number' && n >= I32_MIN && n <= I32_MAX;


const _typeCheckAsyncFn= (f) => {
  return f instanceof ASYNC_FN_CTOR;
};

let RESOURCE_CALL_BORROWS = [];const ASYNC_FN_CTOR = (async () => {}).constructor;

function clearCurrentTask(componentIdx, taskID) {
  _debugLog('[clearCurrentTask()] args', { componentIdx, taskID });
  
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing/invalid component instance index while ending current task');
  }
  
  const tasks = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  if (!tasks || !Array.isArray(tasks)) {
    throw new Error('missing/invalid tasks for component instance while ending task');
  }
  if (tasks.length == 0) {
    throw new Error(`no current tasks for component instance [${componentIdx}] while ending task`);
  }
  
  if (taskID !== undefined) {
    const last = tasks[tasks.length - 1];
    if (last.id !== taskID) {
      // throw new Error('current task does not match expected task ID');
      return;
    }
  }
  
  ASYNC_CURRENT_TASK_IDS.pop();
  ASYNC_CURRENT_COMPONENT_IDXS.pop();
  
  const taskMeta = tasks.pop();
  return taskMeta.task;
}

const CURRENT_TASK_MAY_BLOCK= globalThis.WebAssembly ? new globalThis.WebAssembly.Global({ value: 'i32', mutable: true }, 0) : false;

const ASYNC_CURRENT_TASK_IDS = [];
const ASYNC_CURRENT_COMPONENT_IDXS = [];

function unpackCallbackResult(result) {
  if (!(_typeCheckValidI32(result))) { throw new Error('invalid callback return value [' + result + '], not a valid i32'); }
  const eventCode = result & 0xF;
  if (eventCode < 0 || eventCode > 3) {
    throw new Error('invalid async return value [' + eventCode + '], outside callback code range');
  }
  if (result < 0 || result >= 2**32) { throw new Error('invalid callback result'); }
  // TODO: table max length check?
  const waitableSetRep = result >> 4;
  return [eventCode, waitableSetRep];
}

class AsyncSubtask {
  static _ID = 0n;
  
  static State = {
    STARTING: 0,
    STARTED: 1,
    RETURNED: 2,
    CANCELLED_BEFORE_STARTED: 3,
    CANCELLED_BEFORE_RETURNED: 4,
  };
  
  #id;
  #state = AsyncSubtask.State.STARTING;
  #componentIdx;
  
  #parentTask;
  #childTask = null;
  
  #dropped = false;
  #cancelRequested = false;
  
  #memoryIdx = null;
  #lenders = null;
  
  #waitable = null;
  
  #callbackFn = null;
  #callbackFnName = null;
  
  #postReturnFn = null;
  #onProgressFn = null;
  #pendingEventFn = null;
  
  #callMetadata = {};
  
  #resolved = false;
  
  #onResolveHandlers = [];
  #onStartHandlers = [];
  
  #result = null;
  #resultSet = false;
  
  fnName;
  target;
  isAsync;
  isManualAsync;
  
  constructor(args) {
    if (typeof args.componentIdx !== 'number') {
      throw new Error('invalid componentIdx for subtask creation');
    }
    this.#componentIdx = args.componentIdx;
    
    this.#id = ++AsyncSubtask._ID;
    this.fnName = args.fnName;
    
    if (!args.parentTask) { throw new Error('missing parent task during subtask creation'); }
    this.#parentTask = args.parentTask;
    
    if (args.childTask) { this.#childTask = args.childTask; }
    
    if (args.memoryIdx) { this.#memoryIdx = args.memoryIdx; }
    
    if (!args.waitable) { throw new Error("missing/invalid waitable"); }
    this.#waitable = args.waitable;
    
    if (args.callMetadata) { this.#callMetadata = args.callMetadata; }
    
    this.#lenders = [];
    this.target = args.target;
    this.isAsync = args.isAsync;
    this.isManualAsync = args.isManualAsync;
  }
  
  id() { return this.#id; }
  parentTaskID() { return this.#parentTask?.id(); }
  childTaskID() { return this.#childTask?.id(); }
  state() { return this.#state; }
  
  waitable() { return this.#waitable; }
  waitableRep() { return this.#waitable.idx(); }
  
  join() { return this.#waitable.join(...arguments); }
  getPendingEvent() { return this.#waitable.getPendingEvent(...arguments); }
  hasPendingEvent() { return this.#waitable.hasPendingEvent(...arguments); }
  setPendingEvent() { return this.#waitable.setPendingEvent(...arguments); }
  
  setTarget(tgt) { this.target = tgt; }
  
  getResult() {
    if (!this.#resultSet) { throw new Error("subtask result has not been set") }
    return this.#result;
  }
  setResult(v) {
    if (this.#resultSet) { throw new Error("subtask result has already been set"); }
    this.#result = v;
    this.#resultSet = true;
  }
  
  componentIdx() { return this.#componentIdx; }
  
  setChildTask(t) {
    if (!t) { throw new Error('cannot set missing/invalid child task on subtask'); }
    if (this.#childTask) { throw new Error('child task is already set on subtask'); }
    if (this.#parentTask === t) { throw new Error("parent cannot be child"); }
    this.#childTask = t;
  }
  getChildTask(t) { return this.#childTask; }
  
  getParentTask() { return this.#parentTask; }
  
  setCallbackFn(f, name) {
    if (!f) { return; }
    if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
    this.#callbackFn = f;
    this.#callbackFnName = name;
  }
  
  getCallbackFnName() {
    if (!this.#callbackFn) { return undefined; }
    return this.#callbackFn.name;
  }
  
  setPostReturnFn(f) {
    if (!f) { return; }
    if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
    this.#postReturnFn = f;
  }
  
  setOnProgressFn(f) {
    if (this.#onProgressFn) { throw new Error('on progress fn can only be set once'); }
    this.#onProgressFn = f;
  }
  
  isNotStarted() {
    return this.#state == AsyncSubtask.State.STARTING;
  }
  
  registerOnStartHandler(f) {
    this.#onStartHandlers.push(f);
  }
  
  onStart(args) {
    _debugLog('[AsyncSubtask#onStart()] args', {
      componentIdx: this.#componentIdx,
      subtaskID: this.#id,
      parentTaskID: this.parentTaskID(),
      fnName: this.fnName,
      args,
    });
    
    if (this.#onProgressFn) { this.#onProgressFn(); }
    
    this.#state = AsyncSubtask.State.STARTED;
    
    let result;
    
    // If we have been provided a helper start function as a result of
    // component fusion performed by wasmtime tooling, then we can call that helper and lifts/lowers will
    // be performed for us.
    //
    // See also documentation on `HostIntrinsic::PrepareCall`
    //
    if (this.#callMetadata.startFn) {
      result = this.#callMetadata.startFn.apply(null, args?.startFnParams ?? []);
    }
    
    return result;
  }
  
  
  registerOnResolveHandler(f) {
    this.#onResolveHandlers.push(f);
  }
  
  reject(subtaskErr) {
    this.#childTask?.reject(subtaskErr);
  }
  
  onResolve(subtaskValue) {
    _debugLog('[AsyncSubtask#onResolve()] args', {
      componentIdx: this.#componentIdx,
      subtaskID: this.#id,
      isAsync: this.isAsync,
      childTaskID: this.childTaskID(),
      parentTaskID: this.parentTaskID(),
      parentTaskFnName: this.#parentTask?.entryFnName(),
      fnName: this.fnName,
    });
    
    if (this.#resolved) {
      throw new Error('subtask has already been resolved');
    }
    
    if (this.#onProgressFn) { this.#onProgressFn(); }
    
    if (subtaskValue === null && this.#cancelRequested) {
      if (this.#state === AsyncSubtask.State.STARTING) {
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_STARTED;
      } else {
        if (this.#state !== AsyncSubtask.State.STARTED) {
          throw new Error('resolved subtask must have been started before cancellation');
        }
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_RETURNED;
      }
    } else {
      if (this.#state !== AsyncSubtask.State.STARTED) {
        throw new Error('resolved subtask must have been started before completion');
      }
      this.#state = AsyncSubtask.State.RETURNED;
    }
    
    this.setResult(subtaskValue);
    
    for (const f of this.#onResolveHandlers) {
      try {
        f(subtaskValue);
      } catch (err) {
        console.error("error during subtask resolve handler", err);
        throw err;
      }
    }
    
    const callMetadata = this.getCallMetadata();
    
    // TODO(fix): we should be able to easily have the caller's meomry
    // to lower into here, but it's not present in PrepareCall
    const memory = callMetadata.memory ?? this.#parentTask?.getReturnMemory() ?? lookupMemoriesForComponent({ componentIdx: this.#parentTask?.componentIdx() })[0];
    if (callMetadata && !callMetadata.returnFn && this.isAsync && callMetadata.resultPtr && memory) {
      const { resultPtr, realloc } = callMetadata;
      const lowers = callMetadata.lowers; // may have been updated in task.return of the child
      if (lowers && lowers.length > 0) {
        lowers[0]({
          componentIdx: this.#componentIdx,
          memory,
          realloc,
          vals: [subtaskValue],
          storagePtr: resultPtr,
          stringEncoding: callMetadata.stringEncoding,
        });
      }
    }
    
    this.#resolved = true;
    this.#parentTask.removeSubtask(this);
    
    if (!this.isAsync) {
      this.deliverResolve();
      const rep = this.waitableRep();
      if (rep) {
        try {
          const removed = this.#getComponentState().handles.remove(rep);
          if (removed !== this) {
            throw new Error("unexpectedly received non-self Subtask from handle removal");
          }
          this.drop();
        } catch (err) {
          _debugLog('[AsyncSubtask#onResolve()] failed to remove subtask after sync subtask completion', err);
        }
      }
    }
  }
  
  getStateNumber() { return this.#state; }
  isReturned() { return this.#state === AsyncSubtask.State.RETURNED; }
  
  getCallMetadata() { return this.#callMetadata; }
  
  isResolved() {
    if (this.#state === AsyncSubtask.State.STARTING
    || this.#state === AsyncSubtask.State.STARTED) {
      return false;
    }
    if (this.#state === AsyncSubtask.State.RETURNED
    || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_STARTED
    || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_RETURNED) {
      return true;
    }
    throw new Error('unrecognized internal Subtask state [' + this.#state + ']');
  }
  
  addLender(handle) {
    _debugLog('[AsyncSubtask#addLender()] args', { handle });
    if (!Number.isNumber(handle)) { throw new Error('missing/invalid lender handle [' + handle + ']'); }
    
    if (this.#lenders.length === 0 || this.isResolved()) {
      throw new Error('subtask has no lendors or has already been resolved');
    }
    
    handle.lends++;
    this.#lenders.push(handle);
  }
  
  deliverResolve() {
    _debugLog('[AsyncSubtask#deliverResolve()] args', {
      lenders: this.#lenders,
      parentTaskID: this.parentTaskID(),
      subtaskID: this.#id,
      childTaskID: this.childTaskID(),
      resolved: this.isResolved(),
      resolveDelivered: this.resolveDelivered(),
    });
    
    const cannotDeliverResolve = this.resolveDelivered() || !this.isResolved();
    if (cannotDeliverResolve) {
      throw new Error('subtask cannot deliver resolution twice, and the subtask must be resolved');
    }
    
    for (const lender of this.#lenders) {
      lender.lends--;
    }
    
    this.#lenders = null;
  }
  
  resolveDelivered() {
    _debugLog('[AsyncSubtask#resolveDelivered()] args', { });
    if (this.#lenders === null && !this.isResolved()) {
      throw new Error('invalid subtask state, lenders missing and subtask has not been resolved');
    }
    return this.#lenders === null;
  }
  
  drop() {
    _debugLog('[AsyncSubtask#drop()] args', {
      componentIdx: this.#componentIdx,
      parentTaskID: this.#parentTask?.id(),
      parentTaskFnName: this.#parentTask?.entryFnName(),
      childTaskID: this.#childTask?.id(),
      childTaskFnName: this.#childTask?.entryFnName(),
      subtaskFnName: this.fnName,
    });
    if (!this.#waitable) { throw new Error('missing/invalid inner waitable'); }
    if (!this.resolveDelivered()) {
      throw new Error('cannot drop subtask before resolve is delivered');
    }
    if (this.#waitable) { this.#waitable.drop() }
    this.#dropped = true;
  }
  
  #getComponentState() {
    const state = getOrCreateAsyncState(this.#componentIdx);
    if (!state) {
      throw new Error('invalid/missing async state for component [' + componentIdx + ']');
    }
    return state;
  }
  
  getWaitableHandleIdx() {
    _debugLog('[AsyncSubtask#getWaitableHandleIdx()] args', { });
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    return this.waitableRep();
  }
}

function _prepareCall(
memoryIdx,
getMemoryFn,
startFn,
returnFn,
callerComponentIdx,
calleeComponentIdx,
taskReturnTypeIdx,
calleeIsAsyncInt,
stringEncoding,
resultCountOrAsync,
) {
  _debugLog('[_prepareCall()]', {
    memoryIdx,
    callerComponentIdx,
    calleeComponentIdx,
    taskReturnTypeIdx,
    calleeIsAsyncInt,
    stringEncoding,
    resultCountOrAsync,
  });
  const argArray = [...arguments];
  
  // value passed in *may* be as large as u32::MAX which may be mangled into -2
  resultCountOrAsync >>>= 0;
  
  let isAsync = false;
  let hasResultPointer = false;
  if (resultCountOrAsync === 2**32 - 1) {
    // prepare async with no result (u32::MAX)
    isAsync = true;
    hasResultPointer = false;
  } else if (resultCountOrAsync === 2**32 - 2) {
    // prepare async with result (u32::MAX - 1)
    isAsync = true;
    hasResultPointer = true;
  }
  
  const currentCallerTaskMeta = getCurrentTask(callerComponentIdx);
  if (!currentCallerTaskMeta) {
    throw new Error('invalid/missing current task for caller during prepare call');
  }
  
  const currentCallerTask = currentCallerTaskMeta.task;
  if (!currentCallerTask) {
    throw new Error('unexpectedly missing task in meta for caller during prepare call');
  }
  
  if (currentCallerTask.componentIdx() !== callerComponentIdx) {
    throw new Error(`task component idx [${ currentCallerTask.componentIdx() }] !== [${ callerComponentIdx }] (callee ${ calleeComponentIdx })`);
  }
  
  let getCalleeParamsFn;
  let resultPtr = null;
  let directParamsArr;
  if (hasResultPointer) {
    directParamsArr = argArray.slice(10, argArray.length - 1);
    getCalleeParamsFn = () => directParamsArr;
    resultPtr = argArray[argArray.length - 1];
  } else {
    directParamsArr = argArray.slice(10);
    getCalleeParamsFn = () => directParamsArr;
  }
  
  let encoding;
  switch (stringEncoding) {
    case 0:
    encoding = 'utf8';
    break;
    case 1:
    encoding = 'utf16';
    break;
    case 2:
    encoding = 'compact-utf16';
    break;
    default:
    throw new Error(`unrecognized string encoding enum [${stringEncoding}]`);
  }
  
  const subtask = currentCallerTask.createSubtask({
    componentIdx: callerComponentIdx,
    parentTask: currentCallerTask,
    isAsync,
    callMetadata: {
      getMemoryFn,
      memoryIdx,
      resultPtr,
      returnFn,
      startFn,
      stringEncoding,
    }
  });
  
  const [newTask, newTaskID] = createNewCurrentTask({
    componentIdx: calleeComponentIdx,
    isAsync,
    getCalleeParamsFn,
    entryFnName: [
    'task',
    subtask.getParentTask().id(),
    'subtask',
    subtask.id(),
    'new-prepared-async-task'
    ].join('/'),
    stringEncoding,
  });
  newTask.setParentSubtask(subtask);
  newTask.setReturnMemoryIdx(memoryIdx);
  newTask.setReturnMemory(getMemoryFn);
  subtask.setChildTask(newTask);
  
  newTask.subtaskMeta = {
    subtask,
    calleeComponentIdx,
    callerComponentIdx,
    getCalleeParamsFn,
    stringEncoding,
    isAsync,
  };
  
  _setGlobalCurrentTaskMeta({
    taskID: newTask.id(),
    componentIdx: newTask.componentIdx(),
  });
}

function _asyncStartCall(args, callee, paramCount, resultCount, flags) {
  const componentIdx = ASYNC_CURRENT_COMPONENT_IDXS.at(-1);
  
  const globalTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
  if (!globalTaskMeta) { throw new Error('missing global current task globalTaskMeta'); }
  const taskID = globalTaskMeta.taskID;
  
  _debugLog('[_asyncStartCall()] args', { args, componentIdx });
  const { getCallbackFn, callbackIdx, getPostReturnFn, postReturnIdx } = args;
  
  const preparedTaskMeta = getCurrentTask(componentIdx, taskID);
  if (!preparedTaskMeta) { throw new Error('unexpectedly missing current task'); }
  
  const preparedTask = preparedTaskMeta.task;
  if (!preparedTask) { throw new Error('unexpectedly missing current task'); }
  if (!preparedTask.subtaskMeta) { throw new Error('missing subtask meta from prepare'); }
  
  const {
    subtask,
    returnMemoryIdx,
    getReturnMemoryFn,
    callerComponentIdx,
    calleeComponentIdx,
    getCalleeParamsFn,
    isAsync,
    stringEncoding,
  } = preparedTask.subtaskMeta;
  if (!subtask) { throw new Error("missing subtask from cstate during async start call"); }
  if (calleeComponentIdx !== preparedTask.componentIdx()) {
    throw new Error(`meta callee idx [${calleeComponentIdx}] != current task idx [${preparedTask.componentIdx()}] during async start call`);
  }
  if (calleeComponentIdx !== componentIdx) {
    throw new Error("mismatched componentIdx for async start call (does not match prepare)");
  }
  
  const argArray = [...arguments];
  
  if (resultCount < 0 || resultCount > 1) { throw new Error('invalid/unsupported result count'); }
  
  const callbackFnName = 'callback_' + callbackIdx;
  const callbackFn = getCallbackFn();
  preparedTask.setCallbackFn(callbackFn, callbackFnName);
  preparedTask.setPostReturnFn(getPostReturnFn());
  
  if (resultCount < 0 || resultCount > 1) {
    throw new Error(`unsupported result count [${ resultCount }]`);
  }
  
  const params = preparedTask.getCalleeParams();
  if (paramCount !== params.length) {
    throw new Error(`unexpected callee param count [${ params.length }], _asyncStartCall invocation expected [${ paramCount }]`);
  }
  
  const callerComponentState = getOrCreateAsyncState(subtask.componentIdx());
  
  const calleeComponentState = getOrCreateAsyncState(preparedTask.componentIdx());
  const calleeBackpressure = calleeComponentState.hasBackpressure();
  
  // Set up a handler on subtask completion to lower results from the call into the caller's memory region.
  //
  // NOTE: during fused guest->guest calls this handler is triggered, but does not actually perform
  // lowering manually, as fused modules provider helper functions that can
  subtask.registerOnResolveHandler((res) => {
    _debugLog('[_asyncStartCall()] handling subtask result', { res, subtaskID: subtask.id() });
    
    let subtaskCallMeta = subtask.getCallMetadata();
    
    // NOTE: in the case of guest -> guest async calls, there may be no memory/realloc present,
    // as the host will intermediate the value storage/movement between calls.
    //
    // We can simply take the value and lower it as a parameter
    if (subtaskCallMeta.memory || subtaskCallMeta.realloc) {
      throw new Error("call metadata unexpectedly contains memory/realloc for guest->guest call");
    }
    
    const callerTask = subtask.getParentTask();
    const calleeTask = preparedTask;
    const callerMemoryIdx = callerTask.getReturnMemoryIdx();
    const callerComponentIdx = callerTask.componentIdx();
    
    // If a helper function was provided we are likely in a fused guest->guest call,
    // and the result will be delivered (lift/lowered) via helper function
    if (subtaskCallMeta && subtaskCallMeta.returnFn) {
      _debugLog('[_asyncStartCall()] return function present while handling subtask result, returning early (skipping lower)', {
        calleeTaskID: calleeTask.id(),
        calleeComponentIdx,
      });
      
      // TODO: centralize calling of returnFn to *one place* (if possible)
      if (subtaskCallMeta.returnFnCalled) { return; }
      
      const res = subtaskCallMeta.returnFn.apply(null, [subtaskCallMeta.resultPtr]);
      
      _debugLog('[_asyncStartCall()] finished calling return fn', {
        calleeTaskID: calleeTask.id(),
        calleeComponentIdx,
        res,
      });
      
      return;
    }
    
    // If there is no where to lower the results, exit early
    if (!subtaskCallMeta.resultPtr) {
      _debugLog('[_asyncStartCall()] no result ptr during subtask result handling, returning early (skipping lower)');
      return;
    }
    
    let callerMemory;
    if (callerMemoryIdx !== null && callerMemoryIdx !== undefined) {
      callerMemory = lookupMemoriesForComponent({ componentIdx: callerComponentIdx, memoryIdx: callerMemoryIdx });
    } else {
      const callerMemories = lookupMemoriesForComponent({ componentIdx: callerComponentIdx });
      if (callerMemories.length !== 1) { throw new Error(`unsupported amount of caller memories`); }
      callerMemory = callerMemories[0];
    }
    
    if (!callerMemory) {
      _debugLog('[_asyncStartCall()] missing memory', { subtaskID: subtask.id(), res });
      throw new Error(`missing memory for to guest->guest call result (subtask [${subtask.id()}])`);
    }
    
    const lowerFns = calleeTask.getReturnLowerFns();
    if (!lowerFns || lowerFns.length === 0) {
      _debugLog('[_asyncStartCall()] missing result lower metadata for guest->guest call', { subtaskID: subtask.id() });
      throw new Error(`missing result lower metadata for guest->guest call (subtask [${subtask.id()}])`);
    }
    
    if (lowerFns.length !== 1) {
      _debugLog('[_asyncStartCall()] only single result reportetd for guest->guest call', { subtaskID: subtask.id() });
      throw new Error(`only single result supported for guest->guest calls (subtask [${subtask.id()}])`);
    }
    
    _debugLog('[_asyncStartCall()] lowering results', { subtaskID: subtask.id() });
    lowerFns[0]({
      realloc: undefined,
      memory: callerMemory,
      vals: [res],
      storagePtr: subtaskCallMeta.resultPtr,
      componentIdx: callerComponentIdx,
      stringEncoding: subtaskCallMeta.stringEncoding,
    });
    
  });
  
  subtask.setOnProgressFn(() => {
    subtask.setPendingEvent(() => {
      if (subtask.isResolved()) { subtask.deliverResolve(); }
      const event = {
        code: ASYNC_EVENT_CODE.SUBTASK,
        payload0: subtask.waitableRep(),
        payload1: subtask.getStateNumber(),
      };
      return event;
    });
  });
  
  // Start the (event) driver loop that will resolve the subtask
  // in a new JS task
  setTimeout(async () => {
    _debugLog('[_asyncStartCall()] continuing started subtask (in JS task)', {
      taskID: preparedTask.id(),
      subtaskID: subtask.id(),
      callerComponentIdx,
      calleeComponentIdx,
    });
    
    let startRes = subtask.onStart({ startFnParams: params });
    startRes = Array.isArray(startRes) ? startRes : [startRes];
    
    if (calleeComponentState.isExclusivelyLocked()) {
      _debugLog('[_asyncStartCall()] during continuation callee is exclusively locked, suspending...', {
        taskID: preparedTask.id(),
        subtaskID: subtask.id(),
        callerComponentIdx,
        calleeComponentIdx,
      });
      await calleeComponentState.suspendTask({
        task: preparedTask,
        readyFn: () => !calleeComponentState.isExclusivelyLocked(),
      });
    }
    
    const started = await preparedTask.enter();
    if (!started) {
      _debugLog('[_asyncStartCall()] task failed early', {
        taskID: preparedTask.id(),
        subtaskID: subtask.id(),
      });
      throw new Error("task failed to start");
      return;
    }
    
    let callbackResult;
    try {
      let jspiCallee;
      if (callee._cachedPromising) {
        jspiCallee = callee._cachedPromising;
      } else {
        callee._cachedPromising = WebAssembly.promising(callee);
        jspiCallee = callee._cachedPromising;
      }
      
      callbackResult = await _withGlobalCurrentTaskMetaAsync({
        taskID: preparedTask.id(),
        componentIdx: preparedTask.componentIdx(),
        fn: () => {
          return jspiCallee.apply(null, startRes);
        }
      });
    } catch(err) {
      _debugLog("[_asyncStartCall()] initial subtask callee run failed", err);
      // NOTE: a good place to rejectt the parent task, if rejection API is enabled
      // subtask.reject(err);
      // subtask.getParentTask().reject(err);
      
      subtask.getParentTask().setErrored(err);
      
      return;
    }
    
    // If there was no callback function, we're dealing with a sync function
    // that was lifted as async without one, there is only the callee.
    if (!callbackFn) {
      _debugLog("[_asyncStartCall()] no callback, resolving w/ callee result", {
        taskID: preparedTask.id(),
        componentIdx: preparedTask.componentIdx(),
        preparedTask,
        stateNumber: preparedTask.taskState(),
        isResolved: preparedTask.isResolved(),
        callbackFn,
      });
      preparedTask.resolve([callbackResult]);
      return;
    }
    
    let fnName = callbackFn.fnName;
    if (!fnName) {
      fnName = [
      '<task ',
      subtask.parentTaskID(),
      '/subtask ',
      subtask.id(),
      '/task ',
      preparedTask.id(),
      '>',
      ].join("");
    }
    
    try {
      _debugLog("[_asyncStartCall()] starting driver loop", {
        fnName,
        componentIdx: preparedTask.componentIdx(),
        subtaskID: subtask.id(),
        childTaskID: subtask.childTaskID(),
        parentTaskID: subtask.parentTaskID(),
      });
      
      await _driverLoop({
        componentState: calleeComponentState,
        task: preparedTask,
        fnName,
        isAsync: true,
        callbackResult,
        resolve,
        reject
      });
    } catch (err) {
      _debugLog("[AsyncStartCall] drive loop call failure", { err });
    }
    
  }, 0);
  
  const subtaskState = subtask.getStateNumber();
  if (subtaskState < 0 || subtaskState > 2**5) {
    throw new Error('invalid subtask state, out of valid range');
  }
  
  _debugLog('[_asyncStartCall()] returning subtask rep & state', {
    subtask: {
      rep: subtask.waitableRep(),
      state: subtaskState,
    }
  });
  
  return Number(subtask.waitableRep()) << 4 | subtaskState;
}

function _syncStartCall(callbackIdx) {
  _debugLog('[_syncStartCall()] args', { callbackIdx });
  throw new Error('synchronous start call not implemented!');
}

class Waitable {
  #componentIdx;
  
  #pendingEventFn = null;
  
  #promise;
  #resolve;
  #reject;
  
  #waitableSet = null;
  
  #hasSyncWaiter = false;
  
  #idx = null; // to component-global waitables
  
  target;
  
  constructor(args) {
    const { componentIdx, target } = args;
    this.#componentIdx = componentIdx;
    this.target = args.target;
    this.#resetPromise();
  }
  
  componentIdx() { return this.#componentIdx; }
  isInSet() { return this.#waitableSet !== null; }
  
  idx() { return this.#idx; }
  setIdx(idx) {
    if (idx === 0) { throw new Error("waitable idx cannot be zero"); }
    this.#idx = idx;
  }
  
  setTarget(tgt) { this.target = tgt; }
  
  #resetPromise() {
    const { promise, resolve, reject } = promiseWithResolvers()
    this.#promise = promise;
    this.#resolve = resolve;
    this.#reject = reject;
  }
  
  resolve() { this.#resolve(); }
  reject(err) { this.#reject(err); }
  promise() { return this.#promise; }
  
  hasPendingEvent() {
    // _debugLog('[Waitable#hasPendingEvent()]', {
      //     componentIdx: this.#componentIdx,
      //     waitable: this,
      //     waitableSet: this.#waitableSet,
      //     hasPendingEvent: this.#pendingEventFn !== null,
      // });
      return this.#pendingEventFn !== null;
    }
    
    setPendingEvent(fn) {
      _debugLog('[Waitable#setPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
      });
      this.#pendingEventFn = fn;
    }
    
    getPendingEvent() {
      _debugLog('[Waitable#getPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
        hasPendingEvent: this.#pendingEventFn !== null,
      });
      if (this.#pendingEventFn === null) { return null; }
      const eventFn = this.#pendingEventFn;
      this.#pendingEventFn = null;
      const e = eventFn();
      this.#resetPromise();
      return e;
    }
    
    join(waitableSet) {
      _debugLog('[Waitable#join()] args', {
        waitable: this,
        waitableSet: waitableSet,
        isRemoval: waitableSet === null,
      });
      
      if (this.#waitableSet === undefined) {
        throw new TypeError('waitable set must be not be undefined');
      }
      
      if (this.#waitableSet) {
        this.#waitableSet.removeWaitable(this);
      }
      
      this.#waitableSet = waitableSet;
      
      if (waitableSet) {
        this.#waitableSet.addWaitable(this);
      }
    }
    
    drop() {
      _debugLog('[Waitable#drop()] args', {
        componentIdx: this.#componentIdx,
        waitable: this,
      });
      if (this.hasPendingEvent()) {
        throw new Error('waitables with pending events cannot be dropped');
      }
      this.join(null);
    }
    
    async waitForPendingEvent(args) {
      const { cstate } = args;
      if (!cstate) { throw new TypeError('missing component state'); }
      
      if (this.#waitableSet !== null || this.#hasSyncWaiter) {
        throw new Error("waitable is already in a set/has a sync waiter");
      }
      this.#hasSyncWaiter = true;
      await cstate.waitUntil({
        cancellable: false,
        readyFn: () => this.hasPendingEvent(),
      });
      this.#hasSyncWaiter = false;
    }
    
  }
  
  const ERR_CTX_TABLES = {};
  
  function contextGet(ctx) {
    const { componentIdx, slot } = ctx;
    if (componentIdx === undefined) { throw new TypeError("missing component idx"); }
    if (slot === undefined) { throw new TypeError("missing slot"); }
    
    const currentTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
    if (!currentTaskMeta) {
      throw new Error(`missing/incomplete global current task meta for component idx [${componentIdx}] during context set`);
    }
    const taskID = currentTaskMeta.taskID;
    
    const taskMeta = getCurrentTask(componentIdx, taskID);
    if (!taskMeta) { throw new Error('failed to retrieve current task'); }
    
    let task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing current task in metadata while getting context'); }
    
    _debugLog('[contextGet()] args', {
      slot,
      storage: task.storage,
      taskID: task.id(),
      componentIdx: task.componentIdx(),
    });
    
    if (slot < 0 || slot >= task.storage.length) { throw new Error('invalid slot for current task'); }
    
    return task.storage[slot];
  }
  
  
  function contextSet(ctx, value) {
    const { componentIdx, slot } = ctx;
    if (componentIdx === undefined) { throw new TypeError("missing component idx"); }
    if (slot === undefined) { throw new TypeError("missing slot"); }
    if (!(_typeCheckValidI32(value))) { throw new Error('invalid value for context set (not valid i32)'); }
    
    const currentTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
    if (!currentTaskMeta) {
      throw new Error(`missing/incomplete global current task meta for component idx [${componentIdx}] during context set`);
    }
    const taskID = currentTaskMeta.taskID;
    
    const taskMeta = getCurrentTask(componentIdx, taskID);
    if (!taskMeta) { throw new Error('failed to retrieve current task'); }
    
    let task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing current task in metadata while setting context'); }
    
    _debugLog('[contextSet()] args', {
      slot,
      value,
      storage: task.storage,
      taskID: task.id(),
      componentIdx: task.componentIdx(),
    });
    
    if (slot < 0 || slot >= task.storage.length) { throw new Error('invalid slot for current task'); }
    task.storage[slot] = value;
  }
  
  const ASYNC_TASKS_BY_COMPONENT_IDX = new Map();
  
  class AsyncTask {
    static _ID = 0n;
    
    static State = {
      INITIAL: 'initial',
      CANCELLED: 'cancelled',
      CANCEL_PENDING: 'cancel-pending',
      CANCEL_DELIVERED: 'cancel-delivered',
      RESOLVED: 'resolved',
    }
    
    static BlockResult = {
      CANCELLED: 'block.cancelled',
      NOT_CANCELLED: 'block.not-cancelled',
    }
    
    #id;
    #componentIdx;
    #state;
    #isAsync;
    #isManualAsync;
    #preserveFutureResult;
    #entryFnName = null;
    
    #onResolveHandlers = [];
    #completionPromise = null;
    #rejected = false;
    
    #exitPromise = null;
    #onExitHandlers = [];
    
    #memoryIdx = null;
    #memory = null;
    
    #callbackFn = null;
    #callbackFnName = null;
    
    #postReturnFn = null;
    
    #getCalleeParamsFn = null;
    
    #stringEncoding = null;
    
    #parentSubtask = null;
    
    #errHandling;
    
    #backpressurePromise;
    #backpressureWaiters = 0n;
    
    #returnLowerFns = null;
    
    #subtasks = [];
    
    #entered = false;
    #exited = false;
    #errored = null;
    
    cancelled = false;
    cancelRequested = false;
    alwaysTaskReturn = false;
    
    returnCalls =  0;
    storage = [0, 0];
    borrowedHandles = {};
    
    tmpRetI64HighBits = 0|0;
    
    constructor(opts) {
      this.#id = ++AsyncTask._ID;
      
      if (opts?.componentIdx === undefined) {
        throw new TypeError('missing component id during task creation');
      }
      this.#componentIdx = opts.componentIdx;
      
      this.#state = AsyncTask.State.INITIAL;
      this.#isAsync = opts?.isAsync ?? false;
      this.#isManualAsync = opts?.isManualAsync ?? false;
      this.#preserveFutureResult = opts?.preserveFutureResult ?? false;
      this.#entryFnName = opts.entryFnName;
      
      const {
        promise: completionPromise,
        resolve: resolveCompletionPromise,
        reject: rejectCompletionPromise,
      } = promiseWithResolvers();
      this.#completionPromise = completionPromise;
      
      this.#onResolveHandlers.push((results) => {
        if (this.#parentSubtask !== null) { return; }
        if (!this.#isAsync) { return; }
        
        if (this.#errored !== null) {
          rejectCompletionPromise(this.#errored);
          return;
        } else if (this.#rejected) {
          rejectCompletionPromise(results);
          return;
        }
        
        if (this.#preserveFutureResult && results instanceof FutureValue) {
          results.resolveAsValue(resolveCompletionPromise);
        } else {
          resolveCompletionPromise(results);
        }
      });
      
      const {
        promise: exitPromise,
        resolve: resolveExitPromise,
        reject: rejectExitPromise,
      } = promiseWithResolvers();
      this.#exitPromise = exitPromise;
      
      this.#onExitHandlers.push(() => {
        resolveExitPromise();
      });
      
      if (opts.callbackFn) { this.#callbackFn = opts.callbackFn; }
      if (opts.callbackFnName) { this.#callbackFnName = opts.callbackFnName; }
      
      if (opts.getCalleeParamsFn) { this.#getCalleeParamsFn = opts.getCalleeParamsFn; }
      
      if (opts.stringEncoding) { this.#stringEncoding = opts.stringEncoding; }
      
      if (opts.parentSubtask) { this.#parentSubtask = opts.parentSubtask; }
      
      
      if (opts.errHandling) { this.#errHandling = opts.errHandling; }
    }
    
    taskState() { return this.#state; }
    id() { return this.#id; }
    componentIdx() { return this.#componentIdx; }
    entryFnName() { return this.#entryFnName; }
    
    completionPromise() { return this.#completionPromise; }
    exitPromise() { return this.#exitPromise; }
    
    isAsync() { return this.#isAsync; }
    isSync() { return !this.isAsync(); }
    
    getErrHandling() { return this.#errHandling; }
    
    hasCallback() { return this.#callbackFn !== null; }
    
    getReturnMemoryIdx() { return this.#memoryIdx; }
    setReturnMemoryIdx(idx) {
      if (idx === null) { return; }
      this.#memoryIdx = idx;
    }
    
    getReturnMemory() { return this.#memory; }
    setReturnMemory(m) {
      if (m === null) { return; }
      this.#memory = m;
    }
    
    setReturnLowerFns(fns) { this.#returnLowerFns = fns; }
    getReturnLowerFns() { return this.#returnLowerFns; }
    
    setParentSubtask(subtask) {
      if (!subtask || !(subtask instanceof AsyncSubtask)) { return }
      if (this.#parentSubtask) { throw new Error('parent subtask can only be set once'); }
      this.#parentSubtask = subtask;
    }
    
    getParentSubtask() { return this.#parentSubtask; }
    
    // TODO(threads): this is very inefficient, we can pass along a root task,
    // and ideally do not need this once thread support is in place
    getRootTask() {
      let currentSubtask = this.getParentSubtask();
      let task = this;
      while (currentSubtask) {
        task = currentSubtask.getParentTask();
        currentSubtask = task.getParentSubtask();
      }
      return task;
    }
    
    setPostReturnFn(f) {
      if (!f) { return; }
      if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
      this.#postReturnFn = f;
    }
    
    setCallbackFn(f, name) {
      if (!f) { return; }
      if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
      this.#callbackFn = f;
      this.#callbackFnName = name;
    }
    
    getCallbackFnName() {
      if (!this.#callbackFnName) { return undefined; }
      return this.#callbackFnName;
    }
    
    async runCallbackFn(...args) {
      if (!this.#callbackFn) { throw new Error('no callback function has been set for task'); }
      return _withGlobalCurrentTaskMetaAsync({
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        fn: () => { return this.#callbackFn.apply(null, args); }
      });
    }
    
    getCalleeParams() {
      if (!this.#getCalleeParamsFn) { throw new Error('missing/invalid getCalleeParamsFn'); }
      return this.#getCalleeParamsFn();
    }
    
    mayBlock() { return this.isAsync() || this.isResolvedState() }
    
    mayEnter(task) {
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      if (cstate.hasBackpressure()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to backpressure', { taskID: this.#id });
        return false;
      }
      if (!cstate.callingSyncImport()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync import call', { taskID: this.#id });
        return false;
      }
      const callingSyncExportWithSyncPending = cstate.callingSyncExport && !task.isAsync;
      if (!callingSyncExportWithSyncPending) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync export w/ sync pending', { taskID: this.#id });
        return false;
      }
      return true;
    }
    
    enterSync() {
      if (this.needsExclusiveLock()) {
        const cstate = getOrCreateAsyncState(this.#componentIdx);
        // TODO(???): it is *very possible* for a the line below to fail if
        // an async function is already running (and holding the exclusive lock)
        //
        // It's not really possible to fix this unless we turn every sync export into
        // an async export that will use the regular async enabled `enter()`.
        cstate.exclusiveLock();
      }
      return true;
    }
    
    async enter(opts) {
      _debugLog('[AsyncTask#enter()] args', {
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        subtaskID: this.getParentSubtask()?.id(),
        args: opts,
        entryFnName: this.#entryFnName,
      });
      
      if (this.#entered) {
        throw new Error(`task with ID [${this.#id}] should not be entered twice`);
      }
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      
      if (opts?.isHost) {
        this.#entered = true;
        return this.#entered;
      }
      
      await cstate.nextTaskExecutionSlot({ task: this });
      
      // If a task is synchronous then we can avoid component-relevant
      // tracking and immediately enter.
      if (this.isSync()) {
        this.#entered = true;
        
        // TODO(breaking): remove once manually-specifying async fns is removed
        // It is currently possible for an actually sync export to be specified
        // as async via JSPI
        if (this.#isManualAsync) {
          if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
        }
        
        return this.#entered;
      }
      
      // Perform intial backpressure check
      if (cstate.hasBackpressure() || this.needsExclusiveLock() && cstate.isExclusivelyLocked()) {
        cstate.addBackpressureWaiter();
        
        const result = await this.waitUntil({
          readyFn: () => {
            return !(cstate.hasBackpressure()
            || this.needsExclusiveLock() && cstate.isExclusivelyLocked());
          },
          cancellable: true,
        });
        
        cstate.removeBackpressureWaiter();
        
        if (result === AsyncTask.BlockResult.CANCELLED) {
          this.cancel();
          return false;
        }
      }
      
      // Lock the component state or keep trying until we can/do
      try {
        if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
      } catch {
        // Continuously attempt to lock until we can
        while (cstate.hasBackpressure() || this.needsExclusiveLock() && cstate.isExclusivelyLocked()) {
          try {
            if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
            break;
          } catch(err) {
            cstate.addBackpressureWaiter();
            const result = await this.waitUntil({
              readyFn: () => {
                return !(cstate.hasBackpressure()
                || this.needsExclusiveLock() && cstate.isExclusivelyLocked());
              },
              cancellable: true,
            });
            cstate.removeBackpressureWaiter();
            if (result === AsyncTask.BlockResult.CANCELLED) {
              this.cancel();
              return false;
            }
          }
        }
      }
      
      this.#entered = true;
      return this.#entered;
    }
    
    isRunningState() { return this.#state !== AsyncTask.State.RESOLVED; }
    isResolvedState() { return this.#state === AsyncTask.State.RESOLVED; }
    isResolved() { return this.#state === AsyncTask.State.RESOLVED; }
    
    async waitUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#waitUntil()] args', { taskID: this.#id, args: { cancellable } });
      
      // TODO(fix): check for cancel
      // TODO(fix): determinism
      // TODO(threads): add this thread to waiting list
      
      const keepGoing = await this.suspendUntil({
        readyFn,
        cancellable,
      });
      
      return keepGoing;
    }
    
    async yieldUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#yieldUntil()]', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      const keepGoing = await this.suspendUntil({ readyFn, cancellable });
      if (keepGoing) {
        return {
          code: ASYNC_EVENT_CODE.NONE,
          payload0: 0,
          payload1: 0,
        };
      }
      
      return {
        code: ASYNC_EVENT_CODE.TASK_CANCELLED,
        payload0: 0,
        payload1: 0,
      };
    }
    
    async suspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#suspendUntil()] args', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      const pendingCancelled = this.deliverPendingCancel({ cancellable });
      if (pendingCancelled) { return false; }
      
      const completed = await this.immediateSuspendUntil({ readyFn, cancellable });
      return completed;
    }
    
    // TODO(threads): equivalent to thread.suspend_until()
    async immediateSuspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#immediateSuspendUntil()] args', {
        args: {
          cancellable,
          readyFn,
        },
        taskID: this.#id,
        componentIdx: this.#componentIdx,
      });
      
      const ready = readyFn();
      if (ready && ASYNC_DETERMINISM === 'random') {
        const coinFlip = _coinFlip();
        if (coinFlip) { return true }
      }
      
      const keepGoing = await this.immediateSuspend({ cancellable, readyFn });
      return keepGoing;
    }
    
    async immediateSuspend(opts) { // NOTE: equivalent to thread.suspend()
    // TODO(threads): store readyFn on the thread
    const { cancellable, readyFn } = opts;
    _debugLog('[AsyncTask#immediateSuspend()] args', { cancellable, readyFn });
    
    const pendingCancelled = this.deliverPendingCancel({ cancellable });
    if (pendingCancelled) { return false; }
    
    const cstate = getOrCreateAsyncState(this.#componentIdx);
    const keepGoing = await cstate.suspendTask({ task: this, readyFn });
    return keepGoing;
  }
  
  deliverPendingCancel(opts) {
    const { cancellable } = opts;
    _debugLog('[AsyncTask#deliverPendingCancel()]', {
      args: { cancellable },
      taskID: this.#id,
      componentIdx: this.#componentIdx,
    });
    
    if (cancellable && this.#state === AsyncTask.State.PENDING_CANCEL) {
      this.#state = AsyncTask.State.CANCEL_DELIVERED;
      return true;
    }
    
    return false;
  }
  
  isCancelled() { return this.cancelled }
  
  cancel(args) {
    _debugLog('[AsyncTask#cancel()] args', { });
    if (this.taskState() !== AsyncTask.State.CANCEL_DELIVERED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}] invalid task state [${this.taskState()}] for cancellation`);
    }
    if (this.borrowedHandles.length > 0) { throw new Error('task still has borrow handles'); }
    this.cancelled = true;
    this.onResolve(args?.error ?? new Error('task cancelled'));
    this.#state = AsyncTask.State.RESOLVED;
  }
  
  onResolve(taskValue) {
    const handlers = this.#onResolveHandlers;
    this.#onResolveHandlers = [];
    for (const f of handlers) {
      try {
        f(taskValue);
      } catch (err) {
        _debugLog("[AsyncTask#onResolve] error during task resolve handler", err);
        throw err;
      }
    }
    
    if (this.#parentSubtask) {
      const meta = this.#parentSubtask.getCallMetadata();
      // Run the rturn fn if it has not already been called -- this *should* have happened in
      // `task.return`, but some paths do not go through task.return (e.g. async lower of sync fn
      // which goes through prepare + async-start-call)
      if (meta.returnFn && !meta.returnFnCalled) {
        _debugLog('[AsyncTask#onResolve()] running returnFn', {
          componentIdx: this.#componentIdx,
          taskID: this.#id,
          subtaskID: this.#parentSubtask.id(),
        });
        const memory = meta.getMemoryFn();
        meta.returnFn.apply(null, [taskValue, meta.resultPtr]);
        meta.returnFnCalled = true;
      }
    }
    
    if (this.#postReturnFn) {
      _debugLog('[AsyncTask#onResolve()] running post return ', {
        componentIdx: this.#componentIdx,
        taskID: this.#id,
      });
      try {
        this.#postReturnFn(taskValue);
      } catch (err) {
        _debugLog("[AsyncTask#onResolve] error during task resolve handler", err);
        throw err;
      }
    }
    
    if (this.#parentSubtask) {
      this.#parentSubtask.onResolve(taskValue);
    }
  }
  
  registerOnResolveHandler(f) {
    this.#onResolveHandlers.push(f);
  }
  
  isRejected() { return this.#rejected; }
  
  isErrored() { return this.#errored; }
  setErrored(err) { this.#errored = err; }
  
  reject(taskErr) {
    _debugLog('[AsyncTask#reject()] args', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
      parentSubtask: this.#parentSubtask,
      parentSubtaskID: this.#parentSubtask?.id(),
      entryFnName: this.entryFnName(),
      callbackFnName: this.#callbackFnName,
      errMsg: taskErr.message,
    });
    
    if (this.isResolvedState() || this.#rejected) { return; }
    
    this.#rejected = true;
    this.cancelRequested = true;
    this.#state = AsyncTask.State.PENDING_CANCEL;
    const cancelled = this.deliverPendingCancel({ cancellable: true });
    
    // TODO: do cleanup here to reset the machinery so we can run again?
    
    this.cancel({ error: taskErr });
  }
  
  resolve(results) {
    _debugLog('[AsyncTask#resolve()] args', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
      entryFnName: this.entryFnName(),
      callbackFnName: this.#callbackFnName,
    });
    
    if (this.#state === AsyncTask.State.RESOLVED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}]  is already resolved (did you forget to wait for an import?)`);
    }
    
    if (this.borrowedHandles.length > 0) {
      throw new Error('task still has borrow handles');
    }
    
    this.#state = AsyncTask.State.RESOLVED;
    
    switch (results.length) {
      case 0:
      this.onResolve(undefined);
      break;
      case 1:
      this.onResolve(results[0]);
      break;
      default:
      _debugLog('[AsyncTask#resolve()] unexpected number of results', {
        componentIdx: this.#componentIdx,
        results,
        taskID: this.#id,
        subtaskID: this.#parentSubtask?.id(),
        entryFnName: this.#entryFnName,
        callbackFnName: this.#callbackFnName,
      });
      throw new Error('unexpected number of results');
    }
  }
  
  exit(args) {
    _debugLog('[AsyncTask#exit()]', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
    });
    
    if (this.#exited)  { throw new Error("task has already exited"); }
    
    if (this.#state !== AsyncTask.State.RESOLVED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}] exited without resolution`);
    }
    
    if (this.borrowedHandles > 0) {
      throw new Error('task [${this.#id}] exited without clearing borrowed handles');
    }
    
    const state = getOrCreateAsyncState(this.#componentIdx);
    if (!state) { throw new Error('missing async state for component [' + this.#componentIdx + ']'); }
    
    // Exempt the host from exclusive lock check
    if (this.#componentIdx !== -1 && !args?.skipExclusiveLockCheck) {
      if (this.needsExclusiveLock() && !state.isExclusivelyLocked()) {
        throw new Error(`task [${this.#id}] exit: component [${this.#componentIdx}] should have been exclusively locked`);
      }
    }
    
    state.exclusiveRelease();
    
    for (const f of this.#onExitHandlers) {
      try {
        f();
      } catch (err) {
        console.error("error during task exit handler", err);
        throw err;
      }
    }
    
    this.#exited = true;
    clearCurrentTask(this.#componentIdx, this.id());
  }
  
  needsExclusiveLock() {
    return !this.#isAsync || this.hasCallback();
  }
  
  createSubtask(args) {
    _debugLog('[AsyncTask#createSubtask()] args', args);
    const { componentIdx, childTask, callMetadata, fnName, isAsync, isManualAsync } = args;
    
    const cstate = getOrCreateAsyncState(this.#componentIdx);
    if (!cstate) {
      throw new Error(`invalid/missing async state for component idx [${componentIdx}]`);
    }
    
    const waitable = new Waitable({
      componentIdx: this.#componentIdx,
      target: `subtask (internal ID [${this.#id}])`,
    });
    
    const newSubtask = new AsyncSubtask({
      componentIdx,
      childTask,
      parentTask: this,
      callMetadata,
      isAsync,
      isManualAsync,
      fnName,
      waitable,
    });
    this.#subtasks.push(newSubtask);
    newSubtask.setTarget(`subtask (internal ID [${newSubtask.id()}], waitable [${waitable.idx()}], component [${componentIdx}])`);
    waitable.setIdx(cstate.handles.insert(newSubtask));
    waitable.setTarget(`waitable for subtask (waitable id [${waitable.idx()}], subtask internal ID [${newSubtask.id()}])`);
    return newSubtask;
  }
  
  getLatestSubtask() {
    return this.#subtasks.at(-1);
  }
  
  getSubtaskByWaitableRep(rep) {
    if (rep === undefined) { throw new TypeError('missing rep'); }
    return this.#subtasks.find(s => s.waitableRep() === rep);
  }
  
  currentSubtask() {
    _debugLog('[AsyncTask#currentSubtask()]');
    if (this.#subtasks.length === 0) { return undefined; }
    return this.#subtasks.at(-1);
  }
  
  removeSubtask(subtask) {
    if (this.#subtasks.length === 0) {
      throw new Error('cannot end current subtask: no current subtask');
    }
    this.#subtasks = this.#subtasks.filter(t => t !== subtask);
    return subtask;
  }
}

const ASYNC_EVENT_CODE = {
  NONE: 0,
  SUBTASK: 1,
  STREAM_READ: 2,
  STREAM_WRITE: 3,
  FUTURE_READ: 4,
  FUTURE_WRITE: 5,
  TASK_CANCELLED: 6,
};

function getCurrentTask(componentIdx, taskID) {
  let usedGlobal = false;
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing component idx'); // TODO(fix)
    // componentIdx = ASYNC_CURRENT_COMPONENT_IDXS.at(-1);
    // usedGlobal = true;
  }
  
  const taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  if (taskMetas === undefined || taskMetas.length === 0) { return undefined; }
  
  if (taskID) {
    return taskMetas.find(meta => meta.task.id() === taskID);
  }
  
  const taskMeta = taskMetas[taskMetas.length - 1];
  if (!taskMeta || !taskMeta.task) { return undefined; }
  
  return taskMeta;
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

function toUint64(val) {
  const converted = BigInt(val)
  
  return BigInt.asUintN(64, converted);
}


function toInt32(val) {
  
  return val >> 0;
}


function toUint32(val) {
  
  return val >>> 0;
}


function _utf16AllocateAndEncode(str, realloc, memory) {
  const len = str.length;
  const ptr = realloc(0, 0, 2, len * 2);
  const out = new Uint16Array(memory.buffer, ptr, len);
  let i = 0;
  if (isLE) {
    while (i < len) { out[i] = str.charCodeAt(i++); }
  } else {
    while (i < len) {
      const ch = str.charCodeAt(i);
      out[i++] = (ch & 0xff) << 8 | ch >>> 8;
    }
  }
  return { ptr, len, codepoints: [...str].length };
}

const TEXT_ENCODER_UTF8 = new TextEncoder();

function _utf8AllocateAndEncode(s, realloc, memory) {
  if (typeof s !== 'string') {
    throw new TypeError('expected a string, received [' + typeof s + ']');
  }
  if (s.length === 0) { return { ptr: 1, len: 0 }; }
  let buf = TEXT_ENCODER_UTF8.encode(s);
  let ptr = realloc(0, 0, 1, buf.length);
  new Uint8Array(memory.buffer).set(buf, ptr);
  const res = { ptr, len: buf.length, codepoints: [...s].length };
  return res;
}


const T_FLAG = 1 << 30;

function rscTableCreateOwn(table, rep) {
  const free = table[0] & ~T_FLAG;
  table._createdReps.add(rep);
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove(table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0) {
    throw new TypeError("Invalid handle");
  }
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}

let curResourceBorrows = [];

function taskReturn(ctx) {
  const {
    componentIdx,
    getMemoryFn,
    memoryIdx,
    callbackFnIdx,
    liftFns,
    lowerFns,
    stringEncoding,
  } = ctx;
  const params = [...arguments].slice(1);
  const memory = getMemoryFn();
  let useDirectParams = ctx.useDirectParams;
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('failed to retrieve current task metadata'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing current task in metadata'); }
  
  _debugLog('[taskReturn()] args', {
    componentIdx,
    taskID: task.id(),
    subtaskID: task.getParentSubtask()?.id(),
    callbackFnIdx,
    memoryIdx,
    liftFns,
    lowerFns,
    params,
  });
  
  // If we are in a subtask, and have a fused helper function provided to use
  // via PrepareCall, we can use that function rather than performing lifting manually.
  //
  // See also documentation on `HostIntrinsic::PrepareCall`
  const subtaskCallMetadata = task.getParentSubtask()?.getCallMetadata();
  if (subtaskCallMetadata?.returnFn && !subtaskCallMetadata.returnFnCalled) {
    _debugLog('[taskReturn()] calling return fn on subtask', {
      componentIdx,
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      returnFnParams: [...params, subtaskCallMetadata.resultPtr],
    });
    const res = subtaskCallMetadata.returnFn.apply(null, [...params, subtaskCallMetadata.resultPtr]);
    subtaskCallMetadata.returnFnCalled = true;
    task.resolve([]);
    return;
  }
  
  const expectedMemoryIdx = task.getReturnMemoryIdx();
  if (expectedMemoryIdx !== null && memoryIdx !== null && expectedMemoryIdx !== memoryIdx) {
    _debugLog("[taskReturn()] mismatched memory indices", { expectedMemoryIdx, memoryIdx });
    throw new Error('task.return memory [' + memoryIdx + '] does not match task [' + expectedMemoryIdx + ']');
  }
  
  task.callbackFnIdx = callbackFnIdx;
  
  if (!memory && liftFns.length > 4) {
    _debugLog("[taskReturn()] memory not present for max async flat lifts");
    throw new Error('memory must be present if more than max async flat lifts are performed');
  }
  
  let liftCtx = { memory, useDirectParams, params, componentIdx, stringEncoding };
  if (!useDirectParams) {
    if (!ctx.memory) {
      _debugLog('missing memory despite indirect param usage', { useDirectParams, liftCtx, ctx });
      throw new Error('missing memory despite indirect param usage');
    }
    liftCtx.storagePtr = params[0];
    liftCtx.storageLen = params[1];
  }
  
  const liftedResults = [];
  _debugLog('[taskReturn()] lifting results out of memory', { liftCtx });
  for (const liftFn of liftFns) {
    if (liftCtx.storageLen !== undefined && liftCtx.storageLen <= 0) {
      _debugLog(`[taskReturn()] ran out of range while writing storageLen = [${liftCtx.storageLen}]`);
      throw new Error('ran out of storage while writing');
    }
    const [ val, newLiftCtx ] = liftFn(liftCtx);
    liftCtx = newLiftCtx;
    liftedResults.push(val);
  }
  
  task.resolve(liftedResults);
}

function subtaskDrop(componentIdx, subtaskWaitableRep) {
  _debugLog('[subtaskDrop()] args', { componentIdx, subtaskWaitableRep });
  
  const cstate = getOrCreateAsyncState(componentIdx);
  if (!cstate.mayLeave) { throw new Error('component is not marked as may leave, cannot be cancelled'); }
  
  const subtask = cstate.handles.remove(subtaskWaitableRep);
  if (!subtask) { throw new Error('missing/invalid subtask specified for drop in component instance'); }
  
  subtask.drop();
}


function subtaskCancel(componentIdx, isAsync) {
  _debugLog('[subtaskCancel()] args', { componentIdx, isAsync });
  
  const state = getOrCreateAsyncState(componentIdx);
  if (!state.mayLeave) { throw new Error('component instance is not marked as may leave, cannot be cancelled'); }
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('invalid/missing async task meta'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing async task'); }
  
  if (task.sync && !task.alwaysTaskReturn) {
    throw new Error('cannot cancel sync tasks without always task return set');
  }
  if (!task.cancelRequested) { throw new Error('task cancellation has not been requested'); }
  if (task.borrowedHandles.length > 0) { throw new Error('task still has borrow handles'); }
  if (task.returnCalls > 0) { throw new Error('cannot cancel task that has already returned a value'); }
  if (task.cancelled) { throw new Error('cannot cancel task that has already been cancelled'); }
  
  task.cancelled = true;
}

function taskCancel(componentIdx) {
  _debugLog('[taskCancel()] args', { componentIdx, isAsync });
  
  const state = getOrCreateAsyncState(componentIdx);
  if (!state.mayLeave) { throw new Error('component instance is not marked as may leave, cannot be cancelled'); }
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('invalid/missing async task meta'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing async task'); }
  
  if (task.sync && !task.alwaysTaskReturn) {
    throw new Error('cannot cancel sync tasks without always task return set');
  }
  
  task.cancel();
}

function createNewCurrentTask(args) {
  _debugLog('[createNewCurrentTask()] args', args);
  const {
    componentIdx,
    isAsync,
    isManualAsync,
    preserveFutureResult,
    entryFnName,
    parentSubtaskID,
    callbackFnName,
    getCallbackFn,
    getParamsFn,
    stringEncoding,
    errHandling,
    getCalleeParamsFn,
    resultPtr,
    callingWasmExport,
  } = args;
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing/invalid component instance index while starting task');
  }
  let taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  const callbackFn = getCallbackFn ? getCallbackFn() : null;
  
  const newTask = new AsyncTask({
    componentIdx,
    isAsync,
    isManualAsync,
    preserveFutureResult,
    entryFnName,
    callbackFn,
    callbackFnName,
    stringEncoding,
    getCalleeParamsFn,
    resultPtr,
    errHandling,
  });
  
  const newTaskID = newTask.id();
  const newTaskMeta = { id: newTaskID, componentIdx, task: newTask };
  
  // NOTE: do not track host tasks
  ASYNC_CURRENT_TASK_IDS.push(newTaskID);
  ASYNC_CURRENT_COMPONENT_IDXS.push(componentIdx);
  
  if (!taskMetas) {
    taskMetas = [newTaskMeta];
    ASYNC_TASKS_BY_COMPONENT_IDX.set(componentIdx, [newTaskMeta]);
  } else {
    taskMetas.push(newTaskMeta);
  }
  
  return [newTask, newTaskID];
}
const ASYNC_BLOCKED_CODE = 0xFFFF_FFFF;
async function _driverLoop(args) {
  _debugLog('[_driverLoop()] args', args);
  const {
    componentState,
    task,
    fnName,
    isAsync,
  } = args;
  let callbackResult = args.callbackResult;
  
  const callbackFnName = task.getCallbackFnName();
  const componentIdx = task.componentIdx();
  
  if (callbackResult instanceof Promise) {
    throw new Error("callbackResult should be a value, not a promise");
  }
  
  if (callbackResult === undefined) {
    throw new Error("callback result should never be undefined");
  }
  
  let callbackCode;
  let waitableSetRep;
  let unpacked;
  try {
    if (!(_typeCheckValidI32(callbackResult))) {
      throw new Error('invalid callback result [' + callbackResult + '], not a number');
    }
    
    unpacked = unpackCallbackResult(callbackResult);
    callbackCode = unpacked[0];
    waitableSetRep = unpacked[1];
  } catch(err) {
    console.error("failed to unpack callback result", err);
    throw err;
  }
  
  if (callbackCode < 0 || callbackCode > 3) {
    throw new Error('invalid async return value, outside callback code range');
  }
  
  const cstate = getOrCreateAsyncState(componentIdx);
  
  let eventCode;
  let index;
  let result;
  let asyncRes;
  let wset;
  try {
    while (true) {
      if (callbackCode !== 0) { componentState.exclusiveRelease(); }
      
      switch (callbackCode) {
        case 0: // EXIT
        _debugLog('[_driverLoop()] async exit indicated', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id()
        });
        task.exit({ skipExclusiveLockCheck: true });
        return;
        
        case 1: // YIELD
        _debugLog('[_driverLoop()] yield', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id()
        });
        asyncRes = await task.yieldUntil({
          cancellable: true,
          readyFn: () => !componentState.isExclusivelyLocked(),
        });
        _debugLog('[_driverLoop()] finished yield', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          asyncRes,
        });
        break;
        
        case 2: // WAIT for a given waitable set
        _debugLog('[_driverLoop()] waiting for event', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          waitableSetRep,
          waitableSetTargets: cstate.handles.get(waitableSetRep).targets(),
        });
        
        wset = cstate.handles.get(waitableSetRep);
        if (!(wset instanceof WaitableSet)) {
          throw new Error(`non-waitable set returned from component state handles @ [${waitableSetRep}]`);
        }
        
        asyncRes = await wset.waitUntil({
          readyFn: () => !componentState.isExclusivelyLocked(),
          task,
          cancellable: true,
        });
        
        _debugLog('[_driverLoop()] finished waiting for event', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          waitableSetRep,
          asyncRes,
        });
        
        break;
        
        default:
        throw new Error(`Unrecognized async function result [${ret}]`);
      }
      
      componentState.exclusiveLock();
      
      // If the task failed via any means, leave early and reject.
      if (task.isRejected()) {
        _debugLog('[_driverLoop()] detected task rejection, leaving early');
        return;
      }
      
      if (asyncRes.code === undefined) { throw new Error("missing event code from event"); }
      if (asyncRes.payload0 === undefined) { throw new Error("missing payload0 from event"); }
      if (asyncRes.payload1 === undefined) { throw new Error("missing payload1 from event"); }
      
      eventCode = asyncRes.code; // async event enum code
      index = asyncRes.payload0; // varies (e.g. idx of related waitable set)
      result = asyncRes.payload1; // varies (e.g. task state)
      asyncRes = null;
      
      _debugLog('[_driverLoop()] performing callback', {
        fnName,
        componentIdx,
        taskID: task.id(),
        callbackFnName,
        eventCode,
        index,
        result
      });
      
      const callbackRes = await task.runCallbackFn(
      toInt32(eventCode),
      toInt32(index),
      toInt32(result),
      );
      
      unpacked = unpackCallbackResult(callbackRes);
      callbackCode = unpacked[0];
      waitableSetRep = unpacked[1];
      
      _debugLog('[_driverLoop()] callback result unpacked', {
        fnName,
        componentIdx,
        callbackFnName,
        callbackRes,
        callbackCode,
        waitableSetRep,
      });
    }
  } catch (err) {
    _debugLog('[_driverLoop()] error during async driver loop', {
      fnName,
      callbackFnName,
      componentIdx,
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      parentTaskID: task.getParentSubtask()?.getParentTask()?.id(),
      event: {
        eventCode,
        index,
        result,
      },
      err,
    });
    task.setErrored(err);
    task.reject(err);
  }
}

async function _lowerImport(args) {
  const params = [...arguments].slice(1);
  _debugLog('[_lowerImport()] args', { args, params });
  const {
    functionIdx,
    componentIdx,
    isAsync,
    isManualAsync,
    paramLiftFns,
    resultLowerFns,
    hasResultPointer,
    funcTypeIsAsync,
    metadata,
    memoryIdx,
    getMemoryFn,
    getReallocFn,
    stringEncoding,
    importFn,
  } = args;
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('invalid/missing async task meta'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing async task'); }
  
  const cstate = getOrCreateAsyncState(componentIdx);
  
  // TODO: re-enable this check -- postReturn can call imports though,
  // and that breaks things.
  //
  // if (!cstate.mayLeave) {
    //     throw new Error(`cannot leave instance [${componentIdx}]`);
    // }
    
    if (!task.mayBlock() && funcTypeIsAsync && !isAsync) {
      throw new Error("non async exports cannot synchronously call async functions");
    }
    
    // If there is an existing task, this should be part of a subtask
    const memory = getMemoryFn();
    // Canonical ABI lower appends result storage as a trailing
    // param when async lower has any flat result, or sync lower
    // has more than one flat result.
    const resultPtr = hasResultPointer ? params[params.length - 1] : undefined;
    const subtask = task.createSubtask({
      componentIdx,
      parentTask: task,
      fnName: importFn.fnName,
      isAsync,
      isManualAsync,
      callMetadata: {
        memoryIdx,
        memory,
        realloc: getReallocFn?.(),
        getReallocFn,
        resultPtr,
        lowers: resultLowerFns,
        stringEncoding,
      }
    });
    task.setReturnMemoryIdx(memoryIdx);
    task.setReturnMemory(getMemoryFn());
    
    subtask.onStart();
    
    // If dealing with a sync lowered sync function, we can directly return results
    //
    // TODO(breaking): remove once we get rid of manual async import specification,
    // as func types cannot be detected in that case only (and we don't need that w/ p3)
    if (!isManualAsync && !isAsync && !funcTypeIsAsync) {
      const res = importFn(...params);
      // TODO(breaking): remove once we get rid of manual async import specification,
      // as func types cannot be detected in that case only (and we don't need that w/ p3)
      if (!funcTypeIsAsync && !subtask.isReturned()) {
        throw new Error('post-execution subtasks must either be async or returned');
      }
      return subtask.getResult();
    }
    
    // Sync-lowered async functions requires async behavior because the callee *can* block,
    // but this call must *act* synchronously and return immediately with the result
    // (i.e. not returning until the work is done)
    //
    // TODO(breaking): remove checking for manual async specification here, once we can go p3-only
    //
    if (!isManualAsync && !isAsync && funcTypeIsAsync) {
      const { promise, resolve } = new Promise();
      queueMicrotask(async () => {
        if (!subtask.isResolvedState()) {
          await task.suspendUntil({ readyFn: () => task.isResolvedState() });
        }
        resolve(subtask.getResult());
      });
      return promise;
    }
    
    // NOTE: at this point we know that we are working with an async lowered import
    
    subtask.setOnProgressFn(() => {
      subtask.setPendingEvent(() => {
        if (subtask.isResolved()) { subtask.deliverResolve(); }
        const event = {
          code: ASYNC_EVENT_CODE.SUBTASK,
          payload0: subtask.waitableRep(),
          payload1: subtask.getStateNumber(),
        }
        return event;
      });
    });
    
    // This is a hack to maintain backwards compatibility with
    // manually-specified async imports, used in wasm exports that are
    // not actually async (but are specified as so).
    //
    // This is not normal p3 sync behavior but instead anticipating that
    // the caller that is doing manual async will be waiting for a promise that
    // resolves to the *actual* result.
    //
    // TODO(breaking): remove once manually specified async is removed
    //
    // There are a few cases:
    // 1. sync function with async types (e.g. `f: func() -> stream<u32>`)
    // 2. async function with async types (e.g. `f: async func() -> stream<u32>`)
    // 3. async function with sync types (e.g. `f: async func() -> list<u32>`)
    // 4. sync function with non-async types (e.g. `f: func() -> list<u32>`)
    //
    // This hack *only* applies to 4 -- the case where an async JS host function
    // is supplied to a Wasm export which does *not* need to do any async abi
    // lifting/lowering (async ABI did not exist when JSPI integratiton was
    // initially merged to enable asynchronously returning values from the host)
    //
    const requiresManualAsyncResult = !isAsync && !funcTypeIsAsync && isManualAsync;
    let manualAsyncResult;
    if (requiresManualAsyncResult) {
      manualAsyncResult = promiseWithResolvers();
    }
    
    // Build a response that *may* resolve quickly
    
    queueMicrotask(async () => {
      try {
        _debugLog('[_lowerImport()] calling lowered import', { importFn, params });
        await importFn(...params);
        if (requiresManualAsyncResult) {
          manualAsyncResult.resolve(subtask.getResult());
        }
      } catch (err) {
        _debugLog("[_lowerImport()] import fn error:", err);
        if (requiresManualAsyncResult) {
          manualAsyncResult.reject(err);
        }
        throw err;
      }
    });
    
    if (requiresManualAsyncResult) { return manualAsyncResult.promise; }
    
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const subtaskState = subtask.getStateNumber();
        if (subtaskState < 0 || subtaskState >= 2**4) {
          // throw new Error('invalid subtask state, out of valid range');
          reject(new Error('invalid subtask state, out of valid range'));
        }
        let res;
        // An async-lowered import whose callee resolved synchronously returns
        // [Subtask.State.RETURNED] only an no subtask handle is exposed.
        if (subtask.isReturned()) {
          if (!subtask.resolveDelivered()) {
            subtask.deliverResolve();
          }
          const removed = cstate.handles.remove(subtask.waitableRep());
          if (removed !== subtask) {
            throw new Error('subtask handle cleanup removed unexpected entry');
            reject(new Error('subtask handle cleanup removed unexpected entry'));
          }
          res = subtaskState;
        } else {
          res = Number(subtask.waitableRep()) << 4 | subtaskState;
        }
        resolve(res);
      }, 0);
    });
  }
  
  function _lowerImportBackwardsCompat(args) {
    const params = [...arguments].slice(1);
    _debugLog('[_lowerImportBackwardsCompat()] args', { args, params });
    const {
      functionIdx,
      componentIdx,
      isAsync,
      isManualAsync,
      paramLiftFns,
      resultLowerFns,
      hasResultPointer,
      funcTypeIsAsync,
      metadata,
      memoryIdx,
      getMemoryFn,
      getReallocFn,
      importFn,
      stringEncoding,
    } = args;
    
    let meta = _getGlobalCurrentTaskMeta(componentIdx);
    let createdTask;
    
    // Some components depend on initialization logic (i.e. `_initialize` or some such
    // core wasm export) that is embedded in the component, but is not executed or wizer'd
    // away before the transpiled component is attempted to be used.
    //
    // These components execut their initialization logic *when they are imported* in the
    // transpiled context -- so we may get a call to an export that is lowered without going
    // through `CallWasm` or `CallInterface`.
    //
    if (!meta) {
      if (funcTypeIsAsync || (isAsync && !isManualAsync)) {
        throw new Error('p3 async wasm exports cannot use backwards compat auto-task init');
      }
      
      const [newTask, newTaskID] = createNewCurrentTask({
        componentIdx,
        isAsync,
        isManualAsync,
        callingWasmExport: false,
      });
      createdTask = newTask;
      
      // Since we're managing the task creation ourselves we must clear ourselves
      createdTask.registerOnResolveHandler(() => {
        _clearCurrentTask({
          taskID: task.id(),
          componentIdx: task.componentIdx(),
        });
      });
      
      _setGlobalCurrentTaskMeta({
        componentIdx,
        taskID: newTaskID,
      });
      
      meta = _getGlobalCurrentTaskMeta(componentIdx);
    }
    
    const { taskID } = meta;
    
    const taskMeta = getCurrentTask(componentIdx, taskID);
    if (!taskMeta) {
      throw new Error('invalid/missing async task meta');
    }
    
    const task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing async task'); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    
    // TODO: re-enable this check -- postReturn can call imports though,
    // and that breaks things.
    //
    // if (!cstate.mayLeave) {
      //     throw new Error(`cannot leave instance [${componentIdx}]`);
      // }
      
      if (!task.mayBlock() && funcTypeIsAsync && !isAsync) {
        throw new Error("non async exports cannot synchronously call async functions");
      }
      
      // If there is an existing task, this should be part of a subtask
      const memory = getMemoryFn();
      // Canonical ABI lower appends result storage as a trailing
      // param when async lower has any flat result, or sync lower
      // has more than one flat result.
      const resultPtr = hasResultPointer ? params[params.length - 1] : undefined;
      const subtask = task.createSubtask({
        componentIdx,
        parentTask: task,
        fnName: importFn.fnName,
        isAsync,
        isManualAsync,
        callMetadata: {
          memoryIdx,
          memory,
          realloc: getReallocFn?.(),
          getReallocFn,
          resultPtr,
          lowers: resultLowerFns,
          stringEncoding,
        }
      });
      task.setReturnMemoryIdx(memoryIdx);
      task.setReturnMemory(getMemoryFn());
      
      subtask.onStart();
      
      // If dealing with a sync lowered sync function, we can directly return results
      //
      // TODO(breaking): remove once we get rid of manual async import specification,
      // as func types cannot be detected in that case only (and we don't need that w/ p3)
      if (!isManualAsync && !isAsync && !funcTypeIsAsync) {
        if (createdTask) { createdTask.enterSync(); }
        
        const res = importFn(...params);
        
        // TODO(breaking): remove once we get rid of manual async import specification,
        // as func types cannot be detected in that case only (and we don't need that w/ p3)
        if (!funcTypeIsAsync && !subtask.isReturned()) {
          throw new Error('post-execution subtasks must either be async or returned');
        }
        
        const syncRes = subtask.getResult();
        if (createdTask) { createdTask.resolve([syncRes]); }
        
        return syncRes;
      }
      
      // Sync-lowered async functions requires async behavior because the callee *can* block,
      // but this call must *act* synchronously and return immediately with the result
      // (i.e. not returning until the work is done)
      //
      // TODO(breaking): remove checking for manual async specification here, once we can go p3-only
      //
      if (!isManualAsync && !isAsync && funcTypeIsAsync) {
        const { promise, resolve } = new Promise();
        queueMicrotask(async () => {
          if (!subtask.isResolvedState()) {
            await task.suspendUntil({ readyFn: () => task.isResolvedState() });
          }
          resolve(subtask.getResult());
        });
        return promise;
      }
      
      // NOTE: at this point we know that we are working with an async lowered import
      
      const subtaskState = subtask.getStateNumber();
      if (subtaskState < 0 || subtaskState >= 2**4) {
        throw new Error('invalid subtask state, out of valid range');
      }
      
      subtask.setOnProgressFn(() => {
        subtask.setPendingEvent(() => {
          if (subtask.isResolved()) { subtask.deliverResolve(); }
          const event = {
            code: ASYNC_EVENT_CODE.SUBTASK,
            payload0: subtask.waitableRep(),
            payload1: subtask.getStateNumber(),
          }
          return event;
        });
      });
      
      // This is a hack to maintain backwards compatibility with
      // manually-specified async imports, used in wasm exports that are
      // not actually async (but are specified as so).
      //
      // This is not normal p3 sync behavior but instead anticipating that
      // the caller that is doing manual async will be waiting for a promise that
      // resolves to the *actual* result.
      //
      // TODO(breaking): remove once manually specified async is removed
      //
      // There are a few cases:
      // 1. sync function with async types (e.g. `f: func() -> stream<u32>`)
      // 2. async function with async types (e.g. `f: async func() -> stream<u32>`)
      // 3. async function with sync types (e.g. `f: async func() -> list<u32>`)
      // 4. sync function with non-async types (e.g. `f: func() -> list<u32>`)
      //
      // This hack *only* applies to 4 -- the case where an async JS host function
      // is supplied to a Wasm export which does *not* need to do any async abi
      // lifting/lowering (async ABI did not exist when JSPI integratiton was
      // initially merged to enable asynchronously returning values from the host)
      //
      const requiresManualAsyncResult = !isAsync && !funcTypeIsAsync && isManualAsync;
      let manualAsyncResult;
      if (requiresManualAsyncResult) {
        manualAsyncResult = promiseWithResolvers();
      }
      
      queueMicrotask(async () => {
        try {
          _debugLog('[_lowerImportBackwardsCompat()] calling lowered import', { importFn, params });
          if (createdTask) { await createdTask.enter(); }
          
          const asyncRes = await importFn(...params);
          if (requiresManualAsyncResult) {
            manualAsyncResult.resolve(subtask.getResult());
          }
          
          if (createdTask) { createdTask.resolve([asyncRes]); }
          
          
        } catch (err) {
          _debugLog("[_lowerImportBackwardsCompat()] import fn error:", err);
          if (requiresManualAsyncResult) {
            manualAsyncResult.reject(err);
          }
          throw err;
        }
      });
      
      if (requiresManualAsyncResult) { return manualAsyncResult.promise; }
      
      return Number(subtask.waitableRep()) << 4 | subtaskState;
    }
    
    class WaitableSet {
      #componentIdx;
      #waitables = [];
      #pendingEvent = null;
      #waiting = 0;
      
      target;
      
      constructor(componentIdx) {
        if (componentIdx === undefined) { throw new TypeError("missing/invalid component idx"); }
        this.#componentIdx = componentIdx;
        this.target = `component [${this.#componentIdx}] waitable set`;
      }
      
      componentIdx() { return this.#componentIdx; }
      
      numWaitables() { return this.#waitables.length; }
      numWaiting() { return this.#waiting; }
      
      incrementNumWaiting(n) { this.#waiting += n ?? 1; }
      decrementNumWaiting(n) { this.#waiting -= n ?? 1; }
      
      targets() { return this.#waitables.map(w => w.target); }
      
      setTarget(tgt) { this.target = tgt; }
      
      shuffleWaitables() {
        this.#waitables = this.#waitables
        .map(value => ({ value, sort: Math.random() }))
        .sort((a, b) => a.sort - b.sort)
        .map(({ value }) => value);
      }
      
      removeWaitable(waitable) {
        const existing = this.#waitables.find(w => w === waitable);
        if (!existing) { return undefined; }
        this.#waitables = this.#waitables.filter(w => w !== waitable);
        return waitable;
      }
      
      addWaitable(waitable) {
        this.removeWaitable(waitable);
        this.#waitables.push(waitable);
      }
      
      hasPendingEvent() {
        _debugLog('[WaitableSet#hasPendingEvent()] args', {
          componentIdx: this.#componentIdx,
          waitableSet: this,
          waitableSetTargets: this.targets(),
        });
        const waitable = this.#waitables.find(w => w.hasPendingEvent());
        return waitable !== undefined;
      }
      
      getPendingEvent() {
        _debugLog('[WaitableSet#getPendingEvent()] args', {
          componentIdx: this.#componentIdx,
          waitableSet: this,
        });
        for (const waitable of this.#waitables) {
          if (!waitable.hasPendingEvent()) { continue; }
          const event = waitable.getPendingEvent();
          _debugLog('[WaitableSet#getPendingEvent()] found pending event', {
            waitable,
            event,
          });
          return event;
        }
        throw new Error('no waitables had a pending event');
      }
      
      async waitUntil(opts) {
        _debugLog('[WaitableSet#waitUntil()] args', { opts });
        // TODO(threads): this task should be the thread
        const { readyFn, task, cancellable } = opts;
        
        let event;
        
        this.incrementNumWaiting();
        
        const keepGoing = await task.suspendUntil({
          readyFn: () => {
            const hasPendingEvent = this.hasPendingEvent();
            const ready = readyFn();
            return ready && hasPendingEvent;
          },
          cancellable,
        });
        
        if (keepGoing) {
          event = this.getPendingEvent();
        } else {
          event = {
            code: ASYNC_EVENT_CODE.TASK_CANCELLED,
            payload0: 0,
            payload1: 0,
          };
        }
        
        this.decrementNumWaiting();
        
        return event;
      }
      
    }
    
    function waitableSetNew(componentIdx) {
      _debugLog('[waitableSetNew()] args', { componentIdx });
      
      const state = getOrCreateAsyncState(componentIdx);
      if (!state) {throw new Error(`missing async state for component idx [${componentIdx}]`); }
      
      const wset = new WaitableSet(componentIdx);
      const rep = state.handles.insert(wset);
      if (typeof rep !== 'number') { throw new Error(`invalid/missing waitable set rep [${rep}]`); }
      
      _debugLog('[waitableSetNew()] created waitable set', { componentIdx, rep });
      return rep;
    }
    
    function waitableSetPoll(ctx, waitableSetRep, resultPtr) {
      const { componentIdx, memoryIdx, getMemoryFn, isAsync, isCancellable } = ctx;
      _debugLog('[waitableSetPoll()] args', {
        componentIdx,
        memoryIdx,
        waitableSetRep,
        resultPtr,
      });
      
      const taskMeta = getCurrentTask(componentIdx);
      if (!taskMeta) { throw Error('invalid/missing current task meta'); }
      if (taskMeta.componentIdx !== componentIdx) {
        throw Error('task component idx [' + task.componentIdx + '] != component instance ID [' + componentIdx + ']');
      }
      
      const task = taskMeta.task;
      if (!task) { throw Error('invalid/missing async task in task meta'); }
      
      if (task.componentIdx() !== componentIdx) {
        throw Error(`task component idx [${task.componentIdx()}] does not match generated [${componentIdx}]`);
      }
      
      const cstate = getOrCreateAsyncState(task.componentIdx());
      const wset = cstate.handles.get(waitableSetRep);
      if (!wset) {
        throw new Error(`missing waitable set [${waitableSetRep}] in component [${componentIdx}]`);
      }
      
      let event;
      const cancelDelivered = task.deliverPendingCancel({ cancellable: isCancellable });
      if (cancelDelivered) {
        _debugLog('[waitableSetPoll()] detected cancel delivered', {
          componentIdx,
          waitableSetRep,
        });
        event = { code: ASYNC_EVENT_CODE.TASK_CANCELLED, payload0: 0, payload1: 0 };
      } else if (!wset.hasPendingEvent()) {
        _debugLog('[waitableSetPoll()] no pending event', {
          componentIdx,
          waitableSetRep,
        });
        event = { code: ASYNC_EVENT_CODE.NONE, payload0: 0, payload1: 0 };
      } else {
        _debugLog('[waitableSetPoll()] retrieving waiting pending event', {
          componentIdx,
          waitableSetRep,
        });
        event = wset.getPendingEvent();
      }
      
      const eventCode = _storeEventInComponentMemory({
        event,
        ptr: resultPtr,
        memory: getMemoryFn(),
        componentIdx,
        task,
        memoryIdx,
      });
      
      return eventCode;
    }
    
    function waitableSetDrop(componentIdx, waitableSetRep) {
      _debugLog('[waitableSetDrop()] args', { componentIdx, waitableSetRep });
      const task = getCurrentTask(componentIdx);
      
      if (!task) { throw new Error('invalid/missing async task'); }
      if (task.componentIdx !== componentIdx) {
        throw Error('task component idx [' + task.componentIdx + '] != component instance ID [' + componentIdx + ']');
      }
      
      const state = getOrCreateAsyncState(componentIdx);
      if (!state.mayLeave) { throw new Error('component instance is not marked as may leave, cannot be cancelled'); }
      
      _removeWaitableSet({ state, waitableSetRep });
    }
    
    function _removeWaitableSet(args) {
      _debugLog('[_removeWaitableSet()] args', args);
      const { state, waitableSetRep } = args;
      if (!state) { throw new TypeError("missing component state"); }
      if (!waitableSetRep) { throw new TypeError("missing component waitableSetRep"); }
      
      const ws = state.handles.get(waitableSetRep);
      if (!ws) {
        throw new Error('cannot remove waitable set: no set present with rep [' + waitableSetRep + ']');
      }
      if (ws.hasPendingEvent()) {
        throw new Error('waitable set cannot be removed with pending items remaining');
      }
      
      const waitableSet = state.handles.get(waitableSetRep);
      if (ws.numWaitables() > 0) {
        throw new Error('waitable set still contains waitables');
      }
      if (ws.numWaiting() > 0) {
        throw new Error('waitable set still has other tasks waiting on it');
      }
      
      state.handles.remove(waitableSetRep);
    }
    
    function waitableJoin(componentIdx, waitableRep, waitableSetRep) {
      _debugLog('[waitableJoin()] args', {
        componentIdx,
        waitableSetRep,
        isRemoval: waitableSetRep === 0,
        waitableRep,
      });
      
      const state = getOrCreateAsyncState(componentIdx);
      if (!state) {
        throw new Error(`invalid/missing async state for component instance [${componentIdx}]`);
      }
      
      if (!state.mayLeave) {
        throw new Error('component instance is not marked as may leave, cannot join waitable');
      }
      
      const waitableObj = state.handles.get(waitableRep);
      if (!waitableObj) {
        throw new Error(`missing waitable obj (rep [${waitableRep}]), component idx [${componentIdx}])`);
      }
      const waitable = waitableObj.getWaitable ? waitableObj.getWaitable() : waitableObj;
      if (!waitable.join) {
        throw new Error("invalid waitable object, does not have join()");
      }
      
      const waitableSet = waitableSetRep === 0 ? null : state.handles.get(waitableSetRep);
      if (waitableSetRep !== 0 && !waitableSet) {
        throw new Error(`missing waitable set [${waitableSetRep}] in component idx [${componentIdx}]`);
      }
      
      waitable.join(waitableSet);
    }
    
    function _liftFlatU8(ctx) {
      _debugLog('[_liftFlatU8()] args', { ctx });
      let val;
      
      if (ctx.useDirectParams) {
        if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
        val = ctx.params[0];
        ctx.params = ctx.params.slice(1);
        return [val, ctx];
      }
      
      if (ctx.storageLen !== undefined && ctx.storageLen < 1) {
        throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u8 requires 1 byte)`);
      }
      
      val = new DataView(ctx.memory.buffer).getUint8(ctx.storagePtr, true);
      
      ctx.storagePtr += 1;
      if (ctx.storageLen !== undefined) { ctx.storageLen -= 1; }
      
      return [val, ctx];
    }
    
    
    function _liftFlatU16(ctx) {
      _debugLog('[_liftFlatU16()] args', { ctx });
      let val;
      
      if (ctx.useDirectParams) {
        if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
        val = ctx.params[0];
        ctx.params = ctx.params.slice(1);
        return [val, ctx];
      }
      
      if (ctx.storageLen !== undefined && ctx.storageLen < 2) {
        throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u16 requires 2 bytes)`);
      }
      
      val = new DataView(ctx.memory.buffer).getUint16(ctx.storagePtr, true);
      
      ctx.storagePtr += 2;
      if (ctx.storageLen !== undefined) { ctx.storageLen -= 2; }
      
      const rem = ctx.storagePtr % 2;
      if (rem !== 0) { ctx.storagePtr += (2 - rem); }
      
      return [val, ctx];
    }
    
    
    function _liftFlatU32(ctx) {
      _debugLog('[_liftFlatU32()] args', { ctx });
      let val;
      
      if (ctx.useDirectParams) {
        if (ctx.params.length === 0) { throw new Error('expected at least a single i34 argument'); }
        val = ctx.params[0];
        ctx.params = ctx.params.slice(1);
        return [val, ctx];
      }
      
      if (ctx.storageLen !== undefined && ctx.storageLen < 4) {
        throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u32 requires 4 bytes)`);
      }
      val = new DataView(ctx.memory.buffer).getUint32(ctx.storagePtr, true);
      ctx.storagePtr += 4;
      if (ctx.storageLen !== undefined) { ctx.storageLen -= 4; }
      
      return [val, ctx];
    }
    
    
    function _liftFlatU64(ctx) {
      _debugLog('[_liftFlatU64()] args', { ctx });
      let val;
      
      if (ctx.useDirectParams) {
        if (ctx.params.length === 0) { throw new Error('expected at least one single i64 argument'); }
        if (typeof ctx.params[0] !== 'bigint') { throw new Error('expected bigint'); }
        val = ctx.params[0];
        ctx.params = ctx.params.slice(1);
        return [val, ctx];
      }
      
      if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
        throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u64 requires 8 bytes)`);
      }
      
      val = new DataView(ctx.memory.buffer).getBigUint64(ctx.storagePtr, true);
      ctx.storagePtr += 8;
      if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
      
      return [val, ctx];
    }
    
    
    function _liftFlatFloat64(ctx) {
      _debugLog('[_liftFlatFloat64()] args', { ctx });
      let val;
      
      if (ctx.useDirectParams) {
        if (ctx.params.length === 0) {
          throw new Error('expected at least one single f64 argument');
        }
        val = ctx.params[0];
        ctx.params = ctx.params.slice(1);
        
        if (ctx.inVariant) {
          const dv = new DataView(new ArrayBuffer(8));
          dv.setBigInt64(0, val);
          val = dv.getFloat64(0);
        }
        
        return [val, ctx];
      }
      
      if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
        throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (f64 requires 8 bytes)`);
      }
      
      val = new DataView(ctx.memory.buffer).getFloat64(ctx.storagePtr, true);
      ctx.storagePtr += 8;
      if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
      
      return [val, ctx];
    }
    
    
    function _liftFlatVariant(meta) {
      const {
        caseMetas,
        variantSize32,
        variantAlign32,
        variantPayloadOffset32,
        variantFlatCount,
        isEnum,
      } = meta;
      
      return function _liftFlatVariantInner(ctx) {
        _debugLog('[_liftFlatVariant()] args', { ctx });
        const origUseParams = ctx.useDirectParams;
        
        // If we're in the process of lifting a variant, we note
        // we are during any lifting that happens (e.g. to accomodate f32/f64 mechanics)
        const wasInVariant = ctx.inVariant;
        ctx.inVariant = true;
        
        let caseIdx;
        let liftRes;
        const originalPtr = ctx.storagePtr;
        const numCases =  caseMetas.length;
        if (caseMetas.length < 256) {
          liftRes = _liftFlatU8(ctx);
        } else if (numCases >= 256 && numCases < 65536) {
          liftRes = _liftFlatU16(ctx);
        } else if (numCases >= 65536 && numCases < 4_294_967_296) {
          liftRes = _liftFlatU32(ctx);
        } else {
          throw new Error(`unsupported number of variant cases [${numCases}]`);
        }
        caseIdx = liftRes[0];
        ctx = liftRes[1];
        
        const [
        tag,
        liftFn,
        caseSize32,
        caseAlign32,
        caseFlatCount,
        ] = caseMetas[caseIdx];
        
        if (variantPayloadOffset32 === undefined) {
          throw new Error('unexpectedly missing payload offset');
        }
        
        if (originalPtr !== undefined) {
          ctx.storagePtr = originalPtr + variantPayloadOffset32;
        }
        
        let val;
        if (liftFn === null) {
          val = { tag };
          // NOTE: here we need to move past the entire object in memory
          // despite moving to the payload which we now know is missing/unnecessary
          if (originalPtr !== undefined) {
            ctx.storagePtr = originalPtr + variantSize32;
          }
        } else {
          if (ctx.useDirectParams && ctx.params && liftFn !== _liftFlatFloat64 && typeof ctx.params[0] === 'bigint') {
            if (ctx.params[0] > BigInt(Number.MAX_SAFE_INTEGER)) {
              throw new Error(`invalid value, reinterpreted i32/f32 too large: [${ctx.params[0]}]`);
            }
            ctx.params[0] = Number(ctx.params[0]);
          }
          
          const [newVal, newCtx] = liftFn(ctx);
          val = { tag, val: newVal };
          ctx = newCtx;
        }
        
        if (origUseParams) {
          if (variantFlatCount === undefined || variantFlatCount === null) {
            _debugLog('[_liftFlatVariant()] variant with unknown flat count', { ctx, meta });
            throw new Error('cannot lift variant with unknown flat count');
          }
          if (caseFlatCount === undefined || caseFlatCount === null) {
            _debugLog('[_liftFlatVariant()] case with unknown flat count', { ctx, meta, case: meta.caseMetas[caseIdx] });
            throw new Error('cannot lift case with unknown flat count');
          }
          // NOTE: enums can be tightly packed and do not have a descriminant
          const remainingPayloadParams = variantFlatCount - caseFlatCount - (isEnum ? 0 : 1);
          if (remainingPayloadParams < 0) {
            throw new Error(`invalid variant flat count metadata`);
          }
          if (ctx.params.length < remainingPayloadParams) {
            throw new Error(`expected at least [${remainingPayloadParams}] remaining variant payload params, but got [${ctx.params.length}]`);
          }
          ctx.params = ctx.params.slice(remainingPayloadParams);
        }
        
        if (ctx.storagePtr !== undefined) {
          const rem = ctx.storagePtr % variantAlign32;
          if (rem !== 0) { ctx.storagePtr += variantAlign32 - rem; }
        }
        
        ctx.inVariant = wasInVariant;
        
        return [val, ctx];
      }
    }
    
    function _liftFlatList(meta) {
      const { elemLiftFn, elemSize32, elemAlign32, knownLen, typedArray } = meta;
      
      const listValue =
      typedArray === undefined
      ? values => values
      : values => new typedArray(values);
      
      const readValuesAndReset = (ctx, originalPtr, originalLen, dataPtr, len) => {
        ctx.storagePtr = dataPtr;
        const val = [];
        for (var i = 0; i < len; i++) {
          const elemPtr = dataPtr + i * elemSize32;
          ctx.storagePtr = elemPtr;
          const [res, nextCtx] = elemLiftFn(ctx);
          val.push(res);
          ctx = nextCtx;
          
          ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
        }
        if (originalPtr !== null) { ctx.storagePtr = originalPtr; }
        if (originalLen !== null) { ctx.storageLen = originalLen; }
        return [listValue(val), ctx];
      };
      
      return function _liftFlatListInner(ctx) {
        _debugLog('[_liftFlatList()] args', { ctx });
        
        let liftResults;
        if (knownLen !== undefined) { // list with known length
        if (ctx.useDirectParams) {
          _debugLog('memory unexpectedly missing while lifting unknown length list', { ctx });
          liftResults = [listValue(ctx.params.slice(0, knownLen)), ctx];
          ctx.params = ctx.params.slice(knownLen);
        } else { // indirect params
        if (ctx.memory === null) {
          _debugLog('memory unexpectedly missing while lifting known length list', { knownLen, ctx });
          throw new Error(`memory missing while lifting known length (${knownLen}) list`);
        }
        
        const originalLen = ctx.storageLen;
        const originalPtr = ctx.storagePtr;
        
        ctx.storageLen = knownLen * elemSize32;
        liftResults = readValuesAndReset(ctx, null, originalLen, ctx.storagePtr, knownLen);
      }
      
    } else { // unknown length list
    
    if (ctx.useDirectParams) {
      // unknown length list ptr w/ direct params
      const dataPtr = ctx.params[0];
      const len = ctx.params[1];
      ctx.params = ctx.params.slice(2);
      
      ctx.useDirectParams = false;
      const originalPtr = ctx.storagePtr;
      const originalLen = ctx.storageLen;
      ctx.storageLen = len * elemSize32;
      
      liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
      
      ctx.useDirectParams = true;
    } else {
      // unknown length list ptr w/ in-memory params
      const originalLen = ctx.storageLen;
      ctx.storageLen = 8;
      
      const dataPtrLiftRes = _liftFlatU32(ctx);
      const dataPtr = dataPtrLiftRes[0];
      ctx = dataPtrLiftRes[1];
      
      const lenLiftRes = _liftFlatU32(ctx);
      const len = lenLiftRes[0];
      ctx = lenLiftRes[1];
      
      const originalPtr = ctx.storagePtr;
      ctx.storagePtr = dataPtr;
      
      ctx.storageLen = len * elemSize32;
      liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
    }
  }
  
  return liftResults;
}
}

function _liftFlatResult(meta) {
  const f = _liftFlatVariant(meta);
  return function _liftFlatResultInner(ctx) {
    _debugLog('[_liftFlatResult()] args', { ctx });
    return f(ctx);
  }
}

function _liftFlatBorrow(componentTableIdx, size, memory, vals, storagePtr, storageLen) {
  _debugLog('[_liftFlatBorrow()] args', { size, memory, vals, storagePtr, storageLen });
  throw new Error('flat lift for borrowed resources is not supported!');
}


function _lowerFlatU8(ctx) {
  _debugLog('[_lowerFlatU8()] args', ctx);
  
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  _requireValidNumericPrimitive.bind('u8', ctx.vals[0]);
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  new DataView(ctx.memory.buffer).setUint8(ctx.storagePtr, ctx.vals[0]);
  
  ctx.storagePtr += 1;
}

function _lowerFlatU16(ctx) {
  _debugLog('[_lowerFlatU16()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  const rem = ctx.storagePtr % 2;
  if (rem !== 0) { ctx.storagePtr += (2 - rem); }
  
  _requireValidNumericPrimitive.bind('u16', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setUint16(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 2;
}

function _lowerFlatU32(ctx) {
  _debugLog('[_lowerFlatU32()] args', { ctx });
  
  if (ctx.vals.length !== 1) {
    throw new Error(`expected single value to lower, got [${ctx.vals.length}]`);
  }
  
  const rem = ctx.storagePtr % 4;
  if (rem !== 0) { ctx.storagePtr += (4 - rem); }
  
  _requireValidNumericPrimitive.bind('u32', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setUint32(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 4;
}

function _lowerFlatU64(ctx) {
  _debugLog('[_lowerFlatU64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  _requireValidNumericPrimitive.bind('u64', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setBigUint64(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringAny(ctx) {
  switch (ctx.stringEncoding) {
    case 'utf8':
    return _lowerFlatStringUTF8(ctx);
    case 'utf16':
    return _lowerFlatStringUTF16(ctx);
    default:
    throw new Error(`missing/unrecognized/unsupported string encoding [${ctx.stringEncoding}]`);
  }
}

function _lowerFlatStringUTF8(ctx) {
  _debugLog('[_lowerFlatStringUTF8()] args', ctx);
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const s = ctx.vals[0];
  const { ptr, codepoints } = _utf8AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, codepoints, true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringUTF16(ctx) {
  _debugLog('[_lowerFlatStringUTF16()] args', { ctx });
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const s = ctx.vals[0];
  const { ptr, len, codepoints } = _utf16AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, codepoints, true);
  
  const bytes = new Uint16Array(ctx.memory.buffer, start, codeUnits);
  if (ctx.memory.buffer.byteLength < start + bytes.byteLength) {
    throw new Error('memory out of bounds');
  }
  if (ctx.storageLen !== undefined && ctx.storageLen !== bytes.byteLength) {
    throw new Error(`storage length [${ctx.storageLen}] != [${bytes.byteLength}])`);
  }
  new Uint16Array(ctx.memory.buffer, ctx.storagePtr).set(bytes);
  
  ctx.storagePtr += len;
}

function _lowerFlatVariant(meta) {
  const { variantSize32, variantAlign32, variantPayloadOffset32, caseMetas } = meta;
  
  let caseLookup = {};
  for (const [idx, meta] of caseMetas.entries()) {
    let tag = meta[0];
    caseLookup[tag] = { discriminant: idx, meta };
  }
  
  return function _lowerFlatVariantInner(ctx) {
    _debugLog('[_lowerFlatVariant()] args', { ctx });
    
    const { tag, val } = ctx.vals[0];
    const variantCase = caseLookup[tag];
    if (!variantCase) {
      throw new Error(`missing tag [${tag}] (valid tags: ${Object.keys(caseLookup)})`);
    }
    
    const [ _tag, lowerFn, caseSize32, caseAlign32, caseFlatCount ] = variantCase.meta;
    
    const originalPtr = ctx.storagePtr;
    ctx.vals = [variantCase.discriminant];
    let discLowerRes;
    if (caseMetas.length < 256) {
      discLowerRes = _lowerFlatU8(ctx);
    } else if (caseMetas.length >= 256 && caseMetas.length < 65536) {
      discLowerRes = _lowerFlatU16(ctx);
    } else if (caseMetas.length >= 65536 && caseMetas.length < 4_294_967_296) {
      discLowerRes = _lowerFlatU32(ctx);
    } else {
      throw new Error(`unsupported number of cases [${caseMetas.length}]`);
    }
    
    const payloadOffsetPtr = originalPtr + variantPayloadOffset32;
    ctx.storagePtr = payloadOffsetPtr;
    ctx.vals = [val];
    if (lowerFn) { lowerFn(ctx); }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + variantSize32);
    
    const rem = ctx.storagePtr % variantAlign32;
    if (rem !== 0) { ctx.storagePtr += varianttAlign32 - rem; }
  }
}

function _lowerFlatList(meta) {
  const {
    elemLowerFn,
    knownLen,
    size32,
    align32,
    elemSize32,
    elemAlign32,
  } = meta;
  
  if (!elemLowerFn) { throw new TypeError("missing/invalid element lower fn for list"); }
  
  return function _lowerFlatListInner(ctx) {
    _debugLog('[_lowerFlatList()] args', { ctx });
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('insufficient params left to lower list'); }
      const storagePtr = ctx.params[0];
      const elemCount = ctx.params[1];
      ctx.params = ctx.params.slice(2);
      
      const list = ctx.vals[0];
      if (!list) { throw new Error("missing direct param value"); }
      
      const lowerCtx = {
        storagePtr,
        memory: ctx.memory,
        stringEncoding: ctx.stringEncoding,
      };
      for (let idx = 0; idx < list.length; idx++) {
        const elemPtr = storagePtr + idx * elemSize32;
        lowerCtx.storagePtr = elemPtr;
        lowerCtx.vals = list.slice(idx, idx+1);
        elemLowerFn(lowerCtx);
        lowerCtx.storagePtr = Math.max(lowerCtx.storagePtr, elemPtr + elemSize32);
      }
      ctx.storagePtr = lowerCtx.storagePtr;
      
      // TODO: implement parma-only known-length processing
      
      return;
    }
    
    // TODO(fix): is it possible to get a vals that are a addr and length here from
    // a component lower?
    
    const elems = ctx.vals[0];
    if (knownLen === undefined) {
      // unknown length
      if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
      const dataPtr = ctx.realloc(0, 0, elemAlign32, elemSize32 * elems.length);
      
      ctx.vals[0] = dataPtr;
      _lowerFlatU32(ctx);
      
      ctx.vals[0] = elems.length;
      _lowerFlatU32(ctx);
      
      const origPtr = ctx.storagePtr;
      ctx.storagePtr = dataPtr;
      
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = dataPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
      
      ctx.storagePtr = origPtr;
      
    } else {
      // known length
      
      if (elems.length !== knownLen) {
        throw new TypeError(`invalid list input of length [${elems.length}], must be length [${knownLen}]`);
      }
      
      const originalPtr = ctx.storagePtr;
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = originalPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
    }
    
    // TODO(fix): special case for u8/u16/etc, we can do a direct copy
    
    const totalSizeBytes = elems.length * size32;
    if (ctx.storageLen !== undefined && totalSizeBytes > ctx.storageLen) {
      throw new Error('not enough storage remaining for list flat lower');
    }
  }
}

function _lowerFlatTuple(meta) {
  const { elemLowerMetas, size32: tupleSize32, align32: tupleAlign32 } = meta;
  return function _lowerFlatTupleInner(ctx) {
    _debugLog('[_lowerFlatTuple()] args', { ctx });
    const originalPtr = ctx.storagePtr;
    const tuple = ctx.vals[0];
    for (const [idx, [ lowerFn, size32, align32 ]]  of elemLowerMetas.entries()) {
      const rem = ctx.storagePtr % align32;
      if (rem !== 0) { ctx.storagePtr += align32 - rem; }
      
      const elemPtr = ctx.storagePtr;
      ctx.vals = [tuple[idx]];
      lowerFn(ctx);
      ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + size32);
    }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + tupleSize32);
    
    const rem = ctx.storagePtr % tupleAlign32;
    if (rem !== 0) {
      ctx.storagePtr += tupleAlign32 - rem;
    }
  }
}

function _lowerFlatOption(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatOptionInner(ctx) {
    _debugLog('[_lowerFlatOption()] args', { ctx });
    
    const v = ctx.vals[0];
    if (v === null || v === undefined) {
      ctx.vals[0] = { tag: 'none' };
    } else {
      const isNotOptionObject = typeof v !== 'object'
      || Object.keys(v).length !== 2
      || !('tag' in v)
      || !(v.tag === 'some' || v.tag === 'none')
      || !('val' in v);
      if (isNotOptionObject) {
        ctx.vals[0] = { tag: 'some', val: v };
      }
    }
    
    f(ctx);
  }
}

function _lowerFlatResult(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatResultInner(ctx) {
    _debugLog('[_lowerFlatResult()] args', { ctx });
    
    const v = ctx.vals[0];
    const isNotResultObject = typeof v !== 'object'
    || Object.keys(v).length !== 2
    || !('tag' in v)
    || !('ok' === v.tag || 'err' === v.tag)
    || !('val' in v);
    if (isNotResultObject) {
      ctx.vals[0] = { tag: 'ok', val: v };
    }
    
    f(ctx);
  };
}

function _lowerFlatOwn(meta) {
  const { lowerFn, componentIdx } = meta;
  
  return function _lowerFlatOwnInner(ctx) {
    _debugLog('[_lowerFlatOwn()] args', { ctx });
    const { createFn } = ctx;
    
    if (ctx.componentIdx !== componentIdx) {
      throw new Error(`component index mismatch (expected [${componentIdx}], lift called from [${ctx.componentIdx}])`);
    }
    
    const obj = ctx.vals[0];
    if (obj === undefined || obj === null) { throw new Error('missing resource'); }
    const handle = lowerFn(obj);
    
    ctx.vals[0] = handle;
    _lowerFlatU32(ctx);
  };
}

function _lowerFlatStream(meta) {
  const {
    componentIdx,
    streamTableIdx,
    elemMeta,
  } = meta;
  
  return function _lowerFlatStreamInner(ctx) {
    _debugLog('[_lowerFlatStream()] args', { ctx });
    
    const stream = ctx.vals[0];
    if (!stream) { throw new Error("missing external stream value"); }
    
    let globalRep;
    let waitableIdx;
    if (stream instanceof Stream) {
      globalRep = stream[symbolRscRep];
      const internalStream = STREAMS.get(globalRep);
      if (!internalStream || !(internalStream instanceof InternalStream)) {
        throw new Error(`failed to find internal stream with rep [${globalRep}]`);
      }
      waitableIdx = internalStream.readEnd().waitableIdx();
    } else if (_isStreamLowerableObject(stream)) {
      globalRep = stream[symbolRscRep];
      
      if (globalRep) {
        const hostStream = STREAMS.get(globalRep);
        if (!hostStream) {
          throw new Error(`missing host stream with global rep [${globalRep}]`);
        }
        waitableIdx = hostStream.getStreamEndWaitableIdx();
      } else {
        const cstate = getOrCreateAsyncState(componentIdx);
        if (!cstate) {
          throw new Error(`missing async state for component [${componentIdx}]`);
        }
        
        const { writeEnd, readEnd } = cstate.createStream({
          tableIdx: streamTableIdx,
          elemMeta,
        });
        
        const readFn = _genReadFnFromLowerableStream(stream);
        const hostInjectFn = _genStreamHostInjectFn({
          readFn,
          hostWriteEnd: writeEnd,
          readEnd,
        });
        readEnd.setHostInjectFn(hostInjectFn);
        readEnd.setHostDropFn(readFn.drop);
        
        waitableIdx = readEnd.waitableIdx();
      }
    } else {
      throw new Error('object does not conform to supported stream interfaces');
    }
    
    // Write the idx of the waitable to memory (a waiting async task or caller)
    if (ctx.storagePtr) {
      ctx.vals[0] = waitableIdx;
      _lowerFlatU32(ctx);
    }
    
    return waitableIdx;
  }
}

const STREAMS = new RepTable({ target: 'global stream map' });

const STREAM_TABLES = {};

class StreamEnd {
  static CopyResult = {
    COMPLETED: 0,
    DROPPED: 1,
    CANCELLED: 2,
  };
  
  static CopyState = {
    IDLE: 1,
    SYNC_COPYING: 2,
    ASYNC_COPYING: 3,
    CANCELLING_COPY: 4,
    DONE: 5,
  };
  
  #waitable = null;
  
  #tableIdx = null; // stream table that contains the stream end
  #idx = null; // stream end index in the table
  
  #componentIdx = null;
  
  #copyState = StreamEnd.CopyState.IDLE;
  
  #dropped;
  #setDroppedFn;
  #isDroppedFn;
  
  target;
  
  constructor(args) {
    const { tableIdx, componentIdx } = args;
    if (tableIdx === undefined || typeof tableIdx !== 'number') {
      throw new TypeError(`missing table idx [${tableIdx}]`);
    }
    if (tableIdx < 0 || tableIdx > 2_147_483_647) {
      throw new TypeError(`invalid  tableIdx [${tableIdx}]`);
    }
    if (!args.waitable) { throw new Error('missing/invalid waitable'); }
    
    this.#tableIdx = args.tableIdx;
    this.#waitable = args.waitable;
    
    if (args.setDroppedFn && args.isDroppedFn) {
      this.#setDroppedFn = args.setDroppedFn;
      this.#isDroppedFn = args.isDroppedFn;
    } else if (args.setDroppedFn === undefined && args.isDroppedFn === undefined) {
      this.#setDroppedFn = (v) => { this.#dropped = v; };
      this.#isDroppedFn = () => { return this.#dropped; };
    } else {
      throw new TypeError('setDroppedFn and isDroppedFn must both be specified or neither');
    }
    
    this.target = args.target;
  }
  
  tableIdx() { return this.#tableIdx; }
  
  idx() { return this.#idx; }
  setIdx(idx) { this.#idx = idx; }
  
  setTarget(tgt) { this.target = tgt; }
  
  getWaitable() { return this.#waitable; }
  setWaitable(w) { this.#waitable = w; }
  
  setCopyState(state) { this.#copyState = state; }
  getCopyState() { return this.#copyState; }
  
  isCopying() {
    switch (this.#copyState) {
      case StreamEnd.CopyState.IDLE:
      case StreamEnd.CopyState.DONE:
      return false;
      break;
      case StreamEnd.CopyState.SYNC_COPYING:
      case StreamEnd.CopyState.ASYNC_COPYING:
      case StreamEnd.CopyState.CANCELLING_COPY:
      return true;
      break;
      default:
      throw new Error('invalid/unknown copying state');
    }
  }
  
  setPendingEvent(fn) {
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    _debugLog('[StreamEnd#setPendingEvent()]', {
      waitable: this.#waitable,
      waitableinSet: this.#waitable.isInSet(),
      componentIdx: this.#waitable.componentIdx(),
    });
    this.#waitable.setPendingEvent(fn);
  }
  
  hasPendingEvent() {
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    return this.#waitable.hasPendingEvent();
  }
  
  isInSet() {
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    return this.#waitable.isInSet();
  }
  
  getPendingEvent() {
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    _debugLog('[StreamEnd#getPendingEvent()]', {
      waitable: this.#waitable,
      waitableinSet: this.#waitable.isInSet(),
      componentIdx: this.#waitable.componentIdx(),
    });
    const event = this.#waitable.getPendingEvent();
    return event;
  }
  
  isDropped() { return this.#isDroppedFn(); }
  setDropped() { return this.#setDroppedFn(); }
  
  drop() {
    _debugLog('[StreamEnd#drop()]', {
      waitable: this.#waitable,
      waitableinSet: this.#waitable.isInSet(),
      componentIdx: this.#waitable.componentIdx(),
    });
    
    if (this.isDropped()) {
      _debugLog('[StreamEnd#drop()] already dropped', {
        waitable: this.#waitable,
        waitableinSet: this.#waitable.isInSet(),
        componentIdx: this.#waitable.componentIdx(),
      });
      return;
    }
    
    if (this.#waitable) {
      const w = this.#waitable;
      w.drop();
    }
    
    this.setDropped();
  }
}

class InternalStream {
  #pendingBufferMeta = {}; // shared between read/write ends
  #elemMeta;
  
  #globalStreamMapRep;
  
  #readEnd;
  #writeEnd;
  
  constructor(args) {
    _debugLog('[InternalStream#constructor()] args', args);
    if (!args.elemMeta) { throw new Error('missing/invalid stream element metadata'); }
    if (args.tableIdx === undefined) { throw new Error('missing/invalid stream table idx'); }
    if (!args.readWaitable) { throw new Error('missing/invalid read waitable'); }
    if (!args.writeWaitable) { throw new Error('missing/invalid write waitable'); }
    const { tableIdx, elemMeta, readWaitable, writeWaitable, } = args;
    
    this.#elemMeta = elemMeta;
    
    let dropped = false;
    const setDroppedFn = () => { dropped = true };
    const isDroppedFn = () => dropped;
    
    this.#readEnd = new StreamReadableEnd({
      tableIdx,
      elemMeta: this.#elemMeta,
      pendingBufferMeta: this.#pendingBufferMeta,
      target: "stream read end (@ init)",
      waitable: readWaitable,
      // Only in-component read-ends need the host inject fn if provided,
      // as that function will *inject* a write when a read is performed
      // from inside the guest.
      hostInjectFn: args.hostInjectFn,
      setDroppedFn,
      isDroppedFn,
    });
    
    this.#writeEnd = new StreamWritableEnd({
      tableIdx,
      elemMeta: this.#elemMeta,
      pendingBufferMeta: this.#pendingBufferMeta,
      target: "stream write end (@ init)",
      waitable: writeWaitable,
      hostOwned: true,
      setDroppedFn,
      isDroppedFn,
    });
  }
  
  elemMeta() { return this.#elemMeta; }
  
  globalStreamMapRep() { return this.#globalStreamMapRep; }
  setGlobalStreamMapRep(rep) {
    this.#globalStreamMapRep = rep;
    this.#readEnd.setGlobalStreamMapRep(rep);
    this.#writeEnd.setGlobalStreamMapRep(rep);
  }
  
  readEnd() { return this.#readEnd; }
  writeEnd() { return this.#writeEnd; }
}

class StreamReadableEnd extends StreamEnd {
  #copying = false;
  #done = false;
  
  #elemMeta = null;
  // held by both write and read ends
  #pendingBufferMeta = null;
  
  // table index that the stream is in (can change after a stream transfer)
  #streamTableIdx;
  // handle (index) inside the given table (can change after a stream transfer)
  #handle;
  
  // internal stream (which has both ends) rep
  #globalStreamMapRep;
  
  // only populated for lowered (read) stream ends
  #hostInjectFn;
  #hostDropFn;
  #hostCancelFn;
  // only populated for the write side of a lowered read stream end
  #isHostOwned;
  
  #result = null;
  
  #endOfStream = false;
  #rejectedLength = null;
  
  constructor(args) {
    _debugLog('[StreamReadableEnd#constructor()] args', args);
    super(args);
    
    if (!args.elemMeta) { throw new Error('missing/invalid element meta'); }
    this.#elemMeta = args.elemMeta;
    
    if (!args.pendingBufferMeta) { throw new Error('missing/invalid shared pending buffer meta'); }
    this.#pendingBufferMeta = args.pendingBufferMeta;
    
    if (args.tableIdx === undefined) { throw new Error('missing index for stream table idx'); }
    this.#streamTableIdx = args.tableIdx;
    
    this.#hostInjectFn = args.hostInjectFn;
    this.#isHostOwned = args.hostOwned;
  }
  
  streamTableIdx() { return this.#streamTableIdx; }
  setStreamTableIdx(idx) { this.#streamTableIdx = idx; }
  
  handle() { return this.#handle; }
  setHandle(h) { this.#handle = h; }
  
  globalStreamMapRep() { return this.#globalStreamMapRep; }
  setGlobalStreamMapRep(rep) { this.#globalStreamMapRep = rep; }
  
  waitableIdx() { return this.getWaitable().idx(); }
  setWaitableIdx(idx) {
    const w = this.getWaitable();
    w.setIdx(idx);
    w.setTarget(`waitable for read end (waitable [${idx}])`);
  }
  
  setHostInjectFn(f) {
    if (this.#hostInjectFn) { throw new Error('host injection fn is already set'); }
    this.#hostInjectFn = f;
  }
  setHostDropFn(f) {
    if (this.#hostDropFn) { throw new Error('host drop fn is already set'); }
    this.#hostDropFn = f;
  }
  setHostCancelFn(f) { this.#hostCancelFn = f; }
  
  getElemMeta() { return {...this.#elemMeta}; }
  
  
  isReadable() { return true; }
  isWritable() { return false; }
  
  
  isDoneState() { return this.getCopyState() === StreamEnd.CopyState.DONE; }
  isCancelledState() { return this.getCopyState() === StreamEnd.CopyState.CANCELLED; }
  isIdleState() { return this.getCopyState() === StreamEnd.CopyState.IDLE; }
  
  
  async read(opts = 1) {
    _debugLog('[StreamReadableEnd#read()]');
    
    if (this.#endOfStream) {
      return { value: undefined, done: true };
    }
    let { count, rejectLength } = this.#readOpts(opts);
    
    // Wait for an existing read operation to end, if present,
    // otherwise register this read for any future operations.
    //
    // NOTE: this complexity below is an attempt to sequence operations
    // to ensure consecutive reads only wait on their direct predecessors,
    // (i.e. read #3 must wait on read #2, *not* read #1)
    //
    const newResult = promiseWithResolvers();
    if (this.#result) {
      try {
        const p = this.#result.promise;
        this.#result = newResult;
        await p;
      } catch (err) {
        _debugLog('[StreamReadableEnd#read()] error waiting for previous read', err);
        // If the previous write we were waiting on errors for any reason,
        // we can ignore it and attempt to continue with this read
        // which may also fail for a similar reason
      }
    } else {
      this.#result = newResult;
    }
    const { promise, resolve, reject } = newResult;
    
    // TODO(fix): when we do a read, we need to GET the string encoding from the
    // other side, via the lift/lower fn?
    
    count = Math.min(count, ManagedBuffer.MAX_LENGTH);
    try {
      const { id: bufferID, buffer } = BUFFER_MGR.createBuffer({
        componentIdx: -1, // componentIdx of -1 indicates the host
        count,
        isReadable: false,
        isWritable: true, // we need to write out the pending buffer (if present)
        elemMeta: this.#elemMeta,
        data: [],
      });
      buffer.setTarget(`host stream read buffer (id [${bufferID}], count [${count}])`);
      
      let packedResult;
      packedResult = await this.copy({
        isAsync: true,
        count,
        bufferID,
        buffer,
        eventCode: ASYNC_EVENT_CODE.STREAM_READ,
        componentIdx: -1,
        rejectLength,
      });
      
      if (packedResult === ASYNC_BLOCKED_CODE) {
        // If the read was blocked, the pending event produced by the
        // write side represents the completed copy.
        
        await new Promise((resolve) => {
          let waitInterval = setInterval(() => {
            if (!this.hasPendingEvent()) { return; }
            clearInterval(waitInterval);
            resolve();
          });
        });
        
        if (!this.hasPendingEvent()) {
          throw new Error("missing pending event after blocked stream read");
        }
        
        const event = this.getPendingEvent();
        if (!event) { throw new Error("missing pending event after blocked stream read"); }
        
        const { code, payload0: index, payload1: payload } = event;
        
        if (code !== ASYNC_EVENT_CODE.STREAM_READ) {
          throw new Error(`mismatched event code [${code}] for host stream read`);
        }
        
        if (index !== this.waitableIdx()) { throw new Error('invalid stream end index'); }
        if (event.rejectedLength !== undefined) {
          this.#rejectedLength = event.rejectedLength;
        }
        packedResult = payload;
        
        if (packedResult === ASYNC_BLOCKED_CODE) {
          throw new Error("unexpected double block during read");
        }
      }
      
      const resultKind = packedResult & 0xF;
      const transferred = packedResult >> 4;
      
      if (resultKind === StreamEnd.CopyResult.DROPPED) {
        this.#endOfStream = true;
      }
      
      if (transferred > 0) {
        const values = buffer.read(transferred);
        const { typedArray } = this.#elemMeta;
        const value = typedArray === undefined ? count === 1 ? values[0] : values : new typedArray(values);
        this.#result = null;
        resolve(value);
      } else {
        this.#result = null;
        resolve(undefined);
      }
      
    } catch (err) {
      _debugLog('[StreamReadableEnd#read()] error', err);
      reject(err);
    }
    
    const res = await promise;
    const rejectedLength = this.#rejectedLength;
    this.#rejectedLength = null;
    const result = { value: res, done: res === undefined };
    if (rejectedLength !== null) {
      result.rejectedLength = rejectedLength;
    }
    return result;
  }
  
  #readOpts(opts) {
    const count = opts === undefined ? 1 : typeof opts === "number" ? opts : opts && typeof opts === "object" ? opts.count ?? 1 : undefined;
    const rejectLength = opts && typeof opts === "object" ? opts.rejectLength : undefined;
    if (!Number.isInteger(count) || count < (rejectLength !== undefined ? 0 : 1)) {
      throw new TypeError(`invalid stream read count [${count}]`);
    }
    if (rejectLength !== undefined && (!Number.isInteger(rejectLength) || rejectLength < 0)) {
      throw new TypeError(`invalid stream read reject length [${rejectLength}]`);
    }
    return { count, rejectLength };
  }
  
  
  _read(args) {
    const { buffer, onCopyDoneFn, onCopyFn, componentIdx, rejectLength } = args;
    if (this.isDropped()) {
      onCopyDoneFn(StreamEnd.CopyResult.DROPPED);
      return;
    }
    
    if (!this.#pendingBufferMeta.buffer) {
      this.setPendingBufferMeta({
        componentIdx,
        buffer,
        onCopyFn,
        onCopyDoneFn,
        rejectLength,
      });
      return;
    }
    
    const pendingElemMeta = this.#pendingBufferMeta.buffer.getElemMeta();
    const newBufferElemMeta = buffer.getElemMeta();
    if (pendingElemMeta.payloadTypeName !== newBufferElemMeta.payloadTypeName) {
      throw new Error("trap: stream end type does not match internal buffer");
    }
    
    // Since we do not know the string encoding until a write is performed, it is possible that
    // one end (i.e. the read end) does not yet know the appropriate string encoding to use when
    // lifting/lowering.
    if (newBufferElemMeta.stringEncoding === undefined || pendingElemMeta.stringEncoding === undefined) {
      const encoding = pendingElemMeta.stringEncoding ?? newBufferElemMeta.stringEncoding;
      if (encoding === undefined) { throw new Error('both writer & reader missing string encoding'); }
      newBufferElemMeta.stringEncoding = encoding;
      pendingElemMeta.stringEncoding = encoding;
    }
    
    // If the buffer came from the same component that is currently doing the operation
    // we're doing a inter-component read, and only unit or numeric types are allowed
    const pendingElemIsNoneOrNumeric = pendingElemMeta.isNone || pendingElemMeta.isNumeric;
    if (this.#pendingBufferMeta.componentIdx === buffer.componentIdx() && buffer.componentIdx() !== -1 && !pendingElemIsNoneOrNumeric) {
      throw new Error(`trap: cannot stream non-numeric types within the same component (component [${buffer.componentIdx()}] read)`);
    }
    
    const pendingRemaining = this.#pendingBufferMeta.buffer.remaining();
    let transferred = false;
    if (pendingRemaining > 0) {
      const bufferRemaining = buffer.remaining();
      if (rejectLength !== undefined && pendingRemaining > rejectLength) {
        this.resetAndNotifyPending(StreamEnd.CopyResult.DROPPED);
        onCopyDoneFn(StreamEnd.CopyResult.DROPPED, pendingRemaining);
        return;
      }
      if (bufferRemaining > 0) {
        const count = Math.min(pendingRemaining, bufferRemaining);
        buffer.write(this.#pendingBufferMeta.buffer.read(count))
        this.#pendingBufferMeta.onCopyFn(() => this.resetPendingBufferMeta());
        transferred = true;
      }
      
      onCopyDoneFn(StreamEnd.CopyResult.COMPLETED);
      
      return;
    }
    
    this.resetAndNotifyPending(StreamEnd.CopyResult.COMPLETED);
    this.setPendingBufferMeta({ componentIdx, buffer, onCopyFn, onCopyDoneFn, rejectLength });
  }
  
  
  setupCopy(args) {
    const {
      memory,
      ptr,
      count,
      eventCode,
      componentIdx,
      skipStateCheck,
    } = args;
    if (eventCode === undefined) { throw new Error("missing/invalid event code"); }
    
    let buffer = args.buffer;
    let bufferID = args.bufferID;
    
    // Only check invariants if we are *not* doing a follow-up/post-blocked read
    if (!skipStateCheck) {
      if (this.isCopying()) {
        throw new Error('stream is currently undergoing a separate copy');
      }
      if (this.getCopyState() !== StreamEnd.CopyState.IDLE) {
        throw new Error(`stream copy state is not idle`);
      }
    }
    
    const elemMeta = this.getElemMeta();
    if (elemMeta.isBorrowed) { throw new Error('borrowed types cannot be sent over streams'); }
    
    // If we already have a managed buffer (likely host case), we can use that, otherwise we must
    // create a buffer (likely in the guest case)
    if (!buffer) {
      const newBufferMeta = BUFFER_MGR.createBuffer({
        componentIdx,
        memory,
        start: ptr,
        count,
        // If creating a buffer for a write operation, the buffer we are encapsulating
        // is a *readable* buffer from the view of the component (as it has written to that buffer data that)
        // should be sent out
        isReadable: this.isWritable(),
        // If creating a buffer for a read operation, the buffer we are encapsulating
        // is a *writable* buffer from the view of the component (as it has prepared space to receive data)
        isWritable: this.isReadable(),
        elemMeta,
      });
      bufferID = newBufferMeta.id;
      buffer = newBufferMeta.buffer;
      buffer.setTarget(`component [${componentIdx}] StreamReadableEnd buffer (id [${bufferID}], count [${count}], eventCode [${eventCode}])`);
    }
    
    const streamEnd = this;
    const processFn = (result, reclaimBufferFn, rejectedLength) => {
      if (reclaimBufferFn) { reclaimBufferFn(); }
      
      if (result === StreamEnd.CopyResult.DROPPED) {
        streamEnd.setCopyState(StreamEnd.CopyState.DONE);
      } else {
        streamEnd.setCopyState(StreamEnd.CopyState.IDLE);
      }
      
      if (result < 0 || result >= 16) {
        throw new Error(`unsupported stream copy result [${result}]`);
      }
      if (buffer.processed >= ManagedBuffer.MAX_LENGTH) {
        throw new Error(`processed count [${buf.length}] greater than max length`);
      }
      if (buffer.length > 2**28) { throw new Error('buffer uses reserved space'); }
      
      const packedResult = (Number(buffer.processed) << 4) | result;
      const event = { code: eventCode, payload0: streamEnd.waitableIdx(), payload1: packedResult };
      if (rejectedLength !== undefined) {
        event.rejectedLength = rejectedLength;
      }
      
      return event;
    };
    
    const onCopyFn = (reclaimBufferFn) => {
      streamEnd.setPendingEvent(() => {
        return processFn(StreamEnd.CopyResult.COMPLETED, reclaimBufferFn);
      });
    };
    
    const onCopyDoneFn = (result, rejectedLength) => {
      streamEnd.setPendingEvent(() => {
        return processFn(result, undefined, rejectedLength);
      });
    };
    
    return { bufferID, buffer, onCopyFn, onCopyDoneFn };
  }
  
  
  async copy(args) {
    const {
      isAsync,
      memory,
      componentIdx,
      ptr,
      count,
      eventCode,
      initial,
      skipStateCheck,
      stringEncoding,
      reallocFn,
      rejectLength,
    } = args;
    if (eventCode === undefined) { throw new TypeError('missing/invalid event code'); }
    
    if (this.#elemMeta.stringEncoding === undefined && stringEncoding) {
      this.#elemMeta.stringEncoding = stringEncoding;
    }
    if (this.#elemMeta.stringEncoding && stringEncoding && this.#elemMeta.stringEncoding !== stringEncoding) {
      throw new Error(`inconsistent string encoding (previously [${this.#elemMeta.stringEncoding}], now [${stringEncoding}])`);
    }
    
    if (args.getReallocFn && this.#elemMeta.getReallocFn === undefined) {
      this.#elemMeta.getReallocFn = args.getReallocFn;
    }
    
    if (this.isDropped()) {
      if (this.#pendingBufferMeta?.onCopyDoneFn) {
        const f = this.#pendingBufferMeta.onCopyDoneFn;
        this.#pendingBufferMeta.onCopyDoneFn = null;
        f(StreamEnd.CopyResult.DROPPED);
      }
      this.setCopyState(StreamEnd.CopyState.DONE);
      return StreamEnd.CopyResult.DROPPED;
    }
    
    const { buffer, onCopyFn, onCopyDoneFn } = this.setupCopy({
      memory,
      eventCode,
      componentIdx,
      ptr,
      count,
      buffer: args.buffer,
      bufferID: args.bufferID,
      initial,
      skipStateCheck,
    });
    
    // If the stream is readable and was lowered from the host, the
    // writer is host-side. Register the read first; host injection
    // will no-op if the read already produced a pending event.
    const injectHostWrite = this.isReadable() && !!this.#hostInjectFn;
    
    // Perform the read/write
    this._read({
      buffer,
      onCopyFn,
      onCopyDoneFn,
      componentIdx,
      rejectLength,
    });
    
    let injectedWritePromise;
    if (injectHostWrite) {
      injectedWritePromise = this.#hostInjectFn({ count });
    }
    
    // If sync, wait forever but allow task to do other things
    if (!this.hasPendingEvent()) {
      if (isAsync) {
        this.setCopyState(StreamEnd.CopyState.ASYNC_COPYING);
        _debugLog('[StreamEnd#copy()] blocked', { componentIdx, eventCode, self: this });
        if (injectedWritePromise) {
          // Do not await here: the injected write may depend on sibling
          // guest work running, so the canonical read must return BLOCKED.
          injectedWritePromise.then(
          cleanupFn => cleanupFn(),
          err => this.setPendingEvent(() => { throw err; }),
          );
        }
        return ASYNC_BLOCKED_CODE;
      } else {
        this.setCopyState(StreamEnd.CopyState.SYNC_COPYING);
        
        const taskMeta = getCurrentTask(componentIdx);
        if (!taskMeta) { throw new Error(`missing task meta for component idx [${componentIdx}]`); }
        
        const task = taskMeta.task;
        if (!task) { throw new Error('missing task task from task meta'); }
        
        const streamEnd = this;
        await task.suspendUntil({
          readyFn: () => streamEnd.hasPendingEvent(),
        });
      }
    }
    
    // If the read completed immediately after injecting a host write,
    // it is safe to await injection cleanup before consuming the event.
    if (injectedWritePromise) {
      const cleanupFn = await injectedWritePromise;
      cleanupFn();
    }
    
    const event = this.getPendingEvent();
    if (!event) { throw new Error("unexpectedly missing pending event"); }
    if (event.code === undefined || event.payload0 === undefined || event.payload1 === undefined) {
      throw new Error("unexpectedly malformed event");
    }
    
    const { code, payload0: index, payload1: payload } = event;
    
    const waitableIdx = this.getWaitable().idx();
    if (code !== eventCode  || index !== waitableIdx || payload === ASYNC_BLOCKED_CODE) {
      const errMsg = "invalid event code/event during stream operation";
      _debugLog(errMsg, {
        event,
        payload,
        payloadIsBlockedConst: payload === ASYNC_BLOCKED_CODE,
        code,
        eventCode,
        codeDoesNotMatchEventCode: code !== eventCode,
        index,
        internalEndIdx: waitableIdx,
        indexDoesNotMatch: index !== waitableIdx,
      });
      throw new Error(errMsg);
    }
    
    if (event.rejectedLength !== undefined) {
      this.#rejectedLength = event.rejectedLength;
    }
    return payload;
  }
  
  
  setPendingBufferMeta(args) {
    const { componentIdx, buffer, onCopyFn, onCopyDoneFn, rejectLength } = args;
    this.#pendingBufferMeta.componentIdx = componentIdx;
    this.#pendingBufferMeta.buffer = buffer;
    this.#pendingBufferMeta.onCopyFn = onCopyFn;
    this.#pendingBufferMeta.onCopyDoneFn = onCopyDoneFn;
    this.#pendingBufferMeta.rejectLength = rejectLength;
  }
  
  resetPendingBufferMeta() {
    this.setPendingBufferMeta({ componentIdx: null, buffer: null, onCopyFn: null, onCopyDoneFn: null, rejectLength: undefined });
  }
  
  getPendingBufferMeta() { return this.#pendingBufferMeta; }
  
  resetAndNotifyPending(result) {
    const f = this.#pendingBufferMeta.onCopyDoneFn;
    this.resetPendingBufferMeta();
    if (f) { f(result); }
  }
  
  cancel() {
    _debugLog('[StreamEnd#cancel()]');
    const completeCancel = () => {
      if (this.isDropped()) { return; }
      if (this.#hostCancelFn?.()) { return; }
      const result = this.#pendingBufferMeta?.buffer?.processed > 0
      ? StreamEnd.CopyResult.COMPLETED
      : StreamEnd.CopyResult.CANCELLED;
      this.resetAndNotifyPending(result);
    };
    if (this.#hostInjectFn) {
      setTimeout(completeCancel, 0);
    } else {
      completeCancel();
    }
  }
  
  drop() {
    _debugLog('[StreamEnd#drop()]');
    if (this.isDropped()) { return; }
    if (this.#hostDropFn) {
      Promise.resolve(this.#hostDropFn()).catch(err => {
        _debugLog('[StreamEnd#drop()] host drop failed', err);
      });
      this.#hostDropFn = null;
    }
    super.drop();
    if (this.#pendingBufferMeta) {
      const result = this.#pendingBufferMeta.buffer?.processed > 0
      ? StreamEnd.CopyResult.COMPLETED
      : StreamEnd.CopyResult.DROPPED;
      this.resetAndNotifyPending(result);
    }
  }
}

class StreamWritableEnd extends StreamEnd {
  #copying = false;
  #done = false;
  
  #elemMeta = null;
  // held by both write and read ends
  #pendingBufferMeta = null;
  
  // table index that the stream is in (can change after a stream transfer)
  #streamTableIdx;
  // handle (index) inside the given table (can change after a stream transfer)
  #handle;
  
  // internal stream (which has both ends) rep
  #globalStreamMapRep;
  
  // only populated for lowered (read) stream ends
  #hostInjectFn;
  #hostDropFn;
  #hostCancelFn;
  // only populated for the write side of a lowered read stream end
  #isHostOwned;
  
  #result = null;
  
  #endOfStream = false;
  #rejectedLength = null;
  
  constructor(args) {
    _debugLog('[StreamWritableEnd#constructor()] args', args);
    super(args);
    
    if (!args.elemMeta) { throw new Error('missing/invalid element meta'); }
    this.#elemMeta = args.elemMeta;
    
    if (!args.pendingBufferMeta) { throw new Error('missing/invalid shared pending buffer meta'); }
    this.#pendingBufferMeta = args.pendingBufferMeta;
    
    if (args.tableIdx === undefined) { throw new Error('missing index for stream table idx'); }
    this.#streamTableIdx = args.tableIdx;
    
    this.#hostInjectFn = args.hostInjectFn;
    this.#isHostOwned = args.hostOwned;
  }
  
  streamTableIdx() { return this.#streamTableIdx; }
  setStreamTableIdx(idx) { this.#streamTableIdx = idx; }
  
  handle() { return this.#handle; }
  setHandle(h) { this.#handle = h; }
  
  globalStreamMapRep() { return this.#globalStreamMapRep; }
  setGlobalStreamMapRep(rep) { this.#globalStreamMapRep = rep; }
  
  waitableIdx() { return this.getWaitable().idx(); }
  setWaitableIdx(idx) {
    const w = this.getWaitable();
    w.setIdx(idx);
    w.setTarget(`waitable for write end (waitable [${idx}])`);
  }
  
  setHostInjectFn(f) {
    if (this.#hostInjectFn) { throw new Error('host injection fn is already set'); }
    this.#hostInjectFn = f;
  }
  setHostDropFn(f) {
    if (this.#hostDropFn) { throw new Error('host drop fn is already set'); }
    this.#hostDropFn = f;
  }
  setHostCancelFn(f) { this.#hostCancelFn = f; }
  
  getElemMeta() { return {...this.#elemMeta}; }
  
  
  isReadable() { return false; }
  isWritable() { return true; }
  
  
  isDoneState() { return this.getCopyState() === StreamEnd.CopyState.DONE; }
  isCancelledState() { return this.getCopyState() === StreamEnd.CopyState.CANCELLED; }
  isIdleState() { return this.getCopyState() === StreamEnd.CopyState.IDLE; }
  
  
  async write(v) {
    _debugLog('[StreamWritableEnd#write()] args', { v });
    
    let data;
    if (this.#elemMeta.isNumeric) {
      if (v instanceof ArrayBuffer) {
        v = new Uint8Array(v);
      }
      data = Array.isArray(v) || (ArrayBuffer.isView(v) && typeof v.length === 'number') ? Array.from(v) : [v];
    } else {
      data = [v];
    }
    return this.writeMany(data);
  }
  
  async writeMany(values) {
    _debugLog('[StreamWritableEnd#writeMany()] args', { values });
    if (!Array.isArray(values)) { throw new TypeError("writeMany values must be an array"); }
    
    // Wait for an existing write operation to end, if present,
    // otherwise register this write for any future operations.
    //
    // NOTE: this complexity below is an attempt to sequence operations
    // to ensure consecutive writes only wait on their direct predecessors,
    // (i.e. write #3 must wait on write #2, *not* write #1)
    //
    let newResult = promiseWithResolvers();
    if (this.#result && !this.#isHostOwned) {
      try {
        const p = this.#result.promise;
        this.#result = newResult;
        await p;
      } catch (err) {
        _debugLog('[StreamWritableEnd#writeMany()] error waiting for previous write', err);
        // If the previous write we were waiting on errors for any reason,
        // we can ignore it and attempt to continue with this write
        // which may also fail for a similar reason
      }
    } else {
      this.#result = newResult;
    }
    const { promise, resolve, reject } = newResult;
    
    const data = values;
    const count = data.length;
    if (this.#elemMeta.stringEncoding === undefined) {
      this.#elemMeta.string = 'utf8';
    }
    
    try {
      const { id: bufferID, buffer } = BUFFER_MGR.createBuffer({
        componentIdx: -1,
        count,
        isReadable: true, // we need to read from this buffer later
        isWritable: false,
        elemMeta: this.#elemMeta,
        data,
      });
      buffer.setTarget(`host stream write buffer (id [${bufferID}], count [${count}], data len [${data.length}])`);
      
      let packedResult;
      const copyPromise = this.copy({
        isAsync: true,
        count,
        bufferID,
        buffer,
        eventCode: ASYNC_EVENT_CODE.STREAM_WRITE,
        componentIdx: -1,
      });
      if (this.#isHostOwned && this.hasPendingEvent()) {
        // Host owned writes are just-in-time writes for an already pending guest read.
        // The guest read path consumes the pending event, so waiting here can deadlock.
        copyPromise.catch(err => reject(err));
        this.#result = null;
        resolve();
        return await promise;
      }
      packedResult = await copyPromise;
      
      // If we are dealing with a blocked component write operation, we do an immedaite wait
      // on the host side to pause the host until the write can be completed.
      //
      // We do not do this if we're dealing with a host injection,
      // (i.e. a lowered read end into a component does a read() and forces
      // data to be read from the host side), we must signal the write is completed
      // and we are waiting for the read.
      //
      //  In the host injection case, it is OK that the write is blocked, because we
      //  know the read is about to occur (we control the writes to the stream to be
      // just-before reads, no matter what the user does on the other end).
      //
      if (packedResult === ASYNC_BLOCKED_CODE && !this.#isHostOwned) {
        // If the write was blocked, the pending event produced by the
        // read side represents the completed copy.
        
        await new Promise((resolve) => {
          let waitInterval = setInterval(async () => {
            if (!this.hasPendingEvent()) { return; }
            clearInterval(waitInterval);
            resolve();
          });
        });
        
        if (!this.hasPendingEvent()) {
          throw new Error("missing pending event after blocked stream write");
        }
        
        const event = this.getPendingEvent();
        if (!event) { throw new Error("missing pending event after blocked stream write"); }
        
        const { code, payload0: index, payload1: payload } = event;
        
        if (code !== ASYNC_EVENT_CODE.STREAM_WRITE) {
          throw new Error(`mismatched event code [${code}] for host stream write`);
        }
        
        if (index !== this.waitableIdx()) { throw new Error('invalid stream end index'); }
        packedResult = payload;
        
        const copied = packedResult >> 4;
        if (copied === 0 && this.isDoneState()) {
          reject(new Error("read end dropped during write"));
        }
        
        if (packedResult === ASYNC_BLOCKED_CODE) {
          throw new Error("unexpected double block during write");
        }
      }
      
      
      // Host owned writes were not necessarily unblocked, but are always blocked
      // because they happen just-before a component read (via a lowered end).
      //
      // In this case, we cant to declare the copy state back to idle
      // for the next write that is performed, assuming there may be more writes
      // to do.
      //
      // if (this.#hostOwned) {
        //    this.setCopyState(StreamEnd.CopyState.IDLE);
        // }
        
        // If the write was not blocked, we can resolve right away
        this.#result = null;
        resolve();
        
      } catch (err) {
        _debugLog('[StreamWritableEnd#write()] error', err);
        reject(err);
      }
      
      return await promise;
    }
    
    
    _write(args) {
      const { buffer, onCopyFn, onCopyDoneFn, componentIdx } = args;
      if (!buffer) { throw new TypeError('missing/invalid buffer'); }
      if (!onCopyFn) { throw new TypeError("missing/invalid onCopy handler"); }
      if (!onCopyDoneFn) { throw new TypeError("missing/invalid onCopyDone handler"); }
      if (this.isDropped()) {
        onCopyDoneFn(StreamEnd.CopyResult.DROPPED);
        return;
      }
      
      if (!this.#pendingBufferMeta.buffer) {
        this.setPendingBufferMeta({ componentIdx, buffer, onCopyFn, onCopyDoneFn });
        return;
      }
      
      const pendingElemMeta = this.#pendingBufferMeta.buffer.getElemMeta();
      const newBufferElemMeta = buffer.getElemMeta();
      if (pendingElemMeta.payloadTypeName !== newBufferElemMeta.payloadTypeName) {
        throw new Error("trap: stream end type does not match internal buffer");
      }
      
      // If the buffer came from the same component that is currently doing the operation
      // we're doing a inter-component write, and only unit or numeric types are allowed
      const pendingElemIsNoneOrNumeric = pendingElemMeta.isNone || pendingElemMeta.isNumeric;
      if (this.#pendingBufferMeta.componentIdx === buffer.componentIdx() && buffer.componentIdx() !== -1 && !pendingElemIsNoneOrNumeric) {
        throw new Error(`trap: cannot stream non-numeric types within the same component (component [${buffer.componentIdx()}], send)`);
      }
      
      // If original capacities were zero, we're dealing with a unit stream,
      // a write to the unit stream is instantly copied without any work.
      if (buffer.capacity === 0 && this.#pendingBufferMeta.buffer.capacity === 0) {
        onCopyDoneFn(StreamEnd.CopyResult.COMPLETED);
        return;
      }
      
      // If the internal buffer has no space left to take writes,
      // the write is complete, we must reset and wait for another read
      // to clear up space in the buffer.
      if (this.#pendingBufferMeta.buffer.remaining() === 0) {
        this.resetAndNotifyPending(StreamEnd.CopyResult.COMPLETED);
        this.setPendingBufferMeta({ componentIdx, buffer, onCopyFn, onCopyDoneFn });
        return;
      }
      
      // At this point it is implied that remaining is > 0,
      // so if there is still remaining capacity in the incoming buffer, perform copy of values
      // to the internal buffer from the incoming buffer
      let transferred = false;
      if (buffer.remaining() > 0) {
        const rejectLength = this.#pendingBufferMeta.rejectLength;
        if (rejectLength !== undefined && buffer.remaining() > rejectLength) {
          const pendingOnCopyDoneFn = this.#pendingBufferMeta.onCopyDoneFn;
          this.resetPendingBufferMeta();
          pendingOnCopyDoneFn(StreamEnd.CopyResult.DROPPED, buffer.remaining());
          onCopyDoneFn(StreamEnd.CopyResult.DROPPED);
          return;
        }
        const numElements = Math.min(buffer.remaining(), this.#pendingBufferMeta.buffer.remaining());
        this.#pendingBufferMeta.buffer.write(buffer.read(numElements));
        this.#pendingBufferMeta.onCopyFn(() => this.resetPendingBufferMeta());
        transferred = true;
      }
      
      onCopyDoneFn(StreamEnd.CopyResult.COMPLETED);
    }
    
    
    setupCopy(args) {
      const {
        memory,
        ptr,
        count,
        eventCode,
        componentIdx,
        skipStateCheck,
      } = args;
      if (eventCode === undefined) { throw new Error("missing/invalid event code"); }
      
      let buffer = args.buffer;
      let bufferID = args.bufferID;
      
      // Only check invariants if we are *not* doing a follow-up/post-blocked read
      if (!skipStateCheck) {
        if (this.isCopying()) {
          throw new Error('stream is currently undergoing a separate copy');
        }
        if (this.getCopyState() !== StreamEnd.CopyState.IDLE) {
          throw new Error(`stream copy state is not idle`);
        }
      }
      
      const elemMeta = this.getElemMeta();
      if (elemMeta.isBorrowed) { throw new Error('borrowed types cannot be sent over streams'); }
      
      // If we already have a managed buffer (likely host case), we can use that, otherwise we must
      // create a buffer (likely in the guest case)
      if (!buffer) {
        const newBufferMeta = BUFFER_MGR.createBuffer({
          componentIdx,
          memory,
          start: ptr,
          count,
          // If creating a buffer for a write operation, the buffer we are encapsulating
          // is a *readable* buffer from the view of the component (as it has written to that buffer data that)
          // should be sent out
          isReadable: this.isWritable(),
          // If creating a buffer for a read operation, the buffer we are encapsulating
          // is a *writable* buffer from the view of the component (as it has prepared space to receive data)
          isWritable: this.isReadable(),
          elemMeta,
        });
        bufferID = newBufferMeta.id;
        buffer = newBufferMeta.buffer;
        buffer.setTarget(`component [${componentIdx}] StreamWritableEnd buffer (id [${bufferID}], count [${count}], eventCode [${eventCode}])`);
      }
      
      const streamEnd = this;
      const processFn = (result, reclaimBufferFn, rejectedLength) => {
        if (reclaimBufferFn) { reclaimBufferFn(); }
        
        if (result === StreamEnd.CopyResult.DROPPED) {
          streamEnd.setCopyState(StreamEnd.CopyState.DONE);
        } else {
          streamEnd.setCopyState(StreamEnd.CopyState.IDLE);
        }
        
        if (result < 0 || result >= 16) {
          throw new Error(`unsupported stream copy result [${result}]`);
        }
        if (buffer.processed >= ManagedBuffer.MAX_LENGTH) {
          throw new Error(`processed count [${buf.length}] greater than max length`);
        }
        if (buffer.length > 2**28) { throw new Error('buffer uses reserved space'); }
        
        const packedResult = (Number(buffer.processed) << 4) | result;
        const event = { code: eventCode, payload0: streamEnd.waitableIdx(), payload1: packedResult };
        if (rejectedLength !== undefined) {
          event.rejectedLength = rejectedLength;
        }
        
        return event;
      };
      
      const onCopyFn = (reclaimBufferFn) => {
        streamEnd.setPendingEvent(() => {
          return processFn(StreamEnd.CopyResult.COMPLETED, reclaimBufferFn);
        });
      };
      
      const onCopyDoneFn = (result, rejectedLength) => {
        streamEnd.setPendingEvent(() => {
          return processFn(result, undefined, rejectedLength);
        });
      };
      
      return { bufferID, buffer, onCopyFn, onCopyDoneFn };
    }
    
    
    async copy(args) {
      const {
        isAsync,
        memory,
        componentIdx,
        ptr,
        count,
        eventCode,
        initial,
        skipStateCheck,
        stringEncoding,
        reallocFn,
        rejectLength,
      } = args;
      if (eventCode === undefined) { throw new TypeError('missing/invalid event code'); }
      
      if (this.#elemMeta.stringEncoding === undefined && stringEncoding) {
        this.#elemMeta.stringEncoding = stringEncoding;
      }
      if (this.#elemMeta.stringEncoding && stringEncoding && this.#elemMeta.stringEncoding !== stringEncoding) {
        throw new Error(`inconsistent string encoding (previously [${this.#elemMeta.stringEncoding}], now [${stringEncoding}])`);
      }
      
      if (args.getReallocFn && this.#elemMeta.getReallocFn === undefined) {
        this.#elemMeta.getReallocFn = args.getReallocFn;
      }
      
      if (this.isDropped()) {
        if (this.#pendingBufferMeta?.onCopyDoneFn) {
          const f = this.#pendingBufferMeta.onCopyDoneFn;
          this.#pendingBufferMeta.onCopyDoneFn = null;
          f(StreamEnd.CopyResult.DROPPED);
        }
        this.setCopyState(StreamEnd.CopyState.DONE);
        return StreamEnd.CopyResult.DROPPED;
      }
      
      const { buffer, onCopyFn, onCopyDoneFn } = this.setupCopy({
        memory,
        eventCode,
        componentIdx,
        ptr,
        count,
        buffer: args.buffer,
        bufferID: args.bufferID,
        initial,
        skipStateCheck,
      });
      
      // If the stream is readable and was lowered from the host, the
      // writer is host-side. Register the read first; host injection
      // will no-op if the read already produced a pending event.
      const injectHostWrite = this.isReadable() && !!this.#hostInjectFn;
      
      // Perform the read/write
      this._write({
        buffer,
        onCopyFn,
        onCopyDoneFn,
        componentIdx,
        rejectLength,
      });
      
      let injectedWritePromise;
      if (injectHostWrite) {
        injectedWritePromise = this.#hostInjectFn({ count });
      }
      
      // If sync, wait forever but allow task to do other things
      if (!this.hasPendingEvent()) {
        if (isAsync) {
          this.setCopyState(StreamEnd.CopyState.ASYNC_COPYING);
          _debugLog('[StreamEnd#copy()] blocked', { componentIdx, eventCode, self: this });
          if (injectedWritePromise) {
            // Do not await here: the injected write may depend on sibling
            // guest work running, so the canonical read must return BLOCKED.
            injectedWritePromise.then(
            cleanupFn => cleanupFn(),
            err => this.setPendingEvent(() => { throw err; }),
            );
          }
          return ASYNC_BLOCKED_CODE;
        } else {
          this.setCopyState(StreamEnd.CopyState.SYNC_COPYING);
          
          const taskMeta = getCurrentTask(componentIdx);
          if (!taskMeta) { throw new Error(`missing task meta for component idx [${componentIdx}]`); }
          
          const task = taskMeta.task;
          if (!task) { throw new Error('missing task task from task meta'); }
          
          const streamEnd = this;
          await task.suspendUntil({
            readyFn: () => streamEnd.hasPendingEvent(),
          });
        }
      }
      
      // If the read completed immediately after injecting a host write,
      // it is safe to await injection cleanup before consuming the event.
      if (injectedWritePromise) {
        const cleanupFn = await injectedWritePromise;
        cleanupFn();
      }
      
      const event = this.getPendingEvent();
      if (!event) { throw new Error("unexpectedly missing pending event"); }
      if (event.code === undefined || event.payload0 === undefined || event.payload1 === undefined) {
        throw new Error("unexpectedly malformed event");
      }
      
      const { code, payload0: index, payload1: payload } = event;
      
      const waitableIdx = this.getWaitable().idx();
      if (code !== eventCode  || index !== waitableIdx || payload === ASYNC_BLOCKED_CODE) {
        const errMsg = "invalid event code/event during stream operation";
        _debugLog(errMsg, {
          event,
          payload,
          payloadIsBlockedConst: payload === ASYNC_BLOCKED_CODE,
          code,
          eventCode,
          codeDoesNotMatchEventCode: code !== eventCode,
          index,
          internalEndIdx: waitableIdx,
          indexDoesNotMatch: index !== waitableIdx,
        });
        throw new Error(errMsg);
      }
      
      if (event.rejectedLength !== undefined) {
        this.#rejectedLength = event.rejectedLength;
      }
      return payload;
    }
    
    
    setPendingBufferMeta(args) {
      const { componentIdx, buffer, onCopyFn, onCopyDoneFn, rejectLength } = args;
      this.#pendingBufferMeta.componentIdx = componentIdx;
      this.#pendingBufferMeta.buffer = buffer;
      this.#pendingBufferMeta.onCopyFn = onCopyFn;
      this.#pendingBufferMeta.onCopyDoneFn = onCopyDoneFn;
      this.#pendingBufferMeta.rejectLength = rejectLength;
    }
    
    resetPendingBufferMeta() {
      this.setPendingBufferMeta({ componentIdx: null, buffer: null, onCopyFn: null, onCopyDoneFn: null, rejectLength: undefined });
    }
    
    getPendingBufferMeta() { return this.#pendingBufferMeta; }
    
    resetAndNotifyPending(result) {
      const f = this.#pendingBufferMeta.onCopyDoneFn;
      this.resetPendingBufferMeta();
      if (f) { f(result); }
    }
    
    cancel() {
      _debugLog('[StreamEnd#cancel()]');
      const completeCancel = () => {
        if (this.isDropped()) { return; }
        if (this.#hostCancelFn?.()) { return; }
        const result = this.#pendingBufferMeta?.buffer?.processed > 0
        ? StreamEnd.CopyResult.COMPLETED
        : StreamEnd.CopyResult.CANCELLED;
        this.resetAndNotifyPending(result);
      };
      if (this.#hostInjectFn) {
        setTimeout(completeCancel, 0);
      } else {
        completeCancel();
      }
    }
    
    drop() {
      _debugLog('[StreamEnd#drop()]');
      if (this.isDropped()) { return; }
      if (this.#hostDropFn) {
        Promise.resolve(this.#hostDropFn()).catch(err => {
          _debugLog('[StreamEnd#drop()] host drop failed', err);
        });
        this.#hostDropFn = null;
      }
      super.drop();
      if (this.#pendingBufferMeta) {
        const result = this.#pendingBufferMeta.buffer?.processed > 0
        ? StreamEnd.CopyResult.COMPLETED
        : StreamEnd.CopyResult.DROPPED;
        this.resetAndNotifyPending(result);
      }
    }
  }
  
  class Stream {
    #globalRep = null;
    #isReadable;
    #isWritable;
    #writeFn;
    #readFn;
    #dropFn;
    
    constructor(args) {
      _debugLog('[Stream#constructor()] args', args);
      
      if (args.globalRep === undefined) { throw new TypeError("missing host stream rep"); }
      this[symbolRscRep] = args.globalRep;
      
      if (args.isReadable === undefined) { throw new TypeError("missing readable setting"); }
      this.#isReadable = args.isReadable;
      
      if (args.isWritable === undefined) { throw new TypeError("missing writable setting"); }
      this.#isWritable = args.isWritable;
      
      if (this.#isWritable && args.writeFn === undefined) { throw new TypeError("missing write fn"); }
      this.#writeFn = args.writeFn;
      
      if (this.#isReadable && args.readFn === undefined) { throw new TypeError("missing read fn"); }
      this.#readFn = args.readFn;
      
      this.#dropFn = args.dropFn;
    }
    
    [symbolAsyncIterator]() { return this; }
    
    async return() {
      this[symbolDispose]();
      return { done: true };
    }
    
    async next() {
      _debugLog('[Stream#next()]');
      return this.read();
    }
    
    async read(opts) {
      _debugLog('[Stream#read()]', { opts });
      if (!this.#isReadable) { throw new Error("stream is not marked as readable and cannot be read from"); }
      const readOpts = this.#readOpts(opts);
      return this.#readFn(readOpts);
    }
    
    #readOpts(opts) {
      const count = opts === undefined ? 1 : typeof opts === "number" ? opts : opts && typeof opts === "object" ? opts.count ?? 1 : undefined;
      const rejectLength = opts && typeof opts === "object" ? opts.rejectLength : undefined;
      if (!Number.isInteger(count) || count < (rejectLength !== undefined ? 0 : 1)) { throw new TypeError(`invalid stream read count [${count}]`); }
      if (rejectLength !== undefined && (!Number.isInteger(rejectLength) || rejectLength < 0)) {
        throw new TypeError(`invalid stream read reject length [${rejectLength}]`);
      }
      return { count, rejectLength };
    }
    
    async write() {
      _debugLog('[Stream#write()]');
      if (!this.#isWritable) { throw new Error("stream is not marked as writable and cannot be written to"); }
      
      const objects = [...arguments];
      if (!objects.length !== 1) {
        throw new Error("only single object writes are currently supported");
      }
      const obj = objects[0];
      
      this.#writeFn(obj);
    }
    
    [symbolDispose]() {
      this.#dropFn?.();
    }
    
  }
  
  class PendingValueQueue {
    #readFn;
    #elemMeta;
    #done = false;
    #sourceReadPromise = null;
    #chunks = [];
    #offset = 0;
    #length = 0;
    
    constructor(readFn, elemMeta) {
      this.#readFn = readFn;
      this.#elemMeta = elemMeta;
    }
    
    get length() { return this.#length; }
    get done() { return this.#done; }
    
    push(source) {
      if (source.length === 0) { return 0; }
      this.#chunks.push(source);
      this.#length += source.length;
      return source.length;
    }
    
    appendReadValue(value) {
      if (value === undefined) { return 0; }
      if (this.#elemMeta.isNumeric) {
        if (value instanceof ArrayBuffer) {
          value = new Uint8Array(value);
        }
        if (Array.isArray(value) || (ArrayBuffer.isView(value) && typeof value.length === 'number')) {
          return this.push(value);
        }
      }
      return this.push([value]);
    }
    
    async readSource() {
      if (!this.#sourceReadPromise) {
        this.#sourceReadPromise = (async () => {
          const res = await this.#readFn();
          const appended = this.appendReadValue(res.value);
          this.#done = res.done;
          return appended;
        })().finally(() => {
          this.#sourceReadPromise = null;
        });
      }
      return this.#sourceReadPromise;
    }
    
    prepend(source) {
      if (source.length === 0) { return; }
      if (this.#offset !== 0 && this.#chunks.length > 0) {
        this.#chunks[0] = this.#chunks[0].slice(this.#offset);
        this.#offset = 0;
      }
      this.#chunks.unshift(source);
      this.#length += source.length;
    }
    
    drainInto(target, maxCount) {
      let transferred = 0;
      let remaining = Math.min(maxCount, this.#length);
      while (remaining > 0) {
        const chunk = this.#chunks[0];
        const transfer = Math.min(remaining, chunk.length - this.#offset);
        for (let i = 0; i < transfer; i++) {
          target.push(chunk[this.#offset + i]);
        }
        this.#offset += transfer;
        this.#length -= transfer;
        transferred += transfer;
        remaining -= transfer;
        if (this.#offset === chunk.length) {
          this.#chunks.shift();
          this.#offset = 0;
        }
      }
      return transferred;
    }
  }
  
  function streamNew(ctx) {
    _debugLog('[streamNew()] args', { ctx });
    const {
      streamTableIdx,
      callerComponentIdx,
      elemMeta,
    } = ctx;
    if (callerComponentIdx === undefined) { throw new Error("missing caller component idx during stream.new"); }
    
    const taskMeta = getCurrentTask(callerComponentIdx);
    if (!taskMeta) { throw new Error('missing async task metadata during stream.new'); }
    
    const task = taskMeta.task
    if (!task) { throw new Error('invalid/missing async task during stream.new'); }
    
    if (task.componentIdx() !== callerComponentIdx) {
      throw new Error(`task component idx [${task.componentIdx()}] does not match stream new intrinsic component idx [${callerComponentIdx}]`);
    }
    
    const cstate = getOrCreateAsyncState(callerComponentIdx);
    if (!cstate.mayLeave) {
      throw new Error('component instance is not marked as may leave during stream.new');
    }
    
    const { writeEndWaitableIdx, readEndWaitableIdx, writeEndHandle, readEndHandle } = cstate.createStream({
      tableIdx: streamTableIdx,
      elemMeta,
    });
    
    _debugLog('[streamNew()] created stream ends', {
      writeEnd: {
        waitableIdx: writeEndWaitableIdx,
        handle: writeEndHandle,
      },
      readEnd: {
        waitableIdx: readEndWaitableIdx,
        handle: readEndHandle,
      },
      streamTableIdx,
      callerComponentIdx,
    });
    
    return (BigInt(writeEndWaitableIdx) << 32n) | BigInt(readEndWaitableIdx);
  }
  
  async function streamRead(
  ctx,
  streamEndWaitableIdx,
  ptr,
  count,
  ) {
    _debugLog('[streamRead()] args', { ctx, streamEndWaitableIdx, ptr, count });
    const {
      componentIdx,
      memoryIdx,
      getMemoryFn,
      reallocIdx,
      getReallocFn,
      stringEncoding,
      isAsync,
      streamTableIdx,
    } = ctx;
    
    if (componentIdx === undefined) { throw new TypeError("missing/invalid component idx"); }
    if (streamTableIdx === undefined) { throw new TypeError("missing/invalid stream table idx"); }
    if (streamEndWaitableIdx === undefined) { throw new TypeError("missing/invalid stream end idx"); }
    
    // count may come in as u32::MAX which is mangled by JS into a negative value
    count = Math.min(count >>> 0, ManagedBuffer.MAX_LENGTH);
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate.mayLeave) { throw new Error('component instance is not marked as may leave'); }
    
    if (!CURRENT_TASK_MAY_BLOCK && !isAsync) {
      throw new Error('trap: only async tasks or otherwise blocking-allowed tasks my stream.streamRead');
    }
    
    const streamEnd = cstate.getStreamEnd({ tableIdx: streamTableIdx, streamEndWaitableIdx });
    if (!streamEnd) {
      throw new Error(`missing stream end [${streamEndWaitableIdx}] (table [${streamTableIdx}], component [${componentIdx}])`);
    }
    if (!(streamEnd instanceof StreamReadableEnd)) {
      throw new Error('invalid stream type, expected StreamReadableEnd');
    }
    if (streamEnd.streamTableIdx() !== streamTableIdx) {
      throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] != operation table idx [${streamTableIdx}]`);
    }
    
    const result = await streamEnd.copy({
      isAsync,
      memory: getMemoryFn(),
      ptr,
      count,
      eventCode: ASYNC_EVENT_CODE.STREAM_READ,
      componentIdx,
      stringEncoding,
      realloc: getReallocFn?.(),
      getReallocFn,
    });
    
    return result;
  }
  
  async function streamWrite(
  ctx,
  streamEndWaitableIdx,
  ptr,
  count,
  ) {
    _debugLog('[streamWrite()] args', { ctx, streamEndWaitableIdx, ptr, count });
    const {
      componentIdx,
      memoryIdx,
      getMemoryFn,
      reallocIdx,
      getReallocFn,
      stringEncoding,
      isAsync,
      streamTableIdx,
    } = ctx;
    
    if (componentIdx === undefined) { throw new TypeError("missing/invalid component idx"); }
    if (streamTableIdx === undefined) { throw new TypeError("missing/invalid stream table idx"); }
    if (streamEndWaitableIdx === undefined) { throw new TypeError("missing/invalid stream end idx"); }
    
    // count may come in as u32::MAX which is mangled by JS into a negative value
    count = Math.min(count >>> 0, ManagedBuffer.MAX_LENGTH);
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate.mayLeave) { throw new Error('component instance is not marked as may leave'); }
    
    if (!CURRENT_TASK_MAY_BLOCK && !isAsync) {
      throw new Error('trap: only async tasks or otherwise blocking-allowed tasks my stream.streamWrite');
    }
    
    const streamEnd = cstate.getStreamEnd({ tableIdx: streamTableIdx, streamEndWaitableIdx });
    if (!streamEnd) {
      throw new Error(`missing stream end [${streamEndWaitableIdx}] (table [${streamTableIdx}], component [${componentIdx}])`);
    }
    if (!(streamEnd instanceof StreamWritableEnd)) {
      throw new Error('invalid stream type, expected StreamWritableEnd');
    }
    if (streamEnd.streamTableIdx() !== streamTableIdx) {
      throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] != operation table idx [${streamTableIdx}]`);
    }
    
    const result = await streamEnd.copy({
      isAsync,
      memory: getMemoryFn(),
      ptr,
      count,
      eventCode: ASYNC_EVENT_CODE.STREAM_WRITE,
      componentIdx,
      stringEncoding,
      realloc: getReallocFn?.(),
      getReallocFn,
    });
    
    return result;
  }
  
  async function streamCancelRead(ctx, streamEndWaitableIdx) {
    _debugLog('[streamCancelRead()] args', { ctx, streamEndWaitableIdx });
    const { streamTableIdx, isAsync, componentIdx } = ctx;
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate.mayLeave) { throw new Error('component instance is not marked as may leave'); }
    
    const streamEnd = cstate.getStreamEnd({ streamEndWaitableIdx, tableIdx: streamTableIdx });
    if (!streamEnd) { throw new Error('missing stream end with idx [' + streamEndWaitableIdx + ']'); }
    if (!(streamEnd instanceof StreamReadableEnd)) { throw new Error('invalid stream end, expected value of type [StreamReadableEnd]'); }
    
    if (!streamEnd.isCopying()) { throw new Error('stream end is not copying, cannot cancel'); }
    
    streamEnd.setCopyState(StreamReadableEnd.CopyState.CANCELLING_COPY);
    
    if (!streamEnd.hasPendingEvent()) {
      
      streamEnd.cancel();
      
      if (!streamEnd.hasPendingEvent()) {
        if (isAsync) { return ASYNC_BLOCKED_CODE; }
        
        const taskMeta = getCurrentTask(componentIdx);
        if (!taskMeta) { throw new Error('missing current task metadata while doing stream transfer'); }
        const task = taskMeta.task;
        if (!task) { throw new Error('missing task while doing stream transfer'); }
        await task.suspendUntil({ readyFn: () => streamEnd.hasPendingEvent() });
      }
    }
    
    const event = streamEnd.getPendingEvent();
    const { code, payload0: index, payload1: payload } = event;
    if (streamEnd.isCopying()) {
      throw new Error(`stream end (idx [${streamEndWaitableIdx}]) is still in copying state`);
    }
    if (code !== ASYNC_EVENT_CODE.STREAM_READ) {
      throw new Error(`unexpected event code [${code}], expected [ASYNC_EVENT_CODE.STREAM_READ]`);
    }
    if (index !== streamEnd.waitableIdx()) { throw new Error('event index does not match stream end'); }
    
    _debugLog('[streamCancelRead()] successful cancel', { ctx, streamEndWaitableIdx, streamEnd, event });
    return payload;
  }
  
  async function streamCancelWrite(ctx, streamEndWaitableIdx) {
    _debugLog('[streamCancelWrite()] args', { ctx, streamEndWaitableIdx });
    const { streamTableIdx, isAsync, componentIdx } = ctx;
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate.mayLeave) { throw new Error('component instance is not marked as may leave'); }
    
    const streamEnd = cstate.getStreamEnd({ streamEndWaitableIdx, tableIdx: streamTableIdx });
    if (!streamEnd) { throw new Error('missing stream end with idx [' + streamEndWaitableIdx + ']'); }
    if (!(streamEnd instanceof StreamWritableEnd)) { throw new Error('invalid stream end, expected value of type [StreamWritableEnd]'); }
    
    if (!streamEnd.isCopying()) { throw new Error('stream end is not copying, cannot cancel'); }
    
    streamEnd.setCopyState(StreamWritableEnd.CopyState.CANCELLING_COPY);
    
    if (!streamEnd.hasPendingEvent()) {
      
      streamEnd.cancel();
      
      if (!streamEnd.hasPendingEvent()) {
        if (isAsync) { return ASYNC_BLOCKED_CODE; }
        
        const taskMeta = getCurrentTask(componentIdx);
        if (!taskMeta) { throw new Error('missing current task metadata while doing stream transfer'); }
        const task = taskMeta.task;
        if (!task) { throw new Error('missing task while doing stream transfer'); }
        await task.suspendUntil({ readyFn: () => streamEnd.hasPendingEvent() });
      }
    }
    
    const event = streamEnd.getPendingEvent();
    const { code, payload0: index, payload1: payload } = event;
    if (streamEnd.isCopying()) {
      throw new Error(`stream end (idx [${streamEndWaitableIdx}]) is still in copying state`);
    }
    if (code !== ASYNC_EVENT_CODE.STREAM_WRITE) {
      throw new Error(`unexpected event code [${code}], expected [ASYNC_EVENT_CODE.STREAM_WRITE]`);
    }
    if (index !== streamEnd.waitableIdx()) { throw new Error('event index does not match stream end'); }
    
    _debugLog('[streamCancelWrite()] successful cancel', { ctx, streamEndWaitableIdx, streamEnd, event });
    return payload;
  }
  
  function streamDropReadable(ctx, streamEndWaitableIdx) {
    _debugLog('[streamDropReadable()] args', { ctx, streamEndWaitableIdx });
    const { streamTableIdx, componentIdx } = ctx;
    
    const task = getCurrentTask(componentIdx);
    if (!task) { throw new Error('invalid/missing async task'); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate) { throw new Error(`missing component state for component idx [${componentIdx}]`); }
    
    const streamEnd = cstate.deleteStreamEnd({ tableIdx: streamTableIdx, streamEndWaitableIdx });
    if (!streamEnd) {
      throw new Error(`missing stream (waitable [${streamEndWaitableIdx}], table [${streamTableIdx}], component [${componentIdx}])`);
    }
    
    if (!(streamEnd instanceof StreamReadableEnd)) {
      throw new Error('invalid stream end class, expected [StreamReadableEnd]');
    }
    
    streamEnd.drop();
  }
  
  function streamDropWritable(ctx, streamEndWaitableIdx) {
    _debugLog('[streamDropWritable()] args', { ctx, streamEndWaitableIdx });
    const { streamTableIdx, componentIdx } = ctx;
    
    const task = getCurrentTask(componentIdx);
    if (!task) { throw new Error('invalid/missing async task'); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    if (!cstate) { throw new Error(`missing component state for component idx [${componentIdx}]`); }
    
    const streamEnd = cstate.deleteStreamEnd({ tableIdx: streamTableIdx, streamEndWaitableIdx });
    if (!streamEnd) {
      throw new Error(`missing stream (waitable [${streamEndWaitableIdx}], table [${streamTableIdx}], component [${componentIdx}])`);
    }
    
    if (!(streamEnd instanceof StreamWritableEnd)) {
      throw new Error('invalid stream end class, expected [StreamWritableEnd]');
    }
    
    streamEnd.drop();
  }
  
  function _isStreamLowerableObject(obj) {
    if (typeof obj !== 'object') { return false; }
    return obj instanceof Stream
    || symbolAsyncIterator in obj
    || symbolIterator in obj
    || obj instanceof _PlatformReadableStream;
  }
  
  function _genStreamHostInjectFn(genArgs) {
    const { readFn, hostWriteEnd, readEnd } = genArgs;
    if (!readEnd) { throw new TypeError('missing read end'); }
    const doNothingFn = () => {};
    const resetWriteEndToIdleFn = () => {
      // After the write is finished, we consume the event that was generated
      // by the just-in-time write (and the subsequent read), if one was generated
      if (hostWriteEnd.hasPendingEvent()) { hostWriteEnd.getPendingEvent(); }
    };
    
    const elemMeta = hostWriteEnd.getElemMeta();
    
    const pendingValues = new PendingValueQueue(readFn, elemMeta);
    
    return async function generatedStreamHostInject(args) {
      let { count } = args;
      if (count < 0) { throw new Error('invalid count'); }
      if (readEnd.hasPendingEvent()) { return resetWriteEndToIdleFn; }
      
      if (hostWriteEnd.isDoneState()) {
        return doNothingFn;
      }
      
      const values = [];
      const hasPendingReadBuffer = () => !!readEnd.getPendingBufferMeta?.().buffer;
      
      const drainPendingValues = () => {
        count -= pendingValues.drainInto(values, count);
      };
      
      const writeValues = async (writeValues) => {
        const writePromise = hostWriteEnd.writeMany(writeValues);
        if (hostWriteEnd.hasPendingEvent()) {
          void writePromise.catch(() => {});
        } else {
          await writePromise;
        }
        resetWriteEndToIdleFn();
      };
      
      const bail = () => {
        pendingValues.prepend(values);
        return doNothingFn;
      };
      
      readEnd.setHostCancelFn?.(() => {
        const buffer = readEnd.getPendingBufferMeta?.().buffer;
        if (!buffer || pendingValues.length === 0) { return false; }
        const cancelValues = [];
        pendingValues.drainInto(cancelValues, buffer.remaining());
        if (cancelValues.length === 0) { return false; }
        const writePromise = hostWriteEnd.writeMany(cancelValues);
        if (!hostWriteEnd.hasPendingEvent()) {
          pendingValues.prepend(cancelValues);
          return false;
        }
        void writePromise.catch(() => {});
        resetWriteEndToIdleFn();
        return true;
      });
      
      if (!hasPendingReadBuffer()) { return doNothingFn; }
      if (count === 0) {
        if (pendingValues.length === 0 && !pendingValues.done) {
          await pendingValues.readSource();
          if (readEnd.hasPendingEvent() || !hasPendingReadBuffer()) { return doNothingFn; }
        }
        if (pendingValues.length > 0) {
          const readyValues = [];
          pendingValues.drainInto(readyValues, 1);
          await writeValues(readyValues);
        } else if (pendingValues.done) {
          hostWriteEnd.getPendingEvent();
          hostWriteEnd.drop();
        }
        return doNothingFn;
      }
      drainPendingValues();
      
      while (count > 0 && !pendingValues.done) {
        const appended = await pendingValues.readSource();
        if (readEnd.hasPendingEvent()) { return bail(); }
        if (!hasPendingReadBuffer()) { return bail(); }
        drainPendingValues();
        if (values.length > 0) { break; }
        if (appended === 0 && !pendingValues.done) { count -= 1; }
        if (pendingValues.done) { break; }
      }
      
      // Iterator provided `done: true` with no final value
      if (pendingValues.done && values.length === 0 && pendingValues.length > 0 && hasPendingReadBuffer()) {
        drainPendingValues();
      }
      if (pendingValues.done && values.length === 0) {
        hostWriteEnd.getPendingEvent();
        hostWriteEnd.drop();
        return doNothingFn;
      }
      
      if (!hasPendingReadBuffer()) { return bail(); }
      await writeValues(values);
      
      return doNothingFn;
    };
  }
  
  function _genReadFnFromLowerableStream(stream) {
    if (!_isStreamLowerableObject(stream)) {
      throw new Error("cannot generate read fn: object is not a stream lowerable object");
    }
    
    let readFn;
    if (symbolAsyncIterator in stream) {
      let asyncIterator = stream[symbolAsyncIterator]();
      readFn = () => asyncIterator.next();
      readFn.drop = (reason) => asyncIterator.return?.(reason) ?? stream[symbolDispose]?.();
    } else if (symbolIterator in stream) {
      let iterator = stream[symbolIterator]();
      readFn = async () => iterator.next();
      readFn.drop = (reason) => iterator.return?.(reason) ?? stream[symbolDispose]?.();
    } else if (stream instanceof _PlatformReadableStream) {
      // At this point we're dealing with a readable stream that *somehow *does not*
      // implement the async iterator protocol.
      const lockedReader = stream.getReader();
      readFn = () => lockedReader.read();
      readFn.drop = (reason) => lockedReader.cancel(reason).finally(() => lockedReader.releaseLock());
    } else {
      throw new Error("invalid stream object, cannot generate read fn");
    }
    
    return readFn;
  }
  const ASYNC_STATE = new Map();
  
  function getOrCreateAsyncState(componentIdx, init) {
    if (!ASYNC_STATE.has(componentIdx)) {
      const newState = new ComponentAsyncState({ componentIdx });
      ASYNC_STATE.set(componentIdx, newState);
    }
    return ASYNC_STATE.get(componentIdx);
  }
  
  class ComponentAsyncState {
    static EVENT_HANDLER_EVENTS = [ 'backpressure-change' ];
    
    #componentIdx;
    #callingAsyncImport = false;
    #syncImportWait = promiseWithResolvers();
    #locked = false;
    #parkedTasks = new Map();
    #suspendedTasksByTaskID = new Map();
    #suspendedTaskIDs = [];
    #errored = null;
    
    #backpressure = 0;
    #backpressureWaiters = 0n;
    
    #handlerMap = new Map();
    #nextHandlerID = 0n;
    
    #tickLoop = null;
    #tickLoopInterval = null;
    
    #onExclusiveReleaseHandlers = [];
    
    mayLeave = true;
    
    handles;
    subtasks;
    
    constructor(args) {
      this.#componentIdx = args.componentIdx;
      this.handles = new RepTable({ target: `component [${this.#componentIdx}] handles (waitable objects)` });
      this.subtasks = new RepTable({ target: `component [${this.#componentIdx}] subtasks` });
    };
    
    componentIdx() { return this.#componentIdx; }
    
    errored() { return this.#errored !== null; }
    setErrored(err) {
      _debugLog('[ComponentAsyncState#setErrored()] component errored', { err, componentIdx: this.#componentIdx });
      if (this.#errored) { return; }
      if (!err) {
        err = new Error('error elswehere (see other component instance error)')
        err.componentIdx = this.#componentIdx;
      }
      this.#errored = err;
    }
    
    callingSyncImport(val) {
      if (val === undefined) { return this.#callingAsyncImport; }
      if (typeof val !== 'boolean') { throw new TypeError('invalid setting for async import'); }
      const prev = this.#callingAsyncImport;
      this.#callingAsyncImport = val;
      if (prev === true && this.#callingAsyncImport === false) {
        this.#notifySyncImportEnd();
      }
    }
    
    #notifySyncImportEnd() {
      const existing = this.#syncImportWait;
      this.#syncImportWait = promiseWithResolvers();
      existing.resolve();
    }
    
    async waitForSyncImportCallEnd() {
      await this.#syncImportWait.promise;
    }
    
    setBackpressure(v) {
      this.#backpressure = v;
      return this.#backpressure
    }
    getBackpressure() { return this.#backpressure; }
    
    incrementBackpressure() {
      const current = this.#backpressure;
      if (current < 0 || current > 2**16) {
        throw new Error(`invalid current backpressure value [${current}]`);
      }
      const newValue = this.getBackpressure() + 1;
      if (newValue >= 2**16) {
        throw new Error(`invalid new backpressure value [${newValue}], overflow`);
      }
      return this.setBackpressure(newValue);
    }
    
    decrementBackpressure() {
      const current = this.#backpressure;
      if (current < 0 || current > 2**16) {
        throw new Error(`invalid current backpressure value [${current}]`);
      }
      const newValue = Math.max(0, current - 1);
      if (newValue < 0) {
        throw new Error(`invalid new backpressure value [${newValue}], underflow`);
      }
      return this.setBackpressure(newValue);
    }
    hasBackpressure() { return this.#backpressure > 0; }
    
    waitForBackpressure() {
      let backpressureCleared = false;
      const cstate = this;
      cstate.addBackpressureWaiter();
      const handlerID = this.registerHandler({
        event: 'backpressure-change',
        fn: (bp) => {
          if (bp === 0) {
            cstate.removeHandler(handlerID);
            backpressureCleared = true;
          }
        }
      });
      return new Promise((resolve) => {
        const interval = setInterval(() => {
          if (backpressureCleared) { return; }
          clearInterval(interval);
          cstate.removeBackpressureWaiter();
          resolve(null);
        }, 0);
      });
    }
    
    registerHandler(args) {
      const { event, fn } = args;
      if (!event) { throw new Error("missing handler event"); }
      if (!fn) { throw new Error("missing handler fn"); }
      
      if (!ComponentAsyncState.EVENT_HANDLER_EVENTS.includes(event)) {
        throw new Error(`unrecognized event handler [${event}]`);
      }
      
      const handlerID = this.#nextHandlerID++;
      let handlers = this.#handlerMap.get(event);
      if (!handlers) {
        handlers = [];
        this.#handlerMap.set(event, handlers)
      }
      
      handlers.push({ id: handlerID, fn, event });
      return handlerID;
    }
    
    removeHandler(args) {
      const { event, handlerID } = args;
      const registeredHandlers = this.#handlerMap.get(event);
      if (!registeredHandlers) { return; }
      const found = registeredHandlers.find(h => h.id === handlerID);
      if (!found) { return; }
      this.#handlerMap.set(event, this.#handlerMap.get(event).filter(h => h.id !== handlerID));
    }
    
    getBackpressureWaiters() { return this.#backpressureWaiters; }
    addBackpressureWaiter() { this.#backpressureWaiters++; }
    removeBackpressureWaiter() {
      this.#backpressureWaiters--;
      if (this.#backpressureWaiters < 0) {
        throw new Error("unexepctedly negative number of backpressure waiters");
      }
    }
    
    isExclusivelyLocked() { return this.#locked === true; }
    setLocked(locked) {
      this.#locked = locked;
    }
    
    exclusiveLock() {
      _debugLog('[ComponentAsyncState#exclusiveLock()]', {
        locked: this.#locked,
        componentIdx: this.#componentIdx,
      });
      this.setLocked(true);
    }
    
    exclusiveRelease() {
      _debugLog('[ComponentAsyncState#exclusiveRelease()] args', {
        locked: this.#locked,
        componentIdx: this.#componentIdx,
      });
      this.setLocked(false);
      
      this.#onExclusiveReleaseHandlers = this.#onExclusiveReleaseHandlers.filter(v => !!v);
      for (const [idx, f] of this.#onExclusiveReleaseHandlers.entries()) {
        try {
          this.#onExclusiveReleaseHandlers[idx] = null;
          f();
        } catch (err) {
          _debugLog("error while executing handler for next exclusive release", err);
          throw err;
        }
      }
    }
    
    onNextExclusiveRelease(fn) {
      _debugLog('[ComponentAsyncState#()onNextExclusiveRelease] registering');
      this.#onExclusiveReleaseHandlers.push(fn);
    }
    
    // nextTaskPromise & nextTaskQueue are used to await current task completion and queues
    // any tasks attempting to enter() and complete.
    //
    // see: nextTaskExecutionSlot()
    //
    // TODO(threads): this should be unnecessary once threads are properly implemented,
    // as the task.enter() logic should suffice (it should be guaranteed that we cannot re-enter
    // unless the task in question is the current task in the thread execution, and only one can
    // run at a time)
    #nextTaskPromise = Promise.resolve(true);
    #nextTaskQueue = [];
    
    async nextTaskExecutionSlot(args) {
      const { task } = args;
      
      const placeholder = {
        completed: false,
        task,
        promise: task.exitPromise().then(() => {
          placeholder.completed = true;
        }),
      };
      this.#nextTaskQueue.push(placeholder);
      
      let next;
      while (true) {
        await this.#nextTaskPromise;
        
        next = this.#nextTaskQueue.find(placeholder => !placeholder.completed);
        
        // This task is next in the queue, we can continue
        if (next === undefined || next === placeholder) {
          this.#nextTaskPromise = next.promise;
          if (this.#nextTaskQueue.length > 1000) {
            this.#nextTaskQueue = this.#nextTaskQueue.filter(p => !p.completed);
            if (this.#nextTaskQueue.length > 1000) {
              _debugLog('[ComponentAsyncState#()nextTaskExecutionSlot] next task queue length > 1000 even after cleanup, tasks may be leaking');
            }
          }
          break;
        }
        
        // If we get here, this task was *not* next in the queue, continue waiting
        // (at this point the task that *is* next will likely have already set itself
        // as this.#nextTaskPromise)
      }
    }
    
    #getSuspendedTaskMeta(taskID) {
      return this.#suspendedTasksByTaskID.get(taskID);
    }
    
    #removeSuspendedTaskMeta(taskID) {
      _debugLog('[ComponentAsyncState#removeSuspendedTaskMeta()] removing suspended task', {
        taskID,
        componentIdx: this.#componentIdx,
      });
      const idx = this.#suspendedTaskIDs.findIndex(t => t === taskID);
      const meta = this.#suspendedTasksByTaskID.get(taskID);
      this.#suspendedTaskIDs[idx] = null;
      this.#suspendedTasksByTaskID.delete(taskID);
      return meta;
    }
    
    #addSuspendedTaskMeta(meta) {
      if (!meta) { throw new Error('missing task meta'); }
      const taskID = meta.taskID;
      this.#suspendedTasksByTaskID.set(taskID, meta);
      this.#suspendedTaskIDs.push(taskID);
      if (this.#suspendedTasksByTaskID.size < this.#suspendedTaskIDs.length - 10) {
        this.#suspendedTaskIDs = this.#suspendedTaskIDs.filter(t => t !== null);
      }
    }
    
    // TODO(threads): readyFn is normally on the thread
    suspendTask(args) {
      const { task, readyFn } = args;
      const taskID = task.id();
      const componentIdx = task.componentIdx();
      _debugLog('[ComponentAsyncState#suspendTask()]', {
        taskID,
        componentIdx: this.#componentIdx,
        taskEntryFnName: task.entryFnName(),
        subtask: task.getParentSubtask(),
      });
      
      if (componentIdx !== this.#componentIdx) {
        throw new Error('assert: task component idx should match async state');
      }
      
      if (this.#getSuspendedTaskMeta(taskID)) {
        throw new Error(`task [${taskID}] already suspended`);
      }
      
      const { promise, resolve, reject } = promiseWithResolvers();
      this.#addSuspendedTaskMeta({
        task,
        taskID,
        readyFn,
        resume: () => {
          _debugLog('[ComponentAsyncState] resuming suspended task', {
            taskID,
            componentIdx: this.#componentIdx,
          });
          // TODO(threads): it's thread cancellation we should be checking for below, not task
          resolve(!task.isCancelled());
        },
      });
      
      this.runTickLoop();
      
      return promise;
    }
    
    resumeTaskByID(taskID) {
      const meta = this.#removeSuspendedTaskMeta(taskID);
      if (!meta) { return; }
      if (meta.taskID !== taskID) { throw new Error('task ID does not match'); }
      meta.resume();
    }
    
    async runTickLoop() {
      if (this.#tickLoop !== null) { return; }
      this.#tickLoop = 1;
      setTimeout(async () => {
        let done = this.tick();
        while (!done) {
          await new Promise((resolve) => setTimeout(resolve, 30));
          done = this.tick();
        }
        this.#tickLoop = null;
      }, 10);
    }
    
    tick() {
      // _debugLog('[ComponentAsyncState#tick()]', { suspendedTaskIDs: this.#suspendedTaskIDs });
      
      const resumableTasks = this.#suspendedTaskIDs.filter(t => t !== null);
      for (const taskID of resumableTasks) {
        const meta = this.#suspendedTasksByTaskID.get(taskID);
        if (!meta || !meta.readyFn) {
          throw new Error(`missing/invalid task despite ID [${taskID}] being present`);
        }
        
        // If the task failed via any means, allow the task to resume because
        // it's been cancelled -- the callback should immediately exit as well
        if (meta.task.isRejected()) {
          _debugLog('[ComponentAsyncState#tick()] detected task rejection, leaving early', { meta });
          this.resumeTaskByID(taskID);
          return;
        }
        
        const isReady = meta.readyFn();
        if (!isReady) { continue; }
        
        _debugLog('[ComponentAsyncState#tick()] resuming task via tick', {
          taskID,
          componentIdx: this.#componentIdx,
        });
        this.resumeTaskByID(taskID);
      }
      
      return this.#suspendedTaskIDs.filter(t => t !== null).length === 0;
    }
    
    addStreamEndToTable(args) {
      _debugLog('[ComponentAsyncState#addStreamEnd()] args', args);
      const { tableIdx, streamEnd } = args;
      if (typeof streamEnd === 'number') { throw new Error("INSERTING BAD STREAMEND"); }
      
      let { table, componentIdx } = STREAM_TABLES[tableIdx];
      if (componentIdx === undefined || !table) {
        throw new Error(`invalid global stream table state for table [${tableIdx}]`);
      }
      
      const handle = table.insert(streamEnd);
      streamEnd.setHandle(handle);
      streamEnd.setStreamTableIdx(tableIdx);
      
      const cstate = getOrCreateAsyncState(componentIdx);
      const waitableIdx = cstate.handles.insert(streamEnd);
      streamEnd.setWaitableIdx(waitableIdx);
      
      _debugLog('[ComponentAsyncState#addStreamEnd()] added stream end', {
        tableIdx,
        table,
        handle,
        streamEnd,
        destComponentIdx: componentIdx,
      });
      
      return { handle, waitableIdx };
    }
    
    createWaitable(args) {
      return new Waitable({ target: args?.target, });
    }
    
    createReadableStreamEnd(args) {
      _debugLog('[ComponentAsyncState#createStreamEnd()] args', args);
      const { tableIdx, elemMeta, hostInjectFn } = args;
      
      const { table: localStreamTable, componentIdx } = STREAM_TABLES[tableIdx];
      if (!localStreamTable) {
        throw new Error(`missing global stream table lookup for table [${tableIdx}] while creating stream`);
      }
      if (componentIdx !== this.#componentIdx) {
        throw new Error('component idx mismatch while creating stream');
      }
      
      const waitable = this.createWaitable();
      const streamEnd = new StreamReadableEnd({
        tableIdx,
        elemMeta,
        hostInjectFn,
        pendingBufferMeta: {},
        target: `stream read end (lowered, @init)`,
        waitable,
      });
      
      streamEnd.setWaitableIdx(this.handles.insert(streamEnd));
      streamEnd.setHandle(localStreamTable.insert(streamEnd));
      if (streamEnd.streamTableIdx() !== tableIdx) {
        throw new Error("unexpectedly mismatched stream table");
      }
      const streamEndWaitableIdx = streamEnd.waitableIdx();
      const streamEndHandle = streamEnd.handle();
      waitable.setTarget(`waitable for stream read end (lowered, waitable [${streamEndWaitableIdx}])`);
      streamEnd.setTarget(`stream read end (lowered, waitable [${streamEndWaitableIdx}])`);
      
      return {
        waitableIdx: streamEndWaitableIdx,
        handle: streamEndHandle,
        streamEnd,
      };
    }
    
    createStream(args) {
      _debugLog('[ComponentAsyncState#createStream()] args', args);
      const { tableIdx, elemMeta, hostInjectFn } = args;
      if (tableIdx === undefined) { throw new Error("missing table idx while adding stream"); }
      if (elemMeta === undefined) { throw new Error("missing element metadata while adding stream"); }
      
      const { table: localStreamTable, componentIdx } = STREAM_TABLES[tableIdx];
      if (!localStreamTable) {
        throw new Error(`missing global stream table lookup for table [${tableIdx}] while creating stream`);
      }
      if (componentIdx !== this.#componentIdx) {
        throw new Error('component idx mismatch while creating stream');
      }
      
      const readWaitable = this.createWaitable();
      const writeWaitable = this.createWaitable();
      
      const stream = new InternalStream({
        tableIdx,
        elemMeta,
        readWaitable,
        writeWaitable,
        hostInjectFn,
      });
      stream.setGlobalStreamMapRep(STREAMS.insert(stream));
      
      const writeEnd = stream.writeEnd();
      writeEnd.setWaitableIdx(this.handles.insert(writeEnd));
      writeEnd.setHandle(localStreamTable.insert(writeEnd));
      if (writeEnd.streamTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched stream table"); }
      
      const writeEndWaitableIdx = writeEnd.waitableIdx();
      const writeEndHandle = writeEnd.handle();
      writeWaitable.setTarget(`waitable for stream write end (waitable [${writeEndWaitableIdx}])`);
      writeEnd.setTarget(`stream write end (waitable [${writeEndWaitableIdx}])`);
      
      const readEnd = stream.readEnd();
      readEnd.setWaitableIdx(this.handles.insert(readEnd));
      readEnd.setHandle(localStreamTable.insert(readEnd));
      if (readEnd.streamTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched stream table"); }
      
      const readEndWaitableIdx = readEnd.waitableIdx();
      const readEndHandle = readEnd.handle();
      readWaitable.setTarget(`waitable for read end (waitable [${readEndWaitableIdx}])`);
      readEnd.setTarget(`stream read end (waitable [${readEndWaitableIdx}])`);
      
      return {
        writeEnd,
        writeEndWaitableIdx,
        writeEndHandle,
        readEndWaitableIdx,
        readEndHandle,
        readEnd,
      };
    }
    
    getStreamEnd(args) {
      _debugLog('[ComponentAsyncState#getStreamEnd()] args', args);
      const { tableIdx, streamEndHandle, streamEndWaitableIdx } = args;
      if (tableIdx === undefined) {
        throw new Error('missing table idx while getting stream end');
      }
      
      const { table, componentIdx } = STREAM_TABLES[tableIdx];
      const cstate = getOrCreateAsyncState(componentIdx);
      
      let streamEnd;
      if (streamEndWaitableIdx !== undefined) {
        streamEnd = cstate.handles.get(streamEndWaitableIdx);
      } else if (streamEndHandle !== undefined) {
        if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while getting stream end`); }
        streamEnd = table.get(streamEndHandle);
      } else {
        throw new TypeError("must specify either waitable idx or handle to retrieve stream");
      }
      
      if (!streamEnd) {
        throw new Error(`missing stream end (tableIdx [${tableIdx}], handle [${streamEndHandle}], waitableIdx [${streamEndWaitableIdx}])`);
      }
      if (tableIdx && streamEnd.streamTableIdx() !== tableIdx) {
        throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] does not match [${tableIdx}]`);
      }
      
      return streamEnd;
    }
    
    deleteStreamEnd(args) {
      _debugLog('[ComponentAsyncState#deleteStreamEnd()] args', args);
      const { tableIdx, streamEndWaitableIdx } = args;
      if (tableIdx === undefined) { throw new Error("missing table idx while removing stream end"); }
      if (streamEndWaitableIdx === undefined) { throw new Error("missing stream idx while removing stream end"); }
      
      const { table, componentIdx } = STREAM_TABLES[tableIdx];
      const cstate = getOrCreateAsyncState(componentIdx);
      
      const streamEnd = cstate.handles.get(streamEndWaitableIdx);
      if (!streamEnd) {
        throw new Error(`missing stream end [${streamEndWaitableIdx}] in component handles while deleting stream`);
      }
      if (streamEnd.streamTableIdx() !== tableIdx) {
        throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] does not match [${tableIdx}]`);
      }
      
      let removed = cstate.handles.remove(streamEnd.waitableIdx());
      if (!removed) {
        throw new Error(`failed to remove stream end [${streamEndWaitableIdx}] waitable obj in component [${componentIdx}]`);
      }
      
      removed = table.remove(streamEnd.handle());
      if (!removed) {
        throw new Error(`failed to remove stream end with handle [${streamEnd.handle()}] from stream table [${tableIdx}] in component [${componentIdx}]`);
      }
      
      return streamEnd;
    }
    
    removeStreamEndFromTable(args) {
      _debugLog('[ComponentAsyncState#removeStreamEndFromTable()] args', args);
      
      const { tableIdx, streamWaitableIdx } = args;
      if (tableIdx === undefined) { throw new Error("missing table idx while removing stream end"); }
      if (streamWaitableIdx === undefined) {
        throw new Error("missing stream end waitable idx while removing stream end");
      }
      
      const { table, componentIdx } = STREAM_TABLES[tableIdx];
      if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while removing stream end`); }
      
      const cstate = getOrCreateAsyncState(componentIdx);
      
      const streamEnd = cstate.handles.get(streamWaitableIdx);
      if (!streamEnd) {
        throw new Error(`missing stream end (handle [${streamWaitableIdx}], table [${tableIdx}])`);
      }
      const handle = streamEnd.handle();
      
      let removed = cstate.handles.remove(streamWaitableIdx);
      if (!removed) {
        throw new Error(`failed to remove streamEnd from handles (waitable idx [${streamWaitableIdx}]), component [${componentIdx}])`);
      }
      
      removed = table.remove(handle);
      if (!removed) {
        throw new Error(`failed to remove streamEnd from table (handle [${handle}]), table [${tableIdx}], component [${componentIdx}])`);
      }
      
      return streamEnd;
    }
    
    createFuture(args) {
      _debugLog('[ComponentAsyncState#createFuture()] args', args);
      const { tableIdx, elemMeta, hostInjectFn } = args;
      if (tableIdx === undefined) { throw new Error("missing table idx while adding future"); }
      if (elemMeta === undefined) { throw new Error("missing element metadata while adding future"); }
      
      const { table: futureTable, componentIdx } = FUTURE_TABLES[tableIdx];
      if (!futureTable) {
        throw new Error(`missing global future table lookup for table [${tableIdx}] while creating future`);
      }
      if (componentIdx !== this.#componentIdx) {
        throw new Error('component idx mismatch while creating future');
      }
      
      const readWaitable = this.createWaitable();
      const writeWaitable = this.createWaitable();
      
      const future = new InternalFuture({
        tableIdx,
        componentIdx: this.#componentIdx,
        elemMeta,
        readWaitable,
        writeWaitable,
        hostInjectFn,
      });
      future.setGlobalFutureMapRep(FUTURES.insert(future));
      
      const writeEnd = future.writeEnd();
      writeEnd.setWaitableIdx(this.handles.insert(writeEnd));
      writeEnd.setHandle(futureTable.insert(writeEnd));
      if (writeEnd.futureTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched future table"); }
      
      const writeEndWaitableIdx = writeEnd.waitableIdx();
      const writeEndHandle = writeEnd.handle();
      writeWaitable.setTarget(`waitable for future write end (waitable [${writeEndWaitableIdx}])`);
      writeEnd.setTarget(`future write end (waitable [${writeEndWaitableIdx}])`);
      
      const readEnd = future.readEnd();
      readEnd.setWaitableIdx(this.handles.insert(readEnd));
      readEnd.setHandle(futureTable.insert(readEnd));
      if (readEnd.futureTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched future table"); }
      
      const readEndWaitableIdx = readEnd.waitableIdx();
      const readEndHandle = readEnd.handle();
      readWaitable.setTarget(`waitable for read end (waitable [${readEndWaitableIdx}])`);
      readEnd.setTarget(`future read end (waitable [${readEndWaitableIdx}])`);
      
      return {
        writeEnd,
        writeEndWaitableIdx,
        writeEndHandle,
        readEndWaitableIdx,
        readEndHandle,
        readEnd,
      };
    }
    
    getFutureEnd(args) {
      _debugLog('[ComponentAsyncState#getFutureEnd()] args', args);
      const { tableIdx, futureEndHandle, futureEndWaitableIdx } = args;
      if (tableIdx === undefined) {
        throw new Error('missing table idx while getting future end');
      }
      
      const { table, componentIdx } = FUTURE_TABLES[tableIdx];
      const cstate = getOrCreateAsyncState(componentIdx);
      
      let futureEnd;
      if (futureEndWaitableIdx !== undefined) {
        futureEnd = cstate.handles.get(futureEndWaitableIdx);
      } else if (futureEndHandle !== undefined) {
        if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while getting future end`); }
        futureEnd = table.get(futureEndHandle);
      } else {
        throw new TypeError("must specify either waitable idx or handle to retrieve future");
      }
      
      if (!futureEnd) {
        throw new Error(`missing future end (tableIdx [${tableIdx}], handle [${futureEndHandle}], waitableIdx [${futureEndWaitableIdx}])`);
      }
      if (tableIdx && futureEnd.futureTableIdx() !== tableIdx) {
        throw new Error(`future end table idx [${futureEnd.futureTableIdx()}] does not match [${tableIdx}]`);
      }
      
      return futureEnd;
    }
    
    removeFutureEndFromTable(args) {
      _debugLog('[ComponentAsyncState#removeFutureEndFromTable()] args', args);
      
      const { tableIdx, futureWaitableIdx } = args;
      if (tableIdx === undefined) { throw new Error("missing table idx while removing future end"); }
      if (futureWaitableIdx === undefined) {
        throw new Error("missing future end waitable idx while removing future end");
      }
      
      const { table, componentIdx } = FUTURE_TABLES[tableIdx];
      if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while removing future end`); }
      
      const cstate = getOrCreateAsyncState(componentIdx);
      
      const futureEnd = cstate.handles.get(futureWaitableIdx);
      if (!futureEnd) {
        throw new Error(`missing future end (handle [${futureWaitableIdx}], table [${tableIdx}])`);
      }
      const handle = futureEnd.handle();
      
      let removed = cstate.handles.remove(futureWaitableIdx);
      if (!removed) {
        throw new Error(`failed to remove futureEnd from handles (waitable idx [${futureWaitableIdx}]), component [${componentIdx}])`);
      }
      
      removed = table.remove(handle);
      if (!removed) {
        throw new Error(`failed to remove futureEnd from table (handle [${handle}]), table [${tableIdx}], component [${componentIdx}])`);
      }
      
      return futureEnd;
    }
    
  }
  
  function _ComponentStateSetAllError() {
    _debugLog('[_ComponentStateSetAllError()]');
    for (const state of ASYNC_STATE.values()) {
      state.setErrored();
    }
  }
  
  function _storeEventInComponentMemory(args) {
    _debugLog('[_storeEventInComponentMemory()] args', args);
    const { memory, ptr, event } = args;
    
    if (!memory) { throw new Error('unexpectedly missing memory'); }
    if (ptr === undefined || ptr === null) { throw new Error('unexpectedly missing pointer'); }
    if (!event) { throw new Error('event object missing'); }
    if (event.code === undefined) { throw new Error('invalid event object, missing code'); }
    if (event.payload0 === undefined) { throw new Error('invalid event object, missing payload0'); }
    if (event.payload1 === undefined) { throw new Error('invalid event object, missing payload1'); }
    
    const dv = new DataView(memory.buffer);
    dv.setUint32(ptr, event.payload0, true);
    dv.setUint32(ptr + 4, event.payload1, true);
    
    return event.code;
  }
  
  const base64Compile = str => WebAssembly.compile(
  typeof Buffer !== 'undefined'
  ? Buffer.from(str, 'base64')
  : Uint8Array.from(atob(str), b => b.charCodeAt(0))
  );
  
  
  const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
  let _fs;
  async function fetchCompile (url) {
    if (isNode) {
      _fs = _fs || await import('node:fs/promises');
      return WebAssembly.compile(await _fs.readFile(url));
    }
    return fetch(url).then(WebAssembly.compileStreaming);
  }
  
  const symbolCabiDispose = Symbol.for('cabiDispose');
  
  const symbolRscHandle = Symbol('handle');
  
  const symbolRscRep = Symbol.for('cabiRep');
  
  const HANDLE_TABLES= [];
  
  
  function getErrorPayload(e) {
    if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
    if (e instanceof Error) throw e;
    return e;
  }
  
  class ManagedBuffer {
    static MAX_LENGTH = 2**28 - 1;
    #componentIdx;
    #memory;
    
    #elemMeta = null;
    
    #start;
    #ptr;
    capacity;
    processed = 0;
    
    #hostOnlyData; // initial data (only filled out for host-owned)
    
    target;
    
    constructor(args) {
      if (args.capacity > ManagedBuffer.MAX_LENGTH) {
        throw new Error(`buffer size [${args.capacity}] greater than max length`);
      }
      if (args.componentIdx === undefined) { throw new TypeError('missing/invalid component idx'); }
      if (args.capacity === undefined) { throw new TypeError('missing/invalid capacity'); }
      if (!args.elemMeta || typeof args.elemMeta.align32 !== 'number') {
        throw new TypeError('missing/invalid element metadata');
      }
      
      if (!args.memory && args.start === undefined && args.data === undefined) {
        throw new TypeError('either memory and start ptr or data must be provided for managed buffers');
      }
      
      if (args.memory && args.start == undefined) {
        throw new TypeError('missing/invalid start ptr, depsite memory being present');
      }
      
      if (!args.elemMeta.isNone && args.capacity > 0) {
        if (args.start && args.start % args.elemMeta.align32 !== 0) {
          throw new Error(`invalid alignment: type with 32bit alignment [${args.elemMeta.align32}] at starting pointer [${args.start}]`);
        }
        // TODO: memory lenght bounds check
      }
      
      this.#componentIdx = args.componentIdx;
      this.#memory = args.memory;
      this.#start = args.start;
      this.#ptr = this.#start;
      this.capacity = args.capacity;
      this.#elemMeta = args.elemMeta;
      
      if (args.data !== undefined && !Array.isArray(args.data)) {
        throw new TypeError('host-only data must be an array');
      }
      this.#hostOnlyData = args.data;
      
      this.target = args.target;
    }
    
    setTarget(tgt) { this.target = tgt; }
    
    remaining() {
      return this.capacity - this.processed;
    }
    
    componentIdx() { return this.#componentIdx; }
    
    getElemMeta() { return this.#elemMeta; }
    
    isHostOwned() { return !this.#memory; }
    
    read(count) {
      _debugLog('[ManagedBuffer#read()] args', { count });
      if (count === undefined || count <= 0) {
        throw new TypeError(`missing/invalid count [${count}]`);
      }
      
      const cap = this.capacity;
      if (count > cap) {
        throw new Error(`cannot read [${count}] elements from buffer with capacity [${cap}]`);
      }
      
      let values = [];
      if (this.#elemMeta.isNone) {
        values = [...new Array(count)].map(() => null);
      } else {
        if (this.isHostOwned()) {
          values = this.#hostOnlyData.slice(0, count);
          this.#hostOnlyData = this.#hostOnlyData.slice(count);
        } else if (this.#elemMeta.payloadTypeName === 'U8') {
          values = Array.from(new Uint8Array(this.#memory.buffer, this.#ptr, count));
          this.#ptr += count;
        } else {
          let currentCount = count;
          let startPtr = this.#ptr;
          if (this.#elemMeta.stringEncoding === undefined) {
            throw new Error('string encoding unknown during read');
          }
          let liftCtx = {
            storagePtr: startPtr,
            memory: this.#memory,
            componentIdx: this.#componentIdx,
            stringEncoding: this.#elemMeta.stringEncoding,
          };
          if (currentCount < 0) { throw new Error('unexpectedly invalid count'); }
          while (currentCount > 0) {
            const [value, _ctx] = this.#elemMeta.liftFn(liftCtx);
            values.push(value);
            currentCount -= 1;
          }
          this.#ptr = liftCtx.storagePtr;
        }
      }
      
      this.processed += count;
      return values;
    }
    
    write(values) {
      _debugLog('[ManagedBuffer#write()] args', { values });
      
      if (!Array.isArray(values)) { throw new TypeError('values input to write() must be an array'); }
      let rc = this.remaining();
      if (values.length > rc) {
        throw new Error(`cannot write [${values.length}] elements to managed buffer with remaining capacity [${rc}]`);
      }
      
      if (this.#elemMeta.isNone) {
        if (!values.every(v => v === null)) {
          throw new Error('non-null values in write() to unit managed buffer');
        }
      } else {
        if (this.isHostOwned()) {
          this.#hostOnlyData = this.#hostOnlyData.concat(values);
        } else if (this.#elemMeta.payloadTypeName === 'U8') {
          new Uint8Array(this.#memory.buffer, this.#ptr, values.length).set(values);
          this.#ptr += values.length;
        } else {
          let startPtr = this.#ptr;
          if (this.#elemMeta.stringEncoding === undefined) {
            throw new Error('string encoding unknown during write');
          }
          
          const lowerCtx = {
            memory: this.#memory,
            storagePtr: startPtr,
            componentIdx: this.#componentIdx,
            stringEncoding: this.#elemMeta.stringEncoding,
            realloc: this.#elemMeta.getReallocFn?.(),
            getReallocFn: this.#elemMeta.getReallocFn,
          }
          for (const v of values) {
            lowerCtx.vals = [v];
            this.#elemMeta.lowerFn(lowerCtx);
          }
          
          this.#ptr = lowerCtx.storagePtr;
        }
      }
      
      this.processed += values.length;
    }
    
  }
  
  class BufferManager {
    #buffers = new Map();
    #bufferIDs = new Map();
    
    // NOTE: componentIdx === -1 indicates the host
    getNextBufferID(componentIdx) {
      const current = this.#bufferIDs.get(componentIdx);
      if (current === undefined) {
        this.#bufferIDs.set(componentIdx, 1n);
        return 1n;
      }
      const next = current + 1n;
      this.#bufferIDs.set(componentIdx, next);
      return next;
    }
    
    getBuffer(componentIdx, bufferID) {
      _debugLog('[BufferManager#getBuffer()] args', { componentIdx, bufferID });
      return this.#buffers.get(componentIdx)?.get(bufferID);
    }
    
    createBuffer(args) {
      _debugLog('[BufferManager#createBuffer()] args', args);
      if (!args || typeof args !== 'object') { throw new TypeError('missing/invalid argument object'); }
      
      if (args.start === undefined && args.data === undefined) {
        throw new  TypeError('either a starting pointer or initial values must be provided');
      }
      
      if (args.start !== undefined && args.componentIdx === undefined) { throw new TypeError('missing/invalid component idx'); }
      if (args.count === undefined) { throw new TypeError('missing/invalid obj count'); }
      if (!args.elemMeta) { throw new TypeError('missing/invalid element metadata for use with managed buffer'); }
      
      const { componentIdx, data, start, count } = args;
      
      if (!this.#buffers.has(componentIdx)) { this.#buffers.set(componentIdx, new Map()); }
      const instanceBuffers = this.#buffers.get(componentIdx);
      
      const nextBufID = this.getNextBufferID(componentIdx);
      
      const buffer = new ManagedBuffer({
        componentIdx,
        memory: args.memory,
        start: args.start,
        capacity: args.count,
        elemMeta: args.elemMeta,
        data: args.data,
        target: args.target,
        stringEncoding: args.stringEncoding,
      });
      
      if (instanceBuffers.has(nextBufID)) {
        throw new Error(`managed buffer with ID [${nextBufID}] already exists`);
      }
      instanceBuffers.set(nextBufID, buffer);
      
      return { id: nextBufID, buffer };
    }
    
    deleteBuffer(componentIdx, bufferID) {
      _debugLog('[BufferManager#deleteBuffer()] args', { componentIdx, bufferID });
      return this.#buffers.get(componentIdx)?.delete(bufferID);
    }
    
  }
  const BUFFER_MGR = new BufferManager();
  const isLE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;
  
  const hasOwnProperty = Object.prototype.hasOwnProperty;
  
  const instantiateCore = WebAssembly.instantiate;
  
  
  STREAM_TABLES[0] = { componentIdx: 0, table: new RepTable() };
  let exports0;
  
  const _trampoline18 = function(arg0) {
    let variant0;
    switch (arg0) {
      case 0: {
        variant0= {
          tag: 'ok',
          val: undefined
        };
        break;
      }
      case 1: {
        variant0= {
          tag: 'err',
          val: undefined
        };
        break;
      }
      default: {
        throw new TypeError('invalid variant discriminant for expected');
      }
    }
    _debugLog('[iface="wasi:cli/exit@0.2.9", function="exit"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'exit',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => exit(variant0),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    _debugLog('[iface="wasi:cli/exit@0.2.9", function="exit"][Instruction::Return]', {
      funcName: 'exit',
      paramCount: 0,
      async: false,
      postReturn: false
    });
    task.resolve([ret]);
    task.exit();
  }
  _trampoline18.fnName = 'wasi:cli/exit@0.2.9#exit';
  
  const handleTable0 = [T_FLAG, 0];
  handleTable0._createdReps = new Set();
  
  
  const captureTable0= new Map();
  let captureCnt0= 0;
  
  HANDLE_TABLES[0] = handleTable0;
  
  const _trampoline19 = function(arg0) {
    var handle1 = arg0;
    
    var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable0.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(Pollable.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
      Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
    }
    
    curResourceBorrows.push(rsc0);
    _debugLog('[iface="wasi:io/poll@0.2.9", function="[method]pollable.block"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'block',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => rsc0.block(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    for (const entry of curResourceBorrows) {
      const rsc = entry.rsc ?? entry;
      if (entry.drop) {
        if (rsc[symbolRscHandle]) {
          entry.drop(rsc[symbolRscHandle]);
        }
      }
      rsc[symbolRscHandle] = undefined;
    }
    curResourceBorrows = [];
    _debugLog('[iface="wasi:io/poll@0.2.9", function="[method]pollable.block"][Instruction::Return]', {
      funcName: '[method]pollable.block',
      paramCount: 0,
      async: false,
      postReturn: false
    });
    task.resolve([ret]);
    task.exit();
  }
  _trampoline19.fnName = 'wasi:io/poll@0.2.9#block';
  
  const handleTable3 = [T_FLAG, 0];
  handleTable3._createdReps = new Set();
  
  
  const captureTable3= new Map();
  let captureCnt3= 0;
  
  HANDLE_TABLES[3] = handleTable3;
  
  const _trampoline20 = function(arg0) {
    var handle1 = arg0;
    
    var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable3.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(OutputStream.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
      Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
    }
    
    curResourceBorrows.push(rsc0);
    _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.subscribe"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'subscribe',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => rsc0.subscribe(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    for (const entry of curResourceBorrows) {
      const rsc = entry.rsc ?? entry;
      if (entry.drop) {
        if (rsc[symbolRscHandle]) {
          entry.drop(rsc[symbolRscHandle]);
        }
      }
      rsc[symbolRscHandle] = undefined;
    }
    curResourceBorrows = [];
    
    if (!(ret instanceof Pollable)) {
      throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
    }
    var handle3 = ret[symbolRscHandle];
    if (!handle3) {
      const rep = ret[symbolRscRep] || ++captureCnt0;
      captureTable0.set(rep, ret);
      handle3 = rscTableCreateOwn(handleTable0, rep);
    }
    
    _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.subscribe"][Instruction::Return]', {
      funcName: '[method]output-stream.subscribe',
      paramCount: 1,
      async: false,
      postReturn: false
    });
    task.resolve([handle3]);
    task.exit();
    return handle3;
  }
  _trampoline20.fnName = 'wasi:io/streams@0.2.9#subscribe';
  
  const handleTable2 = [T_FLAG, 0];
  handleTable2._createdReps = new Set();
  
  
  const captureTable2= new Map();
  let captureCnt2= 0;
  
  HANDLE_TABLES[2] = handleTable2;
  
  const _trampoline21 = function() {
    _debugLog('[iface="wasi:cli/stdin@0.2.9", function="get-stdin"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'getStdin',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => getStdin(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    
    if (!(ret instanceof InputStream)) {
      throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
    }
    var handle0 = ret[symbolRscHandle];
    if (!handle0) {
      const rep = ret[symbolRscRep] || ++captureCnt2;
      captureTable2.set(rep, ret);
      handle0 = rscTableCreateOwn(handleTable2, rep);
    }
    
    _debugLog('[iface="wasi:cli/stdin@0.2.9", function="get-stdin"][Instruction::Return]', {
      funcName: 'get-stdin',
      paramCount: 1,
      async: false,
      postReturn: false
    });
    task.resolve([handle0]);
    task.exit();
    return handle0;
  }
  _trampoline21.fnName = 'wasi:cli/stdin@0.2.9#getStdin';
  
  const _trampoline22 = function() {
    _debugLog('[iface="wasi:cli/stdout@0.2.9", function="get-stdout"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'getStdout',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => getStdout(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    
    if (!(ret instanceof OutputStream)) {
      throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
    }
    var handle0 = ret[symbolRscHandle];
    if (!handle0) {
      const rep = ret[symbolRscRep] || ++captureCnt3;
      captureTable3.set(rep, ret);
      handle0 = rscTableCreateOwn(handleTable3, rep);
    }
    
    _debugLog('[iface="wasi:cli/stdout@0.2.9", function="get-stdout"][Instruction::Return]', {
      funcName: 'get-stdout',
      paramCount: 1,
      async: false,
      postReturn: false
    });
    task.resolve([handle0]);
    task.exit();
    return handle0;
  }
  _trampoline22.fnName = 'wasi:cli/stdout@0.2.9#getStdout';
  
  const _trampoline23 = function() {
    _debugLog('[iface="wasi:cli/stderr@0.2.9", function="get-stderr"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'getStderr',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => getStderr(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    
    if (!(ret instanceof OutputStream)) {
      throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
    }
    var handle0 = ret[symbolRscHandle];
    if (!handle0) {
      const rep = ret[symbolRscRep] || ++captureCnt3;
      captureTable3.set(rep, ret);
      handle0 = rscTableCreateOwn(handleTable3, rep);
    }
    
    _debugLog('[iface="wasi:cli/stderr@0.2.9", function="get-stderr"][Instruction::Return]', {
      funcName: 'get-stderr',
      paramCount: 1,
      async: false,
      postReturn: false
    });
    task.resolve([handle0]);
    task.exit();
    return handle0;
  }
  _trampoline23.fnName = 'wasi:cli/stderr@0.2.9#getStderr';
  
  const _trampoline24 = function(arg0) {
    _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.9", function="subscribe-duration"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'subscribeDuration',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => subscribeDuration(BigInt.asUintN(64, BigInt(arg0))),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    
    if (!(ret instanceof Pollable)) {
      throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
    }
    var handle0 = ret[symbolRscHandle];
    if (!handle0) {
      const rep = ret[symbolRscRep] || ++captureCnt0;
      captureTable0.set(rep, ret);
      handle0 = rscTableCreateOwn(handleTable0, rep);
    }
    
    _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.9", function="subscribe-duration"][Instruction::Return]', {
      funcName: 'subscribe-duration',
      paramCount: 1,
      async: false,
      postReturn: false
    });
    task.resolve([handle0]);
    task.exit();
    return handle0;
  }
  _trampoline24.fnName = 'wasi:clocks/monotonic-clock@0.2.9#subscribeDuration';
  let exports1;
  let memory0;
  let realloc0;
  let realloc0Async;
  
  const _trampoline26 = async function(arg0) {
    _debugLog('[iface="semio:jcoprobe/probe-host@0.1.0", function="fetch-body"] [Instruction::CallInterface] (async, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: true,
        entryFnName: 'fetchBody',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    
    const started = await task.enter({ isHost: hostProvided });
    if (!started) {
      _debugLog('[Instruction::CallInterface] failed to enter task', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
      });
      throw new Error("failed to enter task");
    }
    
    
    let ret;
    
    try {
      ret = await  _withGlobalCurrentTaskMetaAsync({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => fetchBody(),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during async call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      return task.completionPromise();
      
    }
    
    _debugLog('[iface="semio:jcoprobe/probe-host@0.1.0", function="fetch-body"][Instruction::AsyncTaskReturn]', {
      funcName: '[task-return]fetch-body',
      paramCount: 1,
      componentIdx: 0,
      postReturn: false,
      hostProvided,
    });
    
    if (hostProvided) {
      _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
        task: task.id(),
        subtask: subtask?.id(),
        result: ret,
      })
      task.resolve([ret]);
      task.exit();
      return await task.completionPromise();
    }
    
    const componentState = getOrCreateAsyncState(0);
    if (!componentState) { throw new Error('failed to lookup current component state'); }
    
    queueMicrotask(async (resolve, reject) => {
      try {
        _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
          fnName: '[task-return]fetch-body',
          componentInstanceIdx: 0,
          taskID: task.id(),
        });
        await _driverLoop({
          componentInstanceIdx: 0,
          componentState,
          task,
          fnName: '[task-return]fetch-body',
          isAsync: true,
          callbackResult: ret,
        });
      } catch (err) {
        _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
      }
    });
    
    let taskRes = await task.completionPromise();
    if (task.getErrHandling() === 'throw-result-err') {
      if (typeof taskRes !== 'object') {
        return taskRes;
      }
      if (taskRes.tag === 'err') { throw taskRes.val; }
      if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
    }
    
    return taskRes;
    
  }
  _trampoline26.fnName = 'semio:jcoprobe/probe-host@0.1.0#fetchBody';
  
  const _trampoline27 = async function(arg0, arg1, arg2) {
    _debugLog('[iface="semio:jcoprobe/probe-host@0.1.0", function="slow-echo"] [Instruction::CallInterface] (async, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: true,
        entryFnName: 'slowEcho',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    
    const started = await task.enter({ isHost: hostProvided });
    if (!started) {
      _debugLog('[Instruction::CallInterface] failed to enter task', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
      });
      throw new Error("failed to enter task");
    }
    
    
    let ret;
    
    try {
      ret = await  _withGlobalCurrentTaskMetaAsync({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => slowEcho(arg0 >>> 0, arg1 >>> 0),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during async call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      return task.completionPromise();
      
    }
    
    _debugLog('[iface="semio:jcoprobe/probe-host@0.1.0", function="slow-echo"][Instruction::AsyncTaskReturn]', {
      funcName: '[task-return]slow-echo',
      paramCount: 1,
      componentIdx: 0,
      postReturn: false,
      hostProvided,
    });
    
    if (hostProvided) {
      _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
        task: task.id(),
        subtask: subtask?.id(),
        result: ret,
      })
      task.resolve([ret]);
      task.exit();
      return await task.completionPromise();
    }
    
    const componentState = getOrCreateAsyncState(0);
    if (!componentState) { throw new Error('failed to lookup current component state'); }
    
    queueMicrotask(async (resolve, reject) => {
      try {
        _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
          fnName: '[task-return]slow-echo',
          componentInstanceIdx: 0,
          taskID: task.id(),
        });
        await _driverLoop({
          componentInstanceIdx: 0,
          componentState,
          task,
          fnName: '[task-return]slow-echo',
          isAsync: true,
          callbackResult: ret,
        });
      } catch (err) {
        _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
      }
    });
    
    let taskRes = await task.completionPromise();
    if (task.getErrHandling() === 'throw-result-err') {
      if (typeof taskRes !== 'object') {
        return taskRes;
      }
      if (taskRes.tag === 'err') { throw taskRes.val; }
      if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
    }
    
    return taskRes;
    
  }
  _trampoline27.fnName = 'semio:jcoprobe/probe-host@0.1.0#slowEcho';
  
  const _trampoline30 = function(arg0, arg1, arg2) {
    var len3 = arg1;
    var base3 = arg0;
    var result3 = [];
    for (let i = 0; i < len3; i++) {
      const base = base3 + i * 4;
      var handle1 = dataView(memory0).getInt32(base + 0, true);
      
      var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
      var rsc0 = captureTable0.get(rep2);
      if (!rsc0) {
        rsc0 = Object.create(Pollable.prototype);
        Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
        Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
      }
      
      curResourceBorrows.push(rsc0);
      result3.push(rsc0);
    }
    _debugLog('[iface="wasi:io/poll@0.2.9", function="poll"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'poll',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'none',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    
    try {
      ret = _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => poll(result3),
      })
      ;
    } catch (err) {
      
      _debugLog('[Instruction::CallInterface] error during sync call', {
        taskID: task.id(),
        subtaskID: task.getParentSubtask()?.id(),
        err,
      });
      task.setErrored(err);
      task.reject(err);
      task.exit();
      throw err;
      
    }
    
    for (const entry of curResourceBorrows) {
      const rsc = entry.rsc ?? entry;
      if (entry.drop) {
        if (rsc[symbolRscHandle]) {
          entry.drop(rsc[symbolRscHandle]);
        }
      }
      rsc[symbolRscHandle] = undefined;
    }
    curResourceBorrows = [];
    var val4 = ret;
    var len4 = val4.length;
    var ptr4 = realloc0(0, 0, 4, len4 * 4);
    
    let valData4;
    const valLenBytes4 = len4 * 4;
    if (Array.isArray(val4)) {
      // Regular array likely containing numbers, write values to memory
      let offset = 0;
      const dv4 = new DataView(memory0.buffer);
      for (const v of val4) {
        _requireValidNumericPrimitive.bind(null, 'u32')(v);
        dv4.setUint32(ptr4+ offset, v, true);
        offset += 4;
      }
    } else {
      // TypedArray / ArrayBuffer-like, direct copy
      valData4 = new Uint8Array(val4.buffer || val4, val4.byteOffset, valLenBytes4);
      const out4 = new Uint8Array(memory0.buffer, ptr4, valLenBytes4);
      out4.set(valData4);
    }
    
    dataView(memory0).setUint32(arg2 + 4, len4, true);
    dataView(memory0).setUint32(arg2 + 0, ptr4, true);
    _debugLog('[iface="wasi:io/poll@0.2.9", function="poll"][Instruction::Return]', {
      funcName: 'poll',
      paramCount: 0,
      async: false,
      postReturn: false
    });
    task.resolve([ret]);
    task.exit();
  }
  _trampoline30.fnName = 'wasi:io/poll@0.2.9#poll';
  
  const handleTable1 = [T_FLAG, 0];
  handleTable1._createdReps = new Set();
  
  
  const captureTable1= new Map();
  let captureCnt1= 0;
  
  HANDLE_TABLES[1] = handleTable1;
  
  const _trampoline31 = function(arg0, arg1) {
    var handle1 = arg0;
    
    var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable3.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(OutputStream.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
      Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
    }
    
    curResourceBorrows.push(rsc0);
    _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.check-write"] [Instruction::CallInterface] (sync, @ enter)');
    const hostProvided = true;
    
    let parentTask;
    let task;
    let subtask;
    
    const createTask = () => {
      const results = createNewCurrentTask({
        componentIdx: -1,
        isAsync: false,
        entryFnName: 'checkWrite',
        getCallbackFn: () => null,
        callbackFnName: null,
        errHandling: 'result-catch-handler',
        callingWasmExport: false,
      });
      task = results[0];
    };
    
    taskCreation: {
      parentTask = getCurrentTask(
      0,
      _getGlobalCurrentTaskMeta(0)?.taskID,
      )?.task;
      
      if (!parentTask) {
        createTask();
        break taskCreation;
      }
      
      createTask();
      
      if (hostProvided) {
        subtask = parentTask.getLatestSubtask();
        if (!subtask) {
          throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
        }
        task.setParentSubtask(subtask);
      }
    }
    
    const started = task.enterSync();
    
    let ret;
    try {
      ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
        componentIdx: task.componentIdx(),
        taskID: task.id(),
        fn: () => rsc0.checkWrite(),
      })
    };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var variant5 = ret;
  switch (variant5.tag) {
    case 'ok': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg1 + 0, 0, true);
      dataView(memory0).setBigInt64(arg1 + 8, toUint64(e), true);
      
      break;
    }
    case 'err': {
      const e = variant5.val;
      dataView(memory0).setInt8(arg1 + 0, 1, true);
      var variant4 = e;
      switch (variant4.tag) {
        case 'last-operation-failed': {
          const e = variant4.val;
          dataView(memory0).setInt8(arg1 + 8, 0, true);
          
          if (!(e instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error\" resource.');
          }
          var handle3 = e[symbolRscHandle];
          if (!handle3) {
            const rep = e[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, e);
            handle3 = rscTableCreateOwn(handleTable1, rep);
          }
          
          dataView(memory0).setInt32(arg1 + 12, handle3, true);
          break;
        }
        case 'closed': {
          dataView(memory0).setInt8(arg1 + 8, 1, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant4.tag)}\` (received \`${variant4}\`) specified for \`StreamError\``);
        }
      }
      
      break;
    }
    default: {
      _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant5, valueType: typeof variant5});
      throw new TypeError('invalid variant specified for result');
    }
  }
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.check-write"][Instruction::Return]', {
    funcName: '[method]output-stream.check-write',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline31.fnName = 'wasi:io/streams@0.2.9#checkWrite';

const _trampoline32 = function(arg0, arg1, arg2, arg3) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  var ptr3 = arg1;
  var len3 = arg2;
  var result3 = new Uint8Array(memory0.buffer.slice(ptr3, ptr3 + len3 * 1));
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.write"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'write',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  try {
    ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.write(result3),
    })
  };
} catch (e) {
  ret = { tag: 'err', val: getErrorPayload(e) };
}

for (const entry of curResourceBorrows) {
  const rsc = entry.rsc ?? entry;
  if (entry.drop) {
    if (rsc[symbolRscHandle]) {
      entry.drop(rsc[symbolRscHandle]);
    }
  }
  rsc[symbolRscHandle] = undefined;
}
curResourceBorrows = [];
var variant6 = ret;
switch (variant6.tag) {
  case 'ok': {
    const e = variant6.val;
    dataView(memory0).setInt8(arg3 + 0, 0, true);
    
    break;
  }
  case 'err': {
    const e = variant6.val;
    dataView(memory0).setInt8(arg3 + 0, 1, true);
    var variant5 = e;
    switch (variant5.tag) {
      case 'last-operation-failed': {
        const e = variant5.val;
        dataView(memory0).setInt8(arg3 + 4, 0, true);
        
        if (!(e instanceof Error$1)) {
          throw new TypeError('Resource error: Not a valid \"Error\" resource.');
        }
        var handle4 = e[symbolRscHandle];
        if (!handle4) {
          const rep = e[symbolRscRep] || ++captureCnt1;
          captureTable1.set(rep, e);
          handle4 = rscTableCreateOwn(handleTable1, rep);
        }
        
        dataView(memory0).setInt32(arg3 + 8, handle4, true);
        break;
      }
      case 'closed': {
        dataView(memory0).setInt8(arg3 + 4, 1, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant5.tag)}\` (received \`${variant5}\`) specified for \`StreamError\``);
      }
    }
    
    break;
  }
  default: {
    _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant6, valueType: typeof variant6});
    throw new TypeError('invalid variant specified for result');
  }
}
_debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.write"][Instruction::Return]', {
  funcName: '[method]output-stream.write',
  paramCount: 0,
  async: false,
  postReturn: false
});
task.resolve([ret]);
task.exit();
}
_trampoline32.fnName = 'wasi:io/streams@0.2.9#write';

const _trampoline33 = function(arg0, arg1) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.blocking-flush"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'blockingFlush',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  try {
    ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.blockingFlush(),
    })
  };
} catch (e) {
  ret = { tag: 'err', val: getErrorPayload(e) };
}

for (const entry of curResourceBorrows) {
  const rsc = entry.rsc ?? entry;
  if (entry.drop) {
    if (rsc[symbolRscHandle]) {
      entry.drop(rsc[symbolRscHandle]);
    }
  }
  rsc[symbolRscHandle] = undefined;
}
curResourceBorrows = [];
var variant5 = ret;
switch (variant5.tag) {
  case 'ok': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 0, true);
    
    break;
  }
  case 'err': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 1, true);
    var variant4 = e;
    switch (variant4.tag) {
      case 'last-operation-failed': {
        const e = variant4.val;
        dataView(memory0).setInt8(arg1 + 4, 0, true);
        
        if (!(e instanceof Error$1)) {
          throw new TypeError('Resource error: Not a valid \"Error\" resource.');
        }
        var handle3 = e[symbolRscHandle];
        if (!handle3) {
          const rep = e[symbolRscRep] || ++captureCnt1;
          captureTable1.set(rep, e);
          handle3 = rscTableCreateOwn(handleTable1, rep);
        }
        
        dataView(memory0).setInt32(arg1 + 8, handle3, true);
        break;
      }
      case 'closed': {
        dataView(memory0).setInt8(arg1 + 4, 1, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant4.tag)}\` (received \`${variant4}\`) specified for \`StreamError\``);
      }
    }
    
    break;
  }
  default: {
    _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant5, valueType: typeof variant5});
    throw new TypeError('invalid variant specified for result');
  }
}
_debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.blocking-flush"][Instruction::Return]', {
  funcName: '[method]output-stream.blocking-flush',
  paramCount: 0,
  async: false,
  postReturn: false
});
task.resolve([ret]);
task.exit();
}
_trampoline33.fnName = 'wasi:io/streams@0.2.9#blockingFlush';

const _trampoline34 = function(arg0) {
  _debugLog('[iface="wasi:cli/environment@0.2.9", function="get-environment"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getEnvironment',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getEnvironment(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var vec3 = ret;
  var len3 = vec3.length;
  var result3 = realloc0(0, 0, 4, len3 * 16);
  for (let i = 0; i < vec3.length; i++) {
    const e = vec3[i];
    const base = result3 + i * 16;var [tuple0_0, tuple0_1] = e;
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_0, realloc0, memory0);
    var ptr1= encodeRes.ptr;
    var len1 = encodeRes.len;
    
    dataView(memory0).setUint32(base + 4, len1, true);
    dataView(memory0).setUint32(base + 0, ptr1, true);
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_1, realloc0, memory0);
    var ptr2= encodeRes.ptr;
    var len2 = encodeRes.len;
    
    dataView(memory0).setUint32(base + 12, len2, true);
    dataView(memory0).setUint32(base + 8, ptr2, true);
  }
  dataView(memory0).setUint32(arg0 + 4, len3, true);
  dataView(memory0).setUint32(arg0 + 0, result3, true);
  _debugLog('[iface="wasi:cli/environment@0.2.9", function="get-environment"][Instruction::Return]', {
    funcName: 'get-environment',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline34.fnName = 'wasi:cli/environment@0.2.9#getEnvironment';

const handleTable4 = [T_FLAG, 0];
handleTable4._createdReps = new Set();


const captureTable4= new Map();
let captureCnt4= 0;

HANDLE_TABLES[4] = handleTable4;

const _trampoline35 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.9", function="get-terminal-stdin"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdin',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStdin(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalInput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt4;
      captureTable4.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable4, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.9", function="get-terminal-stdin"][Instruction::Return]', {
    funcName: 'get-terminal-stdin',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline35.fnName = 'wasi:cli/terminal-stdin@0.2.9#getTerminalStdin';

const handleTable5 = [T_FLAG, 0];
handleTable5._createdReps = new Set();


const captureTable5= new Map();
let captureCnt5= 0;

HANDLE_TABLES[5] = handleTable5;

const _trampoline36 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.9", function="get-terminal-stdout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStdout(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt5;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.9", function="get-terminal-stdout"][Instruction::Return]', {
    funcName: 'get-terminal-stdout',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline36.fnName = 'wasi:cli/terminal-stdout@0.2.9#getTerminalStdout';

const _trampoline37 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.9", function="get-terminal-stderr"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStderr',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStderr(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt5;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.9", function="get-terminal-stderr"][Instruction::Return]', {
    funcName: 'get-terminal-stderr',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline37.fnName = 'wasi:cli/terminal-stderr@0.2.9#getTerminalStderr';
let exports2;
let callback_0;
let probe010Poll;

async function poll$1(arg0) {
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="poll"][Instruction::CallWasm] enter', {
    funcName: 'poll',
    paramCount: 1,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'probe010Poll',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory(() => null());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => probe010Poll(toUint32(arg0)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="poll"][Instruction::AsyncTaskReturn]', {
    funcName: 'poll',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'poll',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'poll',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let probe010AwaitEcho;

async function awaitEcho(arg0, arg1) {
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="await-echo"][Instruction::CallWasm] enter', {
    funcName: 'await-echo',
    paramCount: 2,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'probe010AwaitEcho',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory(() => null());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => probe010AwaitEcho(toUint32(arg0), toUint32(arg1)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="await-echo"][Instruction::AsyncTaskReturn]', {
    funcName: 'await-echo',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'await-echo',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'await-echo',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let probe010SpawnDetached;

async function spawnDetached(arg0) {
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="spawn-detached"][Instruction::CallWasm] enter', {
    funcName: 'spawn-detached',
    paramCount: 1,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'probe010SpawnDetached',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory(() => null());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => probe010SpawnDetached(toUint32(arg0)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="spawn-detached"][Instruction::AsyncTaskReturn]', {
    funcName: 'spawn-detached',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'spawn-detached',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'spawn-detached',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let probe010ReadBody;

async function readBody() {
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="read-body"][Instruction::CallWasm] enter', {
    funcName: 'read-body',
    paramCount: 0,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'probe010ReadBody',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory(() => null());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => probe010ReadBody(),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:jcoprobe/probe@0.1.0", function="read-body"][Instruction::AsyncTaskReturn]', {
    funcName: 'read-body',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'read-body',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'read-body',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
const trampoline0 = subtaskCancel.bind(null, 0, false);

const trampoline1 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => null,
  memoryIdx: null,
  callbackFnIdx: null,
  liftFns: [_liftFlatU32],
  lowerFns: [_lowerFlatU32],
  stringEncoding: 'utf8',
},
);

const trampoline2 = new WebAssembly.Suspending(streamCancelWrite.bind(null, {
  streamTableIdx: 0,
  isAsync: false,
  componentIdx: 0,
}));


const trampoline3 = new WebAssembly.Suspending(streamCancelRead.bind(null, {
  streamTableIdx: 0,
  isAsync: false,
  componentIdx: 0,
}));

const trampoline4 = streamDropWritable.bind(null, {
  streamTableIdx: 0,
  componentIdx: 0,
});

const trampoline5 = streamDropReadable.bind(null, {
  streamTableIdx: 0,
  componentIdx: 0,
});

const trampoline6 = streamNew.bind(null, {
  streamTableIdx: 0,
  callerComponentIdx: 0,
  elemMeta: {
    liftFn: _liftFlatU8,
    lowerFn: _lowerFlatU8,
    payloadTypeName: 'U8',
    isNone: false,
    isNumeric: true,
    isBorrowed: false,
    isAsyncValue: false,
    typedArray: Uint8Array,
    flatCount: 1,
    align32: 1,
    size32: 1,
  },
});

const trampoline7 = waitableJoin.bind(null, 0);

const trampoline8 = waitableSetNew.bind(null, 0);

const trampoline9 = waitableSetDrop.bind(null, 0);

const trampoline10 = taskCancel.bind(null, 0);

const trampoline11 = subtaskDrop.bind(
null,
0,
);
function trampoline12(handle) {
  const handleEntry = rscTableRemove(handleTable1, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable1.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable1.delete(handleEntry.rep);
    } else if (Error$1[symbolCabiDispose]) {
      Error$1[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline13(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable0.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable0.delete(handleEntry.rep);
    } else if (Pollable[symbolCabiDispose]) {
      Pollable[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline14(handle) {
  const handleEntry = rscTableRemove(handleTable2, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable2.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable2.delete(handleEntry.rep);
    } else if (InputStream[symbolCabiDispose]) {
      InputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline15(handle) {
  const handleEntry = rscTableRemove(handleTable3, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable3.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable3.delete(handleEntry.rep);
    } else if (OutputStream[symbolCabiDispose]) {
      OutputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline16(handle) {
  const handleEntry = rscTableRemove(handleTable4, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable4.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable4.delete(handleEntry.rep);
    } else if (TerminalInput[symbolCabiDispose]) {
      TerminalInput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline17(handle) {
  const handleEntry = rscTableRemove(handleTable5, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable5.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable5.delete(handleEntry.rep);
    } else if (TerminalOutput[symbolCabiDispose]) {
      TerminalOutput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
let trampoline18 = _trampoline18.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 18,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline18.manuallyAsync,
  paramLiftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0],['err', null, 0, 0, 0],],
    variantSize32: 1,
    variantAlign32: 1,
    variantPayloadOffset32: 1,
    variantFlatCount: 1,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline18,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 18,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline18.manuallyAsync,
  paramLiftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0],['err', null, 0, 0, 0],],
    variantSize32: 1,
    variantAlign32: 1,
    variantPayloadOffset32: 1,
    variantFlatCount: 1,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline18,
},
);
let trampoline19 = _trampoline19.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 19,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline19.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 0)],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline19,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 19,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline19.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 0)],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline19,
},
);
let trampoline20 = _trampoline20.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 20,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline20.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline20,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 20,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline20.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline20,
},
);
let trampoline21 = _trampoline21.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 21,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline21.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_InputStream(obj) {
      if (!(obj instanceof InputStream)) {
        throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt2;
        captureTable2.set(rep, obj);
        handle = rscTableCreateOwn(handleTable2, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline21,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 21,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline21.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_InputStream(obj) {
      if (!(obj instanceof InputStream)) {
        throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt2;
        captureTable2.set(rep, obj);
        handle = rscTableCreateOwn(handleTable2, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline21,
},
);
let trampoline22 = _trampoline22.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 22,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline22.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline22,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 22,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline22.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline22,
},
);
let trampoline23 = _trampoline23.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 23,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline23.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline23,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 23,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline23.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline23,
},
);
let trampoline24 = _trampoline24.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 24,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline24.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline24,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 24,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline24.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline24,
},
);

const trampoline25 = waitableSetPoll.bind(
null,
{
  componentIdx: 0,
  isAsync: false,
  isCancellable: false,
  memoryIdx: 0,
  getMemoryFn: () => memory0,
}
);

let trampoline26 = new WebAssembly.Suspending(_lowerImport.bind(
null,
{
  trampolineIdx: 26,
  componentIdx: 0,
  isAsync: true,
  isManualAsync: _trampoline26.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatStream({
    streamTableIdx: 0,
    componentIdx: 0,
    elemMeta: {
      liftFn: _liftFlatU8,
      lowerFn: _lowerFlatU8,
      payloadTypeName: 'U8',
      isNone: false,
      isNumeric: true,
      isBorrowed: false,
      isAsyncValue: false,
      typedArray: Uint8Array,
      flatCount: 1,
      align32: 1,
      size32: 1,
    },
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: true,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline26,
},
));
let trampoline27 = new WebAssembly.Suspending(_lowerImport.bind(
null,
{
  trampolineIdx: 27,
  componentIdx: 0,
  isAsync: true,
  isManualAsync: _trampoline27.manuallyAsync,
  paramLiftFns: [_liftFlatU32,_liftFlatU32],
  resultLowerFns: [_lowerFlatU32],
  hasResultPointer: true,
  funcTypeIsAsync: true,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline27,
},
));

const trampoline28 = new WebAssembly.Suspending(streamWrite.bind(
null,
{
  componentIdx: 0,
  memoryIdx: 0,
  getMemoryFn: () => memory0,
  reallocIdx: undefined,
  getReallocFn: undefined,
  stringEncoding: 'utf8',
  isAsync: true,
  streamTableIdx: 0,
}
));

const trampoline29 = new WebAssembly.Suspending(streamRead.bind(
null,
{
  componentIdx: 0,
  memoryIdx: 0,
  getMemoryFn: () => memory0,
  reallocIdx: undefined,
  getReallocFn: undefined,
  stringEncoding: 'utf8',
  isAsync: true,
  streamTableIdx: 0,
}
));

let trampoline30 = _trampoline30.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 30,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline30.manuallyAsync,
  paramLiftFns: [_liftFlatList({
    elemLiftFn: _liftFlatBorrow.bind(null, 0),
    elemAlign32: 4,
    elemSize32: 4,
    typedArray: undefined,
  })],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU32,
    elemSize32: 4,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline30,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 30,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline30.manuallyAsync,
  paramLiftFns: [_liftFlatList({
    elemLiftFn: _liftFlatBorrow.bind(null, 0),
    elemAlign32: 4,
    elemSize32: 4,
    typedArray: undefined,
  })],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU32,
    elemSize32: 4,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline30,
},
);
let trampoline31 = _trampoline31.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 31,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline31.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline31,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 31,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline31.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline31,
},
);
let trampoline32 = _trampoline32.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 32,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline32.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3),_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline32,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 32,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline32.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3),_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline32,
},
);
let trampoline33 = _trampoline33.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 33,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline33.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline33,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 33,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline33.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline33,
},
);
let trampoline34 = _trampoline34.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 34,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline34.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
    elemSize32: 16,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline34,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 34,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline34.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
    elemSize32: 16,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline34,
},
);
let trampoline35 = _trampoline35.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 35,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline35.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalInput(obj) {
        if (!(obj instanceof TerminalInput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt4;
          captureTable4.set(rep, obj);
          handle = rscTableCreateOwn(handleTable4, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline35,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 35,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline35.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalInput(obj) {
        if (!(obj instanceof TerminalInput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt4;
          captureTable4.set(rep, obj);
          handle = rscTableCreateOwn(handleTable4, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline35,
},
);
let trampoline36 = _trampoline36.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 36,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline36.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline36,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 36,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline36.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline36,
},
);
let trampoline37 = _trampoline37.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 37,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline37.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline37,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 37,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline37.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline37,
},
);

const $init = (() => {
  let gen = (function* _initGenerator () {
    const module0 = fetchCompile(new URL('./jcoprobe.core.wasm', import.meta.url));
    const module1 = base64Compile('AGFzbQEAAAABMAhgAn9/AX9gAX8Bf2ADf39/AX9gA39/fwF/YAN/f38AYAJ/fwBgBH9/f38AYAF/AAMODQABAgMDBAUGBQcHBwcEBQFwAQ0NB0MOATAAAAExAAEBMgACATMAAwE0AAQBNQAFATYABgE3AAcBOAAIATkACQIxMAAKAjExAAsCMTIADAgkaW1wb3J0cwEACp8BDQsAIAAgAUEAEQAACwkAIABBAREBAAsNACAAIAEgAkECEQIACw0AIAAgASACQQMRAwALDQAgACABIAJBBBEDAAsNACAAIAEgAkEFEQQACwsAIAAgAUEGEQUACw8AIAAgASACIANBBxEGAAsLACAAIAFBCBEFAAsJACAAQQkRBwALCQAgAEEKEQcACwkAIABBCxEHAAsJACAAQQwRBwALAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yNTIuMA');
    const module2 = base64Compile('AGFzbQEAAAABMAhgAn9/AX9gAX8Bf2ADf39/AX9gA39/fwF/YAN/f38AYAJ/fwBgBH9/f38AYAF/AAJUDgABMAAAAAExAAEAATIAAgABMwADAAE0AAMAATUABAABNgAFAAE3AAYAATgABQABOQAHAAIxMAAHAAIxMQAHAAIxMgAHAAgkaW1wb3J0cwFwAQ0NCRMBAEEACw0AAQIDBAUGBwgJCgsMAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yNTIuMA');
    ({ exports: exports0 } = yield instantiateCore(yield module1));
    ({ exports: exports1 } = yield instantiateCore(yield module0, {
      $root: {
        '[context-get-0]': contextGet.bind(null, { componentIdx: 0, slot: 0 }),
        '[context-set-0]': contextSet.bind(null, { componentIdx: 0, slot: 0 }),
        '[subtask-cancel]': trampoline0,
        '[subtask-drop]': trampoline11,
        '[waitable-join]': trampoline7,
        '[waitable-set-drop]': trampoline9,
        '[waitable-set-new]': trampoline8,
        '[waitable-set-poll]': exports0['0'],
      },
      '[export]$root': {
        '[task-cancel]': trampoline10,
      },
      '[export]semio:jcoprobe/probe@0.1.0': {
        '[task-return]await-echo': trampoline1,
        '[task-return]poll': trampoline1,
        '[task-return]read-body': trampoline1,
        '[task-return]spawn-detached': trampoline1,
      },
      'semio:jcoprobe/probe-host@0.1.0': {
        '[async-lower][stream-read-0]fetch-body': exports0['4'],
        '[async-lower][stream-write-0]fetch-body': exports0['3'],
        '[async-lower]fetch-body': exports0['1'],
        '[async-lower]slow-echo': exports0['2'],
        '[stream-cancel-read-0]fetch-body': trampoline3,
        '[stream-cancel-write-0]fetch-body': trampoline2,
        '[stream-drop-readable-0]fetch-body': trampoline5,
        '[stream-drop-writable-0]fetch-body': trampoline4,
        '[stream-new-0]fetch-body': trampoline6,
      },
      'wasi:cli/environment@0.2.0': {
        'get-environment': exports0['9'],
      },
      'wasi:cli/exit@0.2.0': {
        exit: trampoline18,
      },
      'wasi:cli/stderr@0.2.0': {
        'get-stderr': trampoline23,
      },
      'wasi:cli/stdin@0.2.0': {
        'get-stdin': trampoline21,
      },
      'wasi:cli/stdout@0.2.0': {
        'get-stdout': trampoline22,
      },
      'wasi:cli/terminal-input@0.2.0': {
        '[resource-drop]terminal-input': trampoline16,
      },
      'wasi:cli/terminal-output@0.2.0': {
        '[resource-drop]terminal-output': trampoline17,
      },
      'wasi:cli/terminal-stderr@0.2.0': {
        'get-terminal-stderr': exports0['12'],
      },
      'wasi:cli/terminal-stdin@0.2.0': {
        'get-terminal-stdin': exports0['10'],
      },
      'wasi:cli/terminal-stdout@0.2.0': {
        'get-terminal-stdout': exports0['11'],
      },
      'wasi:clocks/monotonic-clock@0.2.0': {
        'subscribe-duration': trampoline24,
      },
      'wasi:io/error@0.2.0': {
        '[resource-drop]error': trampoline12,
      },
      'wasi:io/poll@0.2.0': {
        '[method]pollable.block': trampoline19,
        '[resource-drop]pollable': trampoline13,
        poll: exports0['5'],
      },
      'wasi:io/streams@0.2.0': {
        '[method]output-stream.blocking-flush': exports0['8'],
        '[method]output-stream.check-write': exports0['6'],
        '[method]output-stream.subscribe': trampoline20,
        '[method]output-stream.write': exports0['7'],
        '[resource-drop]input-stream': trampoline14,
        '[resource-drop]output-stream': trampoline15,
      },
    }));
    memory0 = exports1.memory;
    realloc0 = exports1.cabi_realloc;
    
    try {
      realloc0Async = WebAssembly.promising(exports1.cabi_realloc);
    } catch(err) {
      realloc0Async = exports1.cabi_realloc;
    }
    
    ({ exports: exports2 } = yield instantiateCore(yield module2, {
      '': {
        $imports: exports0.$imports,
        '0': trampoline25,
        '1': trampoline26,
        '10': trampoline35,
        '11': trampoline36,
        '12': trampoline37,
        '2': trampoline27,
        '3': trampoline28,
        '4': trampoline29,
        '5': trampoline30,
        '6': trampoline31,
        '7': trampoline32,
        '8': trampoline33,
        '9': trampoline34,
      },
    }));
    
    callback_0 = WebAssembly.promising(exports1['[callback][async-lift]semio:jcoprobe/probe@0.1.0#await-echo']);
    callback_0.fnName = "exports1['[callback][async-lift]semio:jcoprobe/probe@0.1.0#await-echo']";
    
    registerGlobalMemoryForComponent({
      componentIdx: 0,
      memoryIdx: 0,
      memory: memory0,
    });
    registerGlobalMemoryForComponent({
      componentIdx: 0,
      memoryIdx: 0,
      memory: memory0,
    });
    probe010Poll = WebAssembly.promising(exports1['[async-lift]semio:jcoprobe/probe@0.1.0#poll']);
    probe010AwaitEcho = WebAssembly.promising(exports1['[async-lift]semio:jcoprobe/probe@0.1.0#await-echo']);
    probe010SpawnDetached = WebAssembly.promising(exports1['[async-lift]semio:jcoprobe/probe@0.1.0#spawn-detached']);
    probe010ReadBody = WebAssembly.promising(exports1['[async-lift]semio:jcoprobe/probe@0.1.0#read-body']);
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(runNext, reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

await $init;
const probe010 = {
  awaitEcho: awaitEcho,
  poll: poll$1,
  readBody: readBody,
  spawnDetached: spawnDetached,
  
};

export { probe010 as probe, probe010 as 'semio:jcoprobe/probe@0.1.0',  }
export const _util = {
  
}

