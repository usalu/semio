\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{semio}{2026/07/09}{0.2.0}{semio printable documents}

\RequirePackage{expl3}
\RequirePackage{xparse}
\RequirePackage{kvoptions}

\SetupKeyvalOptions{
  family=semio,
  prefix=semio@,
  setkeys=\kvsetkeys
}

\DeclareStringOption[report]{type}
\DeclareStringOption[light]{theme}
\DeclareStringOption[]{language}
\DeclareStringOption[numeric]{citestyle}
\DeclareStringOption[]{bindingoffset}
\ProcessKeyvalOptions*

\ExplSyntaxOn
\clist_if_in:nVTF { assignment, thesis, phd } \semio@type {
  \tl_if_blank:VTF \semio@language
    { \ClassError{semio}{Document~language~is~required}{Select~language=en~or~language=de.} }
    { \str_case:VnF \semio@language { { en } {} { de } {} }
        { \ClassError{semio}{Unsupported~document~language}{Select~language=en~or~language=de.} } }
} {
  \tl_if_blank:VT \semio@language { \tl_set:Nn \semio@language { de } }
}
\ExplSyntaxOff

\ExplSyntaxOn
\tl_if_blank:VT \semio@bindingoffset {
  \str_if_eq:VnTF \semio@type { phd }
    { \tl_set:Nn \semio@bindingoffset { 15mm } }
    { \tl_set:Nn \semio@bindingoffset { 8mm } }
}
\str_case:VnF \semio@type {
  { assignment } {
    \providecommand { \semio@page@margin } { 2cm }
    \RequirePackage[paper=a4paper,margin=2cm]{geometry}
  }
  { flyer } {
    \providecommand { \semio@page@margin } { 1.2cm }
    \RequirePackage[paper=a4paper,landscape,margin=1.2cm]{geometry}
  }
  { paper } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=\semio@bindingoffset]{geometry}
  }
  { report } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=\semio@bindingoffset]{geometry}
  }
  % 🎓 A dissertation is archived as a bound hardcover copy, so its gutter is
  % wider than the 8mm every stapled or glued semio document uses.
  { thesis } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=\semio@bindingoffset]{geometry}
  }
  { phd } {
    \providecommand { \semio@page@margin } { 2.5cm }
    \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=\semio@bindingoffset]{geometry}
  }
} {
  \providecommand { \semio@page@margin } { 2.5cm }
  \RequirePackage[paper=a4paper,margin=2.5cm,twoside,bindingoffset=\semio@bindingoffset]{geometry}
}
\str_case:VnF \semio@type {
  { assignment } { \LoadClass[twoside=false]{scrartcl} }
  { report } { \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt} }
  { thesis } { \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt} }
  { phd } { \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt} }
  { paper } { \LoadClass[twoside=true,open=any]{scrartcl} }
  { flyer } { \LoadClass{scrartcl} }
} {
  \LoadClass[twoside=true,open=right,cleardoublepage=empty]{scrreprt}
}
\ExplSyntaxOff

\RequirePackage{fontspec}
\RequirePackage{semio-tokens}
\RequirePackage{semio-fonts}
\RequirePackage{semio-core}
\RequirePackage{semio-logo}
\RequirePackage{semio-table}

\ExplSyntaxOn
\keys_set:nx { semio / setup } {
  type = \semio@type,
  theme = \semio@theme,
  language = \semio@language,
}
\str_case:VnF \semio@language {
  { de } { \PassOptionsToPackage { main = ngerman } { babel } }
} { }
\ExplSyntaxOff

\RequirePackage[ngerman,english]{babel}
\RequirePackage{microtype}

%region Justification Safety
% 🧵 German compound words carry hard hyphens (e.g. "material- und
% anwendungsbezogen") that are the paragraph's only nearby break point; without
% slack a line ending there can overshoot \linewidth instead of breaking.
% emergencystretch lets justification loosen such lines rather than overflow.
\tolerance=1000
\emergencystretch=3em
%endregion Justification Safety

\RequirePackage[style=\semio@citestyle,backend=bibtex,sorting=nyt,backref=true]{biblatex}
\RequirePackage{semio-window}
\RequirePackage{semio-components}
\RequirePackage{semio-phd}
\RequirePackage{semio-assignment}
\RequirePackage{semio-graph}
\RequirePackage{semio-tree}
\RequirePackage{semio-viz}
\RequirePackage{hyperref}
\hypersetup{colorlinks,linkcolor=semio-chrome-foreground,citecolor=semio-chrome-foreground,urlcolor=semio-chrome-foreground,filecolor=semio-chrome-foreground}
