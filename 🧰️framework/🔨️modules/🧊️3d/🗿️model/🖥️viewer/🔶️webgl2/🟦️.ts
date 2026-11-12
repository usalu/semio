/** 🔶️ WebGL2 {@link RenderBackend}: one interleaved vertex buffer, one skin buffer, one global index buffer and one VAO for the whole scene; per draw only uniforms and `drawElements`.
 *
 * Draw order and state come from the frame: opaque triangles (depth write, back-face culling unless double-sided, polygon offset so lines stay visible), blended triangles (premultiplied alpha, no depth write), lines (depth `LEQUAL`, unlit, 1 device pixel wide — WebGL guarantees no wider lines).
 * Skinning reads a float palette texture. The canvas backing store follows `cssSize * pixelRatio`. Context loss is handled: `webglcontextlost` is cancelled, resources are rebuilt from the retained scene on `webglcontextrestored`, and `onStatus` tells the owner.
 * The draw loop allocates nothing: uniforms are set from the frame arrays by offset, state changes are tracked.
 * @see ../🧬️contract/🟦️.ts */
import type { Operation } from "../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MATERIAL_STRIDE, SKIN_STRIDE, VERTEX_STRIDE, type BackendStatus, type Frame, type RenderBackend, type RenderStyle, type Scene } from "../🧬️contract/🟦️.ts";
import { FRAGMENT_SOURCE, VERTEX_SOURCE } from "./🎨️shaders/🟦️.ts";

type Uniforms = Record<"viewProjection" | "model" | "palette" | "paletteOffset" | "base" | "emissiveUnlit" | "surface" | "eye" | "lightDirection" | "lightColor" | "sky" | "ground", WebGLUniformLocation | null>;

/** 🎮️ The WebGL2 backend. Create it with {@link createWebGl2Backend}. */
export class WebGl2Backend implements RenderBackend {
  readonly kind = "webgl2";
  private gl: WebGL2RenderingContext;
  private program: WebGLProgram | null = null;
  private vao: WebGLVertexArrayObject | null = null;
  private buffers: WebGLBuffer[] = [];
  private palette: WebGLTexture | null = null;
  private paletteRows = 0;
  private uniforms!: Uniforms;
  private scene: Scene | null = null;
  private lost = false;
  private pixels = new Uint8Array(0);
  private readonly onLost = (e: Event) => {
    e.preventDefault();
    this.lost = true;
    this.onStatus?.("lost");
  };
  private readonly onRestored = () => {
    this.lost = false;
    this.initialise();
    if (this.scene) this.upload(this.scene);
    this.onStatus?.("restored");
  };

  constructor(readonly canvas: HTMLCanvasElement, private readonly onStatus?: (status: BackendStatus) => void) {
    const gl = canvas.getContext("webgl2", { antialias: true, alpha: true, premultipliedAlpha: true, preserveDrawingBuffer: true, powerPreference: "high-performance" });
    if (!gl) throw new Error("webgl2 unavailable");
    this.gl = gl;
    canvas.addEventListener("webglcontextlost", this.onLost);
    canvas.addEventListener("webglcontextrestored", this.onRestored);
    this.initialise();
    this.onStatus?.("ready");
  }

  private compile(type: number, source: string): WebGLShader {
    const gl = this.gl;
    const shader = gl.createShader(type)!;
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) throw new Error(`shader: ${gl.getShaderInfoLog(shader)}`);
    return shader;
  }

  private initialise(): void {
    const gl = this.gl;
    const program = gl.createProgram()!;
    gl.attachShader(program, this.compile(gl.VERTEX_SHADER, VERTEX_SOURCE));
    gl.attachShader(program, this.compile(gl.FRAGMENT_SHADER, FRAGMENT_SOURCE));
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error(`program: ${gl.getProgramInfoLog(program)}`);
    this.program = program;
    const u = (name: string) => gl.getUniformLocation(program, name);
    this.uniforms = {
      viewProjection: u("uViewProjection"), model: u("uModel"), palette: u("uPalette"), paletteOffset: u("uPaletteOffset"), base: u("uBase"), emissiveUnlit: u("uEmissiveUnlit"), surface: u("uSurface"),
      eye: u("uEye"), lightDirection: u("uLightDirection"), lightColor: u("uLightColor"), sky: u("uSky"), ground: u("uGround"),
    };
    this.vao = null;
    this.buffers = [];
    this.palette = null;
  }

  private upload(scene: Scene): void {
    const gl = this.gl;
    for (const b of this.buffers) gl.deleteBuffer(b);
    this.buffers = [];
    if (this.vao) gl.deleteVertexArray(this.vao);
    if (this.palette) gl.deleteTexture(this.palette);
    this.vao = gl.createVertexArray();
    gl.bindVertexArray(this.vao);
    const make = (target: number, data: ArrayBufferView): WebGLBuffer => {
      const b = gl.createBuffer()!;
      gl.bindBuffer(target, b);
      gl.bufferData(target, data, gl.STATIC_DRAW);
      this.buffers.push(b);
      return b;
    };
    make(gl.ARRAY_BUFFER, scene.vertices);
    const bytes = VERTEX_STRIDE * 4;
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 3, gl.FLOAT, false, bytes, 0);
    gl.enableVertexAttribArray(1);
    gl.vertexAttribPointer(1, 3, gl.FLOAT, false, bytes, 12);
    gl.enableVertexAttribArray(2);
    gl.vertexAttribPointer(2, 4, gl.FLOAT, false, bytes, 24);
    if (scene.skinning) {
      make(gl.ARRAY_BUFFER, scene.skinning);
      const sb = SKIN_STRIDE * 4;
      gl.enableVertexAttribArray(3);
      gl.vertexAttribPointer(3, 4, gl.FLOAT, false, sb, 0);
      gl.enableVertexAttribArray(4);
      gl.vertexAttribPointer(4, 4, gl.FLOAT, false, sb, 16);
    } else {
      gl.disableVertexAttribArray(3);
      gl.vertexAttrib4f(3, 0, 0, 0, 0);
      gl.disableVertexAttribArray(4);
      gl.vertexAttrib4f(4, 0, 0, 0, 0);
    }
    make(gl.ELEMENT_ARRAY_BUFFER, scene.indices);
    gl.bindVertexArray(null);
    this.paletteRows = Math.max(scene.jointCount, 1);
    this.palette = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, this.palette);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA32F, 4, this.paletteRows, 0, gl.RGBA, gl.FLOAT, new Float32Array(this.paletteRows * 16));
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  }

  load(scene: Scene): void {
    this.scene = scene;
    if (!this.lost) this.upload(scene);
  }

  resize(cssWidth: number, cssHeight: number, pixelRatio: number): void {
    const w = Math.max(1, Math.round(cssWidth * pixelRatio)), h = Math.max(1, Math.round(cssHeight * pixelRatio));
    if (this.canvas.width !== w) this.canvas.width = w;
    if (this.canvas.height !== h) this.canvas.height = h;
  }

  render(frame: Frame, style: RenderStyle, operation: Operation = silentOperation): void {
    const gl = this.gl;
    if (this.lost || gl.isContextLost() || !this.scene || !this.vao) return;
    const pace = new Pace(operation, 256);
    pace.check();
    const scene = this.scene, u = this.uniforms, bg = style.background, m = frame.materials, light = frame.light;
    gl.viewport(0, 0, this.canvas.width, this.canvas.height);
    gl.clearColor(bg[0] * bg[3], bg[1] * bg[3], bg[2] * bg[3], bg[3]);
    gl.clearDepth(1);
    gl.depthMask(true);
    gl.disable(gl.BLEND);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.useProgram(this.program);
    gl.bindVertexArray(this.vao);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, this.palette);
    if (scene.jointCount > 0) gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 4, scene.jointCount, gl.RGBA, gl.FLOAT, frame.palette);
    gl.uniform1i(u.palette, 0);
    gl.uniformMatrix4fv(u.viewProjection, false, frame.viewProjection);
    gl.uniform3f(u.eye, frame.eye[0]!, frame.eye[1]!, frame.eye[2]!);
    gl.uniform3f(u.lightDirection, light.direction[0]!, light.direction[1]!, light.direction[2]!);
    gl.uniform3f(u.lightColor, light.color[0]!, light.color[1]!, light.color[2]!);
    gl.uniform3f(u.sky, light.sky[0]!, light.sky[1]!, light.sky[2]!);
    gl.uniform3f(u.ground, light.ground[0]!, light.ground[1]!, light.ground[2]!);
    gl.enable(gl.DEPTH_TEST);
    gl.depthFunc(gl.LEQUAL);
    gl.enable(gl.CULL_FACE);
    gl.cullFace(gl.BACK);
    gl.frontFace(gl.CCW);
    gl.enable(gl.POLYGON_OFFSET_FILL);
    gl.polygonOffset(1, 1);
    let pass = -1, culling = true;
    for (let i = 0; i < frame.drawCount; i++) {
      try {
        pace.tick(1, i / frame.drawCount);
      } catch (error) {
        gl.bindVertexArray(null);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
        throw error;
      }
      const d = frame.draws[i]!, p = scene.primitives[d.primitive]!, o = d.material * MATERIAL_STRIDE;
      if (d.pass !== pass) {
        pass = d.pass;
        if (pass === 1) {
          gl.enable(gl.BLEND);
          gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
          gl.depthMask(false);
        } else if (pass === 2) {
          gl.disable(gl.BLEND);
          gl.depthMask(true);
          gl.disable(gl.POLYGON_OFFSET_FILL);
        }
      }
      const doubleSided = m[o + 10] === 1;
      if (pass !== 2 && culling === doubleSided) {
        if (doubleSided) gl.disable(gl.CULL_FACE);
        else gl.enable(gl.CULL_FACE);
        culling = !doubleSided;
      }
      gl.uniformMatrix4fv(u.model, false, frame.matrices, d.matrixOffset, 16);
      gl.uniform1i(u.paletteOffset, d.paletteOffset);
      gl.uniform4f(u.base, m[o]!, m[o + 1]!, m[o + 2]!, m[o + 3]!);
      gl.uniform4f(u.emissiveUnlit, m[o + 4]!, m[o + 5]!, m[o + 6]!, pass === 2 ? 1 : m[o + 7]!);
      gl.uniform4f(u.surface, m[o + 8]!, m[o + 9]!, m[o + 10]!, m[o + 11]!);
      gl.drawElements(pass === 2 ? gl.LINES : gl.TRIANGLES, p.indexCount, gl.UNSIGNED_INT, p.indexStart * 4);
    }
    gl.bindVertexArray(null);
  }

  readPixels(): Uint8ClampedArray {
    const gl = this.gl, w = this.canvas.width, h = this.canvas.height;
    if (this.pixels.length !== w * h * 4) this.pixels = new Uint8Array(w * h * 4);
    gl.readPixels(0, 0, w, h, gl.RGBA, gl.UNSIGNED_BYTE, this.pixels);
    const out = new Uint8ClampedArray(w * h * 4);
    for (let y = 0; y < h; y++) {
      const s = (h - 1 - y) * w * 4;
      for (let x = 0; x < w * 4; x += 4) {
        const a = this.pixels[s + x + 3]!;
        const k = a > 0 ? 255 / a : 0;
        out[y * w * 4 + x] = this.pixels[s + x]! * k;
        out[y * w * 4 + x + 1] = this.pixels[s + x + 1]! * k;
        out[y * w * 4 + x + 2] = this.pixels[s + x + 2]! * k;
        out[y * w * 4 + x + 3] = a;
      }
    }
    return out;
  }

  dispose(): void {
    this.canvas.removeEventListener("webglcontextlost", this.onLost);
    this.canvas.removeEventListener("webglcontextrestored", this.onRestored);
    const gl = this.gl;
    for (const b of this.buffers) gl.deleteBuffer(b);
    if (this.vao) gl.deleteVertexArray(this.vao);
    if (this.palette) gl.deleteTexture(this.palette);
    if (this.program) gl.deleteProgram(this.program);
    this.buffers = [];
    this.scene = null;
  }
}

/** 🏭️ Creates the WebGL2 backend on `canvas`; throws when WebGL2 is unavailable. `onStatus` reports ready, lost and restored. */
export const createWebGl2Backend = (canvas: HTMLCanvasElement, onStatus?: (status: BackendStatus) => void): RenderBackend => new WebGl2Backend(canvas, onStatus);
