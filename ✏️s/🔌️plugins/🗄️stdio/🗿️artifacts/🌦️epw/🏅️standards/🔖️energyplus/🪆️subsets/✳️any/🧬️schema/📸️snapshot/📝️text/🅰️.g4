// 🅰️ ANTLR grammar for the real EnergyPlus Weather File (EPW) wire format itself (not a
// serialization of the Rust snapshot struct — the snapshot's typed fields ARE this grammar's parse).
// https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html
grammar Stdio_epw_snapshot;

file    : location CRLF designConditions CRLF typicalExtremePeriods CRLF groundTemperatures CRLF
          holidaysDst CRLF comments1 CRLF comments2 CRLF dataPeriods CRLF record+ EOF ;

location               : 'LOCATION' (COMMA field){9} ;
designConditions       : 'DESIGN CONDITIONS' (COMMA field)* ;
typicalExtremePeriods  : 'TYPICAL/EXTREME PERIODS' (COMMA field)* ;
groundTemperatures     : 'GROUND TEMPERATURES' (COMMA field)* ;
holidaysDst            : 'HOLIDAYS/DAYLIGHT SAVINGS' (COMMA field)* ;
comments1              : 'COMMENTS 1' (COMMA field)* ;
comments2              : 'COMMENTS 2' (COMMA field)* ;
dataPeriods            : 'DATA PERIODS' COMMA field COMMA field (COMMA field COMMA field COMMA field COMMA field)* ;
record                 : field (COMMA field){34} CRLF ;   // exactly 35 columns

COMMA   : ',' ;
CRLF    : '\r\n' | '\n' ;
field   : FIELDDATA? ;
FIELDDATA : ~[,\r\n]+ ;
