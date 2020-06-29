<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

 <a href="http://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.svg" />
 </a>

This repository lists **static analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings and user comments for each tool.

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

> The most important thing I have done as a programmer in recent years is to aggressively pursue static code analysis. Even more valuable than the hundreds of serious bugs I have prevented with it is the change in mindset about the way I view software reliability and code quality. — [John Carmack (Creator of Doom)](https://www.gamasutra.com/view/news/128836/InDepth_Static_Code_Analysis.php) 

![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)

Meaning of symbols:  

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 6 months, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

# Table of Contents

#### [Programming Languages](#programming-languages-1)

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    <li><a href="#abap">ABAP</a></li>
    <li><a href="#ada">Ada</a></li>
    <li><a href="#asm">Assembly</a></li>
    <li><a href="#awk">Awk</a></li>
    <li><a href="#c">C</a></li>
    <li><a href="#csharp">C#</a></li>
    <li><a href="#cpp">C++</a></li>
    <li><a href="#coffeescript">CoffeeScript</a></li>
    <li><a href="#crystal">Crystal</a></li>
    <li><a href="#dart">Dart</a></li>
    <li><a href="#delphi">Delphi</a></li>
    <li><a href="#deno">Deno</a></li>
    <li><a href="#dlang">Dlang</a></li>
    <li><a href="#elixir">Elixir</a></li>
    <li><a href="#elm">Elm</a></li>
    <li><a href="#erlang">Erlang</a></li>
    <li><a href="#fsharp">F#</a></li>
    <li><a href="#fortran">Fortran</a></li>
    <li><a href="#go">Go</a></li>
    <li><a href="#groovy">Groovy</a></li>
    <li><a href="#haskell">Haskell</a></li>
    <li><a href="#haxe">Haxe</a></li>
    <li><a href="#java">Java</a></li>
    <li><a href="#javascript">JavaScript</a></li>
    <li><a href="#kotlin">Kotlin</a></li>
    <li><a href="#lua">Lua</a></li>
    <li><a href="#matlab">MATLAB</a></li>
    <li><a href="#php">PHP</a></li>
    <li><a href="#perl">Perl</a></li>
    <li><a href="#python">Python</a></li>
    <li><a href="#r">R</a></li>
    <li><a href="#rpg">RPG</a></li>
    <li><a href="#ruby">Ruby</a></li>
    <li><a href="#rust">Rust</a></li>
    <li><a href="#sql">SQL</a></li>
    <li><a href="#scala">Scala</a></li>
    <li><a href="#shell">Shell</a></li>
    <li><a href="#solidity">Solidity</a></li>
    <li><a href="#swift">Swift</a></li>
    <li><a href="#tcl">Tcl</a></li>
    <li><a href="#typescript">TypeScript</a></li>
    <li><a href="#vbscript">VBScript</a></li>
    </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

- [Binaries](#binary)
- [Build tools](#buildtool)
- [CSS/SASS/SCSS](#css)
- [Config Files](#configfile)
- [Configuration Management](#configmanagement)
- [Containers](#container)
- [Gherkin](#gherkin)
- [HTML](#html)
- [IDE Plugins](#ide)
- [LaTeX](#latex)
- [Makefiles](#make)
- [Markdown](#markdown)
- [Mobile](#mobile)
- [Packages](#package)
- [Protocol Buffers](#protobuf)
- [Supporting Tools](#support)
- [Template-Languages](#template)
- [Translation](#translation)
- [Web services](#service)
- [Writing](#writing)


---

# Programming Languages

<h2 id="abap">ABAP</h2>

- ![stars](https://img.shields.io/github/stars/larshp/abaplint?style=flat-square&color=ccc) [abaplint](https://abaplint.org/) - Linter for ABAP, written in TypeScript.
- ![stars](https://img.shields.io/github/stars/larshp/abapOpenChecks?style=flat-square&color=ccc) [abapOpenChecks](https://docs.abapopenchecks.org/) - Enhances the SAP Code Inspector with new and customizable checks.


<h2 id="ada">Ada</h2>

- [Codepeer](http://www.adacore.com/codepeer) :copyright: - detects run-time and logic errors.
- [Polyspace for Ada](https://www.mathworks.com/products/polyspace-ada.html) :copyright: - provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in source code.
- [SPARK](https://www.adacore.com/about-spark) :copyright: - Static analysis and formal verification toolset for Ada.
- [Understand](https://scitools.com/ada-programming-essential/) :copyright: - IDE that provides code analysis, standards testing, metrics, graphing, dependency analysis and more for Ada and VHDL.


<h2 id="asm">Assembly</h2>

- ![stars](https://img.shields.io/github/stars/StanfordPL/stoke?style=flat-square&color=ccc) [STOKE](http://stoke.stanford.edu/) - a programming-language agnostic stochastic optimizer for the x86_64 instruction set. It uses random search to explore the extremely high-dimensional space of all possible program transformations.


<h2 id="awk">Awk</h2>

- [gawk --lint](https://www.gnu.org/software/gawk/manual/html_node/Options.html) - warns about constructs that are dubious or nonportable to other awk implementations.


<h2 id="c">C</h2>

- [Astrée](https://www.absint.com/astree/index.htm) :copyright: - Sound static analyzer based on abstract interpretation for C/C++, detecting memory, type and concurrency defects, and MISRA violations.
- ![stars](https://img.shields.io/github/stars/diffblue/cbmc?style=flat-square&color=ccc) [CBMC](http://www.cprover.org/cbmc/) - bounded model-checker for C programs, user-defined assertions, standard assertions, several coverage metric analyses.
- [clang-tidy](http://clang.llvm.org/extra/clang-tidy/) - clang static analyser.
- ![stars](https://img.shields.io/github/stars/MetricsGrimoire/CMetrics?style=flat-square&color=ccc) [CMetrics](https://github.com/MetricsGrimoire/CMetrics) - Measures size and complexity for C files.
- [CodeSonar from GrammaTech](https://www.grammatech.com/products/codesonar) :copyright: - Advanced, whole program, deep path, static analysis of C and C++ with easy-to-understand explanations and code and path visualization.
- ![stars](https://img.shields.io/github/stars/danmar/cppcheck?style=flat-square&color=ccc) [cppcheck](http://cppcheck.sourceforge.net/) - static analysis of C/C++ code.
- [CppDepend](https://www.cppdepend.com) :warning: :copyright: - Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [cpplint](https://github.com/google/styleguide/tree/gh-pages/cpplint) - automated C++ checker that follows Google's style guide.
- ![stars](https://img.shields.io/github/stars/dspinellis/cqmetrics?style=flat-square&color=ccc) [cqmetrics](https://github.com/dspinellis/cqmetrics) - quality metrics for C code.
- ![stars](https://img.shields.io/github/stars/dspinellis/cscout?style=flat-square&color=ccc) [CScout](https://www.spinellis.gr/cscout/) - complexity and quality metrics for for C and C preprocessor code.
- ![stars](https://img.shields.io/github/stars/david-a-wheeler/flawfinder?style=flat-square&color=ccc) [flawfinder](https://www.dwheeler.com/flawfinder/) - finds possible security weaknesses.
- ![stars](https://img.shields.io/github/stars/JossWhittle/FlintPlusPlus?style=flat-square&color=ccc) [flint++](https://github.com/JossWhittle/FlintPlusPlus) - cross-platform, zero-dependency port of flint, a lint program for C++ developed and used at Facebook.
- [Frama-C](http://frama-c.com/) - a sound and extensible static analyzer for C code.
- [Helix QAC](https://www.perforce.com/products/helix-qac) :copyright: - Enterprise-grade static analysis for embedded software. Supports MISRA, CERT, and AUTOSAR coding standards.
- ![stars](https://img.shields.io/github/stars/nasa-sw-vnv/ikos?style=flat-square&color=ccc) [IKOS](https://github.com/nasa-sw-vnv/ikos) - a sound static analyzer for C/C++ code based on LLVM.
- ![stars](https://img.shields.io/github/stars/feddischson/include_gardener?style=flat-square&color=ccc) [include-gardener](https://github.com/feddischson/include_gardener) - a multi-language static analyzer for C/C++/Obj-C/Python/Ruby to create a graph (in dot or graphml format) which shows all `#include` relations of a given set of files.
- [LDRA](https://ldra.com) :copyright: - a tool suite including static analysis (TBVISION) to various standards including MISRA C & C++, JSF++ AV, CWE, CERT C, CERT C++ & Custom Rules.
- ![stars](https://img.shields.io/github/stars/secure-software-engineering/phasar?style=flat-square&color=ccc) [Phasar](https://phasar.org/) - A LLVM-based static analysis framework which comes with a taint and type state analysis.
- [Polyspace Bug Finder](https://www.mathworks.com/products/polyspace-bug-finder.html) :copyright: - Identifies run-time errors, concurrency issues, security vulnerabilities, and other defects in C and C++ embedded software.
- [Polyspace Code Prover](https://www.mathworks.com/products/polyspace-code-prover.html) :copyright: - Provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in C and C++ source code.
- [scan-build](https://clang-analyzer.llvm.org/scan-build.html) - Analyzes C/C++ code using LLVM at compile-time.
- ![stars](https://img.shields.io/github/stars/ravenexp/splint?style=flat-square&color=ccc) [splint](http://splint.org/) - Annotation-assisted static program checker.
- ![stars](https://img.shields.io/github/stars/SVF-tools/SVF?style=flat-square&color=ccc) [SVF](http://svf-tools.github.io/SVF/) - a static tool that enables scalable and precise interprocedural dependence analysis for C and C++ programs.
- [vera++](https://bitbucket.org/verateam/vera/wiki/Introduction) - Vera++ is a programmable tool for verification, analysis and transformation of C++ source code.


<h2 id="csharp">C#</h2>

- [.NET Analyzers](https://github.com/DotNetAnalyzers) - An organization for the development of analyzers (diagnostics and code fixes) using the .NET Compiler Platform.
- [Code Analysis Rule Collection](https://carc.codeplex.com/) :warning: - Contains a set of diagnostics, code fixes and refactorings built on the Microsoft .NET Compiler Platform "Roslyn".
- ![stars](https://img.shields.io/github/stars/code-cracker/code-cracker?style=flat-square&color=ccc) [code-cracker](http://code-cracker.github.io/) - An analyzer library for C# and VB that uses Roslyn to produce refactorings, code analysis, and other niceties.
- ![stars](https://img.shields.io/github/stars/DustinCampbell/CSharpEssentials?style=flat-square&color=ccc) [CSharpEssentials](https://github.com/DustinCampbell/CSharpEssentials) - C# Essentials is a collection of Roslyn diagnostic analyzers, code fixes and refactorings that make it easy to work with C# 6 language features.
- [Designite](http://www.designite-tools.com) :copyright: - Designite supports detection of various architecture, design, and implementation smells, computation of various code quality metrics, and trend analysis.
- ![stars](https://img.shields.io/github/stars/mono/mono-tools?style=flat-square&color=ccc) [Gendarme](https://www.mono-project.com/docs/tools+libraries/tools/gendarme/) - Gendarme inspects programs and libraries that contain code in ECMA CIL format (Mono and .NET).
- [NDepend](http://www.ndepend.com/) :copyright: - Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- ![stars](https://img.shields.io/github/stars/JosefPihrt/Roslynator?style=flat-square&color=ccc) [Roslynator](https://github.com/JosefPihrt/Roslynator/) - A collection of 190+ analyzers and 190+ refactorings for C#, powered by Roslyn.
- ![stars](https://img.shields.io/github/stars/Vannevelj/VSDiagnostics?style=flat-square&color=ccc) [VSDiagnostics](https://github.com/Vannevelj/VSDiagnostics) - A collection of static analyzers based on Roslyn that integrates with VS.
- ![stars](https://img.shields.io/github/stars/Wintellect/Wintellect.Analyzers?style=flat-square&color=ccc) [Wintellect.Analyzers](https://github.com/Wintellect/Wintellect.Analyzers) - .NET Compiler Platform ("Roslyn") diagnostic analyzers and code fixes.


<h2 id="cpp">C++</h2>

- [Astrée](https://www.absint.com/astree/index.htm) :copyright: - Sound static analyzer based on abstract interpretation for C/C++, detecting memory, type and concurrency defects, and MISRA violations.
- ![stars](https://img.shields.io/github/stars/diffblue/cbmc?style=flat-square&color=ccc) [CBMC](http://www.cprover.org/cbmc/) - bounded model-checker for C programs, user-defined assertions, standard assertions, several coverage metric analyses.
- [clang-tidy](http://clang.llvm.org/extra/clang-tidy/) - clang static analyser.
- ![stars](https://img.shields.io/github/stars/MetricsGrimoire/CMetrics?style=flat-square&color=ccc) [CMetrics](https://github.com/MetricsGrimoire/CMetrics) - Measures size and complexity for C files.
- [CodeSonar from GrammaTech](https://www.grammatech.com/products/codesonar) :copyright: - Advanced, whole program, deep path, static analysis of C and C++ with easy-to-understand explanations and code and path visualization.
- ![stars](https://img.shields.io/github/stars/danmar/cppcheck?style=flat-square&color=ccc) [cppcheck](http://cppcheck.sourceforge.net/) - static analysis of C/C++ code.
- [CppDepend](https://www.cppdepend.com) :warning: :copyright: - Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [cpplint](https://github.com/google/styleguide/tree/gh-pages/cpplint) - automated C++ checker that follows Google's style guide.
- ![stars](https://img.shields.io/github/stars/dspinellis/cqmetrics?style=flat-square&color=ccc) [cqmetrics](https://github.com/dspinellis/cqmetrics) - quality metrics for C code.
- ![stars](https://img.shields.io/github/stars/dspinellis/cscout?style=flat-square&color=ccc) [CScout](https://www.spinellis.gr/cscout/) - complexity and quality metrics for for C and C preprocessor code.
- ![stars](https://img.shields.io/github/stars/david-a-wheeler/flawfinder?style=flat-square&color=ccc) [flawfinder](https://www.dwheeler.com/flawfinder/) - finds possible security weaknesses.
- ![stars](https://img.shields.io/github/stars/JossWhittle/FlintPlusPlus?style=flat-square&color=ccc) [flint++](https://github.com/JossWhittle/FlintPlusPlus) - cross-platform, zero-dependency port of flint, a lint program for C++ developed and used at Facebook.
- [Frama-C](http://frama-c.com/) - a sound and extensible static analyzer for C code.
- [Helix QAC](https://www.perforce.com/products/helix-qac) :copyright: - Enterprise-grade static analysis for embedded software. Supports MISRA, CERT, and AUTOSAR coding standards.
- ![stars](https://img.shields.io/github/stars/nasa-sw-vnv/ikos?style=flat-square&color=ccc) [IKOS](https://github.com/nasa-sw-vnv/ikos) - a sound static analyzer for C/C++ code based on LLVM.
- ![stars](https://img.shields.io/github/stars/feddischson/include_gardener?style=flat-square&color=ccc) [include-gardener](https://github.com/feddischson/include_gardener) - a multi-language static analyzer for C/C++/Obj-C/Python/Ruby to create a graph (in dot or graphml format) which shows all `#include` relations of a given set of files.
- [LDRA](https://ldra.com) :copyright: - a tool suite including static analysis (TBVISION) to various standards including MISRA C & C++, JSF++ AV, CWE, CERT C, CERT C++ & Custom Rules.
- ![stars](https://img.shields.io/github/stars/secure-software-engineering/phasar?style=flat-square&color=ccc) [Phasar](https://phasar.org/) - A LLVM-based static analysis framework which comes with a taint and type state analysis.
- [Polyspace Bug Finder](https://www.mathworks.com/products/polyspace-bug-finder.html) :copyright: - Identifies run-time errors, concurrency issues, security vulnerabilities, and other defects in C and C++ embedded software.
- [Polyspace Code Prover](https://www.mathworks.com/products/polyspace-code-prover.html) :copyright: - Provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in C and C++ source code.
- [scan-build](https://clang-analyzer.llvm.org/scan-build.html) - Analyzes C/C++ code using LLVM at compile-time.
- ![stars](https://img.shields.io/github/stars/ravenexp/splint?style=flat-square&color=ccc) [splint](http://splint.org/) - Annotation-assisted static program checker.
- ![stars](https://img.shields.io/github/stars/SVF-tools/SVF?style=flat-square&color=ccc) [SVF](http://svf-tools.github.io/SVF/) - a static tool that enables scalable and precise interprocedural dependence analysis for C and C++ programs.
- [vera++](https://bitbucket.org/verateam/vera/wiki/Introduction) - Vera++ is a programmable tool for verification, analysis and transformation of C++ source code.


<h2 id="coffeescript">CoffeeScript</h2>

- ![stars](https://img.shields.io/github/stars/clutchski/coffeelint?style=flat-square&color=ccc) [coffeelint](http://www.coffeelint.org/) - A style checker that helps keep CoffeeScript code clean and consistent.


<h2 id="crystal">Crystal</h2>

- ![stars](https://img.shields.io/github/stars/veelenga/ameba?style=flat-square&color=ccc) [ameba](https://crystal-ameba.github.io/) - A static code analysis tool for Crystal.
- ![stars](https://img.shields.io/github/stars/crystal-lang/crystal?style=flat-square&color=ccc) [crystal](https://crystal-lang.org/) - The Crystal compiler has built-in linting functionality.


<h2 id="dart">Dart</h2>

- ![stars](https://img.shields.io/github/stars/dart-lang/linter?style=flat-square&color=ccc) [Linter for dart](https://dart-lang.github.io/linter/) - Style linter for Dart.


<h2 id="delphi">Delphi</h2>

- [Fix Insight](https://www.tmssoftware.com/site/fixinsight.asp) :copyright: - A free IDE Plugin for static code analysis. A _Pro_ edition includes a command line tool for automation purposes.
- [Pascal Analyzer](https://peganza.com/products_pal.html) :copyright: - A static code analysis tool with numerous reports. A free _Lite_ version is available with limited reporting.
- [Pascal Expert](https://peganza.com/products_pex.html) :copyright: - IDE plugin for code analysis. Includes a subset of Pascal Analyzer reporting capabilities and is available for Delphi versions 2007 and later.


<h2 id="deno">Deno</h2>

- ![stars](https://img.shields.io/github/stars/denoland/deno_lint?style=flat-square&color=ccc) [deno_lint](https://github.com/denoland/deno_lint) - Official linter for Deno.


<h2 id="dlang">Dlang</h2>

- ![stars](https://img.shields.io/github/stars/dlang-community/D-Scanner?style=flat-square&color=ccc) [D-scanner](https://github.com/dlang-community/D-Scanner) - D-Scanner is a tool for analyzing D source code.


<h2 id="elixir">Elixir</h2>

- ![stars](https://img.shields.io/github/stars/rrrene/credo?style=flat-square&color=ccc) [credo](http://credo-ci.org/) - A static code analysis tool with a focus on code consistency and teaching.
- ![stars](https://img.shields.io/github/stars/nccgroup/sobelow?style=flat-square&color=ccc) [sobelow](https://github.com/nccgroup/sobelow) - Security-focused static analysis for the Phoenix Framework.


<h2 id="elm">Elm</h2>

- ![stars](https://img.shields.io/github/stars/stil4m/elm-analyse?style=flat-square&color=ccc) [elm-analyse](https://stil4m.github.io/elm-analyse/) - A tool that allows you to analyse your Elm code, identify deficiencies and apply best practices.


<h2 id="erlang">Erlang</h2>

- ![stars](https://img.shields.io/github/stars/inaka/elvis?style=flat-square&color=ccc) [elvis](https://github.com/inaka/elvis) - Erlang Style Reviewer.
- ![stars](https://img.shields.io/github/stars/okeuday/pest?style=flat-square&color=ccc) [Primitive Erlang Security Tool (PEST)](https://github.com/okeuday/pest) - A tool to do a basic scan of Erlang source code and report any function calls that may cause Erlang source code to be insecure.


<h2 id="fsharp">F#</h2>

- ![stars](https://img.shields.io/github/stars/fsprojects/FSharpLint?style=flat-square&color=ccc) [FSharpLint](http://fsprojects.github.io/FSharpLint/) - Lint tool for F#.


<h2 id="fortran">Fortran</h2>

- ![stars](https://img.shields.io/github/stars/lequal/i-CodeCNES?style=flat-square&color=ccc) [i-Code CNES for Fortran](https://github.com/lequal/i-CodeCNES) - An open source static code analysis tool for Fortran 77, Fortran 90 and Shell.


<h2 id="go">Go</h2>

- [aligncheck](https://gitlab.com/opennota/check) - Find inefficiently packed structs.
- ![stars](https://img.shields.io/github/stars/timakin/bodyclose?style=flat-square&color=ccc) [bodyclose](https://github.com/timakin/bodyclose) - Checks whether HTTP response body is closed.
- ![stars](https://img.shields.io/github/stars/tsenart/deadcode?style=flat-square&color=ccc) [deadcode](https://github.com/tsenart/deadcode) - Finds unused code.
- ![stars](https://img.shields.io/github/stars/nickng/dingo-hunter?style=flat-square&color=ccc) [dingo-hunter](https://github.com/nickng/dingo-hunter) - Static analyser for finding deadlocks in Go.
- ![stars](https://img.shields.io/github/stars/alexkohler/dogsled?style=flat-square&color=ccc) [dogsled](https://github.com/alexkohler/dogsled) - Finds assignments/declarations with too many blank identifiers.
- ![stars](https://img.shields.io/github/stars/mibk/dupl?style=flat-square&color=ccc) [dupl](https://github.com/mibk/dupl) - Reports potentially duplicated code.
- ![stars](https://img.shields.io/github/stars/kisielk/errcheck?style=flat-square&color=ccc) [errcheck](https://github.com/kisielk/errcheck) - Check that error return values are used.
- ![stars](https://img.shields.io/github/stars/lafolle/flen?style=flat-square&color=ccc) [flen](https://github.com/lafolle/flen) - Get info on length of functions in a Go package.
- [go tool vet --shadow](https://golang.org/cmd/vet/#hdr-Shadowed_variables) - Reports variables that may have been unintentionally shadowed.
- [go vet](https://golang.org/cmd/vet/) - Examines Go source code and reports suspicious.
- ![stars](https://img.shields.io/github/stars/Quasilyte/go-consistent?style=flat-square&color=ccc) [go-consistent](https://github.com/Quasilyte/go-consistent) - Analyzer that helps you to make your Go programs more consistent.
- ![stars](https://img.shields.io/github/stars/go-critic/go-critic?style=flat-square&color=ccc) [go-critic](https://github.com/go-critic/go-critic) - Go source code linter that maintains checks which are currently not implemented in other linters.
- [go/ast](https://golang.org/pkg/go/ast/) - Package ast declares the types used to represent syntax trees for Go packages.
- ![stars](https://img.shields.io/github/stars/leighmcculloch/gochecknoglobals?style=flat-square&color=ccc) [gochecknoglobals](https://github.com/leighmcculloch/gochecknoglobals) - Checks that no globals are present.
- ![stars](https://img.shields.io/github/stars/jgautheron/goconst?style=flat-square&color=ccc) [goconst](https://github.com/jgautheron/goconst) - Finds repeated strings that could be replaced by a constant.
- ![stars](https://img.shields.io/github/stars/fzipp/gocyclo?style=flat-square&color=ccc) [gocyclo](https://github.com/fzipp/gocyclo) - Calculate cyclomatic complexities of functions in Go source code.
- [gofmt -s](https://golang.org/cmd/gofmt/) - Checks if the code is properly formatted and could not be further simplified.
- [goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports) - Checks missing or unreferenced package imports.
- ![stars](https://img.shields.io/github/stars/golangci/golangci-lint?style=flat-square&color=ccc) [GolangCI-Lint](https://golangci-lint.run/) - Alternative to `Go Meta Linter`: GolangCI-Lint is a linters aggregator.
- ![stars](https://img.shields.io/github/stars/golang/lint?style=flat-square&color=ccc) [golint](https://github.com/golang/lint) - Prints out coding style mistakes in Go source code.
- ![stars](https://img.shields.io/github/stars/linuxerwang/goroutine-inspect?style=flat-square&color=ccc) [goroutine-inspect](https://github.com/linuxerwang/goroutine-inspect) - An interactive tool to analyze Golang goroutine dump.
- ![stars](https://img.shields.io/github/stars/securego/gosec?style=flat-square&color=ccc) [gosec (gas)](https://securego.io/) - Inspects source code for security problems by scanning the Go AST.
- [gotype](https://pkg.go.dev/golang.org/x/tools/cmd/gotype) - Syntactic and semantic analysis similar to the Go compiler.
- ![stars](https://img.shields.io/github/stars/gordonklaus/ineffassign?style=flat-square&color=ccc) [ineffassign](https://github.com/gordonklaus/ineffassign) - Detect ineffectual assignments in Go code.
- ![stars](https://img.shields.io/github/stars/mvdan/interfacer?style=flat-square&color=ccc) [interfacer](https://github.com/mvdan/interfacer) :warning: - Suggest narrower interfaces that can be used.
- ![stars](https://img.shields.io/github/stars/walle/lll?style=flat-square&color=ccc) [lll](https://github.com/walle/lll) - Report long lines.
- ![stars](https://img.shields.io/github/stars/mdempsky/maligned?style=flat-square&color=ccc) [maligned](https://github.com/mdempsky/maligned) - Detect structs that would take less memory if their fields were sorted.
- ![stars](https://img.shields.io/github/stars/client9/misspell?style=flat-square&color=ccc) [misspell](https://github.com/client9/misspell) - Finds commonly misspelled English words.
- ![stars](https://img.shields.io/github/stars/alexkohler/nakedret?style=flat-square&color=ccc) [nakedret](https://github.com/alexkohler/nakedret) - Finds naked returns.
- ![stars](https://img.shields.io/github/stars/alexkohler/nargs?style=flat-square&color=ccc) [nargs](https://github.com/alexkohler/nargs) - Finds unused arguments in function declarations.
- ![stars](https://img.shields.io/github/stars/alexkohler/prealloc?style=flat-square&color=ccc) [prealloc](https://github.com/alexkohler/prealloc) - Finds slice declarations that could potentially be preallocated.
- ![stars](https://img.shields.io/github/stars/mgechev/revive?style=flat-square&color=ccc) [revive](https://revive.run/) - Fast, configurable, extensible, flexible, and beautiful linter for Go. Drop-in replacement of golint.
- ![stars](https://img.shields.io/github/stars/stripe/safesql?style=flat-square&color=ccc) [safesql](https://github.com/stripe/safesql) - Static analysis tool for Golang that protects against SQL injections.
- ![stars](https://img.shields.io/github/stars/dominikh/go-tools?style=flat-square&color=ccc) [staticcheck](https://staticcheck.io/) - Go static analysis that specialises in finding bugs, simplifying code and improving performance.
- [structcheck](https://gitlab.com/opennota/check) - Find unused struct fields.
- [test](http://golang.org/pkg/testing/) - Show location of test failures from the stdlib testing module.
- ![stars](https://img.shields.io/github/stars/mdempsky/unconvert?style=flat-square&color=ccc) [unconvert](https://github.com/mdempsky/unconvert) - Detect redundant type conversions.
- ![stars](https://img.shields.io/github/stars/alexkohler/unimport?style=flat-square&color=ccc) [unimport](https://github.com/alexkohler/unimport) - Finds unnecessary import aliases.
- ![stars](https://img.shields.io/github/stars/mvdan/unparam?style=flat-square&color=ccc) [unparam](https://github.com/mvdan/unparam) - Find unused function parameters.
- [varcheck](https://gitlab.com/opennota/check) - Find unused global variables and constants.
- ![stars](https://img.shields.io/github/stars/bombsimon/wsl?style=flat-square&color=ccc) [wsl](https://github.com/bombsimon/wsl) - Enforces empty lines at the right places.


<h2 id="groovy">Groovy</h2>

- ![stars](https://img.shields.io/github/stars/CodeNarc/CodeNarc?style=flat-square&color=ccc) [CodeNarc](https://codenarc.github.io/CodeNarc/) - a static analysis tool for Groovy source code, enabling monitoring and enforcement of many coding standards and best practices.


<h2 id="haskell">Haskell</h2>

- ![stars](https://img.shields.io/github/stars/ndmitchell/hlint?style=flat-square&color=ccc) [HLint](https://github.com/ndmitchell/hlint) - HLint is a tool for suggesting possible improvements to Haskell code.
- ![stars](https://img.shields.io/github/stars/ocharles/weeder?style=flat-square&color=ccc) [Weeder](https://github.com/ocharles/weeder) - A tool for detecting dead exports or package imports in Haskell code.


<h2 id="haxe">Haxe</h2>

- ![stars](https://img.shields.io/github/stars/HaxeCheckstyle/haxe-checkstyle?style=flat-square&color=ccc) [Haxe Checkstyle](http://haxecheckstyle.github.io/docs/haxe-checkstyle/home.html) - A static analysis tool to help developers write Haxe code that adheres to a coding standard.


<h2 id="java">Java</h2>

- ![stars](https://img.shields.io/github/stars/typetools/checker-framework?style=flat-square&color=ccc) [Checker Framework](https://checkerframework.org/) - Pluggable type-checking for Java.
- ![stars](https://img.shields.io/github/stars/checkstyle/checkstyle?style=flat-square&color=ccc) [checkstyle](https://checkstyle.org/) - checking Java source code for adherence to a Code Standard or set of validation rules (best practices).
- ![stars](https://img.shields.io/github/stars/mauricioaniche/ck?style=flat-square&color=ccc) [ck](https://github.com/mauricioaniche/ck) - calculates Chidamber and Kemerer object-oriented metrics by processing the source Java files.
- ![stars](https://img.shields.io/github/stars/dspinellis/ckjm?style=flat-square&color=ccc) [ckjm](http://www.spinellis.gr/sw/ckjm/) - calculates Chidamber and Kemerer object-oriented metrics by processing the bytecode of compiled Java files.
- ![stars](https://img.shields.io/github/stars/eclipse-cognicrypt/CogniCrypt?style=flat-square&color=ccc) [CogniCrypt](https://www.eclipse.org/cognicrypt/) - checks Java source and byte code for incorrect uses of cryptographic APIs.
- [DesigniteJava](http://www.designite-tools.com/designitejava) :copyright: - DesigniteJava supports detection of various architecture, design, and implementation smells along with computation of various code quality metrics.
- ![stars](https://img.shields.io/github/stars/google/error-prone?style=flat-square&color=ccc) [Error-prone](https://errorprone.info/) - Catch common Java mistakes as compile-time errors.
- ![stars](https://img.shields.io/github/stars/mebigfatguy/fb-contrib?style=flat-square&color=ccc) [fb-contrib](http://fb-contrib.sourceforge.net/) - A plugin for FindBugs with additional bug detectors.
- ![stars](https://img.shields.io/github/stars/policeman-tools/forbidden-apis?style=flat-square&color=ccc) [forbidden-apis](https://github.com/policeman-tools/forbidden-apis) - Detects and forbids invocations of specific method/class/field (like reading from a text stream without a charset). Maven/Gradle/Ant compatible.
- ![stars](https://img.shields.io/github/stars/google/google-java-format?style=flat-square&color=ccc) [google-java-format](https://github.com/google/google-java-format) - Google Style Reformat.
- ![stars](https://img.shields.io/github/stars/amaembo/huntbugs?style=flat-square&color=ccc) [HuntBugs](https://github.com/amaembo/huntbugs) :warning: - Bytecode static analyzer tool based on Procyon Compiler Tools aimed to supersede FindBugs.
- [JArchitect](https://www.jarchitect.com) :copyright: - Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [JBMC](https://www.cprover.org/jbmc/) - bounded model-checker for Java (bytecode), verifies user-defined assertions, standard assertions, several coverage metric analyses.
- ![stars](https://img.shields.io/github/stars/uber/NullAway?style=flat-square&color=ccc) [NullAway](https://github.com/uber/NullAway) - Type-based null-pointer checker with low build-time overhead; an [Error Prone](http://errorprone.info/) plugin.
- ![stars](https://img.shields.io/github/stars/jeremylong/DependencyCheck?style=flat-square&color=ccc) [OWASP Dependency Check](https://owasp.org/www-project-dependency-check/) - Checks dependencies for known, publicly disclosed, vulnerabilities.
- ![stars](https://img.shields.io/github/stars/teamed/qulice?style=flat-square&color=ccc) [qulice](https://www.qulice.com/) - Combines a few (pre-configured) static analysis tools (checkstyle, PMD, Findbugs, ...).
- ![stars](https://img.shields.io/github/stars/soot-oss/soot?style=flat-square&color=ccc) [Soot](https://soot-oss.github.io/soot/) - A framework for analyzing and transforming Java and Android applications.
- ![stars](https://img.shields.io/github/stars/INRIA/spoon?style=flat-square&color=ccc) [Spoon](http://spoon.gforge.inria.fr/) - Spoon is a metaprogramming library to analyze and transform Java source code (incl Java 9, 10, 11, 12, 13, 14). It parses source files to build a well-designed AST with powerful analysis and transformation API. Can be integrated in Maven and Gradle.
- ![stars](https://img.shields.io/github/stars/spotbugs/spotbugs?style=flat-square&color=ccc) [SpotBugs](https://spotbugs.github.io/) - SpotBugs is FindBugs' successor. A tool for static analysis to look for bugs in Java code.


<h2 id="javascript">JavaScript</h2>

- ![stars](https://img.shields.io/github/stars/codecombat/aether?style=flat-square&color=ccc) [aether](http://aetherjs.com/) - Lint, analyze, normalize, transform, sandbox, run, step through, and visualize user JavaScript, in node or the browser.
- ![stars](https://img.shields.io/github/stars/google/closure-compiler?style=flat-square&color=ccc) [Closure Compiler](https://developers.google.com/closure/compiler/) - A compiler tool to increase efficiency, reduce size, and provide code warnings in JavaScript files.
- ![stars](https://img.shields.io/github/stars/google/closure-linter?style=flat-square&color=ccc) [ClosureLinter](https://github.com/google/closure-linter) :warning: - ensures that all of your project's JavaScript code follows the guidelines in the Google JavaScript Style Guide. It can also automatically fix many common errors.
- ![stars](https://img.shields.io/github/stars/jared-stilwell/complexity-report?style=flat-square&color=ccc) [complexity-report](https://github.com/jared-stilwell/complexity-report) :warning: - Software complexity analysis for JavaScript projects.
- [DeepScan](https://deepscan.io) :copyright: - An analyzer for JavaScript which targets runtime errors and quality issues rather than coding conventions.
- ![stars](https://img.shields.io/github/stars/jared-stilwell/escomplex?style=flat-square&color=ccc) [escomplex](https://github.com/jared-stilwell/escomplex) - Software complexity analysis of JavaScript-family abstract syntax trees.
- ![stars](https://img.shields.io/github/stars/eslint/eslint?style=flat-square&color=ccc) [eslint](https://eslint.org/) - A fully pluggable tool for identifying and reporting on patterns in JavaScript.
- ![stars](https://img.shields.io/github/stars/jquery/esprima?style=flat-square&color=ccc) [Esprima](https://esprima.org/) - ECMAScript parsing infrastructure for multipurpose analysis.
- ![stars](https://img.shields.io/github/stars/facebook/flow?style=flat-square&color=ccc) [flow](https://flow.org/) - A static type checker for JavaScript.
- ![stars](https://img.shields.io/github/stars/JSMonk/hegel?style=flat-square&color=ccc) [hegel](https://hegel.js.org/) - A static type checker for JavaScript with a bias on type inference and strong type systems.
- ![stars](https://img.shields.io/github/stars/jshint/jshint?style=flat-square&color=ccc) [jshint](https://jshint.com/about/) [:information_source:](https://github.com/analysis-tools-dev/static-analysis/issues/223) - detect errors and potential problems in JavaScript code and enforce your team's coding conventions.
- ![stars](https://img.shields.io/github/stars/douglascrockford/JSLint?style=flat-square&color=ccc) [JSLint](https://github.com/douglascrockford/JSLint) [:information_source:](https://github.com/analysis-tools-dev/static-analysis/issues/223) - The JavaScript Code Quality Tool.
- ![stars](https://img.shields.io/github/stars/dpnishant/jsprime?style=flat-square&color=ccc) [JSPrime](http://dpnishant.github.io/jsprime/) :warning: - static security analysis tool.
- ![stars](https://img.shields.io/github/stars/es-analysis/plato?style=flat-square&color=ccc) [plato](https://github.com/es-analysis/plato) :warning: - Visualize JavaScript source complexity.
- ![stars](https://img.shields.io/github/stars/jden/quality?style=flat-square&color=ccc) [quality](https://github.com/jden/quality) :warning: - zero configuration code and module linting.
- ![stars](https://img.shields.io/github/stars/RetireJS/retire.js?style=flat-square&color=ccc) [retire.js](http://retirejs.github.io/retire.js/) - Scanner detecting the use of JavaScript libraries with known vulnerabilities.
- ![stars](https://img.shields.io/github/stars/ternjs/tern?style=flat-square&color=ccc) [tern](https://ternjs.net/) - A JavaScript code analyzer for deep, cross-editor language support.
- ![stars](https://img.shields.io/github/stars/xojs/xo?style=flat-square&color=ccc) [xo](https://github.com/xojs/xo) - Opinionated but configurable ESLint wrapper with lots of goodies included. Enforces strict and readable code.
- ![stars](https://img.shields.io/github/stars/calmh/yardstick?style=flat-square&color=ccc) [yardstick](https://github.com/calmh/yardstick) :warning: - Javascript code metrics.


<h2 id="kotlin">Kotlin</h2>

- ![stars](https://img.shields.io/github/stars/detekt/detekt?style=flat-square&color=ccc) [detekt](https://detekt.github.io/detekt/) - Static code analysis for Kotlin code.


<h2 id="lua">Lua</h2>

- ![stars](https://img.shields.io/github/stars/mpeterv/luacheck?style=flat-square&color=ccc) [luacheck](https://github.com/mpeterv/luacheck) - A tool for linting and static analysis of Lua code.


<h2 id="matlab">MATLAB</h2>

- [mlint](https://mathworks.com/help/matlab/ref/mlint.html) :copyright: - Check MATLAB code files for possible problems.


<h2 id="php">PHP</h2>

- ![stars](https://img.shields.io/github/stars/mihaeu/dephpend?style=flat-square&color=ccc) [dephpend](https://dephpend.com/) - Dependency analysis tool.
- ![stars](https://img.shields.io/github/stars/sensiolabs-de/deprecation-detector?style=flat-square&color=ccc) [deprecation-detector](https://github.com/sensiolabs-de/deprecation-detector) - Finds usages of deprecated (Symfony) code.
- ![stars](https://img.shields.io/github/stars/sensiolabs-de/deptrac?style=flat-square&color=ccc) [deptrac](https://github.com/sensiolabs-de/deptrac) - Enforce rules for dependencies between software layers.
- ![stars](https://img.shields.io/github/stars/Halleck45/DesignPatternDetector?style=flat-square&color=ccc) [DesignPatternDetector](https://github.com/Halleck45/DesignPatternDetector) - detection of design patterns in PHP code.
- ![stars](https://img.shields.io/github/stars/Symplify/EasyCodingStandard?style=flat-square&color=ccc) [EasyCodingStandard](https://www.tomasvotruba.com/blog/2017/05/03/combine-power-of-php-code-sniffer-and-php-cs-fixer-in-3-lines/) - Combine [PHP_CodeSniffer](https://github.com/squizlabs/PHP_CodeSniffer) and [PHP-CS-Fixer](https://github.com/FriendsOfPHP/PHP-CS-Fixer).
- ![stars](https://img.shields.io/github/stars/phpro/grumphp?style=flat-square&color=ccc) [GrumPHP](https://github.com/phpro/grumphp) - checks code on every commit.
- ![stars](https://img.shields.io/github/stars/Trismegiste/Mondrian?style=flat-square&color=ccc) [Mondrian](http://trismegiste.github.io/Mondrian/) - a set of static analysis and refactoring tools which use graph theory.
- ![stars](https://img.shields.io/github/stars/php-parallel-lint/PHP-Parallel-Lint?style=flat-square&color=ccc) [parallel-lint](https://github.com/php-parallel-lint/PHP-Parallel-Lint) - This tool checks syntax of PHP files faster than serial check with a fancier output.
- ![stars](https://img.shields.io/github/stars/psecio/parse?style=flat-square&color=ccc) [Parse](https://github.com/psecio/parse) - A Static Security Scanner.
- ![stars](https://img.shields.io/github/stars/pdepend/pdepend?style=flat-square&color=ccc) [pdepend](https://pdepend.org/) - Calculates software metrics like cyclomatic complexity for PHP code.
- ![stars](https://img.shields.io/github/stars/etsy/phan?style=flat-square&color=ccc) [phan](https://github.com/phan/phan/wiki) - a modern static analyzer from etsy.
- ![stars](https://img.shields.io/github/stars/carlosas/phpat?style=flat-square&color=ccc) [PHP Architecture Tester](https://github.com/carlosas/phpat) - Easy to use architecture testing tool for PHP.
- ![stars](https://img.shields.io/github/stars/rskuipers/php-assumptions?style=flat-square&color=ccc) [PHP Assumptions](https://github.com/rskuipers/php-assumptions) - Checks for weak assumptions.
- ![stars](https://img.shields.io/github/stars/FriendsOfPHP/PHP-CS-Fixer?style=flat-square&color=ccc) [PHP Coding Standards Fixer](https://cs.symfony.com/) - Fixes your code according to standards like PSR-1, PSR-2, and the Symfony standard.
- ![stars](https://img.shields.io/github/stars/nunomaduro/phpinsights?style=flat-square&color=ccc) [PHP Insights](https://phpinsights.com/) - Instant PHP quality checks from your console. Analysis of code quality and coding style  as well as overview of code architecture and its complexity.

- ![stars](https://img.shields.io/github/stars/kalessil/phpinspectionsea?style=flat-square&color=ccc) [Php Inspections (EA Extended)](https://plugins.jetbrains.com/plugin/7622-php-inspections-ea-extended-) - A Static Code Analyzer for PHP.
- ![stars](https://img.shields.io/github/stars/QafooLabs/php-refactoring-browser?style=flat-square&color=ccc) [PHP Refactoring Browser](http://qafoolabs.github.io/php-refactoring-browser/) - Refactoring helper.
- ![stars](https://img.shields.io/github/stars/tomzx/php-semver-checker?style=flat-square&color=ccc) [PHP Semantic Versioning Checker](https://github.com/tomzx/php-semver-checker) - Suggests a next version according to semantic versioning.
- ![stars](https://img.shields.io/github/stars/nikic/PHP-Parser?style=flat-square&color=ccc) [PHP-Parser](https://github.com/nikic/PHP-Parser) - A PHP parser written in PHP.
- ![stars](https://img.shields.io/github/stars/Andrewsville/PHP-Token-Reflection?style=flat-square&color=ccc) [PHP-Token-Reflection](https://github.com/Andrewsville/PHP-Token-Reflection) - Library emulating the PHP internal reflection.
- ![stars](https://img.shields.io/github/stars/sstalle/php7cc?style=flat-square&color=ccc) [php7cc](https://github.com/sstalle/php7cc) :warning: - PHP 7 Compatibility Checker.
- ![stars](https://img.shields.io/github/stars/Alexia/php7mar?style=flat-square&color=ccc) [php7mar](https://github.com/Alexia/php7mar) :warning: - assist developers in porting their code quickly to PHP 7.
- ![stars](https://img.shields.io/github/stars/squizlabs/PHP_CodeSniffer?style=flat-square&color=ccc) [PHP_CodeSniffer](https://pear.php.net/package/PHP_CodeSniffer) - detects violations of a defined set of coding standards.
- ![stars](https://img.shields.io/github/stars/wapmorgan/PhpCodeAnalyzer?style=flat-square&color=ccc) [phpca](https://github.com/wapmorgan/PhpCodeAnalyzer) - Finds usage of non-built-in extensions.
- ![stars](https://img.shields.io/github/stars/wapmorgan/PhpCodeFixer?style=flat-square&color=ccc) [phpcf](http://wapmorgan.github.io/PhpCodeFixer/) - Finds usage of deprecated PHP features.
- ![stars](https://img.shields.io/github/stars/sebastianbergmann/phpcpd?style=flat-square&color=ccc) [phpcpd](https://github.com/sebastianbergmann/phpcpd) - Copy/Paste Detector for PHP code.
- ![stars](https://img.shields.io/github/stars/sebastianbergmann/phpdcd?style=flat-square&color=ccc) [phpdcd](https://github.com/sebastianbergmann/phpdcd) :warning: - Dead Code Detector (DCD) for PHP code.
- ![stars](https://img.shields.io/github/stars/mamuz/PhpDependencyAnalysis?style=flat-square&color=ccc) [PhpDependencyAnalysis](https://mamuz.github.io/PhpDependencyAnalysis/) - builds a dependency graph for a project.
- ![stars](https://img.shields.io/github/stars/dunglas/phpdoc-to-typehint?style=flat-square&color=ccc) [phpdoc-to-typehint](https://github.com/dunglas/phpdoc-to-typehint) - Add scalar type hints and return types to existing PHP projects using PHPDoc annotations.
- ![stars](https://img.shields.io/github/stars/phpDocumentor/phpDocumentor?style=flat-square&color=ccc) [phpDocumentor](https://www.phpdoc.org/) - Analyzes PHP source code to generate documentation.
- ![stars](https://img.shields.io/github/stars/phpmd/phpmd?style=flat-square&color=ccc) [PHPMD](https://phpmd.org/) - finds possible bugs in your code.
- ![stars](https://img.shields.io/github/stars/phpmetrics/PhpMetrics?style=flat-square&color=ccc) [PhpMetrics](http://www.phpmetrics.org/) - Calculates and visualizes various code quality metrics.
- ![stars](https://img.shields.io/github/stars/povils/phpmnd?style=flat-square&color=ccc) [phpmnd](https://github.com/povils/phpmnd) - Helps to detect magic numbers.
- ![stars](https://img.shields.io/github/stars/EdgedesignCZ/phpqa?style=flat-square&color=ccc) [PHPQA](https://edgedesigncz.github.io/phpqa/) - A tool for running QA tools (phploc, phpcpd, phpcs, pdepend, phpmd, phpmetrics).
- ![stars](https://img.shields.io/github/stars/jakzal/phpqa?style=flat-square&color=ccc) [phpqa - jakzal](https://github.com/jakzal/phpqa) - Many tools for PHP static analysis in one container.
- ![stars](https://img.shields.io/github/stars/jmolivas/phpqa?style=flat-square&color=ccc) [phpqa - jmolivas](https://github.com/jmolivas/phpqa) - PHPQA all-in-one Analyzer CLI tool.
- ![stars](https://img.shields.io/github/stars/ovr/phpsa?style=flat-square&color=ccc) [phpsa](https://github.com/ovr/phpsa) - Static analysis tool for PHP.
- ![stars](https://img.shields.io/github/stars/phpstan/phpstan?style=flat-square&color=ccc) [PHPStan](https://phpstan.org/) - PHP Static Analysis Tool - discover bugs in your code without running it!
- ![stars](https://img.shields.io/github/stars/designsecurity/progpilot?style=flat-square&color=ccc) [Progpilot](https://github.com/designsecurity/progpilot) - A static analysis tool for security purposes.
- ![stars](https://img.shields.io/github/stars/vimeo/psalm?style=flat-square&color=ccc) [Psalm](https://psalm.dev/) - Static analysis tool for finding type errors in PHP applications.
- ![stars](https://img.shields.io/github/stars/Qafoo/QualityAnalyzer?style=flat-square&color=ccc) [Qafoo Quality Analyzer](https://github.com/Qafoo/QualityAnalyzer) - Visualizes metrics and source code.
- ![stars](https://img.shields.io/github/stars/ircmaxell/Tuli?style=flat-square&color=ccc) [Tuli](https://github.com/ircmaxell/Tuli) - A static analysis engine.
- ![stars](https://img.shields.io/github/stars/asm89/twig-lint?style=flat-square&color=ccc) [twig-lint](https://github.com/asm89/twig-lint) - twig-lint is a lint tool for your twig files.
- [WAP](https://securityonline.info/owasp-wap-web-application-protection-project/) - Tool to detect and correct input validation vulnerabilities in PHP (4.0 or higher) web applications and predicts false positives by combining static analysis and data mining.


<h2 id="perl">Perl</h2>

- [Perl::Critic](https://metacpan.org/pod/Perl::Critic) - Critique Perl source code for best-practices.


<h2 id="python">Python</h2>

- ![stars](https://img.shields.io/github/stars/PyCQA/bandit?style=flat-square&color=ccc) [bandit](https://bandit.readthedocs.io/en/latest/) - a tool to find common security issues in Python code.
- ![stars](https://img.shields.io/github/stars/hchasestevens/bellybutton?style=flat-square&color=ccc) [bellybutton](https://github.com/hchasestevens/bellybutton) - a linting engine supporting custom project-specific rules.
- ![stars](https://img.shields.io/github/stars/mschwager/cohesion?style=flat-square&color=ccc) [cohesion](https://github.com/mschwager/cohesion) - a tool for measuring Python class cohesion.
- ![stars](https://img.shields.io/github/stars/dlint-py/dlint?style=flat-square&color=ccc) [Dlint](https://github.com/dlint-py/dlint) - a tool for ensuring Python code is secure.
- ![stars](https://img.shields.io/github/stars/feddischson/include_gardener?style=flat-square&color=ccc) [include-gardener](https://github.com/feddischson/include_gardener) - a multi-language static analyzer for C/C++/Obj-C/Python/Ruby to create a graph (in dot or graphml format) which shows all `#include` relations of a given set of files.
- ![stars](https://img.shields.io/github/stars/davidhalter/jedi?style=flat-square&color=ccc) [jedi](https://jedi.readthedocs.io/en/latest/) - autocompletion/static analysis library for Python.
- ![stars](https://img.shields.io/github/stars/lyft/linty_fresh?style=flat-square&color=ccc) [linty fresh](https://github.com/lyft/linty_fresh) - parse lint errors and report them to Github as comments on a pull request.
- ![stars](https://img.shields.io/github/stars/PyCQA/mccabe?style=flat-square&color=ccc) [mccabe](https://pypi.org/project/mccabe/) - check McCabe complexity.
- ![stars](https://img.shields.io/github/stars/python/mypy?style=flat-square&color=ccc) [mypy](http://www.mypy-lang.org/) - A static type checker that aims to combine the benefits of duck typing and static typing, frequently used with [MonkeyType](https://github.com/Instagram/MonkeyType).
- ![stars](https://img.shields.io/github/stars/uber/py-find-injection?style=flat-square&color=ccc) [py-find-injection](https://github.com/uber/py-find-injection) :warning: - find SQL injection vulnerabilities in Python code.
- ![stars](https://img.shields.io/github/stars/PyCQA/pycodestyle?style=flat-square&color=ccc) [pycodestyle](https://pycodestyle.pycqa.org/en/latest/) - (formerly `pep8`) check Python code against some of the style conventions in PEP 8.
- ![stars](https://img.shields.io/github/stars/PyCQA/pydocstyle?style=flat-square&color=ccc) [pydocstyle](http://www.pydocstyle.org) - Check compliance with Python docstring conventions.
- ![stars](https://img.shields.io/github/stars/pyflakes/pyflakes?style=flat-square&color=ccc) [pyflakes](https://pypi.org/project/pyflakes/) - Check Python source files for errors.
- ![stars](https://img.shields.io/github/stars/PyCQA/pylint?style=flat-square&color=ccc) [pylint](http://pylint.pycqa.org/en/latest/) - looks for programming errors, helps enforcing a coding standard and sniffs for some code smells. It additionally includes `pyreverse` (an UML diagram generator) and `symilar` (a similarities checker).
- ![stars](https://img.shields.io/github/stars/facebook/pyre-check?style=flat-square&color=ccc) [pyre-check](https://pyre-check.org/) - A fast, scalable type checker for large Python codebases.
- ![stars](https://img.shields.io/github/stars/Microsoft/pyright?style=flat-square&color=ccc) [pyright](https://github.com/Microsoft/pyright) - Static type checker for Python, created to address gaps in existing tools like mypy.
- ![stars](https://img.shields.io/github/stars/regebro/pyroma?style=flat-square&color=ccc) [pyroma](https://github.com/regebro/pyroma) - Rate how well a Python project complies with the best practices of the Python packaging ecosystem, and list issues that could be improved.
- ![stars](https://img.shields.io/github/stars/python-security/pyt?style=flat-square&color=ccc) [PyT - Python Taint](https://github.com/python-security/pyt) :warning: - A static analysis tool for detecting security vulnerabilities in Python web applications.
- ![stars](https://img.shields.io/github/stars/google/pytype?style=flat-square&color=ccc) [pytype](https://google.github.io/pytype/) - A static type analyzer for Python code.
- ![stars](https://img.shields.io/github/stars/rubik/radon?style=flat-square&color=ccc) [radon](https://radon.readthedocs.io/en/latest/) - A Python tool that computes various metrics from the source code.
- ![stars](https://img.shields.io/github/stars/jendrikseipp/vulture?style=flat-square&color=ccc) [vulture](https://github.com/jendrikseipp/vulture) - Find unused classes, functions and variables in Python code.
- ![stars](https://img.shields.io/github/stars/wemake-services/wemake-python-styleguide?style=flat-square&color=ccc) [wemake-python-styleguide](https://wemake-python-stylegui.de) - The strictest and most opinionated python linter ever.
- ![stars](https://img.shields.io/github/stars/tonybaloney/wily?style=flat-square&color=ccc) [wily](https://github.com/tonybaloney/wily) - A command-line tool for archiving, exploring and graphing the complexity of Python source code.
- ![stars](https://img.shields.io/github/stars/rubik/xenon?style=flat-square&color=ccc) [xenon](https://xenon.readthedocs.io/) - monitor code complexity using [`radon`](https://github.com/rubik/radon).


<h2 id="r">R</h2>

- ![stars](https://img.shields.io/github/stars/MangoTheCat/cyclocomp?style=flat-square&color=ccc) [cyclocomp](https://github.com/MangoTheCat/cyclocomp) - Quantifies the cyclomatic complexity of R functions / expressions.
- ![stars](https://img.shields.io/github/stars/mangothecat/goodpractice?style=flat-square&color=ccc) [goodpractice](http://mangothecat.github.io/goodpractice/) - Analyses the source code for R packages and provides best-practice recommendations.
- ![stars](https://img.shields.io/github/stars/jimhester/lintr?style=flat-square&color=ccc) [lintr](https://github.com/jimhester/lintr) - Static Code Analysis for R.


<h2 id="rpg">RPG</h2>

- [SourceMeter](https://www.sourcemeter.com/resources/rpg/) :copyright: - Static Code Analysis for RPG III and RPG IV versions (including free-form).


<h2 id="ruby">Ruby</h2>

- ![stars](https://img.shields.io/github/stars/presidentbeef/brakeman?style=flat-square&color=ccc) [brakeman](https://brakemanscanner.org/) - A static analysis security vulnerability scanner for Ruby on Rails applications.
- ![stars](https://img.shields.io/github/stars/square/cane?style=flat-square&color=ccc) [cane](https://github.com/square/cane) - Code quality threshold checking as part of your build.
- ![stars](https://img.shields.io/github/stars/seattlerb/flay?style=flat-square&color=ccc) [flay](https://ruby.sadi.st/Flay.html) - Flay analyzes code for structural similarities.
- ![stars](https://img.shields.io/github/stars/seattlerb/flog?style=flat-square&color=ccc) [flog](https://ruby.sadi.st/Flog.html) - Flog reports the most tortured code in an easy to read pain report. The higher the score, the more pain the code is in.
- ![stars](https://img.shields.io/github/stars/feddischson/include_gardener?style=flat-square&color=ccc) [include-gardener](https://github.com/feddischson/include_gardener) - a multi-language static analyzer for C/C++/Obj-C/Python/Ruby to create a graph (in dot or graphml format) which shows all `#include` relations of a given set of files.
- ![stars](https://img.shields.io/github/stars/michaeledgar/laser?style=flat-square&color=ccc) [laser](https://github.com/michaeledgar/laser) :warning: - Static analysis and style linter for Ruby code.
- ![stars](https://img.shields.io/github/stars/codegram/pelusa?style=flat-square&color=ccc) [pelusa](https://github.com/codegram/pelusa) - Static analysis Lint-type tool to improve your OO Ruby code.
- ![stars](https://img.shields.io/github/stars/soutaro/querly?style=flat-square&color=ccc) [Querly](https://github.com/soutaro/querly) - Pattern Based Checking Tool for Ruby.
- ![stars](https://img.shields.io/github/stars/david-a-wheeler/railroader?style=flat-square&color=ccc) [Railroader](https://railroader.org/) - An open source static analysis security vulnerability scanner for Ruby on Rails applications.
- ![stars](https://img.shields.io/github/stars/troessner/reek?style=flat-square&color=ccc) [reek](https://github.com/troessner/reek) - Code smell detector for Ruby.
- ![stars](https://img.shields.io/github/stars/rubocop-hq/rubocop?style=flat-square&color=ccc) [RuboCop](https://docs.rubocop.org/rubocop/) - A Ruby static code analyzer, based on the community Ruby style guide.
- ![stars](https://img.shields.io/github/stars/blazeeboy/rubrowser?style=flat-square&color=ccc) [Rubrowser](http://www.emadelsaid.com/rubrowser/) - Ruby classes interactive dependency graph generator.
- [ruby-lint](http://code.yorickpeterse.com/ruby-lint/latest/) :warning: - Static code analysis for Ruby.
- ![stars](https://img.shields.io/github/stars/whitesmith/rubycritic?style=flat-square&color=ccc) [rubycritic](https://github.com/whitesmith/rubycritic) - A Ruby code quality reporter.
- ![stars](https://img.shields.io/github/stars/makaroni4/sandi_meter?style=flat-square&color=ccc) [SandiMeter](https://rubygems.org/gems/sandi_meter) :warning: - Static analysis tool for checking Ruby code for Sandi Metz' rules.
- ![stars](https://img.shields.io/github/stars/sorbet/sorbet?style=flat-square&color=ccc) [Sorbet](https://sorbet.org/) - A fast, powerful type checker designed for Ruby.


<h2 id="rust">Rust</h2>

- ![stars](https://img.shields.io/github/stars/RustSec/cargo-audit?style=flat-square&color=ccc) [cargo-audit](https://rustsec.org/) - Audit Cargo.lock for crates with security vulnerabilities reported to the [RustSec Advisory Database](https://github.com/RustSec/advisory-db/).
- ![stars](https://img.shields.io/github/stars/mre/cargo-inspect?style=flat-square&color=ccc) [cargo-inspect](https://github.com/mre/cargo-inspect) - Inspect Rust code without syntactic sugar to see what the compiler does behind the curtains.
- ![stars](https://img.shields.io/github/stars/rust-lang/rust-clippy?style=flat-square&color=ccc) [clippy](https://rust-lang.github.io/rust-clippy/) - A code linter to catch common mistakes and improve your Rust code.
- ![stars](https://img.shields.io/github/stars/Kha/electrolysis?style=flat-square&color=ccc) [electrolysis](http://kha.github.io/electrolysis/) - A tool for formally verifying Rust programs by transpiling them into definitions in the Lean theorem prover.
- ![stars](https://img.shields.io/github/stars/mcarton/rust-herbie-lint?style=flat-square&color=ccc) [herbie](https://github.com/mcarton/rust-herbie-lint) - Adds warnings or errors to your crate when using a numerically unstable floating point expression.
- ![stars](https://img.shields.io/github/stars/AtomLinter/linter-rust?style=flat-square&color=ccc) [linter-rust](https://github.com/AtomLinter/linter-rust) - Linting your Rust-files in Atom, using rustc and cargo.
- ![stars](https://img.shields.io/github/stars/facebookexperimental/MIRAI?style=flat-square&color=ccc) [MIRAI](https://github.com/facebookexperimental/MIRAI) - And abstract interpreter operating on Rust's mid-level intermediate language, and providing warnings based on taint analysis.
- ![stars](https://img.shields.io/github/stars/rust-lang-nursery/rls?style=flat-square&color=ccc) [Rust Language Server](https://github.com/rust-lang-nursery/rls) - Supports functionality such as 'goto definition', symbol search, reformatting, and code completion, and enables renaming and refactorings.
- ![stars](https://img.shields.io/github/stars/rust-lang/rustfix?style=flat-square&color=ccc) [rustfix](https://github.com/rust-lang/rustfix) - read and apply the suggestions made by rustc (and third-party lints, like those offered by clippy).


<h2 id="sql">SQL</h2>

- ![stars](https://img.shields.io/github/stars/jarulraj/sqlcheck?style=flat-square&color=ccc) [sqlcheck](https://github.com/jarulraj/sqlcheck) - Automatically identify anti-patterns in SQL queries.
- ![stars](https://img.shields.io/github/stars/purcell/sqlint?style=flat-square&color=ccc) [sqlint](https://github.com/purcell/sqlint) - Simple SQL linter.
- ![stars](https://img.shields.io/github/stars/tsqllint/tsqllint?style=flat-square&color=ccc) [tsqllint](https://github.com/tsqllint/tsqllint) - T-SQL-specific linter.
- ![stars](https://img.shields.io/github/stars/ashleyglee/TSqlRules?style=flat-square&color=ccc) [TSqlRules](https://github.com/ashleyglee/TSqlRules) - TSQL Static Code Analysis Rules for SQL Server.


<h2 id="scala">Scala</h2>

- ![stars](https://img.shields.io/github/stars/HairyFotr/linter?style=flat-square&color=ccc) [linter](https://github.com/HairyFotr/linter) - Linter is a Scala static analysis compiler plugin which adds compile-time checks for various possible bugs, inefficiencies, and style problems.
- ![stars](https://img.shields.io/github/stars/scalastyle/scalastyle?style=flat-square&color=ccc) [Scalastyle](http://www.scalastyle.org) - Scalastyle examines your Scala code and indicates potential problems with it.
- ![stars](https://img.shields.io/github/stars/sksamuel/scapegoat?style=flat-square&color=ccc) [scapegoat](https://github.com/sksamuel/scapegoat) - Scala compiler plugin for static code analysis.
- ![stars](https://img.shields.io/github/stars/puffnfresh/wartremover?style=flat-square&color=ccc) [WartRemover](https://www.wartremover.org/) - a flexible Scala code linting tool.


<h2 id="shell">Shell</h2>

- ![stars](https://img.shields.io/github/stars/lequal/i-CodeCNES?style=flat-square&color=ccc) [i-Code CNES for Shell](https://github.com/lequal/i-CodeCNES) - An open source static code analysis tool for Shell and Fortran (77 and 90).
- ![stars](https://img.shields.io/github/stars/koalaman/shellcheck?style=flat-square&color=ccc) [shellcheck](https://www.shellcheck.net/) - ShellCheck, a static analysis tool that gives warnings and suggestions for bash/sh shell scripts.


<h2 id="solidity">Solidity</h2>

- ![stars](https://img.shields.io/github/stars/trailofbits/slither?style=flat-square&color=ccc) [slither](https://github.com/trailofbits/slither) - Static analysis framework that runs a suite of vulnerability detectors, prints visual information about contract details, and provides an API to easily write custom analyses.
- ![stars](https://img.shields.io/github/stars/duaraghav8/Solium?style=flat-square&color=ccc) [solium](https://ethlint.readthedocs.io/en/latest/) - Solium is a linter to identify and fix style and security issues in Solidity smart contracts.


<h2 id="swift">Swift</h2>

- ![stars](https://img.shields.io/github/stars/realm/SwiftLint?style=flat-square&color=ccc) [SwiftLint](https://realm.github.io/SwiftLint/) - A tool to enforce Swift style and conventions.
- ![stars](https://img.shields.io/github/stars/sleekbyte/tailor?style=flat-square&color=ccc) [Tailor](https://tailor.sh/) :warning: - A static analysis and lint tool for source code written in Apple's Swift programming language.


<h2 id="tcl">Tcl</h2>

- [Frink](https://catless.ncl.ac.uk/Programs/Frink/) - A Tcl formatting and static check program (can prettify the program, minimise, obfuscate or just sanity check it).
- [Nagelfar](https://sourceforge.net/projects/nagelfar/) - A static syntax checker for Tcl.
- [tclchecker](https://github.com/ActiveState/tdk/blob/master/docs/3.0/TDK_3.0_Checker.txt) - A static syntax analysis module (as part of [TDK](https://github.com/ActiveState/tdk)).


<h2 id="typescript">TypeScript</h2>

- ![stars](https://img.shields.io/github/stars/mgechev/codelyzer?style=flat-square&color=ccc) [Codelyzer](http://codelyzer.com/) - A set of tslint rules for static code analysis of Angular 2 TypeScript projects.
- ![stars](https://img.shields.io/github/stars/typescript-eslint/typescript-eslint?style=flat-square&color=ccc) [ESLint](https://github.com/typescript-eslint/typescript-eslint) - An extensible linter for the TypeScript language.
- ![stars](https://img.shields.io/github/stars/Glavin001/tslint-clean-code?style=flat-square&color=ccc) [tslint-clean-code](https://www.npmjs.com/package/tslint-clean-code) - A set of TSLint rules inspired by the Clean Code handbook.
- ![stars](https://img.shields.io/github/stars/Microsoft/tslint-microsoft-contrib?style=flat-square&color=ccc) [tslint-microsoft-contrib](https://github.com/Microsoft/tslint-microsoft-contrib) - A set of tslint rules for static code analysis of TypeScript projects maintained by Microsoft.


<h2 id="vbscript">VBScript</h2>

- [Test Design Studio](http://patterson-consulting.net/tds) :copyright: - A full IDE with static code analysis for Micro Focus Unified Functional Testing VBScript-based automated tests.


# Multiple languages

- [AppChecker](https://npo-echelon.ru/en/solutions/appchecker.php) :copyright: - Static analysis for C/C++/C#, PHP and Java.
- [Application Inspector](https://www.ptsecurity.com/ww-en/products/ai/) :copyright: - Commercial Static Code Analysis which generates exploits to verify vulnerabilities.
- ![stars](https://img.shields.io/github/stars/microsoft/ApplicationInspector?style=flat-square&color=ccc) [ApplicationInspector](https://github.com/microsoft/ApplicationInspector) - Creates reports of over 400 rule patterns for feature detection (e.g. the use of cryptography or version control in apps).
- [AppScan Source](https://www.hcltechsw.com/wps/portal/products/appscan/home) :copyright: - Commercial Static Code Analysis.
- [APPscreener](https://solarappscreener.com/) :copyright: - Static code analysis for binary and source code - Java/Scala, PHP, Javascript, C#, PL/SQL, Python, T-SQL, C/C++, ObjectiveC/Swift, Visual Basic 6.0, Ruby, Delphi, ABAP, HTML5 and Solidity.
- ![stars](https://img.shields.io/github/stars/TNG/ArchUnit?style=flat-square&color=ccc) [ArchUnit](https://www.archunit.org/) - Unit test your Java or Kotlin architecture.
- ![stars](https://img.shields.io/github/stars/Glavin001/atom-beautify?style=flat-square&color=ccc) [Atom-Beautify](https://atom.io/packages/atom-beautify) - Beautify HTML, CSS, JavaScript, PHP, Python, Ruby, Java, C, C++, C#, Objective-C, CoffeeScript, TypeScript, Coldfusion, SQL, and more in Atom editor.
- [Axivion Bauhaus Suite](https://www.axivion.com/en/products-services-9#products_bauhaussuite) :copyright: - Tracks down error-prone code locations, style violations, cloned or dead code, cyclic dependencies and more for C/C++, C#/.NET, Java and Ada 83/Ada 95.
- ![stars](https://img.shields.io/github/stars/ambv/black?style=flat-square&color=ccc) [Black](https://black.readthedocs.io/en/stable/) - The uncompromising Python code formatter.
- [CAST Highlight](https://www.castsoftware.com/products/highlight) :copyright: - Commercial Static Code Analysis which runs locally, but uploads the results to its cloud for presentation.
- [Checkmarx CxSAST](https://www.checkmarx.com/products/static-application-security-testing/) :copyright: - Commercial Static Code Analysis which doesn't require pre-compilation.
- ![stars](https://img.shields.io/github/stars/ContinuumIO/ciocheck?style=flat-square&color=ccc) [ciocheck](https://github.com/ContinuumIO/ciocheck) - linter, formatter and test suite helper. As a linter, it is a wrapper around `pep8`, `pydocstyle`, `flake8`, and `pylint`.
- ![stars](https://img.shields.io/github/stars/classgraph/classgraph?style=flat-square&color=ccc) [ClassGraph](https://github.com/classgraph/classgraph) - a classpath and module path scanner for querying or visualizing class metadata or class relatedness.
- ![stars](https://img.shields.io/github/stars/coala/coala?style=flat-square&color=ccc) [coala](https://coala.io/) - Language independent framework for creating code analysis - supports [over 60 languages](https://coala.io/languages) by default.
- [Cobra](http://spinroot.com/cobra/) :copyright: - Structural source code analyzer by NASA's Jet Propulsion Laboratory.
- [Codeac](https://www.codeac.io?ref=awesome-static-analysis) :copyright: - Automated code review tool integrates with GitHub, Bitbucket and GitLab (even self-hosted). Available for JavaScript, TypeScript, Python, Ruby, Go, PHP, Java, Docker, and more. (open-source free)
- ![stars](https://img.shields.io/github/stars/groupon/codeburner?style=flat-square&color=ccc) [codeburner](http://groupon.github.io/codeburner/) - Provides a unified interface to sort and act on the issues it finds.
- ![stars](https://img.shields.io/github/stars/Ericsson/codechecker?style=flat-square&color=ccc) [codechecker](https://codechecker.readthedocs.io/en/latest/) - a defect database and viewer extension for the Clang Static Analyzer with web GUI.
- [CodeFactor](https://codefactor.io) :copyright: - Static Code Analysis for C#, C, C++, CoffeeScript, CSS, Groovy, GO, JAVA, JavaScript, Less, Python, Ruby, Scala, SCSS, TypeScript.
- [CodeIt.Right](https://submain.com/products/codeit.right.aspx) :copyright: - CodeIt.Right&trade; provides a fast, automated way to ensure that your source code adheres to (your) predefined design and style guidelines as well as best coding practices.
- [CodePatrol](https://cyber-security.claranet.fr/en/codepatrol) :copyright: - Automated SAST code reviews driven by security, supports 15+ languages and includes security training.
- [CodeRush](https://www.devexpress.com/products/coderush/) :copyright: - Code creation, debugging, navigation, refactoring, analysis and visualization tools that use the Roslyn engine in Visual Studio 2015 and up.
- [CodeScene](https://empear.com/) :copyright: - CodeScene prioritizes technical debt, finds social patterns and identifies hidden risks in your code.
- ![stars](https://img.shields.io/github/stars/jameysharp/corrode?style=flat-square&color=ccc) [Corrode](https://github.com/jameysharp/corrode) - Semi-automatic translation from C to Rust. Could reveal bugs in the original implementation by showing Rust compiler warnings and errors.
- [Coverity](https://www.synopsys.com/software-integrity/security-testing/static-analysis-sast.html) :copyright: - Synopsys Coverity supports 20 languages and over 70 frameworks including Ruby on rails, Scala, PHP, Python, JavaScript, TypeScript, Java, Fortran, C, C++, C#, VB.NET.
- ![stars](https://img.shields.io/github/stars/xcatliu/cqc?style=flat-square&color=ccc) [cqc](https://github.com/xcatliu/cqc) - Check your code quality for js, jsx, vue, css, less, scss, sass and styl files.
- ![stars](https://img.shields.io/github/stars/csscomb/csscomb.js?style=flat-square&color=ccc) [CSScomb](https://github.com/csscomb/csscomb.js) - a coding style formatter for CSS. Supports own configurations to make style sheets beautiful and consistent.
- ![stars](https://img.shields.io/github/stars/thesp0nge/dawnscanner?style=flat-square&color=ccc) [dawnscanner](https://github.com/thesp0nge/dawnscanner) - a static analysis security scanner for ruby written web applications. It supports Sinatra, Padrino and Ruby on Rails frameworks.
- [DeepCode](https://www.deepcode.ai/) :copyright: - DeepCode finds bugs, security vulnerabilities, performance and API issues based on AI. DeepCode's speed of analysis allow us to analyse your code in real time and deliver results when you hit the save button in your IDE. Supported languages are Java, C/C++, JavaScript, Python, and TypeScript. Integrations with GitHub, BitBucket and Gitlab.
- [DeepSource](https://deepsource.io/) :copyright: - In-depth static analysis to monitor source code quality and security. Supports Python and Go and can detect 600+ types of issues in verticals of bug risks, security, anti-patterns, performance, documentation and style. Native integration with GitHub.
- ![stars](https://img.shields.io/github/stars/multilang-depends/depends?style=flat-square&color=ccc) [Depends](https://github.com/multilang-depends/depends) - Analyses the comprehensive dependencies of code elements for Java, C/C++, Ruby.
- ![stars](https://img.shields.io/github/stars/microsoft/devskim?style=flat-square&color=ccc) [DevSkim](https://github.com/microsoft/devskim) - Regex-based static analysis tool for Visual Studio, VS Code, and Sublime Text - C/C++, C#, PHP, ASP, Python, Ruby, Java, and others.
- [Embold](https://embold.io) :copyright: - Intelligent software analytics platform that identifies design issues, code issues, duplication and metrics. Supports Java, C, C++, C#, JavaScript, TypeScript, Python, Go, Kotlin and more.
- ![stars](https://img.shields.io/github/stars/exakat/exakat?style=flat-square&color=ccc) [exakat](https://www.exakat.io/) - An automated code reviewing engine for PHP.
- ![stars](https://img.shields.io/github/stars/find-sec-bugs/find-sec-bugs?style=flat-square&color=ccc) [Find Security Bugs](https://find-sec-bugs.github.io/) - The SpotBugs plugin for security audits of Java web applications and Android applications. (Also work with Kotlin, Groovy and Scala projects)
- ![stars](https://img.shields.io/github/stars/PyCQA/flake8?style=flat-square&color=ccc) [flake8](https://github.com/PyCQA/flake8) - a wrapper around `pyflakes`, `pycodestyle` and `mccabe`.
- [Fortify](https://software.microfocus.com/en-us/products/static-code-analysis-sast/overview) :copyright: - A commercial static analysis platform that supports the scanning of C/C++, C#, VB.NET, VB6, ABAP/BSP, ActionScript, Apex, ASP.NET, Classic ASP, VB Script, Cobol, ColdFusion, HTML, Java, JS, JSP, MXML/Flex, Objective-C, PHP, PL/SQL, T-SQL, Python (2.6, 2.7), Ruby (1.9.3), Swift, Scala, VB, and XML.
- ![stars](https://img.shields.io/github/stars/alecthomas/gometalinter?style=flat-square&color=ccc) [Go Meta Linter](https://github.com/alecthomas/gometalinter) :warning: - Concurrently run Go lint tools and normalise their output. Use `golangci-lint` for new projects.
- ![stars](https://img.shields.io/github/stars/sideci/goodcheck?style=flat-square&color=ccc) [Goodcheck](https://sider.github.io/goodcheck/) - Regexp based customizable linter.
- ![stars](https://img.shields.io/github/stars/360EntSecGroup-Skylar/goreporter?style=flat-square&color=ccc) [goreporter](https://github.com/360EntSecGroup-Skylar/goreporter) - concurrently runs many linters and normalises their output to a report.
- ![stars](https://img.shields.io/github/stars/wireghoul/graudit?style=flat-square&color=ccc) [graudit](http://www.justanotherhacker.com/) - Grep rough audit - source code auditing tool.
- ![stars](https://img.shields.io/github/stars/cuplv/hopper?style=flat-square&color=ccc) [Hopper](https://github.com/cuplv/hopper) - A static analysis tool written in scala for languages that run on JVM.
- ![stars](https://img.shields.io/github/stars/houndci/hound?style=flat-square&color=ccc) [Hound CI](https://houndci.com/) - Comments on style violations in GitHub pull requests. Supports Coffeescript, Go, HAML, JavaScript, Ruby, SCSS and Swift.
- ![stars](https://img.shields.io/github/stars/justinabrahms/imhotep?style=flat-square&color=ccc) [imhotep](https://github.com/justinabrahms/imhotep) - Comment on commits coming into your repository and check for syntactic errors and general lint warnings.
- ![stars](https://img.shields.io/github/stars/facebook/infer?style=flat-square&color=ccc) [Infer](https://fbinfer.com/) - A static analyzer for Java, C and Objective-C
- ![stars](https://img.shields.io/github/stars/insidersec/insider?style=flat-square&color=ccc) [InsiderSec](https://insidersec.io/) - A open source Static Application Security Testing tool (SAST) written in GoLang for Java (Maven and Android), Kotlin (Android), Swift (iOS), .NET Full Framework, C# and Javascript (Node.js).
- [Kiuwan](https://www.kiuwan.com/code-security-sast/) :copyright: - Identify and remediate cyber threats in a blazingly fast, collaborative environment, with seamless integration in your SDLC. Python, C\C++, Java, C#, PHP and more.
- [Klocwork](https://www.perforce.com/products/klocwork) :copyright: - Quality and Security Static analysis for C/C++, Java and C#.
- ![stars](https://img.shields.io/github/stars/shyiko/ktlint?style=flat-square&color=ccc) [ktlint](https://ktlint.github.io/) - An anti-bikeshedding Kotlin linter with built-in formatter.
- [LGTM.com](https://lgtm.com/) :copyright: - Deep code analysis for GitHub and Bitbucket to find security vulnerabilities and critical code quality issues (using Semmle QL). Automatic code review for pull requests; free for public repositories.
- ![stars](https://img.shields.io/github/stars/adamchainz/multilint?style=flat-square&color=ccc) [multilint](https://github.com/adamchainz/multilint) - a wrapper around `flake8`, `isort` and `modernize`.
- [Nitpick CI](https://nitpick-ci.com) :copyright: - Automated PHP code review.
- ![stars](https://img.shields.io/github/stars/ajinabraham/NodeJsScan?style=flat-square&color=ccc) [NodeJSScan](https://opensecurity.in/) - NodeJsScan is a static security code scanner for Node.js applications.
- ![stars](https://img.shields.io/github/stars/oclint/oclint?style=flat-square&color=ccc) [oclint](http://oclint.org/) - A static source code analysis tool to improve quality and reduce defects for C, C++ and Objective-C.
- ![stars](https://img.shields.io/github/stars/returntocorp/pfff?style=flat-square&color=ccc) [pfff](https://github.com/facebookarchive/pfff/wiki/Main) :warning: - Facebook's tools for code analysis, visualizations, or style-preserving source transformation for many languages.
- ![stars](https://img.shields.io/github/stars/pmd/pmd?style=flat-square&color=ccc) [PMD](https://pmd.github.io/) - A source code analyzer for Java, Javascript, PLSQL, XML, XSL and others.
- [Polymer-analyzer](https://github.com/Polymer/tools/tree/master/packages/analyzer) - A static analysis framework for Web Components.
- ![stars](https://img.shields.io/github/stars/pre-commit/pre-commit?style=flat-square&color=ccc) [pre-commit](https://pre-commit.com/) - A framework for managing and maintaining multi-language pre-commit hooks.
- ![stars](https://img.shields.io/github/stars/prettier/prettier?style=flat-square&color=ccc) [Prettier](https://prettier.io/) - An opinionated code formatter.
- ![stars](https://img.shields.io/github/stars/prontolabs/pronto?style=flat-square&color=ccc) [Pronto](https://github.com/prontolabs/pronto) - Quick automated code review of your changes. Supports more than 40 runners for various languages, including Clang, Elixir, JavaScript, PHP, Ruby and more.
- ![stars](https://img.shields.io/github/stars/PyCQA/prospector?style=flat-square&color=ccc) [prospector](https://github.com/PyCQA/prospector) - a wrapper around `pylint`, `pep8`, `mccabe` and others.
- ![stars](https://img.shields.io/github/stars/PositiveTechnologies/PT.PM?style=flat-square&color=ccc) [PT.PM](https://github.com/PositiveTechnologies/PT.PM) :warning: - An engine for searching patterns in the source code, based on Unified AST or UST. At present time C#, Java, PHP, PL/SQL, T-SQL, and JavaScript are supported. Patterns can be described within the code or using a DSL.
- [PullRequest](https://www.pullrequest.com) :copyright: - Code review as a service with built-in static analysis.
- ![stars](https://img.shields.io/github/stars/pumasecurity/puma-scan?style=flat-square&color=ccc) [Puma Scan](https://pumasecurity.io/) - Puma Scan provides real time secure code analysis for common vulnerabilities (XSS, SQLi, CSRF, LDAPi, crypto, deserialization, etc.) as development teams write code in Visual Studio.
- [PVS-Studio](https://www.viva64.com/en/pvs-studio/) :copyright: - a ([conditionally free](https://www.viva64.com/en/b/0614/) for FOSS and individual developers) static analysis of C, C++, C# and Java code. For advertising purposes [you can propose a large FOSS project for analysis by PVS employees](https://github.com/viva64/pvs-studio-check-list). Supports CWE mapping, MISRA and CERT coding standards.
- ![stars](https://img.shields.io/github/stars/apiology/quality?style=flat-square&color=ccc) [quality](https://github.com/apiology/quality) - Runs quality checks on your code using community tools, and makes sure your numbers don't get any worse over time.
- [Qualys Container Security](https://www.qualys.com/apps/container-security/) :copyright: - Container native application protection to provide visibility and control of containerized applications.
- ![stars](https://img.shields.io/github/stars/quantifiedcode/quantifiedcode?style=flat-square&color=ccc) [QuantifiedCode](https://github.com/quantifiedcode/quantifiedcode) :warning: - Automated code review & repair.
- ![stars](https://img.shields.io/github/stars/icsharpcode/RefactoringEssentials?style=flat-square&color=ccc) [Refactoring Essentials](https://marketplace.visualstudio.com/items?itemName=SharpDevelopTeam.RefactoringEssentialsforVisualStudio) - The free Visual Studio 2015 extension for C# and VB.NET refactorings, including code best practice analyzers.
- ![stars](https://img.shields.io/github/stars/codingjoe/relint?style=flat-square&color=ccc) [relint](https://github.com/codingjoe/relint) - A static file linter that allows you to write custom rules using regular expressions (RegEx).
- [ReSharper](https://www.jetbrains.com/resharper/) :copyright: - Extends Visual Studio with on-the-fly code inspections for C#, VB.NET, ASP.NET, JavaScript, TypeScript and other technologies.
- ![stars](https://img.shields.io/github/stars/haya14busa/reviewdog?style=flat-square&color=ccc) [Reviewdog](https://github.com/haya14busa/reviewdog) - A tool for posting review comments from any linter in any code hosting service.
- [RIPS](https://www.ripstech.com/) :copyright: - A static source code analyser for vulnerabilities in PHP scripts.
- ![stars](https://img.shields.io/github/stars/dotnet/roslyn-analyzers?style=flat-square&color=ccc) [Roslyn Analyzers](https://github.com/dotnet/roslyn-analyzers) - Roslyn-based implementation of FxCop analyzers.
- ![stars](https://img.shields.io/github/stars/security-code-scan/security-code-scan?style=flat-square&color=ccc) [Roslyn Security Guard](https://security-code-scan.github.io/) - Project that focuses on the identification of potential vulnerabilities such as SQL injection, cross-site scripting (XSS), CSRF, cryptography weaknesses, hardcoded passwords and many more.
- ![stars](https://img.shields.io/github/stars/security-code-scan/security-code-scan?style=flat-square&color=ccc) [Security Code Scan](https://security-code-scan.github.io/) - Security code analyzer for C# and VB.NET. Detects various security vulnerability patterns: SQLi, XSS, CSRF, XXE, Open Redirect, etc. Integrates into Visual Studio 2015 and newer. Detects various security vulnerability patterns: SQLi, XSS, CSRF, XXE, Open Redirect, etc.
- ![stars](https://img.shields.io/github/stars/returntocorp/semgrep?style=flat-square&color=ccc) [Semgrep](https://semgrep.live) - Free, open-source lightweight static analysis for many languages. Find and block bug variants with patterns that look like source code.
- [Semmle QL and LGTM](https://semmle.com/) :copyright: - Find security vulnerabilities, variants, and critical code quality issues using queries over source code. Automatic PR code review; free for public GitHub/Bitbucket repo: [LGTM.com](https://LGTM.com).
- [SensioLabs Insight](https://insight.sensiolabs.com/) :copyright: - Detect security risks, find bugs and provide actionable metrics for PHP projects.
- ![stars](https://img.shields.io/github/stars/google/shipshape?style=flat-square&color=ccc) [shipshape](https://github.com/google/shipshape) :warning: - Static program analysis platform that allows custom analyzers to plug in through a common interface.
- [Sider](https://sider.review) :copyright: - An automated code reviewing tool. Improving developers' productivity.
- [SmartDec Scanner](https://smartdecscanner.com/) :copyright: - SAST tool which is capable of identifying vulnerabilities and undocumented features. The analyzer scans the source code and executables without debug info (i.e. binaries). Supports: Java/Scala/Kotlin, PHP, C#, JavaScript, TypeScript, VBScript, HTML5, Python, Perl, C/C++, Objective-C/Swift, PL/SQL, T-SQL, ABAP, 1C, Apex, Go, Ruby, Groovy, Delphi, VBA, Visual Basic 6, Solidity, Vyper, COBOL.
- [Snyk](https://snyk.io/) :copyright: - Vulnerability scanner for dependencies of node.js apps (free for Open Source Projects).
- [SonarCloud](https://sonarcloud.io) :copyright: - Multilanguage cloud-based static code analysis. History, trends, security hot-spots, pull request analysis and more. Free for open source.
- ![stars](https://img.shields.io/github/stars/SonarSource/sonarlint-visualstudio?style=flat-square&color=ccc) [SonarLint for Visual Studio](https://vs.sonarlint.org/) - SonarLint is an extension for Visual Studio 2015 and 2017 that provides on-the-fly feedback to developers on new bugs and quality issues injected into .NET code.
- ![stars](https://img.shields.io/github/stars/SonarSource/sonarqube?style=flat-square&color=ccc) [SonarQube](http://www.sonarqube.org/) - SonarQube is an open platform to manage code quality.
- ![stars](https://img.shields.io/github/stars/stoplightio/spectral?style=flat-square&color=ccc) [Spectral](https://stoplight.io/open-source/spectral) - A flexible JSON/YAML linter, with out of the box support for OpenAPI v2/v3 and AsyncAPI v2.
- ![stars](https://img.shields.io/github/stars/standard/standard?style=flat-square&color=ccc) [standard](http://standardjs.com/) - An npm module that checks for Javascript Styleguide issues.
- ![stars](https://img.shields.io/github/stars/r-lib/styler?style=flat-square&color=ccc) [styler](https://styler.r-lib.org/) - Formatting of R source code files and pretty-printing of R code.
- ![stars](https://img.shields.io/github/stars/github/super-linter?style=flat-square&color=ccc) [Super-Linter](https://github.com/github/super-linter) - Combination of multiple linters to install as a GitHub Action.
- ![stars](https://img.shields.io/github/stars/nicklockwood/SwiftFormat?style=flat-square&color=ccc) [SwiftFormat](https://github.com/nicklockwood/SwiftFormat) - A library and command-line formatting tool for reformatting Swift code.
- [Synopsys](https://www.synopsys.com/software-integrity/security-testing/static-analysis-sast.html) :copyright: - A commercial static analysis platform that allows for scanning of multiple languages (C/C++, Android, C#, Java, JS, PHP, Python, Node.JS, Ruby, Fortran, and Swift).
- [Teamscale](http://www.teamscale.com/) :copyright: - Static and dynamic analysis tool supporting more than 25 languages and direct IDE integration. Free hosting for Open Source projects available on request. Free academic licenses available.
- ![stars](https://img.shields.io/github/stars/Tencent/TscanCode?style=flat-square&color=ccc) [TscanCode](https://github.com/Tencent/TscanCode) - A fast and accurate static analysis solution for C/C++, C#, Lua codes provided by Tencent. Using GPLv3 license.
- ![stars](https://img.shields.io/github/stars/Yelp/undebt?style=flat-square&color=ccc) [Undebt](https://github.com/Yelp/undebt) - Language-independent tool for massive, automatic, programmable refactoring based on simple pattern definitions.
- ![stars](https://img.shields.io/github/stars/unibeautify/unibeautify?style=flat-square&color=ccc) [Unibeautify](https://unibeautify.com/) - Universal code beautifier with a GitHub app. Supports HTML, CSS, JavaScript, TypeScript, JSX, Vue, C++, Go, Objective-C, Java, Python, PHP, GraphQL, Markdown, and more.
- [Upsource](https://www.jetbrains.com/upsource/) :copyright: - Code review tool with static code analysis and code-aware navigation for Java, PHP, JavaScript and Kotlin.
- [Veracode](http://www.veracode.com/products/static-analysis-sast/static-code-analysis) :copyright: - Find flaws in binaries and bytecode without requiring source. Support all major programming languages: Java, .NET, JavaScript, Swift, Objective-C, C, C++ and more.
- ![stars](https://img.shields.io/github/stars/tomasbjerre/violations-lib?style=flat-square&color=ccc) [Violations Lib](https://github.com/tomasbjerre/violations-lib) - Java library for parsing report files from static code analysis. Used by a bunch of Jenkins, Maven and Gradle plugins.
- ![stars](https://img.shields.io/github/stars/wala/WALA?style=flat-square&color=ccc) [WALA](http://wala.sourceforge.net) - static analysis capabilities for Java bytecode and related languages and for JavaScript.
- [WhiteHat Application Security Platform](https://www.whitehatsec.com/platform/static-application-security-testing/) :copyright: - WhiteHat Scout (for Developers) combined with WhiteHat Sentinel Source (for Operations) supporting WhiteHat Top 40 and OWASP Top 10.
- ![stars](https://img.shields.io/github/stars/fimbullinter/wotan?style=flat-square&color=ccc) [Wotan](https://github.com/fimbullinter/wotan) - Pluggable TypeScript and JavaScript linter.
- [Xanitizer](https://xanitizer.com/) :copyright: - Xanitizer finds security vulnerabilities in Java/Scala web applications.
- [XCode](https://developer.apple.com/xcode/) :copyright: - XCode provides a pretty decent UI for [Clang's](http://clang-analyzer.llvm.org/xcode.html) static code analyzer (C/C++, Obj-C).
- ![stars](https://img.shields.io/github/stars/adrienverge/yamllint?style=flat-square&color=ccc) [yamllint](https://yamllint.readthedocs.io/) - Checks YAML files for syntax validity, key repetition and cosmetic problems such as lines length, trailing spaces, and indentation.


# Other



<h2 id="binary">Binaries</h2>

- ![stars](https://img.shields.io/github/stars/Microsoft/binskim?style=flat-square&color=ccc) [BinSkim](https://github.com/Microsoft/binskim) - A binary static analysis tool that provides security and correctness results for Windows portable executables.
- ![stars](https://img.shields.io/github/stars/fkie-cad/cwe_checker?style=flat-square&color=ccc) [cwe_checker](https://github.com/fkie-cad/cwe_checker) - cwe_checker finds vulnerable patterns in binary executables.
- ![stars](https://img.shields.io/github/stars/jkinder/jakstab?style=flat-square&color=ccc) [Jakstab](https://github.com/jkinder/jakstab) - Jakstab is an Abstract Interpretation-based, integrated disassembly and static analysis framework for designing analyses on executables and recovering reliable control flow graphs.
- ![stars](https://img.shields.io/github/stars/JusticeRage/Manalyze?style=flat-square&color=ccc) [Manalyze](https://github.com/JusticeRage/Manalyze) - A static analyzer, which checks portable executables for malicious content.
- ![stars](https://img.shields.io/github/stars/rustwasm/twiggy?style=flat-square&color=ccc) [Twiggy](https://rustwasm.github.io/twiggy/) - Analyzes a binary's call graph to profile code size. The goal is to slim down binaries.


<h2 id="buildtool">Build tools</h2>

- ![stars](https://img.shields.io/github/stars/mrtazz/checkmake?style=flat-square&color=ccc) [checkmake](https://github.com/mrtazz/checkmake) - Linter / Analyzer for Makefiles.


<h2 id="css">CSS/SASS/SCSS</h2>

- ![stars](https://img.shields.io/github/stars/cssstats/cssstats?style=flat-square&color=ccc) [CSS Stats](https://cssstats.com/) - Potentially interesting stats on stylesheets.
- ![stars](https://img.shields.io/github/stars/CSSLint/csslint?style=flat-square&color=ccc) [CSSLint](http://csslint.net/) - Does basic syntax checking and finds problematic patterns or signs of inefficiency.
- ![stars](https://img.shields.io/github/stars/TheJaredWilcurt/itcss-specificity-graph?style=flat-square&color=ccc) [GraphMyCSS.com](https://graphmycss.com) - CSS Specificity Graph Generator.
- ![stars](https://img.shields.io/github/stars/katiefenn/parker?style=flat-square&color=ccc) [Parker](https://github.com/katiefenn/parker) - Stylesheet analysis tool.
- ![stars](https://img.shields.io/github/stars/projectwallace/css-analyzer?style=flat-square&color=ccc) [Project Wallace CSS Analyzer](https://www.projectwallace.com/) - Analytics for CSS, part of [Project Wallace](https://www.projectwallace.com).
- ![stars](https://img.shields.io/github/stars/sasstools/sass-lint?style=flat-square&color=ccc) [sass-lint](https://github.com/sasstools/sass-lint) :warning: - A Node-only Sass linter for both sass and scss syntax.
- ![stars](https://img.shields.io/github/stars/brigade/scss-lint?style=flat-square&color=ccc) [scsslint](https://github.com/brigade/scss-lint) :warning: - Linter for SCSS files.
- ![stars](https://img.shields.io/github/stars/pocketjoso/specificity-graph?style=flat-square&color=ccc) [Specificity Graph](https://jonassebastianohlsson.com/specificity-graph/) - CSS Specificity Graph Generator.
- ![stars](https://img.shields.io/github/stars/stylelint/stylelint?style=flat-square&color=ccc) [Stylelint](http://stylelint.io/) - Linter for SCSS/CSS files.


<h2 id="configfile">Config Files</h2>

- ![stars](https://img.shields.io/github/stars/wemake-services/dotenv-linter?style=flat-square&color=ccc) [dotenv-linter](https://dotenv-linter.readthedocs.io/en/latest/) - Linting dotenv files like a charm.
- ![stars](https://img.shields.io/github/stars/yandex/gixy?style=flat-square&color=ccc) [gixy](https://github.com/yandex/gixy) - a tool to analyze Nginx configuration. The main goal is to prevent misconfiguration and automate flaw detection.


<h2 id="configmanagement">Configuration Management</h2>

- ![stars](https://img.shields.io/github/stars/willthames/ansible-lint?style=flat-square&color=ccc) [ansible-lint](https://docs.ansible.com/ansible-lint/) - Checks playbooks for practices and behaviour that could potentially be improved.
- ![stars](https://img.shields.io/github/stars/awslabs/cfn-python-lint?style=flat-square&color=ccc) [cfn-lint](https://github.com/awslabs/cfn-python-lint) - AWS Labs CloudFormation linter.
- ![stars](https://img.shields.io/github/stars/stelligent/cfn_nag?style=flat-square&color=ccc) [cfn_nag](https://github.com/stelligent/cfn_nag) - A linter for AWS CloudFormation templates.
- ![stars](https://img.shields.io/github/stars/bridgecrewio/checkov?style=flat-square&color=ccc) [checkov](https://www.checkov.io/) - Static analysis tool for Terraform files (tf>=v0.12), preventing cloud misconfigs at build time.
- ![stars](https://img.shields.io/github/stars/chef/cookstyle?style=flat-square&color=ccc) [cookstyle](https://docs.chef.io/cookstyle.html) - Cookstyle is a linting tool based on the RuboCop Ruby linting tool for Chef cookbooks.
- ![stars](https://img.shields.io/github/stars/foodcritic/foodcritic?style=flat-square&color=ccc) [foodcritic](http://www.foodcritic.io/) - A lint tool that checks Chef cookbooks for common problems.
- ![stars](https://img.shields.io/github/stars/rodjek/puppet-lint?style=flat-square&color=ccc) [Puppet Lint](https://github.com/rodjek/puppet-lint) - Check that your Puppet manifests conform to the style guide.
- ![stars](https://img.shields.io/github/stars/eerkunt/terraform-compliance?style=flat-square&color=ccc) [terraform-compliance](https://terraform-compliance.com) - A lightweight, compliance- and security focused, BDD test framework against Terraform.
- ![stars](https://img.shields.io/github/stars/cesar-rodriguez/terrascan?style=flat-square&color=ccc) [terrascan](https://github.com/cesar-rodriguez/terrascan) - Collection of security and best practice tests for static code analysis of Terraform templates.
- ![stars](https://img.shields.io/github/stars/wata727/tflint?style=flat-square&color=ccc) [tflint](https://github.com/wata727/tflint) - A Terraform linter for detecting errors that can not be detected by `terraform plan`.


<h2 id="container">Containers</h2>

- ![stars](https://img.shields.io/github/stars/anchore/anchore-engine?style=flat-square&color=ccc) [anchore](https://anchore.io/) - Discover, analyze, and certify container images.
- ![stars](https://img.shields.io/github/stars/coreos/clair?style=flat-square&color=ccc) [clair](https://github.com/coreos/clair) - Vulnerability Static Analysis for Containers.
- ![stars](https://img.shields.io/github/stars/banyanops/collector?style=flat-square&color=ccc) [collector](https://github.com/banyanops/collector) - Run arbitrary scripts inside containers, and gather useful information.
- ![stars](https://img.shields.io/github/stars/eliasgranderubio/dagda?style=flat-square&color=ccc) [dagda](https://github.com/eliasgranderubio/dagda) - Perform static analysis of known vulnerabilities in docker images/containers.
- ![stars](https://img.shields.io/github/stars/garethr/docker-label-inspector?style=flat-square&color=ccc) [Docker Label Inspector](https://github.com/garethr/docker-label-inspector) - Lint and validate Dockerfile labels.
- ![stars](https://img.shields.io/github/stars/lukasmartinelli/hadolint?style=flat-square&color=ccc) [Haskell Dockerfile Linter](https://github.com/lukasmartinelli/hadolint) - A smarter Dockerfile linter that helpsyou build best practice Docker images.
- ![stars](https://img.shields.io/github/stars/zegl/kube-score?style=flat-square&color=ccc) [kube-score](https://kube-score.com/) - Static code analysis of your Kubernetes object definitions.
- ![stars](https://img.shields.io/github/stars/instrumenta/kubeval?style=flat-square&color=ccc) [kubeval](https://kubeval.instrumenta.dev/) - Validates your Kubernetes configuration files and supports multiple Kubernetes versions.


<h2 id="gherkin">Gherkin</h2>

- ![stars](https://img.shields.io/github/stars/vsiakka/gherkin-lint?style=flat-square&color=ccc) [gherkin-lint](https://github.com/vsiakka/gherkin-lint) - A linter for the Gherkin-Syntax written in Javascript.


<h2 id="html">HTML</h2>

- ![stars](https://img.shields.io/github/stars/philipwalton/html-inspector?style=flat-square&color=ccc) [HTML Inspector](https://github.com/philipwalton/html-inspector) :warning: - HTML Inspector is a code quality tool to help you and your team write better markup.
- ![stars](https://img.shields.io/github/stars/htacg/tidy-html5?style=flat-square&color=ccc) [HTML Tidy](http://www.html-tidy.org/) - Corrects and cleans up HTML and XML documents by fixing markup errors and upgrading legacy code to modern standards.
- ![stars](https://img.shields.io/github/stars/yaniswang/HTMLHint?style=flat-square&color=ccc) [HTMLHint](https://htmlhint.com/) - A Static Code Analysis Tool for HTML.


<h2 id="ide">IDE Plugins</h2>

- ![stars](https://img.shields.io/github/stars/w0rp/ale?style=flat-square&color=ccc) [ale](https://github.com/w0rp/ale) - Asynchronous Lint Engine for Vim and NeoVim with support for many languages.
- [Android Studio](https://developer.android.com/studio) - Based on IntelliJ IDEA, and comes bundled with tools for Android including Android Lint.
- [Attackflow Extension](https://www.attackflow.com/Extension) :copyright: - Attackflow plugin for Visual Studio, which enables developers to find critical security bugs at real time in the source code without any prior knowledge.
- ![stars](https://img.shields.io/github/stars/Microsoft/DevSkim?style=flat-square&color=ccc) [DevSkim](https://github.com/Microsoft/DevSkim) - Inline, realtime security analysis. Works with multiple programming languages and IDEs (VS, VS Code, Sublime Text, ...).
- [IntelliJ IDEA](https://www.jetbrains.com/idea/) :copyright: - Comes bundled with a lot of inspections for Java and Kotlin and includes tools for refactoring, formatting and more.
- ![stars](https://img.shields.io/github/stars/Kuniwak/vint?style=flat-square&color=ccc) [vint](https://github.com/Kuniwak/vint) - Fast and Highly Extensible Vim script Language Lint implemented by Python.


<h2 id="latex">LaTeX</h2>

- [ChkTeX](http://www.nongnu.org/chktex/) - A linter for LaTex which catches some typographic errors LaTeX oversees.
- [lacheck](https://www.ctan.org/pkg/lacheck) - A tool for finding common mistakes in LaTeX documents.
- ![stars](https://img.shields.io/github/stars/latex-lsp/texlab?style=flat-square&color=ccc) [TeXLab](https://texlab.netlify.app) - A Language Server Protocol implementation for TeX/LaTeX, including lint capabilities.


<h2 id="make">Makefiles</h2>

- [portlint](https://www.freebsd.org/cgi/man.cgi?query=portlint&sektion=1&manpath=FreeBSD+8.1-RELEASE+and+Ports) - A verifier for FreeBSD and DragonFlyBSD port directories.


<h2 id="markdown">Markdown</h2>

- ![stars](https://img.shields.io/github/stars/DavidAnson/markdownlint?style=flat-square&color=ccc) [markdownlint](https://github.com/DavidAnson/markdownlint) - Node.js -based style checker and lint tool for Markdown/CommonMark files.
- ![stars](https://img.shields.io/github/stars/mivok/markdownlint?style=flat-square&color=ccc) [mdl](https://github.com/mivok/markdownlint) - A tool to check Markdown files and flag style issues.
- ![stars](https://img.shields.io/github/stars/remarkjs/remark-lint?style=flat-square&color=ccc) [remark-lint](https://remark.js.org/) - Pluggable Markdown code style linter written in JavaScript.


<h2 id="mobile">Mobile</h2>

- [Android Lint](http://tools.android.com/tips/lint) - Run static analysis on Android projects.
- ![stars](https://img.shields.io/github/stars/passy/android-lint-summary?style=flat-square&color=ccc) [android-lint-summary](https://passy.github.io/android-lint-summary/) - Combines lint errors of multiple projects into one output, check lint results of multiple sub-projects at once.
- ![stars](https://img.shields.io/github/stars/secure-software-engineering/FlowDroid?style=flat-square&color=ccc) [FlowDroid](https://github.com/secure-software-engineering/FlowDroid) - static taint analysis tool for Android applications.
- ![stars](https://img.shields.io/github/stars/GeoffreyHecht/paprika?style=flat-square&color=ccc) [paprika](https://github.com/GeoffreyHecht/paprika) - A toolkit to detect some code smells in analyzed Android applications.
- ![stars](https://img.shields.io/github/stars/linkedin/qark?style=flat-square&color=ccc) [qark](https://github.com/linkedin/qark) - Tool to look for several security related Android application vulnerabilities.


<h2 id="package">Packages</h2>

- ![stars](https://img.shields.io/github/stars/Debian/lintian?style=flat-square&color=ccc) [lintian](https://lintian.debian.org/) - Static analysis tool for Debian packages.
- ![stars](https://img.shields.io/github/stars/rpm-software-management/rpmlint?style=flat-square&color=ccc) [rpmlint](https://github.com/rpm-software-management/rpmlint) - Tool for checking common errors in rpm packages.


<h2 id="protobuf">Protocol Buffers</h2>

- ![stars](https://img.shields.io/github/stars/yoheimuta/protolint?style=flat-square&color=ccc) [protolint](https://github.com/yoheimuta/protolint) - Pluggable linter and fixer to enforce Protocol Buffer style and conventions.


<h2 id="support">Supporting Tools</h2>

- ![stars](https://img.shields.io/github/stars/uni-bremen-agst/libvcs4j?style=flat-square&color=ccc) [LibVCS4j](https://github.com/uni-bremen-agst/libvcs4j) - A Java library that allows existing tools to analyse the evolution of software systems by providing a common API for different version control systems and issue trackers.


<h2 id="template">Template-Languages</h2>

- ![stars](https://img.shields.io/github/stars/ember-template-lint/ember-template-lint?style=flat-square&color=ccc) [ember-template-lint](https://github.com/ember-template-lint/ember-template-lint) - Linter for Ember or Handlebars templates.
- ![stars](https://img.shields.io/github/stars/sds/haml-lint?style=flat-square&color=ccc) [haml-lint](https://github.com/sds/haml-lint) - Tool for writing clean and consistent HAML.
- ![stars](https://img.shields.io/github/stars/sds/slim-lint?style=flat-square&color=ccc) [slim-lint](https://github.com/sds/slim-lint) - Configurable tool for analyzing Slim templates.


<h2 id="translation">Translation</h2>

- ![stars](https://img.shields.io/github/stars/willkg/dennis?style=flat-square&color=ccc) [dennis](https://github.com/willkg/dennis/) - A set of utilities for working with PO files to ease development and improve quality.


<h2 id="service">Web services</h2>

- [Codacy](https://www.codacy.com/) :copyright: - Code Analysis to ship Better Code, Faster.
- [Code Climate](https://codeclimate.com/) :copyright: - The open and extensible static analysis platform, for everyone.
- [Code Inspector](https://www.code-inspector.com) :copyright: - Code quality and technical debt management platform that supports 10+ languages.
- [CodeFactor](https://codefactor.io) :copyright: - Automated Code Analysis for repos on GitHub or BitBucket.
- [CodeFlow](https://www.getcodeflow.com) :copyright: - Automated code analysis tool to deal with technical depth. Integrates with Bitbucket and Gitlab. (free for Open Source Projects)
- [kiuwan](https://www.kiuwan.com/) :copyright: - Software Analytics in the Cloud supporting more than 22 programming languages.
- [Landscape](https://landscape.io/) :warning: :copyright: - Static code analysis for Python.
- [Reshift](https://www.reshiftsecurity.com/) :copyright: - A source code analysis tool for detecting and managing Java security vulnerabilities.
- [Scrutinizer](https://scrutinizer-ci.com/) :copyright: - A proprietary code quality checker that can be integrated with GitHub.


<h2 id="writing">Writing</h2>

- [After the Deadline](https://afterthedeadline.com/) :warning: - spell, style and grammar checker.
- ![stars](https://img.shields.io/github/stars/codespell-project/codespell?style=flat-square&color=ccc) [codespell](https://github.com/codespell-project/codespell) - check code for common misspellings.
- ![stars](https://img.shields.io/github/stars/languagetool-org/languagetool?style=flat-square&color=ccc) [languagetool](https://languagetool.org/) - Style and grammar checker for 25+ languages. It finds many errors that a simple spell checker cannot detect.
- ![stars](https://img.shields.io/github/stars/vlajos/misspell-fixer?style=flat-square&color=ccc) [misspell-fixer](https://github.com/vlajos/misspell-fixer) - Quick tool for fixing common misspellings, typos in source code.
- ![stars](https://img.shields.io/github/stars/jwilk/mwic?style=flat-square&color=ccc) [Misspelled Words In Context](http://jwilk.net/software/mwic) - a spell-checker that groups possible misspellings and shows them in their contexts.
- ![stars](https://img.shields.io/github/stars/amperser/proselint?style=flat-square&color=ccc) [proselint](http://proselint.com/) - a linter for English prose with a focus on writing style instead of grammar.
- [vale](https://github.com/ValeLint/vale) - A customizable, syntax-aware linter for prose.
- ![stars](https://img.shields.io/github/stars/btford/write-good?style=flat-square&color=ccc) [write-good](https://github.com/btford/write-good) - A linter with a focus on eliminating "weasel words".


# More collections

- [go-tools](https://github.com/dominikh/go-tools) - A collection of tools and libraries for working with Go code, including linters and static analysis
- [linters](https://github.com/mcandre/linters/tree/b044f0628c4a96dfea869cf61e0e96cf4c49cf6b) - An introduction to static code analysis
- [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) -  A reviewed list of useful PHP static analysis tools
- [Tools for C/C++](https://www.peerlyst.com/posts/a-list-of-static-analysis-tools-for-c-c-peerlyst?utm_source=twitter&utm_medium=social&utm_content=peerlyst_post&utm_campaign=peerlyst_resources) - A list of static analysis tools for C/C++
- [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) -  A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).
