grammar Stdio_gltf_diff;
document: clause* EOF;
clause: field '=' TOKEN;
field: 'asset' | 'scene' | 'scenes' | 'nodes' | 'meshes' | 'accessors' | 'bufferViews' | 'buffers' | 'bufferBytes' | 'materials' | 'textures' | 'images' | 'samplers' | 'skins' | 'animations' | 'cameras' | 'extensionsUsed' | 'extensionsRequired' | 'extensions' | 'extras' | 'sourceForm';
TOKEN: ~[= \t\n\r]+;
WS: [ \t]+ -> skip;
