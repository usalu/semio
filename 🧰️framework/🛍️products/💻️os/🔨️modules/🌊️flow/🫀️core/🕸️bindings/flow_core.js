/* @ts-self-types="./flow_core.d.ts" */

//#region exports

export class DagSession {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DagSessionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dagsession_free(ptr, 0);
    }
    /**
     * @param {HTMLCanvasElement} canvas
     * @param {number} logical_w
     * @param {number} logical_h
     * @param {number} dpr
     * @returns {Promise<any>}
     */
    attachCanvas(canvas, logical_w, logical_h, dpr) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(logical_w);
        _assertNum(logical_h);
        const ret = wasm.dagsession_attachCanvas(this.__wbg_ptr, canvas, logical_w, logical_h, dpr);
        return ret;
    }
    /**
     * @returns {string}
     */
    drawLodLabel() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.dagsession_drawLodLabel(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    fixtureJson() {
        let deferred2_0;
        let deferred2_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.dagsession_fixtureJson(this.__wbg_ptr);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    gpuReady() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsession_gpuReady(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {string}
     */
    labelOverlayPaintStateJson() {
        let deferred2_0;
        let deferred2_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.dagsession_labelOverlayPaintStateJson(this.__wbg_ptr);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} json
     */
    loadFixtureJson(json) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dagsession_loadFixtureJson(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @returns {string}
     */
    lodScaleJson() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.dagsession_lodScaleJson(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    constructor() {
        const ret = wasm.dagsession_new();
        this.__wbg_ptr = ret;
        DagSessionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    nodeOverlaysJson() {
        let deferred2_0;
        let deferred2_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.dagsession_nodeOverlaysJson(this.__wbg_ptr);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {boolean} extend
     */
    pointerDown(x, y, extend) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertBoolean(extend);
        wasm.dagsession_pointerDown(this.__wbg_ptr, x, y, extend);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    pointerMove(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.dagsession_pointerMove(this.__wbg_ptr, x, y);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    pointerUp(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.dagsession_pointerUp(this.__wbg_ptr, x, y);
    }
    renderFrame() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsession_renderFrame(this.__wbg_ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {string} options_json
     */
    reorganize(options_json) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(options_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dagsession_reorganize(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {Array<any>}
     */
    screenToWorld(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsession_screenToWorld(this.__wbg_ptr, x, y);
        return ret;
    }
    /**
     * @param {boolean} enabled
     */
    setAutomaticLod(enabled) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertBoolean(enabled);
        wasm.dagsession_setAutomaticLod(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} zoom
     */
    setCamera(x, y, zoom) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.dagsession_setCamera(this.__wbg_ptr, x, y, zoom);
    }
    /**
     * @param {string} json
     */
    setCanvasThemeJson(json) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.dagsession_setCanvasThemeJson(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} label
     */
    setForcedDrawLodLabel(label) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.dagsession_setForcedDrawLodLabel(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} dpr
     */
    setSize(width, height, dpr) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(width);
        _assertNum(height);
        wasm.dagsession_setSize(this.__wbg_ptr, width, height, dpr);
    }
    /**
     * @param {boolean} active
     */
    setWheelZoomActive(active) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertBoolean(active);
        wasm.dagsession_setWheelZoomActive(this.__wbg_ptr, active);
    }
    /**
     * @returns {string | undefined}
     */
    takePendingOpenInstanceId() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsession_takePendingOpenInstanceId(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
}
if (Symbol.dispose) DagSession.prototype[Symbol.dispose] = DagSession.prototype.free;

export class DagSnapshotVcs {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(DagSnapshotVcs.prototype);
        obj.__wbg_ptr = ptr;
        DagSnapshotVcsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DagSnapshotVcsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dagsnapshotvcs_free(ptr, 0);
    }
    /**
     * 🌐️ Constructs the VCS bridge without synchronously blocking the browser host callback.
     * @returns {Promise<DagSnapshotVcs>}
     */
    static create() {
        const ret = wasm.dagsnapshotvcs_create();
        return ret;
    }
    /**
     * @param {Uint8Array} command_bytes
     * @returns {Promise<void>}
     */
    dispatchBinary(command_bytes) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passArray8ToWasm0(command_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dagsnapshotvcs_dispatchBinary(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {string} command_text
     * @returns {Promise<void>}
     */
    dispatchText(command_text) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(command_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dagsnapshotvcs_dispatchText(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @returns {Promise<string>}
     */
    envelopeJson() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsnapshotvcs_envelopeJson(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Promise<number>}
     */
    generation() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsnapshotvcs_generation(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Promise<string>}
     */
    snapshotJson() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.dagsnapshotvcs_snapshotJson(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) DagSnapshotVcs.prototype[Symbol.dispose] = DagSnapshotVcs.prototype.free;

/**
 * 🌉️ wasm-bindgen wrapper around [`Kernel`] for the React-web / wgpu-web hosts (see design
 * §1's three-host list). Every method takes/returns pack-encoded bytes only — this type owns
 * no logic of its own beyond (de)serializing at the boundary and delegating to `Kernel`.
 */
export class KernelHost {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(KernelHost.prototype);
        obj.__wbg_ptr = ptr;
        KernelHostFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KernelHostFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_kernelhost_free(ptr, 0);
    }
    /**
     * ▶️ `activation_bytes` is a pack-encoded `(PackageId, u16 plugin_ordinal, ActorKind, Lane,
     * Option<WindowId>, ActivationEvent)` tuple; returns the pack-encoded fresh `ActorId`.
     * @param {Uint8Array} activation_bytes
     * @returns {Promise<Uint8Array>}
     */
    activate(activation_bytes) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passArray8ToWasm0(activation_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.kernelhost_activate(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {Uint8Array} actor_bytes
     * @param {Uint8Array} turn_result_bytes
     * @param {bigint} now_ms
     * @returns {Promise<void>}
     */
    complete(actor_bytes, turn_result_bytes, now_ms) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passArray8ToWasm0(actor_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(turn_result_bytes, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        _assertBigInt(now_ms);
        const ret = wasm.kernelhost_complete(this.__wbg_ptr, ptr0, len0, ptr1, len1, now_ms);
        return ret;
    }
    /**
     * @param {number} shard_count
     * @param {number} exclusive_reserve
     * @param {number} grants_per_tick
     * @returns {Promise<KernelHost>}
     */
    static create(shard_count, exclusive_reserve, grants_per_tick) {
        _assertNum(shard_count);
        _assertNum(exclusive_reserve);
        _assertNum(grants_per_tick);
        const ret = wasm.kernelhost_create(shard_count, exclusive_reserve, grants_per_tick);
        return ret;
    }
    /**
     * @returns {Promise<Uint8Array>}
     */
    metrics() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.kernelhost_metrics(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array} envelope_bytes
     * @returns {Promise<Uint8Array>}
     */
    submit(envelope_bytes) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passArray8ToWasm0(envelope_bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.kernelhost_submit(this.__wbg_ptr, ptr0, len0);
        return ret;
    }
    /**
     * @param {bigint} now_ms
     * @returns {Promise<Uint8Array>}
     */
    tick(now_ms) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertBigInt(now_ms);
        const ret = wasm.kernelhost_tick(this.__wbg_ptr, now_ms);
        return ret;
    }
}
if (Symbol.dispose) KernelHost.prototype[Symbol.dispose] = KernelHost.prototype.free;

/**
 * 🌐️ Generic JSON-RPC bridge for the CAD `SpatialKernel` (see `🧠️semio/🟦️.ts`): dispatches
 * one `BrepKernel` method by name over JSON args, sharing the same in-process `kernel()` the
 * `tessellate`/`dispose` exports above use so handles stay valid across calls.
 * @param {string} method
 * @param {string} args_json
 * @returns {string}
 */
export function brep_invoke(method, args_json) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passStringToWasm0(method, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(args_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.brep_invoke(ptr0, len0, ptr1, len1);
        deferred3_0 = ret[0];
        deferred3_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {string} handle
 */
export function dispose(handle) {
    const ptr0 = passStringToWasm0(handle, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.dispose(ptr0, len0);
}

/**
 * 🔌️ Binds one flow surface's canvas to a WebGPU device. Resolves `true` when the canvas will
 * present on the GPU and `false` when no adapter/device/surface could be had — never rejects,
 * because a host without WebGPU must still reach a painted canvas through the 2D replay path.
 * @param {number} surface
 * @param {HTMLCanvasElement} canvas
 * @param {number} logical_width
 * @param {number} logical_height
 * @param {number} dpr
 * @returns {Promise<any>}
 */
export function flowAttachSurfaceCanvas(surface, canvas, logical_width, logical_height, dpr) {
    _assertNum(surface);
    _assertNum(logical_width);
    _assertNum(logical_height);
    const ret = wasm.flowAttachSurfaceCanvas(surface, canvas, logical_width, logical_height, dpr);
    return ret;
}

/**
 * 🧹️ Releases a surface's device, swapchain and canvas retention.
 * @param {number} surface
 */
export function flowDetachSurfaceCanvas(surface) {
    _assertNum(surface);
    wasm.flowDetachSurfaceCanvas(surface);
}

/**
 * 📐️ Resizes an attached surface's swapchain; a no-op for a surface that presents in 2D.
 * @param {number} surface
 * @param {number} logical_width
 * @param {number} logical_height
 * @param {number} dpr
 */
export function flowResizeSurfaceCanvas(surface, logical_width, logical_height, dpr) {
    _assertNum(surface);
    _assertNum(logical_width);
    _assertNum(logical_height);
    wasm.flowResizeSurfaceCanvas(surface, logical_width, logical_height, dpr);
}

/**
 * 🔎️ Whether this surface presents on the GPU — the host reads it to decide whether to keep a
 * 2D context off the canvas (a canvas admits exactly one context kind, for its whole life).
 * @param {number} surface
 * @returns {boolean}
 */
export function flowSurfaceCanvasPresentsOnGpu(surface) {
    _assertNum(surface);
    const ret = wasm.flowSurfaceCanvasPresentsOnGpu(surface);
    return ret !== 0;
}

export function initialize_browser_clock() {
    wasm.initialize_browser_clock();
}

/**
 * @param {string} handle
 * @param {number} tolerance
 * @returns {string}
 */
export function tessellate(handle, tolerance) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(handle, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tessellate(ptr0, len0, tolerance);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

//#endregion

//#region wasm imports
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_92b29b0548f8b746: function() { return logError(function (arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_Window_a07901001eb4269f: function() { return logError(function (arg0) {
            const ret = arg0.Window;
            return ret;
        }, arguments); },
        __wbg_WorkerGlobalScope_d1b9459d53a39f3d: function() { return logError(function (arg0) {
            const ret = arg0.WorkerGlobalScope;
            return ret;
        }, arguments); },
        __wbg___wbindgen_debug_string_c25d447a39f5578f: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_1ff95bcc5517c252: function(arg0) {
            const ret = typeof(arg0) === 'function';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_null_ea9085d691f535d3: function(arg0) {
            const ret = arg0 === null;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_object_a27215656b807791: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_undefined_c05833b95a3cf397: function(arg0) {
            const ret = arg0 === undefined;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_number_get_394265ed1e1b84ee: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_rethrow_4915403b40f010b4: function(arg0) {
            throw arg0;
        },
        __wbg___wbindgen_string_get_b0ca35b86a603356: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_fffb441def202758: function() { return logError(function (arg0) {
            arg0._wbg_cb_unref();
        }, arguments); },
        __wbg_adapterInfo_bcf1e34d1f8c621b: function() { return logError(function (arg0) {
            const ret = arg0.adapterInfo;
            return ret;
        }, arguments); },
        __wbg_beginComputePass_705eb14eefc2b94e: function() { return logError(function (arg0, arg1) {
            const ret = arg0.beginComputePass(arg1);
            return ret;
        }, arguments); },
        __wbg_beginOcclusionQuery_da58461ecc9bf9ec: function() { return logError(function (arg0, arg1) {
            arg0.beginOcclusionQuery(arg1 >>> 0);
        }, arguments); },
        __wbg_beginRenderPass_10e1d8bb36f2f74e: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.beginRenderPass(arg1);
            return ret;
        }, arguments); },
        __wbg_call_8a2dd23819f8a60a: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_a6e5c5dce5018821: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_clearBuffer_700f6bba0d974e6c: function() { return logError(function (arg0, arg1, arg2) {
            arg0.clearBuffer(arg1, arg2);
        }, arguments); },
        __wbg_clearBuffer_b67061873f997b6a: function() { return logError(function (arg0, arg1, arg2, arg3) {
            arg0.clearBuffer(arg1, arg2, arg3);
        }, arguments); },
        __wbg_configure_3d64c677c7d68a15: function() { return handleError(function (arg0, arg1) {
            arg0.configure(arg1);
        }, arguments); },
        __wbg_copyBufferToBuffer_8bb974c7f9c5f4dc: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.copyBufferToBuffer(arg1, arg2, arg3, arg4);
        }, arguments); },
        __wbg_copyBufferToBuffer_8fe240a0000c9e22: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.copyBufferToBuffer(arg1, arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_copyBufferToTexture_5c32355b7be376d8: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.copyBufferToTexture(arg1, arg2, arg3);
        }, arguments); },
        __wbg_copyExternalImageToTexture_b8c12db525cb7f31: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.copyExternalImageToTexture(arg1, arg2, arg3);
        }, arguments); },
        __wbg_copyTextureToBuffer_4186c16aef1922a5: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.copyTextureToBuffer(arg1, arg2, arg3);
        }, arguments); },
        __wbg_copyTextureToTexture_1be188df1e535c0a: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.copyTextureToTexture(arg1, arg2, arg3);
        }, arguments); },
        __wbg_createBindGroupLayout_9ea1a44942aaf13e: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createBindGroupLayout(arg1);
            return ret;
        }, arguments); },
        __wbg_createBindGroup_2320df4db188406c: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createBindGroup(arg1);
            return ret;
        }, arguments); },
        __wbg_createBuffer_2f08c0205e04efca: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createBuffer(arg1);
            return ret;
        }, arguments); },
        __wbg_createCommandEncoder_cd88faca35d9ed68: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createCommandEncoder(arg1);
            return ret;
        }, arguments); },
        __wbg_createComputePipeline_3e135ff73c8fc483: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createComputePipeline(arg1);
            return ret;
        }, arguments); },
        __wbg_createPipelineLayout_7a186f2e9bf0d605: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createPipelineLayout(arg1);
            return ret;
        }, arguments); },
        __wbg_createQuerySet_05da8b0de35672ca: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createQuerySet(arg1);
            return ret;
        }, arguments); },
        __wbg_createRenderBundleEncoder_898bc419f724439b: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createRenderBundleEncoder(arg1);
            return ret;
        }, arguments); },
        __wbg_createRenderPipeline_f48187ba9f7701e8: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createRenderPipeline(arg1);
            return ret;
        }, arguments); },
        __wbg_createSampler_248bd67c920af37d: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createSampler(arg1);
            return ret;
        }, arguments); },
        __wbg_createShaderModule_53701de4fb271c90: function() { return logError(function (arg0, arg1) {
            const ret = arg0.createShaderModule(arg1);
            return ret;
        }, arguments); },
        __wbg_createTexture_9e76b80a2dc0d12e: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createTexture(arg1);
            return ret;
        }, arguments); },
        __wbg_createView_cc96b5bdd3d5bf5e: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.createView(arg1);
            return ret;
        }, arguments); },
        __wbg_dagsnapshotvcs_new: function() { return logError(function (arg0) {
            const ret = DagSnapshotVcs.__wrap(arg0);
            return ret;
        }, arguments); },
        __wbg_description_18d0a6d4077fec8e: function() { return logError(function (arg0, arg1) {
            const ret = arg1.description;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_destroy_6601bc024448d3c7: function() { return logError(function (arg0) {
            arg0.destroy();
        }, arguments); },
        __wbg_destroy_b5b39f25f0799295: function() { return logError(function (arg0) {
            arg0.destroy();
        }, arguments); },
        __wbg_destroy_e3cb8ca345cccfc3: function() { return logError(function (arg0) {
            arg0.destroy();
        }, arguments); },
        __wbg_dispatchWorkgroupsIndirect_07e3b56efd02764b: function() { return logError(function (arg0, arg1, arg2) {
            arg0.dispatchWorkgroupsIndirect(arg1, arg2);
        }, arguments); },
        __wbg_dispatchWorkgroups_0cf298d736b85a78: function() { return logError(function (arg0, arg1, arg2, arg3) {
            arg0.dispatchWorkgroups(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0);
        }, arguments); },
        __wbg_document_179650d6cb13c263: function() { return logError(function (arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_done_89b2b13e91a60321: function() { return logError(function (arg0) {
            const ret = arg0.done;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_drawIndexedIndirect_300125bd70bcd09b: function() { return logError(function (arg0, arg1, arg2) {
            arg0.drawIndexedIndirect(arg1, arg2);
        }, arguments); },
        __wbg_drawIndexed_68637ebab6dd8d6e: function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
        }, arguments); },
        __wbg_drawIndirect_3cabcd983032eced: function() { return logError(function (arg0, arg1, arg2) {
            arg0.drawIndirect(arg1, arg2);
        }, arguments); },
        __wbg_draw_ad0811de56a2d768: function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
        }, arguments); },
        __wbg_endOcclusionQuery_a935f17dbc68b7f8: function() { return logError(function (arg0) {
            arg0.endOcclusionQuery();
        }, arguments); },
        __wbg_end_414453a89205612c: function() { return logError(function (arg0) {
            arg0.end();
        }, arguments); },
        __wbg_end_fb560a3ae8e3624e: function() { return logError(function (arg0) {
            arg0.end();
        }, arguments); },
        __wbg_error_287b079609b734b7: function() { return logError(function (arg0) {
            const ret = arg0.error;
            return ret;
        }, arguments); },
        __wbg_executeBundles_78f23090c2162193: function() { return logError(function (arg0, arg1) {
            arg0.executeBundles(arg1);
        }, arguments); },
        __wbg_features_4ce861c12227aa47: function() { return logError(function (arg0) {
            const ret = arg0.features;
            return ret;
        }, arguments); },
        __wbg_features_614e8836a2aaf39a: function() { return logError(function (arg0) {
            const ret = arg0.features;
            return ret;
        }, arguments); },
        __wbg_finish_087cb89c65c06eb1: function() { return logError(function (arg0) {
            const ret = arg0.finish();
            return ret;
        }, arguments); },
        __wbg_finish_cfaeede3baf55be1: function() { return logError(function (arg0, arg1) {
            const ret = arg0.finish(arg1);
            return ret;
        }, arguments); },
        __wbg_getContext_e79ddf6a9cb3cc76: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getContext_fd298c901058eb31: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getCurrentTexture_51975ae7185fd15f: function() { return handleError(function (arg0) {
            const ret = arg0.getCurrentTexture();
            return ret;
        }, arguments); },
        __wbg_getMappedRange_5ed22727c9679168: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getMappedRange(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_getPreferredCanvasFormat_1b8495aeb1d11ab1: function() { return logError(function (arg0) {
            const ret = arg0.getPreferredCanvasFormat();
            return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
        }, arguments); },
        __wbg_get_78f252d074a84d0b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_b2053e9bfdf3ca8e: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_gpu_a7c12045c25d009a: function() { return logError(function (arg0) {
            const ret = arg0.gpu;
            return ret;
        }, arguments); },
        __wbg_has_b5a46804dc5e62bd: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.has(getStringFromWasm0(arg1, arg2));
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_info_22dcf1fd1b12bc7d: function() { return logError(function (arg0) {
            const ret = arg0.info;
            return ret;
        }, arguments); },
        __wbg_insertDebugMarker_1e34d63423410656: function() { return logError(function (arg0, arg1, arg2) {
            arg0.insertDebugMarker(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_insertDebugMarker_771268d850492a06: function() { return logError(function (arg0, arg1, arg2) {
            arg0.insertDebugMarker(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_insertDebugMarker_d2c5be9b5ea288ba: function() { return logError(function (arg0, arg1, arg2) {
            arg0.insertDebugMarker(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_instanceof_GpuAdapter_fc7b89fc546de0bc: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof GPUAdapter;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_GpuCanvasContext_1a39fd0621603553: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof GPUCanvasContext;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_GpuDeviceLostInfo_c0ebce29b32e81e8: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof GPUDeviceLostInfo;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_GpuOutOfMemoryError_5ac5c50ce9ee21d2: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof GPUOutOfMemoryError;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_GpuValidationError_77b97d666afabac1: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof GPUValidationError;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Object_33f20e6f12439f3e: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Window_05ba1ee4f6781663: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_kernelhost_new: function() { return logError(function (arg0) {
            const ret = KernelHost.__wrap(arg0);
            return ret;
        }, arguments); },
        __wbg_keys_2ce19b5f8d7dc1cd: function() { return logError(function (arg0) {
            const ret = arg0.keys();
            return ret;
        }, arguments); },
        __wbg_label_47480289cc2bce71: function() { return logError(function (arg0, arg1) {
            const ret = arg1.label;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_limits_1c25cb4f379a4418: function() { return logError(function (arg0) {
            const ret = arg0.limits;
            return ret;
        }, arguments); },
        __wbg_limits_50a8c5e629dbfe40: function() { return logError(function (arg0) {
            const ret = arg0.limits;
            return ret;
        }, arguments); },
        __wbg_log_d267660666346fb3: function() { return logError(function (arg0) {
            console.log(arg0);
        }, arguments); },
        __wbg_lost_7b1065bddc80e8ac: function() { return logError(function (arg0) {
            const ret = arg0.lost;
            return ret;
        }, arguments); },
        __wbg_mapAsync_bb0029907dd91181: function() { return logError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.mapAsync(arg1 >>> 0, arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_maxBindGroups_14611ac9ed1c6b56: function() { return logError(function (arg0) {
            const ret = arg0.maxBindGroups;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxBindingsPerBindGroup_dd3f66044d2a9bfb: function() { return logError(function (arg0) {
            const ret = arg0.maxBindingsPerBindGroup;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxBufferSize_f7ce3e1856349d2f: function() { return logError(function (arg0) {
            const ret = arg0.maxBufferSize;
            return ret;
        }, arguments); },
        __wbg_maxColorAttachmentBytesPerSample_55e64194645ea041: function() { return logError(function (arg0) {
            const ret = arg0.maxColorAttachmentBytesPerSample;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxColorAttachments_fd9187f9f786da18: function() { return logError(function (arg0) {
            const ret = arg0.maxColorAttachments;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeInvocationsPerWorkgroup_9b3b1fc261129782: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeInvocationsPerWorkgroup;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeWorkgroupSizeX_c55bbbcc02b75241: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeWorkgroupSizeX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeWorkgroupSizeY_96f40b1ec3102a3a: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeWorkgroupSizeY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeWorkgroupSizeZ_c2b1061d521561bb: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeWorkgroupSizeZ;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeWorkgroupStorageSize_fac26e89d99e08f9: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeWorkgroupStorageSize;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxComputeWorkgroupsPerDimension_cd001f910e9b4d70: function() { return logError(function (arg0) {
            const ret = arg0.maxComputeWorkgroupsPerDimension;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxDynamicStorageBuffersPerPipelineLayout_29399b82af020d86: function() { return logError(function (arg0) {
            const ret = arg0.maxDynamicStorageBuffersPerPipelineLayout;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxDynamicUniformBuffersPerPipelineLayout_6d6cf80f3bd08e52: function() { return logError(function (arg0) {
            const ret = arg0.maxDynamicUniformBuffersPerPipelineLayout;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxInterStageShaderVariables_8b000f47a166b1d5: function() { return logError(function (arg0) {
            const ret = arg0.maxInterStageShaderVariables;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxSampledTexturesPerShaderStage_618a49f33217dde2: function() { return logError(function (arg0) {
            const ret = arg0.maxSampledTexturesPerShaderStage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxSamplersPerShaderStage_aa09fa0311712a1a: function() { return logError(function (arg0) {
            const ret = arg0.maxSamplersPerShaderStage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxStorageBufferBindingSize_0ec83ae10ad73180: function() { return logError(function (arg0) {
            const ret = arg0.maxStorageBufferBindingSize;
            return ret;
        }, arguments); },
        __wbg_maxStorageBuffersPerShaderStage_0cca5b468fcf10b6: function() { return logError(function (arg0) {
            const ret = arg0.maxStorageBuffersPerShaderStage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxStorageTexturesPerShaderStage_9d6c35770f37866c: function() { return logError(function (arg0) {
            const ret = arg0.maxStorageTexturesPerShaderStage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxTextureArrayLayers_c2bf9c85285832d4: function() { return logError(function (arg0) {
            const ret = arg0.maxTextureArrayLayers;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxTextureDimension1D_e09f86e22ea6bac9: function() { return logError(function (arg0) {
            const ret = arg0.maxTextureDimension1D;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxTextureDimension2D_2631916ef9a3efa8: function() { return logError(function (arg0) {
            const ret = arg0.maxTextureDimension2D;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxTextureDimension3D_06ee54121b37d431: function() { return logError(function (arg0) {
            const ret = arg0.maxTextureDimension3D;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxUniformBufferBindingSize_af9e8a077907ed64: function() { return logError(function (arg0) {
            const ret = arg0.maxUniformBufferBindingSize;
            return ret;
        }, arguments); },
        __wbg_maxUniformBuffersPerShaderStage_f871b70865df8c11: function() { return logError(function (arg0) {
            const ret = arg0.maxUniformBuffersPerShaderStage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxVertexAttributes_e72dabb2714f5cf5: function() { return logError(function (arg0) {
            const ret = arg0.maxVertexAttributes;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxVertexBufferArrayStride_6a1cd814386082ce: function() { return logError(function (arg0) {
            const ret = arg0.maxVertexBufferArrayStride;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_maxVertexBuffers_9c61c5fd286ebcc6: function() { return logError(function (arg0) {
            const ret = arg0.maxVertexBuffers;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_message_09714b1dbee17e65: function() { return logError(function (arg0, arg1) {
            const ret = arg1.message;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_message_6769962f0009c864: function() { return logError(function (arg0, arg1) {
            const ret = arg1.message;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_minStorageBufferOffsetAlignment_e214f59628fb3558: function() { return logError(function (arg0) {
            const ret = arg0.minStorageBufferOffsetAlignment;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_minUniformBufferOffsetAlignment_58b69e1c3924f6a4: function() { return logError(function (arg0) {
            const ret = arg0.minUniformBufferOffsetAlignment;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_navigator_51379c10a84aeec9: function() { return logError(function (arg0) {
            const ret = arg0.navigator;
            return ret;
        }, arguments); },
        __wbg_navigator_99621db14b3f1099: function() { return logError(function (arg0) {
            const ret = arg0.navigator;
            return ret;
        }, arguments); },
        __wbg_new_32b398fb48b6d94a: function() { return logError(function () {
            const ret = new Array();
            return ret;
        }, arguments); },
        __wbg_new_da52cf8fe3429cb2: function() { return logError(function () {
            const ret = new Object();
            return ret;
        }, arguments); },
        __wbg_new_typed_1824d93f294193e5: function() { return logError(function (arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_458697782d79938e___convert__closures_____invoke___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        }, arguments); },
        __wbg_new_with_byte_offset_and_length_54c7724ee3ec7d82: function() { return logError(function (arg0, arg1, arg2) {
            const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
            return ret;
        }, arguments); },
        __wbg_next_71f2aa1cb3d1e37e: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_now_86c0d4ba3fa605b8: function() { return logError(function () {
            const ret = Date.now();
            return ret;
        }, arguments); },
        __wbg_onSubmittedWorkDone_1460145eecea40ef: function() { return logError(function (arg0) {
            const ret = arg0.onSubmittedWorkDone();
            return ret;
        }, arguments); },
        __wbg_popDebugGroup_0c45034afb9a2d56: function() { return logError(function (arg0) {
            arg0.popDebugGroup();
        }, arguments); },
        __wbg_popDebugGroup_69907e0e5ec2b561: function() { return logError(function (arg0) {
            arg0.popDebugGroup();
        }, arguments); },
        __wbg_popDebugGroup_df63b3bf2b158ccf: function() { return logError(function (arg0) {
            arg0.popDebugGroup();
        }, arguments); },
        __wbg_popErrorScope_4cbc9ce0c8cc5a9f: function() { return logError(function (arg0) {
            const ret = arg0.popErrorScope();
            return ret;
        }, arguments); },
        __wbg_pushDebugGroup_3f0735d624fe2b7a: function() { return logError(function (arg0, arg1, arg2) {
            arg0.pushDebugGroup(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_pushDebugGroup_49bb00d757980bb3: function() { return logError(function (arg0, arg1, arg2) {
            arg0.pushDebugGroup(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_pushDebugGroup_4c8c13db9993e39c: function() { return logError(function (arg0, arg1, arg2) {
            arg0.pushDebugGroup(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_pushErrorScope_aad0eef2ff5b28d3: function() { return logError(function (arg0, arg1) {
            arg0.pushErrorScope(__wbindgen_enum_GpuErrorFilter[arg1]);
        }, arguments); },
        __wbg_push_d2ae3af0c1217ae6: function() { return logError(function (arg0, arg1) {
            const ret = arg0.push(arg1);
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_querySelectorAll_7e98cbe256deaadd: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_queueMicrotask_0ab5b2d2393e99b9: function() { return logError(function (arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        }, arguments); },
        __wbg_queueMicrotask_6a09b7bc46549209: function() { return logError(function (arg0) {
            queueMicrotask(arg0);
        }, arguments); },
        __wbg_queue_65d985f3e6d786a6: function() { return logError(function (arg0) {
            const ret = arg0.queue;
            return ret;
        }, arguments); },
        __wbg_reason_21c585c1cbc2cc8f: function() { return logError(function (arg0) {
            const ret = arg0.reason;
            return (__wbindgen_enum_GpuDeviceLostReason.indexOf(ret) + 1 || 3) - 1;
        }, arguments); },
        __wbg_requestAdapter_9ff5c9d1ff271165: function() { return logError(function (arg0, arg1) {
            const ret = arg0.requestAdapter(arg1);
            return ret;
        }, arguments); },
        __wbg_requestDevice_c1c34f88a477e509: function() { return logError(function (arg0, arg1) {
            const ret = arg0.requestDevice(arg1);
            return ret;
        }, arguments); },
        __wbg_resolveQuerySet_1eef18cd73ebb548: function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.resolveQuerySet(arg1, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
        }, arguments); },
        __wbg_resolve_2191a4dfe481c25b: function() { return logError(function (arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        }, arguments); },
        __wbg_run_5aa314612b150933: function() { return logError(function (arg0, arg1, arg2) {
            try {
                var state0 = {a: arg1, b: arg2};
                var cb0 = () => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_458697782d79938e___convert__closures_____invoke___bool__true_(a, state0.b, );
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = arg0.run(cb0);
                _assertBoolean(ret);
                return ret;
            } finally {
                state0.a = 0;
            }
        }, arguments); },
        __wbg_setBindGroup_4ba56e1e0d26f244: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.setBindGroup(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
        }, arguments); },
        __wbg_setBindGroup_6124849cc8547086: function() { return logError(function (arg0, arg1, arg2) {
            arg0.setBindGroup(arg1 >>> 0, arg2);
        }, arguments); },
        __wbg_setBindGroup_79afcff8b9db8be3: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.setBindGroup(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
        }, arguments); },
        __wbg_setBindGroup_84eb639ac393a9f4: function() { return logError(function (arg0, arg1, arg2) {
            arg0.setBindGroup(arg1 >>> 0, arg2);
        }, arguments); },
        __wbg_setBlendConstant_400c9c253b043929: function() { return handleError(function (arg0, arg1) {
            arg0.setBlendConstant(arg1);
        }, arguments); },
        __wbg_setIndexBuffer_17431786d06c1b7c: function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
        }, arguments); },
        __wbg_setIndexBuffer_a16ed5b869c87507: function() { return logError(function (arg0, arg1, arg2, arg3) {
            arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3);
        }, arguments); },
        __wbg_setPipeline_95c76ab8da697fcf: function() { return logError(function (arg0, arg1) {
            arg0.setPipeline(arg1);
        }, arguments); },
        __wbg_setPipeline_bab24dbce96903b9: function() { return logError(function (arg0, arg1) {
            arg0.setPipeline(arg1);
        }, arguments); },
        __wbg_setScissorRect_40786fdec122b032: function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setScissorRect(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
        }, arguments); },
        __wbg_setStencilReference_f614f80489a0b9b4: function() { return logError(function (arg0, arg1) {
            arg0.setStencilReference(arg1 >>> 0);
        }, arguments); },
        __wbg_setVertexBuffer_91c4b602d0289943: function() { return logError(function (arg0, arg1, arg2, arg3) {
            arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3);
        }, arguments); },
        __wbg_setVertexBuffer_b508baf8d0ffe331: function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3, arg4);
        }, arguments); },
        __wbg_setViewport_f9d423db4f4b4b58: function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.setViewport(arg1, arg2, arg3, arg4, arg5, arg6);
        }, arguments); },
        __wbg_set_8535240470bf2500: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_set_a_5f6e488475272136: function() { return logError(function (arg0, arg1) {
            arg0.a = arg1;
        }, arguments); },
        __wbg_set_access_091f317905cd76a5: function() { return logError(function (arg0, arg1) {
            arg0.access = __wbindgen_enum_GpuStorageTextureAccess[arg1];
        }, arguments); },
        __wbg_set_address_mode_u_a37cf1035585c638: function() { return logError(function (arg0, arg1) {
            arg0.addressModeU = __wbindgen_enum_GpuAddressMode[arg1];
        }, arguments); },
        __wbg_set_address_mode_v_8ac049e029caef76: function() { return logError(function (arg0, arg1) {
            arg0.addressModeV = __wbindgen_enum_GpuAddressMode[arg1];
        }, arguments); },
        __wbg_set_address_mode_w_eb9260ee11729e92: function() { return logError(function (arg0, arg1) {
            arg0.addressModeW = __wbindgen_enum_GpuAddressMode[arg1];
        }, arguments); },
        __wbg_set_alpha_aa2e606e9e647b21: function() { return logError(function (arg0, arg1) {
            arg0.alpha = arg1;
        }, arguments); },
        __wbg_set_alpha_mode_92402195b3ae1ee7: function() { return logError(function (arg0, arg1) {
            arg0.alphaMode = __wbindgen_enum_GpuCanvasAlphaMode[arg1];
        }, arguments); },
        __wbg_set_alpha_to_coverage_enabled_b4ce9c3f7f8b7ad7: function() { return logError(function (arg0, arg1) {
            arg0.alphaToCoverageEnabled = arg1 !== 0;
        }, arguments); },
        __wbg_set_array_layer_count_daec613068108a9d: function() { return logError(function (arg0, arg1) {
            arg0.arrayLayerCount = arg1 >>> 0;
        }, arguments); },
        __wbg_set_array_stride_c2c009eabc18b5f6: function() { return logError(function (arg0, arg1) {
            arg0.arrayStride = arg1;
        }, arguments); },
        __wbg_set_aspect_77332ac136ee94eb: function() { return logError(function (arg0, arg1) {
            arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
        }, arguments); },
        __wbg_set_aspect_9ea7cc5843075321: function() { return logError(function (arg0, arg1) {
            arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
        }, arguments); },
        __wbg_set_aspect_a823a14d00d42d37: function() { return logError(function (arg0, arg1) {
            arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
        }, arguments); },
        __wbg_set_attributes_05f9117fd32ca606: function() { return logError(function (arg0, arg1) {
            arg0.attributes = arg1;
        }, arguments); },
        __wbg_set_b_688365d692bba214: function() { return logError(function (arg0, arg1) {
            arg0.b = arg1;
        }, arguments); },
        __wbg_set_base_array_layer_cc6c68d233489c4b: function() { return logError(function (arg0, arg1) {
            arg0.baseArrayLayer = arg1 >>> 0;
        }, arguments); },
        __wbg_set_base_mip_level_e07a3efe9006d5ea: function() { return logError(function (arg0, arg1) {
            arg0.baseMipLevel = arg1 >>> 0;
        }, arguments); },
        __wbg_set_beginning_of_pass_write_index_27be5b0b35ec3de0: function() { return logError(function (arg0, arg1) {
            arg0.beginningOfPassWriteIndex = arg1 >>> 0;
        }, arguments); },
        __wbg_set_beginning_of_pass_write_index_c12e7856ee670800: function() { return logError(function (arg0, arg1) {
            arg0.beginningOfPassWriteIndex = arg1 >>> 0;
        }, arguments); },
        __wbg_set_bind_group_layouts_5325d038771af328: function() { return logError(function (arg0, arg1) {
            arg0.bindGroupLayouts = arg1;
        }, arguments); },
        __wbg_set_binding_b6b0fe5c281b8c69: function() { return logError(function (arg0, arg1) {
            arg0.binding = arg1 >>> 0;
        }, arguments); },
        __wbg_set_binding_f3c188a8cd21455b: function() { return logError(function (arg0, arg1) {
            arg0.binding = arg1 >>> 0;
        }, arguments); },
        __wbg_set_blend_8d6e9c08b5702a09: function() { return logError(function (arg0, arg1) {
            arg0.blend = arg1;
        }, arguments); },
        __wbg_set_buffer_55f096330c8912b4: function() { return logError(function (arg0, arg1) {
            arg0.buffer = arg1;
        }, arguments); },
        __wbg_set_buffer_aa7bf4ad8f17b2bd: function() { return logError(function (arg0, arg1) {
            arg0.buffer = arg1;
        }, arguments); },
        __wbg_set_buffer_e89095a9f0cafad3: function() { return logError(function (arg0, arg1) {
            arg0.buffer = arg1;
        }, arguments); },
        __wbg_set_buffers_85a7238f4ef28ab4: function() { return logError(function (arg0, arg1) {
            arg0.buffers = arg1;
        }, arguments); },
        __wbg_set_bytes_per_row_68a1ea90d4710bc9: function() { return logError(function (arg0, arg1) {
            arg0.bytesPerRow = arg1 >>> 0;
        }, arguments); },
        __wbg_set_bytes_per_row_91681ca78d744888: function() { return logError(function (arg0, arg1) {
            arg0.bytesPerRow = arg1 >>> 0;
        }, arguments); },
        __wbg_set_clear_value_642701f928a5ccb3: function() { return logError(function (arg0, arg1) {
            arg0.clearValue = arg1;
        }, arguments); },
        __wbg_set_code_56e2d45ec1ff6c2d: function() { return logError(function (arg0, arg1, arg2) {
            arg0.code = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_color_attachments_abe67f6631926e28: function() { return logError(function (arg0, arg1) {
            arg0.colorAttachments = arg1;
        }, arguments); },
        __wbg_set_color_bc393d7efc3c8594: function() { return logError(function (arg0, arg1) {
            arg0.color = arg1;
        }, arguments); },
        __wbg_set_color_formats_239bac9e2221f4e1: function() { return logError(function (arg0, arg1) {
            arg0.colorFormats = arg1;
        }, arguments); },
        __wbg_set_compare_1509dc1a5420943f: function() { return logError(function (arg0, arg1) {
            arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
        }, arguments); },
        __wbg_set_compare_42211fbf15e3b850: function() { return logError(function (arg0, arg1) {
            arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
        }, arguments); },
        __wbg_set_compute_5a859e405c9eb6c6: function() { return logError(function (arg0, arg1) {
            arg0.compute = arg1;
        }, arguments); },
        __wbg_set_count_26a934d1cd07d080: function() { return logError(function (arg0, arg1) {
            arg0.count = arg1 >>> 0;
        }, arguments); },
        __wbg_set_count_f261876d1bb59b88: function() { return logError(function (arg0, arg1) {
            arg0.count = arg1 >>> 0;
        }, arguments); },
        __wbg_set_cull_mode_9d466c1ab414cac8: function() { return logError(function (arg0, arg1) {
            arg0.cullMode = __wbindgen_enum_GpuCullMode[arg1];
        }, arguments); },
        __wbg_set_depth_bias_428c9340b0fd937b: function() { return logError(function (arg0, arg1) {
            arg0.depthBias = arg1;
        }, arguments); },
        __wbg_set_depth_bias_clamp_f009599ca67fa30c: function() { return logError(function (arg0, arg1) {
            arg0.depthBiasClamp = arg1;
        }, arguments); },
        __wbg_set_depth_bias_slope_scale_7125880b4cb7a951: function() { return logError(function (arg0, arg1) {
            arg0.depthBiasSlopeScale = arg1;
        }, arguments); },
        __wbg_set_depth_clear_value_442bf492734f63b6: function() { return logError(function (arg0, arg1) {
            arg0.depthClearValue = arg1;
        }, arguments); },
        __wbg_set_depth_compare_30e9ea552da12fe2: function() { return logError(function (arg0, arg1) {
            arg0.depthCompare = __wbindgen_enum_GpuCompareFunction[arg1];
        }, arguments); },
        __wbg_set_depth_fail_op_5e42dc3e4c382951: function() { return logError(function (arg0, arg1) {
            arg0.depthFailOp = __wbindgen_enum_GpuStencilOperation[arg1];
        }, arguments); },
        __wbg_set_depth_load_op_34d430b74bb36d91: function() { return logError(function (arg0, arg1) {
            arg0.depthLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
        }, arguments); },
        __wbg_set_depth_or_array_layers_4bbbeadacb393f02: function() { return logError(function (arg0, arg1) {
            arg0.depthOrArrayLayers = arg1 >>> 0;
        }, arguments); },
        __wbg_set_depth_read_only_138a11b10c731094: function() { return logError(function (arg0, arg1) {
            arg0.depthReadOnly = arg1 !== 0;
        }, arguments); },
        __wbg_set_depth_read_only_95b1b7ed81cae390: function() { return logError(function (arg0, arg1) {
            arg0.depthReadOnly = arg1 !== 0;
        }, arguments); },
        __wbg_set_depth_stencil_1bd50dbc450c8650: function() { return logError(function (arg0, arg1) {
            arg0.depthStencil = arg1;
        }, arguments); },
        __wbg_set_depth_stencil_attachment_1ee0d93bc3273369: function() { return logError(function (arg0, arg1) {
            arg0.depthStencilAttachment = arg1;
        }, arguments); },
        __wbg_set_depth_stencil_format_d4f66e8e468e9d8c: function() { return logError(function (arg0, arg1) {
            arg0.depthStencilFormat = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_depth_store_op_0ea0a215313dbda7: function() { return logError(function (arg0, arg1) {
            arg0.depthStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
        }, arguments); },
        __wbg_set_depth_write_enabled_64c2e7f6fa4b6b7b: function() { return logError(function (arg0, arg1) {
            arg0.depthWriteEnabled = arg1 !== 0;
        }, arguments); },
        __wbg_set_device_0d774b66e7288f72: function() { return logError(function (arg0, arg1) {
            arg0.device = arg1;
        }, arguments); },
        __wbg_set_dimension_174ad7e2fb67fb4e: function() { return logError(function (arg0, arg1) {
            arg0.dimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
        }, arguments); },
        __wbg_set_dimension_36e13ccecae5af4b: function() { return logError(function (arg0, arg1) {
            arg0.dimension = __wbindgen_enum_GpuTextureDimension[arg1];
        }, arguments); },
        __wbg_set_dst_factor_1ed75271a89a711e: function() { return logError(function (arg0, arg1) {
            arg0.dstFactor = __wbindgen_enum_GpuBlendFactor[arg1];
        }, arguments); },
        __wbg_set_end_of_pass_write_index_e8f52fc08bc0603e: function() { return logError(function (arg0, arg1) {
            arg0.endOfPassWriteIndex = arg1 >>> 0;
        }, arguments); },
        __wbg_set_end_of_pass_write_index_f4ab90c5743df805: function() { return logError(function (arg0, arg1) {
            arg0.endOfPassWriteIndex = arg1 >>> 0;
        }, arguments); },
        __wbg_set_entries_3017e6132f938c6e: function() { return logError(function (arg0, arg1) {
            arg0.entries = arg1;
        }, arguments); },
        __wbg_set_entries_fc76ca4d7da6a709: function() { return logError(function (arg0, arg1) {
            arg0.entries = arg1;
        }, arguments); },
        __wbg_set_entry_point_4443daff87d82ef1: function() { return logError(function (arg0, arg1, arg2) {
            arg0.entryPoint = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_entry_point_6fec5723cc790927: function() { return logError(function (arg0, arg1, arg2) {
            arg0.entryPoint = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_entry_point_8db3b6d103e3b865: function() { return logError(function (arg0, arg1, arg2) {
            arg0.entryPoint = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_external_texture_825fe2bc7a0c0603: function() { return logError(function (arg0, arg1) {
            arg0.externalTexture = arg1;
        }, arguments); },
        __wbg_set_fail_op_77ab26c98f847b65: function() { return logError(function (arg0, arg1) {
            arg0.failOp = __wbindgen_enum_GpuStencilOperation[arg1];
        }, arguments); },
        __wbg_set_flip_y_7e37e283463dd527: function() { return logError(function (arg0, arg1) {
            arg0.flipY = arg1 !== 0;
        }, arguments); },
        __wbg_set_format_1786adb7bc74c7c9: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_format_6606f5c1fba6f459: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuVertexFormat[arg1];
        }, arguments); },
        __wbg_set_format_90860b0321868db4: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_format_abf7a1bc5425c56a: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_format_d347899cd860709c: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_format_e9d4b1475bb3bd3b: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_format_f9341112e43ea182: function() { return logError(function (arg0, arg1) {
            arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
        }, arguments); },
        __wbg_set_fragment_1a595620425637e1: function() { return logError(function (arg0, arg1) {
            arg0.fragment = arg1;
        }, arguments); },
        __wbg_set_front_face_50cdf4eb61504a46: function() { return logError(function (arg0, arg1) {
            arg0.frontFace = __wbindgen_enum_GpuFrontFace[arg1];
        }, arguments); },
        __wbg_set_g_d4d1d77cf8fdd362: function() { return logError(function (arg0, arg1) {
            arg0.g = arg1;
        }, arguments); },
        __wbg_set_has_dynamic_offset_7d30014fdbfe90c5: function() { return logError(function (arg0, arg1) {
            arg0.hasDynamicOffset = arg1 !== 0;
        }, arguments); },
        __wbg_set_height_7d9d8f892e6964c6: function() { return logError(function (arg0, arg1) {
            arg0.height = arg1 >>> 0;
        }, arguments); },
        __wbg_set_height_bbeef8f354041577: function() { return logError(function (arg0, arg1) {
            arg0.height = arg1 >>> 0;
        }, arguments); },
        __wbg_set_height_e8b5483b8c117d5e: function() { return logError(function (arg0, arg1) {
            arg0.height = arg1 >>> 0;
        }, arguments); },
        __wbg_set_label_03d2396d4655a3e1: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_0c1bd0e976cf0a9a: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_1175a3329a06e52b: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_2d2227f4d5991e50: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_2f592bd1be3db6b3: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_44c8df98c1f1e811: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_4a1dd4244f80abc9: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_8b0da33fd11b2572: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_8fd860a36d2c7b74: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_bae57fb9f24fde5c: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_be45aed56e4b9fee: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_c47c451211e2f6d2: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_cd567b7b35838e4c: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_d1c24b5a7a3ac31d: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_da96d497ad3f53e7: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_dcd98efbb9370da8: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_label_f92ae11c77d74198: function() { return logError(function (arg0, arg1, arg2) {
            arg0.label = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_layout_19e558a0fa724e95: function() { return logError(function (arg0, arg1) {
            arg0.layout = arg1;
        }, arguments); },
        __wbg_set_layout_7c5ba5bdcde8a0f0: function() { return logError(function (arg0, arg1) {
            arg0.layout = arg1;
        }, arguments); },
        __wbg_set_layout_eeef59714f5bf48b: function() { return logError(function (arg0, arg1) {
            arg0.layout = arg1;
        }, arguments); },
        __wbg_set_load_op_56844f51434037bf: function() { return logError(function (arg0, arg1) {
            arg0.loadOp = __wbindgen_enum_GpuLoadOp[arg1];
        }, arguments); },
        __wbg_set_lod_max_clamp_3f157633f32c9f94: function() { return logError(function (arg0, arg1) {
            arg0.lodMaxClamp = arg1;
        }, arguments); },
        __wbg_set_lod_min_clamp_7e246c739fb1a854: function() { return logError(function (arg0, arg1) {
            arg0.lodMinClamp = arg1;
        }, arguments); },
        __wbg_set_mag_filter_69d846b974d4bcc0: function() { return logError(function (arg0, arg1) {
            arg0.magFilter = __wbindgen_enum_GpuFilterMode[arg1];
        }, arguments); },
        __wbg_set_mapped_at_creation_48de4735fab51e78: function() { return logError(function (arg0, arg1) {
            arg0.mappedAtCreation = arg1 !== 0;
        }, arguments); },
        __wbg_set_mask_0c49a66362fc0079: function() { return logError(function (arg0, arg1) {
            arg0.mask = arg1 >>> 0;
        }, arguments); },
        __wbg_set_max_anisotropy_3ef0d5bca2336cc7: function() { return logError(function (arg0, arg1) {
            arg0.maxAnisotropy = arg1;
        }, arguments); },
        __wbg_set_min_binding_size_689661b9ed25e083: function() { return logError(function (arg0, arg1) {
            arg0.minBindingSize = arg1;
        }, arguments); },
        __wbg_set_min_filter_fbf2d8d9f503dcd7: function() { return logError(function (arg0, arg1) {
            arg0.minFilter = __wbindgen_enum_GpuFilterMode[arg1];
        }, arguments); },
        __wbg_set_mip_level_246db61be15bdd69: function() { return logError(function (arg0, arg1) {
            arg0.mipLevel = arg1 >>> 0;
        }, arguments); },
        __wbg_set_mip_level_count_72f8bc1f80f7539b: function() { return logError(function (arg0, arg1) {
            arg0.mipLevelCount = arg1 >>> 0;
        }, arguments); },
        __wbg_set_mip_level_count_b19a0d9192e62d5d: function() { return logError(function (arg0, arg1) {
            arg0.mipLevelCount = arg1 >>> 0;
        }, arguments); },
        __wbg_set_mip_level_d480a4a8dc18e56b: function() { return logError(function (arg0, arg1) {
            arg0.mipLevel = arg1 >>> 0;
        }, arguments); },
        __wbg_set_mipmap_filter_17fd50a3898fd5ff: function() { return logError(function (arg0, arg1) {
            arg0.mipmapFilter = __wbindgen_enum_GpuMipmapFilterMode[arg1];
        }, arguments); },
        __wbg_set_module_08ad08e736d8edbf: function() { return logError(function (arg0, arg1) {
            arg0.module = arg1;
        }, arguments); },
        __wbg_set_module_14e471fdd94c582d: function() { return logError(function (arg0, arg1) {
            arg0.module = arg1;
        }, arguments); },
        __wbg_set_module_9b938909233aed50: function() { return logError(function (arg0, arg1) {
            arg0.module = arg1;
        }, arguments); },
        __wbg_set_multisample_85f073947b782d07: function() { return logError(function (arg0, arg1) {
            arg0.multisample = arg1;
        }, arguments); },
        __wbg_set_multisampled_40505c1381e1c32c: function() { return logError(function (arg0, arg1) {
            arg0.multisampled = arg1 !== 0;
        }, arguments); },
        __wbg_set_offset_2c374e604504e0b2: function() { return logError(function (arg0, arg1) {
            arg0.offset = arg1;
        }, arguments); },
        __wbg_set_offset_73156b0e0b41d79a: function() { return logError(function (arg0, arg1) {
            arg0.offset = arg1;
        }, arguments); },
        __wbg_set_offset_8d9d9afffa18b591: function() { return logError(function (arg0, arg1) {
            arg0.offset = arg1;
        }, arguments); },
        __wbg_set_offset_a097a8050a3a9a33: function() { return logError(function (arg0, arg1) {
            arg0.offset = arg1;
        }, arguments); },
        __wbg_set_onuncapturederror_b9a9ff2c881b2b40: function() { return logError(function (arg0, arg1) {
            arg0.onuncapturederror = arg1;
        }, arguments); },
        __wbg_set_operation_b5862f5a1a143b30: function() { return logError(function (arg0, arg1) {
            arg0.operation = __wbindgen_enum_GpuBlendOperation[arg1];
        }, arguments); },
        __wbg_set_origin_9b3b0fbe0a5dc469: function() { return logError(function (arg0, arg1) {
            arg0.origin = arg1;
        }, arguments); },
        __wbg_set_origin_ad4c6de06be29313: function() { return logError(function (arg0, arg1) {
            arg0.origin = arg1;
        }, arguments); },
        __wbg_set_origin_cfbb67a96c9ce9cc: function() { return logError(function (arg0, arg1) {
            arg0.origin = arg1;
        }, arguments); },
        __wbg_set_pass_op_e9470d1262fb8a8b: function() { return logError(function (arg0, arg1) {
            arg0.passOp = __wbindgen_enum_GpuStencilOperation[arg1];
        }, arguments); },
        __wbg_set_power_preference_c0d3fa7ce46b1a2e: function() { return logError(function (arg0, arg1) {
            arg0.powerPreference = __wbindgen_enum_GpuPowerPreference[arg1];
        }, arguments); },
        __wbg_set_premultiplied_alpha_3ed1568015a154c7: function() { return logError(function (arg0, arg1) {
            arg0.premultipliedAlpha = arg1 !== 0;
        }, arguments); },
        __wbg_set_primitive_369241acd17871f1: function() { return logError(function (arg0, arg1) {
            arg0.primitive = arg1;
        }, arguments); },
        __wbg_set_query_set_18679a8580267d5a: function() { return logError(function (arg0, arg1) {
            arg0.querySet = arg1;
        }, arguments); },
        __wbg_set_query_set_f1314b06c84c4b00: function() { return logError(function (arg0, arg1) {
            arg0.querySet = arg1;
        }, arguments); },
        __wbg_set_r_527e5a41c4b1a846: function() { return logError(function (arg0, arg1) {
            arg0.r = arg1;
        }, arguments); },
        __wbg_set_required_features_54918de8185c5fab: function() { return logError(function (arg0, arg1) {
            arg0.requiredFeatures = arg1;
        }, arguments); },
        __wbg_set_required_limits_3b031f66f838f4e3: function() { return logError(function (arg0, arg1) {
            arg0.requiredLimits = arg1;
        }, arguments); },
        __wbg_set_resolve_target_fe76b3f99cf72078: function() { return logError(function (arg0, arg1) {
            arg0.resolveTarget = arg1;
        }, arguments); },
        __wbg_set_resource_fe385d2e3dadaf63: function() { return logError(function (arg0, arg1) {
            arg0.resource = arg1;
        }, arguments); },
        __wbg_set_rows_per_image_d198b7e73a38978b: function() { return logError(function (arg0, arg1) {
            arg0.rowsPerImage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_rows_per_image_f9878f4b10f4fd7f: function() { return logError(function (arg0, arg1) {
            arg0.rowsPerImage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_sample_count_865e1d19b84e27e6: function() { return logError(function (arg0, arg1) {
            arg0.sampleCount = arg1 >>> 0;
        }, arguments); },
        __wbg_set_sample_count_9f819993f95ad2c9: function() { return logError(function (arg0, arg1) {
            arg0.sampleCount = arg1 >>> 0;
        }, arguments); },
        __wbg_set_sample_type_7088b1efddce6a69: function() { return logError(function (arg0, arg1) {
            arg0.sampleType = __wbindgen_enum_GpuTextureSampleType[arg1];
        }, arguments); },
        __wbg_set_sampler_8c5d7fb1b02058c6: function() { return logError(function (arg0, arg1) {
            arg0.sampler = arg1;
        }, arguments); },
        __wbg_set_shader_location_0ff30a733291a396: function() { return logError(function (arg0, arg1) {
            arg0.shaderLocation = arg1 >>> 0;
        }, arguments); },
        __wbg_set_size_1e6281b07cd39177: function() { return logError(function (arg0, arg1) {
            arg0.size = arg1;
        }, arguments); },
        __wbg_set_size_41cd9255ca1e4242: function() { return logError(function (arg0, arg1) {
            arg0.size = arg1;
        }, arguments); },
        __wbg_set_size_a61ff22205255d61: function() { return logError(function (arg0, arg1) {
            arg0.size = arg1;
        }, arguments); },
        __wbg_set_source_6e0a2e56f523024f: function() { return logError(function (arg0, arg1) {
            arg0.source = arg1;
        }, arguments); },
        __wbg_set_src_factor_1c4f755f8676df1b: function() { return logError(function (arg0, arg1) {
            arg0.srcFactor = __wbindgen_enum_GpuBlendFactor[arg1];
        }, arguments); },
        __wbg_set_stencil_back_6ef4683123b19b25: function() { return logError(function (arg0, arg1) {
            arg0.stencilBack = arg1;
        }, arguments); },
        __wbg_set_stencil_clear_value_10b58f674d0177c2: function() { return logError(function (arg0, arg1) {
            arg0.stencilClearValue = arg1 >>> 0;
        }, arguments); },
        __wbg_set_stencil_front_aeb8580a97e5424b: function() { return logError(function (arg0, arg1) {
            arg0.stencilFront = arg1;
        }, arguments); },
        __wbg_set_stencil_load_op_f20a90a66acd3d8c: function() { return logError(function (arg0, arg1) {
            arg0.stencilLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
        }, arguments); },
        __wbg_set_stencil_read_mask_2954f260d47349ea: function() { return logError(function (arg0, arg1) {
            arg0.stencilReadMask = arg1 >>> 0;
        }, arguments); },
        __wbg_set_stencil_read_only_b8a209436979e19f: function() { return logError(function (arg0, arg1) {
            arg0.stencilReadOnly = arg1 !== 0;
        }, arguments); },
        __wbg_set_stencil_read_only_fb489d191b6d969b: function() { return logError(function (arg0, arg1) {
            arg0.stencilReadOnly = arg1 !== 0;
        }, arguments); },
        __wbg_set_stencil_store_op_477c4cf6422dfa3f: function() { return logError(function (arg0, arg1) {
            arg0.stencilStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
        }, arguments); },
        __wbg_set_stencil_write_mask_3f8e9b3781814a95: function() { return logError(function (arg0, arg1) {
            arg0.stencilWriteMask = arg1 >>> 0;
        }, arguments); },
        __wbg_set_step_mode_a35aef328761c452: function() { return logError(function (arg0, arg1) {
            arg0.stepMode = __wbindgen_enum_GpuVertexStepMode[arg1];
        }, arguments); },
        __wbg_set_storage_texture_ab9eed9786337ef0: function() { return logError(function (arg0, arg1) {
            arg0.storageTexture = arg1;
        }, arguments); },
        __wbg_set_store_op_caeede4654b3d847: function() { return logError(function (arg0, arg1) {
            arg0.storeOp = __wbindgen_enum_GpuStoreOp[arg1];
        }, arguments); },
        __wbg_set_strip_index_format_0cd0510e166c4ec4: function() { return logError(function (arg0, arg1) {
            arg0.stripIndexFormat = __wbindgen_enum_GpuIndexFormat[arg1];
        }, arguments); },
        __wbg_set_targets_6b0b3bdd87f35668: function() { return logError(function (arg0, arg1) {
            arg0.targets = arg1;
        }, arguments); },
        __wbg_set_texture_16d2be474ce6ad0c: function() { return logError(function (arg0, arg1) {
            arg0.texture = arg1;
        }, arguments); },
        __wbg_set_texture_e25a73da75cf5808: function() { return logError(function (arg0, arg1) {
            arg0.texture = arg1;
        }, arguments); },
        __wbg_set_texture_f5131fc886cc9ce6: function() { return logError(function (arg0, arg1) {
            arg0.texture = arg1;
        }, arguments); },
        __wbg_set_timestamp_writes_26336a2ad72cdcaf: function() { return logError(function (arg0, arg1) {
            arg0.timestampWrites = arg1;
        }, arguments); },
        __wbg_set_timestamp_writes_c552d52fbb417005: function() { return logError(function (arg0, arg1) {
            arg0.timestampWrites = arg1;
        }, arguments); },
        __wbg_set_topology_beefb3aca0612b00: function() { return logError(function (arg0, arg1) {
            arg0.topology = __wbindgen_enum_GpuPrimitiveTopology[arg1];
        }, arguments); },
        __wbg_set_type_38961e08504ca674: function() { return logError(function (arg0, arg1) {
            arg0.type = __wbindgen_enum_GpuBufferBindingType[arg1];
        }, arguments); },
        __wbg_set_type_c1eebc19f8a6aeb9: function() { return logError(function (arg0, arg1) {
            arg0.type = __wbindgen_enum_GpuSamplerBindingType[arg1];
        }, arguments); },
        __wbg_set_type_f062717b30496eff: function() { return logError(function (arg0, arg1) {
            arg0.type = __wbindgen_enum_GpuQueryType[arg1];
        }, arguments); },
        __wbg_set_unclipped_depth_5a4f7eb57fe006b2: function() { return logError(function (arg0, arg1) {
            arg0.unclippedDepth = arg1 !== 0;
        }, arguments); },
        __wbg_set_usage_7f0dda8309469b1c: function() { return logError(function (arg0, arg1) {
            arg0.usage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_usage_7fa9cd18d1104aca: function() { return logError(function (arg0, arg1) {
            arg0.usage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_usage_908213a4d4bb8bde: function() { return logError(function (arg0, arg1) {
            arg0.usage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_usage_ae014e77ff77ce06: function() { return logError(function (arg0, arg1) {
            arg0.usage = arg1 >>> 0;
        }, arguments); },
        __wbg_set_vertex_a4951dd9a7a4ed54: function() { return logError(function (arg0, arg1) {
            arg0.vertex = arg1;
        }, arguments); },
        __wbg_set_view_bdeab150b5f0768c: function() { return logError(function (arg0, arg1) {
            arg0.view = arg1;
        }, arguments); },
        __wbg_set_view_dbd0294573f64d05: function() { return logError(function (arg0, arg1) {
            arg0.view = arg1;
        }, arguments); },
        __wbg_set_view_dimension_263387976511ebc9: function() { return logError(function (arg0, arg1) {
            arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
        }, arguments); },
        __wbg_set_view_dimension_3ed01b237e85826f: function() { return logError(function (arg0, arg1) {
            arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
        }, arguments); },
        __wbg_set_view_formats_bab284fc81b40e70: function() { return logError(function (arg0, arg1) {
            arg0.viewFormats = arg1;
        }, arguments); },
        __wbg_set_view_formats_fe531a043efb71fa: function() { return logError(function (arg0, arg1) {
            arg0.viewFormats = arg1;
        }, arguments); },
        __wbg_set_visibility_1bca121a89accba5: function() { return logError(function (arg0, arg1) {
            arg0.visibility = arg1 >>> 0;
        }, arguments); },
        __wbg_set_width_1a5e2e86fa5bdcd8: function() { return logError(function (arg0, arg1) {
            arg0.width = arg1 >>> 0;
        }, arguments); },
        __wbg_set_width_49ac9b7d914afc85: function() { return logError(function (arg0, arg1) {
            arg0.width = arg1 >>> 0;
        }, arguments); },
        __wbg_set_width_8e30d010cd66830d: function() { return logError(function (arg0, arg1) {
            arg0.width = arg1 >>> 0;
        }, arguments); },
        __wbg_set_write_mask_144b25e2bd909124: function() { return logError(function (arg0, arg1) {
            arg0.writeMask = arg1 >>> 0;
        }, arguments); },
        __wbg_set_x_56f0c2c08a62725c: function() { return logError(function (arg0, arg1) {
            arg0.x = arg1 >>> 0;
        }, arguments); },
        __wbg_set_x_7f1ce8377ea914e5: function() { return logError(function (arg0, arg1) {
            arg0.x = arg1 >>> 0;
        }, arguments); },
        __wbg_set_y_04fb8ce84735b4e1: function() { return logError(function (arg0, arg1) {
            arg0.y = arg1 >>> 0;
        }, arguments); },
        __wbg_set_y_09965fd0dd252fb5: function() { return logError(function (arg0, arg1) {
            arg0.y = arg1 >>> 0;
        }, arguments); },
        __wbg_set_z_a51316db27a4941e: function() { return logError(function (arg0, arg1) {
            arg0.z = arg1 >>> 0;
        }, arguments); },
        __wbg_size_1356eae711a92515: function() { return logError(function (arg0) {
            const ret = arg0.size;
            return ret;
        }, arguments); },
        __wbg_static_accessor_CREATE_TASK_7ee0dd8bc83df5b2: function() { return logError(function () {
            const ret = typeof console === 'undefined' ? null : console?.createTask;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_4ef717fb391d88b7: function() { return logError(function () {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_THIS_8d1badc68b5a74f4: function() { return logError(function () {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_SELF_146583524fe1469b: function() { return logError(function () {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_WINDOW_f2829a2234d7819e: function() { return logError(function () {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_submit_1290d44bb76ecef4: function() { return logError(function (arg0, arg1) {
            arg0.submit(arg1);
        }, arguments); },
        __wbg_then_16d107c451e9905d: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_then_4a0b9283a66c4a8a: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_then_6ec10ae38b3e92f7: function() { return logError(function (arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        }, arguments); },
        __wbg_then_e0960b859f3ff223: function() { return logError(function (arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        }, arguments); },
        __wbg_unmap_8f06698a75b8331a: function() { return logError(function (arg0) {
            arg0.unmap();
        }, arguments); },
        __wbg_usage_ffc49211c0488f66: function() { return logError(function (arg0) {
            const ret = arg0.usage;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_valueOf_64f89f12f08671ee: function() { return logError(function (arg0) {
            const ret = arg0.valueOf();
            return ret;
        }, arguments); },
        __wbg_value_a5d5488a9589444a: function() { return logError(function (arg0) {
            const ret = arg0.value;
            return ret;
        }, arguments); },
        __wbg_wgslLanguageFeatures_643e989cbd94b299: function() { return logError(function (arg0) {
            const ret = arg0.wgslLanguageFeatures;
            return ret;
        }, arguments); },
        __wbg_writeBuffer_b4bdd36178348ca5: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.writeBuffer(arg1, arg2, getArrayU8FromWasm0(arg3, arg4), arg5, arg6);
        }, arguments); },
        __wbg_writeTexture_b45b69132e46a227: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.writeTexture(arg1, getArrayU8FromWasm0(arg2, arg3), arg4, arg5);
        }, arguments); },
        __wbindgen_cast_0000000000000001: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 2159, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue______true_);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000002: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 2458, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue__core_7a2330d63e03cc2c___result__Result_____wasm_bindgen_458697782d79938e___JsError___true_);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000003: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("GPUUncapturedErrorEvent")], shim_idx: 2160, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_458697782d79938e___convert__closures_____invoke___wgpu_bd4cf1f8f3ffcf01___backend__webgpu__webgpu_sys__gen_GpuUncapturedErrorEvent__GpuUncapturedErrorEvent______true_);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000004: function() { return logError(function (arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000005: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000006: function() { return logError(function (arg0, arg1) {
            var v0 = getArrayU8FromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_free(arg0, arg1 * 1, 1);
            // Cast intrinsic for `Vector(U8) -> Externref`.
            const ret = v0;
            return ret;
        }, arguments); },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./flow_core_bg.js": import0,
    };
}


//#endregion
function wasm_bindgen_458697782d79938e___convert__closures_____invoke___bool__true_(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    const ret = wasm.wasm_bindgen_458697782d79938e___convert__closures_____invoke___bool__true_(arg0, arg1);
    return ret !== 0;
}

function wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue______true_(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue______true_(arg0, arg1, arg2);
}

function wasm_bindgen_458697782d79938e___convert__closures_____invoke___wgpu_bd4cf1f8f3ffcf01___backend__webgpu__webgpu_sys__gen_GpuUncapturedErrorEvent__GpuUncapturedErrorEvent______true_(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen_458697782d79938e___convert__closures_____invoke___wgpu_bd4cf1f8f3ffcf01___backend__webgpu__webgpu_sys__gen_GpuUncapturedErrorEvent__GpuUncapturedErrorEvent______true_(arg0, arg1, arg2);
}

function wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue__core_7a2330d63e03cc2c___result__Result_____wasm_bindgen_458697782d79938e___JsError___true_(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    const ret = wasm.wasm_bindgen_458697782d79938e___convert__closures_____invoke___wasm_bindgen_458697782d79938e___JsValue__core_7a2330d63e03cc2c___result__Result_____wasm_bindgen_458697782d79938e___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_458697782d79938e___convert__closures_____invoke___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen_458697782d79938e___convert__closures_____invoke___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined___js_sys_9fa0e33012cbccdb___Function_fn_wasm_bindgen_458697782d79938e___JsValue_____wasm_bindgen_458697782d79938e___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}


const __wbindgen_enum_GpuAddressMode = ["clamp-to-edge", "repeat", "mirror-repeat"];


const __wbindgen_enum_GpuBlendFactor = ["zero", "one", "src", "one-minus-src", "src-alpha", "one-minus-src-alpha", "dst", "one-minus-dst", "dst-alpha", "one-minus-dst-alpha", "src-alpha-saturated", "constant", "one-minus-constant", "src1", "one-minus-src1", "src1-alpha", "one-minus-src1-alpha"];


const __wbindgen_enum_GpuBlendOperation = ["add", "subtract", "reverse-subtract", "min", "max"];


const __wbindgen_enum_GpuBufferBindingType = ["uniform", "storage", "read-only-storage"];


const __wbindgen_enum_GpuCanvasAlphaMode = ["opaque", "premultiplied"];


const __wbindgen_enum_GpuCompareFunction = ["never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"];


const __wbindgen_enum_GpuCullMode = ["none", "front", "back"];


const __wbindgen_enum_GpuDeviceLostReason = ["unknown", "destroyed"];


const __wbindgen_enum_GpuErrorFilter = ["validation", "out-of-memory", "internal"];


const __wbindgen_enum_GpuFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuFrontFace = ["ccw", "cw"];


const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];


const __wbindgen_enum_GpuLoadOp = ["load", "clear"];


const __wbindgen_enum_GpuMipmapFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuPowerPreference = ["low-power", "high-performance"];


const __wbindgen_enum_GpuPrimitiveTopology = ["point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"];


const __wbindgen_enum_GpuQueryType = ["occlusion", "timestamp"];


const __wbindgen_enum_GpuSamplerBindingType = ["filtering", "non-filtering", "comparison"];


const __wbindgen_enum_GpuStencilOperation = ["keep", "zero", "replace", "invert", "increment-clamp", "decrement-clamp", "increment-wrap", "decrement-wrap"];


const __wbindgen_enum_GpuStorageTextureAccess = ["write-only", "read-only", "read-write"];


const __wbindgen_enum_GpuStoreOp = ["store", "discard"];


const __wbindgen_enum_GpuTextureAspect = ["all", "stencil-only", "depth-only"];


const __wbindgen_enum_GpuTextureDimension = ["1d", "2d", "3d"];


const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];


const __wbindgen_enum_GpuTextureSampleType = ["float", "unfilterable-float", "depth", "sint", "uint"];


const __wbindgen_enum_GpuTextureViewDimension = ["1d", "2d", "2d-array", "cube", "cube-array", "3d"];


const __wbindgen_enum_GpuVertexFormat = ["uint8", "uint8x2", "uint8x4", "sint8", "sint8x2", "sint8x4", "unorm8", "unorm8x2", "unorm8x4", "snorm8", "snorm8x2", "snorm8x4", "uint16", "uint16x2", "uint16x4", "sint16", "sint16x2", "sint16x4", "unorm16", "unorm16x2", "unorm16x4", "snorm16", "snorm16x2", "snorm16x4", "float16", "float16x2", "float16x4", "float32", "float32x2", "float32x3", "float32x4", "uint32", "uint32x2", "uint32x3", "uint32x4", "sint32", "sint32x2", "sint32x3", "sint32x4", "unorm10-10-10-2", "unorm8x4-bgra"];


const __wbindgen_enum_GpuVertexStepMode = ["vertex", "instance"];
const DagSessionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dagsession_free(ptr, 1));
const DagSnapshotVcsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dagsnapshotvcs_free(ptr, 1));
const KernelHostFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_kernelhost_free(ptr, 1));


//#region intrinsics
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertBigInt(n) {
    if (typeof(n) !== 'bigint') throw new Error(`expected a bigint argument, found ${typeof(n)}`);
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


//#endregion

//#region wasm loading
let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('flow_core_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
//#endregion
export { wasm as __wasm }
