\NeedsTeXFormat{LaTeX2e}
\ProvidesExplClass{zukunftbau}{2026/07/06}{0.1.0}{Zukunft Bau report family}

\RequirePackage{kvoptions}

\SetupKeyvalOptions{
  family=zukunftbau,
  prefix=zukunftbau@,
  setkeys=\kvsetkeys
}

\DeclareStringOption[forschungsbericht]{type}
\DeclareStringOption[light]{theme}
\ProcessKeyvalOptions*

\ExplSyntaxOn
\str_case:VnF \zukunftbau@type {
  { forschungsbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=report,citestyle=numeric]{semio}
  }
  { zwischenbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=paper,citestyle=numeric]{semio}
    \SemioSetup{appendix-level=part}
  }
  { kompaktbericht } {
    \LoadClass[language=de,theme=\zukunftbau@theme,type=paper,citestyle=authoryear]{semio}
    \SemioSetup{kompakt-draft=true}
  }
} {
  \LoadClass[language=de,theme=\zukunftbau@theme,type=report]{semio}
}
\ExplSyntaxOff

%region CoverAssets
\makeatletter
\IfFileExists{asset/logo/zukunft-bau/logo.png}{%
  \ifdefstring{\zukunftbau@theme}{dark}{%
    \renewcommand{\semio@cover@zukunftbau@logo}{asset/logo/zukunft-bau/logo-dark.png}%
    \renewcommand{\semio@cover@bbsr@logo}{asset/logo/bbsr/logo-dark.png}%
    \renewcommand{\semio@cover@bundesministerium@logo}{asset/logo/bmwsb/logo-dark.png}%
  }{%
    \renewcommand{\semio@cover@zukunftbau@logo}{asset/logo/zukunft-bau/logo.png}%
    \renewcommand{\semio@cover@bbsr@logo}{asset/logo/bbsr/logo.png}%
    \renewcommand{\semio@cover@bundesministerium@logo}{asset/logo/bmwsb/logo.png}%
  }%
}{}
\IfFileExists{asset/demonstrator/aggegator.png}{%
  \renewcommand{\semio@cover@arbeitsprobe@image}{asset/demonstrator/aggegator.png}%
}{}
\makeatother
%endregion
