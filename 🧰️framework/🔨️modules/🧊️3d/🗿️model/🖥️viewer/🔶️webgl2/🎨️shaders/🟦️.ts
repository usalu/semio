/** 🎨️ GLSL ES 3.00 sources of the WebGL2 backend: lit PBR-lite surfaces with vertex colours, unlit surfaces and lines, GPU skinning from a float palette texture (4 texels per joint matrix, one joint per row).
 * @see ../🟦️.ts */

/** 🔺️ Vertex stage: rigid (`uPaletteOffset < 0`, model matrix) or skinned (blend of up to four palette matrices). */
export const VERTEX_SOURCE = `#version 300 es
precision highp float;
precision highp int;
precision highp sampler2D;
layout(location = 0) in vec3 aPosition;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec4 aColor;
layout(location = 3) in vec4 aJoints;
layout(location = 4) in vec4 aWeights;
uniform mat4 uViewProjection;
uniform mat4 uModel;
uniform sampler2D uPalette;
uniform int uPaletteOffset;
out vec3 vWorld;
out vec3 vNormal;
out vec4 vColor;
mat4 jointMatrix(float joint) {
  int row = uPaletteOffset + int(joint + 0.5);
  return mat4(texelFetch(uPalette, ivec2(0, row), 0), texelFetch(uPalette, ivec2(1, row), 0), texelFetch(uPalette, ivec2(2, row), 0), texelFetch(uPalette, ivec2(3, row), 0));
}
void main() {
  vec4 world;
  vec3 normal;
  if (uPaletteOffset >= 0) {
    mat4 skin = aWeights.x * jointMatrix(aJoints.x) + aWeights.y * jointMatrix(aJoints.y) + aWeights.z * jointMatrix(aJoints.z) + aWeights.w * jointMatrix(aJoints.w);
    world = skin * vec4(aPosition, 1.0);
    normal = mat3(skin) * aNormal;
  } else {
    world = uModel * vec4(aPosition, 1.0);
    normal = transpose(inverse(mat3(uModel))) * aNormal;
  }
  vWorld = world.xyz;
  vNormal = normal;
  vColor = aColor;
  gl_Position = uViewProjection * world;
}
`;

/** 🖌️ Fragment stage: base colour x vertex colour, hemisphere ambient, one directional light, Blinn specular scaled by `(1 - roughness)^2`, emissive, double-sided normal flip, linear to sRGB (gamma 2.2), premultiplied output. */
export const FRAGMENT_SOURCE = `#version 300 es
precision highp float;
in vec3 vWorld;
in vec3 vNormal;
in vec4 vColor;
uniform vec4 uBase;
uniform vec4 uEmissiveUnlit;
uniform vec4 uSurface;
uniform vec3 uEye;
uniform vec3 uLightDirection;
uniform vec3 uLightColor;
uniform vec3 uSky;
uniform vec3 uGround;
out vec4 outColor;
void main() {
  vec4 albedo = uBase * vColor;
  float alpha = uSurface.w > 0.5 ? albedo.a : 1.0;
  vec3 color;
  if (uEmissiveUnlit.w > 0.5) {
    color = albedo.rgb + uEmissiveUnlit.rgb;
  } else {
    vec3 n = normalize(vNormal);
    if (!gl_FrontFacing && uSurface.z > 0.5) n = -n;
    vec3 v = normalize(uEye - vWorld);
    float metallic = uSurface.x;
    float shininess = mix(8.0, 128.0, 1.0 - uSurface.y);
    float ndl = max(dot(n, uLightDirection), 0.0);
    vec3 ambient = mix(uGround, uSky, n.y * 0.5 + 0.5);
    vec3 h = normalize(uLightDirection + v);
    float spec = pow(max(dot(n, h), 0.0), shininess) * ndl * (1.0 - uSurface.y) * (1.0 - uSurface.y);
    vec3 f0 = mix(vec3(0.04), albedo.rgb, metallic);
    color = albedo.rgb * (1.0 - metallic * 0.8) * (ambient + uLightColor * ndl) + f0 * spec + uEmissiveUnlit.rgb;
  }
  color = pow(clamp(color, 0.0, 1.0), vec3(1.0 / 2.2));
  outColor = vec4(color * alpha, alpha);
}
`;
