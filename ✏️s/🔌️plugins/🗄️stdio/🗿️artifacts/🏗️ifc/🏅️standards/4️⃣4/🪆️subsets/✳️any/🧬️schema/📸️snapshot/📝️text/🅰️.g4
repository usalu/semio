grammar StdioIfcSnapshot;

document: 'semio' 'stdio.ifc.dsl' 'v1' frame EOF;
frame: 'schema' '=' text 'file-description' '=' indices 'file-name' '=' indices 'file-schema' '=' indices 'entities' '=' '[' entity* ']' 'values' '=' '[' node* ']';
entity: '{' 'id' '=' INT 'name' '=' text 'arguments' '=' indices 'complex' '=' '[' complex* ']' '}';
complex: '{' 'name' '=' text 'arguments' '=' indices '}';
node: '{' 'kind' '=' text ('integer' '=' INT)? ('real' '=' number)? ('text' '=' text)? ('reference' '=' INT)? ('name' '=' text)? 'children' '=' indices '}';
indices: '[' INT* ']';
text: TEXT | IDENT;
number: FLOAT | INT | IEEE;
IEEE: '-inf' | 'inf' | 'nan64_' HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX HEX;
FLOAT: [+-]? [0-9]+ ('.' [0-9]* [Ee] [+-]? [0-9]+ | '.' [0-9]* | [Ee] [+-]? [0-9]+);
INT: [+-]? [0-9]+;
TEXT: '"' ('\\' . | ~["\\])* '"';
IDENT: [A-Za-z_] [A-Za-z0-9_.:/-]*;
fragment HEX: [0-9a-fA-F];
WS: [ \t\r\n]+ -> skip;
