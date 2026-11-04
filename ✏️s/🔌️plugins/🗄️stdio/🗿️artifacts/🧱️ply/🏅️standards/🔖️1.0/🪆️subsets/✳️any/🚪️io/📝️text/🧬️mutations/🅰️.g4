grammar Stdio_ply_mutations;

op: setFormat | insertComment | removeComment | addElement | removeElement | insertRow | removeRow | setRowProperty;

setFormat: 'set-format' ' ' 'format=' FORMAT_TAG;
insertComment: 'insert-comment' ' ' 'index=' INT ' ' 'comment=' HEX;
removeComment: 'remove-comment' ' ' 'index=' INT;
addElement: 'add-element' ' ' 'index=' INT ' ' 'element=' elementValue;
removeElement: 'remove-element' ' ' 'name=' HEX;
insertRow: 'insert-row' ' ' 'element-name=' HEX ' ' 'index=' INT ' ' 'row=' rowValue;
removeRow: 'remove-row' ' ' 'element-name=' HEX ' ' 'index=' INT;
setRowProperty: 'set-row-property' ' ' 'element-name=' HEX ' ' 'row-index=' INT ' ' 'property-name=' HEX ' ' 'value=' plyValue;

elementValue: '[' HEX ',' INT ',' '[' propertyList? ']' ',' '[' rowList? ']' ']';
propertyList: plyProperty (',' plyProperty)*;
plyProperty: 'S[' HEX ',' SCALAR_KIND ']' | 'L[' HEX ',' SCALAR_KIND ',' SCALAR_KIND ']';
rowList: rowValue (',' rowValue)*;
rowValue: '[' valueList? ']';
valueList: plyValue (',' plyValue)*;
plyValue: INTEGER_TAG '[' INT ']' | REAL_TAG '[' number ']' | 'L[' valueList? ']';
number: INT | FLOAT;

FORMAT_TAG: [alb];
SCALAR_KIND: [cCswiufd];
INTEGER_TAG: [cCswiu];
REAL_TAG: [fd];
INT: '-'? [0-9]+;
FLOAT: INT '.' [0-9]+ ([eE] [+\-]? [0-9]+)?;
HEX: [0-9a-f]*;
