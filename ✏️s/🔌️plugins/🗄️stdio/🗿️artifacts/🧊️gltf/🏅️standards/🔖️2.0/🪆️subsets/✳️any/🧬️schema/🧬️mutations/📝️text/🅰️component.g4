grammar Stdio_gltf_mutations;
document: operation EOF;
operation: keyword argument*;
keyword: 'no-mutation' | 'set-snapshot' | 'set-asset' | 'insert-scene' | 'remove-scene' | 'set-scene' | 'insert-node' | 'remove-node' | 'set-node' | 'transform-node' | 'reparent-node' | 'bind-node-mesh' | 'insert-mesh' | 'remove-mesh' | 'set-mesh' | 'insert-accessor' | 'remove-accessor' | 'set-accessor' | 'insert-material' | 'remove-material' | 'set-material' | 'bind-primitive-material' | 'insert-buffer' | 'remove-buffer' | 'set-buffer' | 'insert-animation' | 'remove-animation' | 'set-animation';
argument: TOKEN '=' TOKEN;
TOKEN: ~[= \t\n\r]+;
WS: [ \t]+ -> skip;
