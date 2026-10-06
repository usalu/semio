grammar Pdf14Mutations;
op: ('insert-page' | 'remove-page' | 'move-page' | 'resize-page' | 'replace-page-text') ' payload=' HEX EOF;
HEX: ([0-9a-fA-F] [0-9a-fA-F])+;
