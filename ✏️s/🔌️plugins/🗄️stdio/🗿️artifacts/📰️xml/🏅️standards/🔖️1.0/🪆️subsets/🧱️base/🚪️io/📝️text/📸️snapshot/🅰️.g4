grammar StdioXmlSnapshot;

document   : declaration? preRoot* element misc* EOF ;
declaration: XMLDECL_OPEN VERSION_ATTR ENCODING_ATTR? STANDALONE_ATTR? PI_CLOSE ;
preRoot    : misc | doctype ;
doctype    : DOCTYPE ;
misc       : COMMENT | PI | WS ;

element    : '<' Name attribute* '/>'
           | '<' Name attribute* '>' content '<' '/' Name '>' ;
attribute  : Name '=' AttValue ;
content    : (element | Reference | CDATA_SECT | PI | COMMENT | CharData)* ;

Name       : NameStartChar NameChar* ;
AttValue   : '"' ~["]* '"' | '\'' ~[']* '\'' ;
CharData   : ~[<&]+ ;
CDATA_SECT : '<![CDATA[' .*? ']]>' ;
COMMENT    : '<!--' .*? '-->' ;
PI         : '<?' Name (WS .*?)? '?>' ;
Reference  : '&' Name ';' | '&#' [0-9]+ ';' | '&#x' [0-9a-fA-F]+ ';' ;
XMLDECL_OPEN: '<?xml' ;
PI_CLOSE   : '?>' ;
DOCTYPE    : '<!DOCTYPE' ( '"' ~["]* '"' | '\'' ~[']* '\'' | '[' ( '"' ~["]* '"' | '\'' ~[']* '\'' | ~[\]'"] )* ']' | ~[>\["'] )* '>' ;
VERSION_ATTR: WS 'version' '=' AttValue ;
ENCODING_ATTR: WS 'encoding' '=' AttValue ;
STANDALONE_ATTR: WS 'standalone' '=' AttValue ;
WS         : [ \t\r\n]+ ;

fragment NameStartChar: [:a-zA-Z_] ;
fragment NameChar     : NameStartChar | [-.0-9] ;
