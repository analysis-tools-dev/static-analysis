<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

 <a href="https://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.png" />
 </a>

This repository lists **static analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

![CI](https://github.com/analysis-tools-dev/static-analysis/workflows/CI/badge.svg)

## What is Static Analysis?

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

> The most important thing I have done as a programmer in recent years is to aggressively pursue static code analysis. Even more valuable than the hundreds of serious bugs I have prevented with it is the change in mindset about the way I view software reliability and code quality. — [John Carmack (Creator of Doom)](https://www.gamasutra.com/view/news/128836/InDepth_Static_Code_Analysis.php)

## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
  <tr>
    <td><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepcode.png" /></a></td>
    <td><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codescene.svg" /></a></td>
    <td><a href="https://r2c.dev/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/r2c.svg" /></a></td>
    <td><a href="https://codiga.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codiga.svg" /></a></td>
    <td><a href="https://offensive360.com/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/offensive360.png" /></a></td>
  </tr>
</table>

If you also want to support this project, head over to our [Github sponsors page](https://github.com/sponsors/analysis-tools-dev).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

## Table of Contents

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
    <li><a href="#clojure">Clojure</a></li>
    <li><a href="#coffeescript">CoffeeScript</a></li>
    <li><a href="#crystal">Crystal</a></li>
    <li><a href="#dart">Dart</a></li>
    <li><a href="#delphi">Delphi</a></li>
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
    <li><a href="#julia">Julia</a></li>
    <li><a href="#kotlin">Kotlin</a></li>
    <li><a href="#lua">Lua</a></li>
    <li><a href="#matlab">MATLAB</a></li>
    <li><a href="#nim">Nim</a></li>
    <li><a href="#ocaml">Ocaml</a></li>
    <li><a href="#php">PHP</a></li>
    <li><a href="#plsql">PL/SQL</a></li>
    <li><a href="#perl">Perl</a></li>
    <li><a href="#python">Python</a></li>
    <li><a href="#r">R</a></li>
    <li><a href="#ruby">Ruby</a></li>
    <li><a href="#rust">Rust</a></li>
    <li><a href="#sql">SQL</a></li>
    <li><a href="#scala">Scala</a></li>
    <li><a href="#shell">Shell</a></li>
    <li><a href="#swift">Swift</a></li>
    <li><a href="#tcl">Tcl</a></li>
    <li><a href="#typescript">TypeScript</a></li>
    <li><a href="#verilog">Verilog/SystemVerilog</a></li>
    <li><a href="#vim-script">Vim Script</a></li>
    </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

- [Angular](#angular)
- [Ansible](#ansible)
- [Azure Resource Manager](#arm)
- [Binaries](#binary)
- [Build tools](#buildtool)
- [CSS/SASS/SCSS](#css)
- [Config Files](#configfile)
- [Configuration Management](#configmanagement)
- [Containers](#container)
- [Continuous Integration](#ci)
- [Deno](#deno)
- [Embedded](#embedded)
- [Embedded Ruby (a.k.a. ERB, eRuby)](#erb)
- [Gherkin](#gherkin)
- [HTML](#html)
- [JSON](#json)
- [Kubernetes](#kubernetes)
- [LaTeX](#latex)
- [Laravel](#laravel)
- [Makefiles](#make)
- [Markdown](#markdown)
- [Metalinter](#meta)
- [Mobile](#mobile)
- [Nix](#nix)
- [Node.js](#nodejs)
- [Packages](#package)
- [Protocol Buffers](#protobuf)
- [Puppet](#puppet)
- [Rails](#rails)
- [Security/SAST](#security)
- [Smart Contracts](#smart-contracts)
- [Support](#support)
- [Template-Languages](#template)
- [Terraform](#terraform)
- [Translation](#translation)
- [Vue.js](#vue)
- [Webassembly](#wasm)
- [Writing](#writing)
- [YAML](#yaml)
- [git](#git)

---

## Programming Languages

<h2 id="abap">ABAP</h2>

- [abaplint](https://abaplint.org) — Linter for ABAP, written in TypeScript.
- [abapOpenChecks](https://docs.abapopenchecks.org) — Enhances the SAP Code Inspector with new and customizable checks.


<h2 id="ada">Ada</h2>

- [Codepeer](http://www.adacore.com/codepeer) :copyright: — Detects run-time and logic errors.
- [Polyspace for Ada](https://www.mathworks.com/products/polyspace-ada.html) :copyright: — Provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in source code.
- [SPARK](https://www.adacore.com/about-spark) :copyright: — Static analysis and formal verification toolset for Ada.


<h2 id="asm">Assembly</h2>

- [STOKE](https://github.com/StanfordPL/stoke) :warning: — A programming-language agnostic stochastic optimizer for the x86_64 instruction set. It uses random search to explore the extremely high-dimensional space of all possible program transformations.


<h2 id="awk">Awk</h2>

- [gawk --lint](https://www.gnu.org/software/gawk/manual/html_node/Options.html) — Warns about constructs that are dubious or nonportable to other awk implementations.


<h2 id="c">C</h2>

- [Astrée](https://www.absint.com/astree/index.htm) :copyright: — Astrée automatically proves the absence of runtime errors and invalid con­current behavior in C/C++ applications. It is sound for floating-point computations, very fast, and exceptionally precise. The analyzer also checks for MISRA/CERT/CWE/Adaptive Autosar coding rules and supports qualification for ISO 26262, DO-178C level A, and other safety standards. Jenkins and Eclipse plugins are available.
- [CBMC](http://www.cprover.org/cbmc) — Bounded model-checker for C programs, user-defined assertions, standard assertions, several coverage metric analyses.
- [clang-tidy](http://clang.llvm.org/extra/clang-tidy) — clang static analyser.
- [clazy](https://github.com/KDE/clazy) — Qt-oriented static code analyzer based on the Clang framework. clazy is a compiler plugin which allows clang to understand Qt semantics. You get more than 50 Qt related compiler warnings, ranging from unneeded memory allocations to misusage of API, including fix-its for automatic refactoring.
- [CMetrics](https://github.com/MetricsGrimoire/CMetrics) — Measures size and complexity for C files.
- [CPAchecker](https://cpachecker.sosy-lab.org) — A tool for configurable software verification of C programs.  The name CPAchecker was chosen to reflect that the tool is based on the CPA concepts and is used for checking software programs.
- [cppcheck](http://cppcheck.sourceforge.net) — Static analysis of C/C++ code.
- [CppDepend](https://www.cppdepend.com) :warning: :copyright: — Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [cpplint](https://github.com/google/styleguide/tree/gh-pages/cpplint) — Automated C++ checker that follows Google's style guide.
- [cqmetrics](https://github.com/dspinellis/cqmetrics) — Quality metrics for C code.
- [CScout](https://www.spinellis.gr/cscout) :warning: — Complexity and quality metrics for for C and C preprocessor code.
- [ESBMC](http://esbmc.org) — ESBMC is an open source, permissively licensed, context-bounded model checker based on satisfiability modulo theories for the verification of single- and multi-threaded C/C++ programs.
- [flawfinder](http://dwheeler.com/flawfinder/) — Finds possible security weaknesses.
- [flint++](https://github.com/JossWhittle/FlintPlusPlus) — Cross-platform, zero-dependency port of flint, a lint program for C++ developed and used at Facebook.
- [Frama-C](http://frama-c.com) — A sound and extensible static analyzer for C code.
- [Helix QAC](https://www.perforce.com/products/helix-qac) :copyright: — Enterprise-grade static analysis for embedded software. Supports MISRA, CERT, and AUTOSAR coding standards.
- [IKOS](https://github.com/nasa-sw-vnv/ikos) — A sound static analyzer for C/C++ code based on LLVM.
- [Joern](https://joern.io) — Open-source code analysis platform for C/C++ based on code property graphs
- [LDRA](https://ldra.com) :copyright: — A tool suite including static analysis (TBVISION) to various standards including MISRA C & C++, JSF++ AV, CWE, CERT C, CERT C++ & Custom Rules.
- [PC-lint](https://www.gimpel.com) :copyright: — Static analysis for C/C++. Runs natively under Windows/Linux/MacOS. Analyzes code for virtually any platform, supporting C11/C18 and C++17.
- [Phasar](https://phasar.org) — A LLVM-based static analysis framework which comes with a taint and type state analysis.
- [Polyspace Bug Finder](https://www.mathworks.com/products/polyspace-bug-finder.html) :copyright: — Identifies run-time errors, concurrency issues, security vulnerabilities, and other defects in C and C++ embedded software.
- [Polyspace Code Prover](https://www.mathworks.com/products/polyspace-code-prover.html) :copyright: — Provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in C and C++ source code.
- [scan-build](https://clang-analyzer.llvm.org/scan-build.html) — Analyzes C/C++ code using LLVM at compile-time.
- [splint](http://splint.org) — Annotation-assisted static program checker.
- [SVF](http://svf-tools.github.io/SVF) — A static tool that enables scalable and precise interprocedural dependence analysis for C and C++ programs.
- [vera++](https://bitbucket.org/verateam/vera/wiki/Introduction) — Vera++ is a programmable tool for verification, analysis and transformation of C++ source code.


<h2 id="csharp">C#</h2>

- [.NET Analyzers](https://github.com/DotNetAnalyzers) — An organization for the development of analyzers (diagnostics and code fixes) using the .NET Compiler Platform.
- [ArchUnitNET](https://github.com/TNG/ArchUnitNET) — A C# architecture test library to specify and assert architecture rules in C# for automated testing.
- [code-cracker](http://code-cracker.github.io) — An analyzer library for C# and VB that uses Roslyn to produce refactorings, code analysis, and other niceties.
- [CSharpEssentials](https://github.com/DustinCampbell/CSharpEssentials) :warning: — C# Essentials is a collection of Roslyn diagnostic analyzers, code fixes and refactorings that make it easy to work with C# 6 language features.
- [Designite](http://www.designite-tools.com) :copyright: — Designite supports detection of various architecture, design, and implementation smells, computation of various code quality metrics, and trend analysis.
- [Gendarme](https://www.mono-project.com/docs/tools+libraries/tools/gendarme) — Gendarme inspects programs and libraries that contain code in ECMA CIL format (Mono and .NET).
- [Infer#](https://github.com/microsoft/infersharp) — InferSharp (also referred to as Infer#) is an interprocedural and  scalable static code analyzer for C#. Via the capabilities of Facebook's Infer,  this tool detects null pointer dereferences and resource leaks.
- [NDepend](http://www.ndepend.com) :copyright: — Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [Puma Scan](https://pumasecurity.io) — Puma Scan provides real time secure code analysis for common vulnerabilities (XSS, SQLi, CSRF, LDAPi, crypto, deserialization, etc.) as development teams write code in Visual Studio.
- [Roslynator](https://github.com/JosefPihrt/Roslynator) — A collection of 190+ analyzers and 190+ refactorings for C#, powered by Roslyn.
- [VSDiagnostics](https://github.com/Vannevelj/VSDiagnostics) :warning: — A collection of static analyzers based on Roslyn that integrates with VS.
- [Wintellect.Analyzers](https://github.com/Wintellect/Wintellect.Analyzers) — .NET Compiler Platform ("Roslyn") diagnostic analyzers and code fixes.


<h2 id="cpp">C++</h2>

- [Astrée](https://www.absint.com/astree/index.htm) :copyright: — Astrée automatically proves the absence of runtime errors and invalid con­current behavior in C/C++ applications. It is sound for floating-point computations, very fast, and exceptionally precise. The analyzer also checks for MISRA/CERT/CWE/Adaptive Autosar coding rules and supports qualification for ISO 26262, DO-178C level A, and other safety standards. Jenkins and Eclipse plugins are available.
- [CBMC](http://www.cprover.org/cbmc) — Bounded model-checker for C programs, user-defined assertions, standard assertions, several coverage metric analyses.
- [clang-tidy](http://clang.llvm.org/extra/clang-tidy) — clang static analyser.
- [clazy](https://github.com/KDE/clazy) — Qt-oriented static code analyzer based on the Clang framework. clazy is a compiler plugin which allows clang to understand Qt semantics. You get more than 50 Qt related compiler warnings, ranging from unneeded memory allocations to misusage of API, including fix-its for automatic refactoring.
- [CMetrics](https://github.com/MetricsGrimoire/CMetrics) — Measures size and complexity for C files.
- [cppcheck](http://cppcheck.sourceforge.net) — Static analysis of C/C++ code.
- [CppDepend](https://www.cppdepend.com) :warning: :copyright: — Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [cpplint](https://github.com/google/styleguide/tree/gh-pages/cpplint) — Automated C++ checker that follows Google's style guide.
- [cqmetrics](https://github.com/dspinellis/cqmetrics) — Quality metrics for C code.
- [CScout](https://www.spinellis.gr/cscout) :warning: — Complexity and quality metrics for for C and C preprocessor code.
- [ESBMC](http://esbmc.org) — ESBMC is an open source, permissively licensed, context-bounded model checker based on satisfiability modulo theories for the verification of single- and multi-threaded C/C++ programs.
- [flawfinder](http://dwheeler.com/flawfinder/) — Finds possible security weaknesses.
- [flint++](https://github.com/JossWhittle/FlintPlusPlus) — Cross-platform, zero-dependency port of flint, a lint program for C++ developed and used at Facebook.
- [Frama-C](http://frama-c.com) — A sound and extensible static analyzer for C code.
- [Helix QAC](https://www.perforce.com/products/helix-qac) :copyright: — Enterprise-grade static analysis for embedded software. Supports MISRA, CERT, and AUTOSAR coding standards.
- [IKOS](https://github.com/nasa-sw-vnv/ikos) — A sound static analyzer for C/C++ code based on LLVM.
- [Joern](https://joern.io) — Open-source code analysis platform for C/C++ based on code property graphs
- [LDRA](https://ldra.com) :copyright: — A tool suite including static analysis (TBVISION) to various standards including MISRA C & C++, JSF++ AV, CWE, CERT C, CERT C++ & Custom Rules.
- [PC-lint](https://www.gimpel.com) :copyright: — Static analysis for C/C++. Runs natively under Windows/Linux/MacOS. Analyzes code for virtually any platform, supporting C11/C18 and C++17.
- [Phasar](https://phasar.org) — A LLVM-based static analysis framework which comes with a taint and type state analysis.
- [Polyspace Bug Finder](https://www.mathworks.com/products/polyspace-bug-finder.html) :copyright: — Identifies run-time errors, concurrency issues, security vulnerabilities, and other defects in C and C++ embedded software.
- [Polyspace Code Prover](https://www.mathworks.com/products/polyspace-code-prover.html) :copyright: — Provide code verification that proves the absence of overflow, divide-by-zero, out-of-bounds array access, and certain other run-time errors in C and C++ source code.
- [scan-build](https://clang-analyzer.llvm.org/scan-build.html) — Analyzes C/C++ code using LLVM at compile-time.
- [splint](http://splint.org) — Annotation-assisted static program checker.
- [SVF](http://svf-tools.github.io/SVF) — A static tool that enables scalable and precise interprocedural dependence analysis for C and C++ programs.
- [vera++](https://bitbucket.org/verateam/vera/wiki/Introduction) — Vera++ is a programmable tool for verification, analysis and transformation of C++ source code.


<h2 id="clojure">Clojure</h2>

- [clj-kondo](https://github.com/borkdude/clj-kondo) — A linter for Clojure code that sparks joy. It informs you about potential errors while you are typing.


<h2 id="coffeescript">CoffeeScript</h2>

- [coffeelint](http://www.coffeelint.org) — A style checker that helps keep CoffeeScript code clean and consistent.


<h2 id="crystal">Crystal</h2>

- [ameba](https://crystal-ameba.github.io) — A static code analysis tool for Crystal.
- [crystal](https://crystal-lang.org) — The Crystal compiler has built-in linting functionality.


<h2 id="dart">Dart</h2>

- [Dart Code Metrics](https://pub.dev/packages/dart_code_metrics) — Additional linter for Dart. Reports code metrics, checks for anti-patterns and provides additional rules for Dart analyzer.
- [effective_dart](https://pub.dev/packages/effective_dart) — Linter rules corresponding to the guidelines in Effective Dart
- [lint](https://github.com/passsy/dart-lint) — An opinionated, community-driven set of lint rules for Dart and Flutter projects. Like pedantic but stricter
- [Linter for dart](https://dart-lang.github.io/linter) — Style linter for Dart.


<h2 id="delphi">Delphi</h2>

- [Fix Insight](https://www.tmssoftware.com/site/fixinsight.asp) :copyright: — A free IDE Plugin for static code analysis. A _Pro_ edition includes a command line tool for automation purposes.
- [Pascal Analyzer](https://peganza.com/products_pal.html) :copyright: — A static code analysis tool with numerous reports. A free _Lite_ version is available with limited reporting.
- [Pascal Expert](https://peganza.com/products_pex.html) :copyright: — IDE plugin for code analysis. Includes a subset of Pascal Analyzer reporting capabilities and is available for Delphi versions 2007 and later.


<h2 id="dlang">Dlang</h2>

- [D-scanner](https://github.com/dlang-community/D-Scanner) — D-Scanner is a tool for analyzing D source code.


<h2 id="elixir">Elixir</h2>

- [credo](https://github.com/rrrene/credo) — A static code analysis tool with a focus on code consistency and teaching.
- [dialyxir](https://github.com/jeremyjh/dialyxir) — Mix tasks to simplify use of Dialyzer in Elixir projects.
- [sobelow](https://github.com/nccgroup/sobelow) — Security-focused static analysis for the Phoenix Framework.


<h2 id="elm">Elm</h2>

- [elm-analyse](https://stil4m.github.io/elm-analyse) :warning: — A tool that allows you to analyse your Elm code, identify deficiencies and apply best practices.
- [elm-review](https://package.elm-lang.org/packages/jfmengels/elm-review/latest) — Analyzes whole Elm projects, with a focus on shareable and custom rules written in Elm that add guarantees the Elm compiler doesn't give you.


<h2 id="erlang">Erlang</h2>

- [dialyzer](https://www.erlang.org/doc/man/dialyzer.html) — The DIALYZER, a DIscrepancy AnaLYZer for ERlang programs. Dialyzer is a static analysis tool that identifies software discrepancies,  such as definite type errors, code that has become dead or unreachable  because of programming error, and unnecessary tests,  in single Erlang modules or entire (sets of) applications.
Dialyzer starts its analysis from either debug-compiled BEAM bytecode  or from Erlang source code. The file and line number of a discrepancy  is reported along with an indication of what the discrepancy is about.  Dialyzer bases its analysis on the concept of success typings,  which allows for sound warnings (no false positives).
- [elvis](https://github.com/inaka/elvis) — Erlang Style Reviewer.
- [Primitive Erlang Security Tool (PEST)](https://github.com/okeuday/pest) — A tool to do a basic scan of Erlang source code and report any function calls that may cause Erlang source code to be insecure.


<h2 id="fsharp">F#</h2>

- [FSharpLint](http://fsprojects.github.io/FSharpLint) — Lint tool for F#.


<h2 id="fortran">Fortran</h2>

- [i-Code CNES for Fortran](https://github.com/lequal/i-CodeCNES) :warning: — An open source static code analysis tool for Fortran 77, Fortran 90 and Shell.


<h2 id="go">Go</h2>

- [aligncheck](https://gitlab.com/opennota/check) — Find inefficiently packed structs.
- [bodyclose](https://github.com/timakin/bodyclose) — Checks whether HTTP response body is closed.
- [deadcode](https://github.com/tsenart/deadcode) — Finds unused code.
- [dingo-hunter](https://github.com/nickng/dingo-hunter) :warning: — Static analyser for finding deadlocks in Go.
- [dogsled](https://github.com/alexkohler/dogsled) — Finds assignments/declarations with too many blank identifiers.
- [dupl](https://github.com/mibk/dupl) :warning: — Reports potentially duplicated code.
- [errcheck](https://github.com/kisielk/errcheck) — Check that error return values are used.
- [errwrap](https://github.com/fatih/errwrap) — Wrap and fix Go errors with the new %w verb directive.  This tool analyzes fmt.Errorf() calls and reports calls that contain a verb directive that  is different than the new %w verb directive introduced in Go v1.13.  It's also capable of rewriting calls to use the new %w wrap verb directive.
- [flen](https://github.com/lafolle/flen) — Get info on length of functions in a Go package.
- [Go Meta Linter](https://github.com/alecthomas/gometalinter) :warning: — Concurrently run Go lint tools and normalise their output. Use `golangci-lint` for new projects.
- [go tool vet --shadow](https://golang.org/cmd/vet#hdr-Shadowed_variables) — Reports variables that may have been unintentionally shadowed.
- [go vet](https://golang.org/cmd/vet) — Examines Go source code and reports suspicious.
- [go-consistent](https://github.com/Quasilyte/go-consistent) :warning: — Analyzer that helps you to make your Go programs more consistent.
- [go-critic](https://github.com/go-critic/go-critic) — Go source code linter that maintains checks which are currently not implemented in other linters.
- [go/ast](https://golang.org/pkg/go/ast) — Package ast declares the types used to represent syntax trees for Go packages.
- [gochecknoglobals](https://github.com/leighmcculloch/gochecknoglobals) — Checks that no globals are present.
- [goconst](https://github.com/jgautheron/goconst) — Finds repeated strings that could be replaced by a constant.
- [gocyclo](https://github.com/fzipp/gocyclo) — Calculate cyclomatic complexities of functions in Go source code.
- [gofmt -s](https://golang.org/cmd/gofmt) — Checks if the code is properly formatted and could not be further simplified.
- [goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports) — Checks missing or unreferenced package imports.
- [gokart](https://github.com/praetorian-inc/gokart) — Golang security analysis with a focus on minimizing false positives. It is capable of tracing the source of variables and function arguments  to determine whether input sources are safe.
- [GolangCI-Lint](https://golangci-lint.run) — Alternative to `Go Meta Linter`: GolangCI-Lint is a linters aggregator.
- [golint](https://github.com/golang/lint) — Prints out coding style mistakes in Go source code.
- [goreporter](https://github.com/360EntSecGroup-Skylar/goreporter) — Concurrently runs many linters and normalises their output to a report.
- [goroutine-inspect](https://github.com/linuxerwang/goroutine-inspect) — An interactive tool to analyze Golang goroutine dump.
- [gosec (gas)](https://securego.io) — Inspects source code for security problems by scanning the Go AST.
- [gotype](https://pkg.go.dev/golang.org/x/tools/cmd/gotype) — Syntactic and semantic analysis similar to the Go compiler.
- [ineffassign](https://github.com/gordonklaus/ineffassign) — Detect ineffectual assignments in Go code.
- [interfacer](https://github.com/mvdan/interfacer) :warning: — Suggest narrower interfaces that can be used.
- [lll](https://github.com/walle/lll) :warning: — Report long lines.
- [maligned](https://github.com/mdempsky/maligned) — Detect structs that would take less memory if their fields were sorted.
- [misspell](https://github.com/client9/misspell) — Finds commonly misspelled English words.
- [nakedret](https://github.com/alexkohler/nakedret) — Finds naked returns.
- [nargs](https://github.com/alexkohler/nargs) — Finds unused arguments in function declarations.
- [prealloc](https://github.com/alexkohler/prealloc) — Finds slice declarations that could potentially be preallocated.
- [Reviewdog](https://github.com/haya14busa/reviewdog) — A tool for posting review comments from any linter in any code hosting service.
- [revive](https://revive.run) — Fast, configurable, extensible, flexible, and beautiful linter for Go. Drop-in replacement of golint.
- [safesql](https://github.com/stripe/safesql) — Static analysis tool for Golang that protects against SQL injections.
- [shisho](https://docs.shisho.dev/) — A lightweight static code analyzer designed for developers and security teams. It allows you to analyze and transform source code with an intuitive DSL similar to sed, but for code.
- [staticcheck](https://staticcheck.io) — Go static analysis that specialises in finding bugs, simplifying code and improving performance.
- [structcheck](https://gitlab.com/opennota/check) — Find unused struct fields.
- [structslop](https://github.com/orijtech/structslop) — Static analyzer for Go that recommends struct field rearrangements to provide for maximum space/allocation efficiency
- [test](http://golang.org/pkg/testing) — Show location of test failures from the stdlib testing module.
- [unconvert](https://github.com/mdempsky/unconvert) :warning: — Detect redundant type conversions.
- [unparam](https://github.com/mvdan/unparam) — Find unused function parameters.
- [varcheck](https://gitlab.com/opennota/check) — Find unused global variables and constants.
- [wsl](https://github.com/bombsimon/wsl) — Enforces empty lines at the right places.


<h2 id="groovy">Groovy</h2>

- [CodeNarc](https://codenarc.github.io/CodeNarc) — A static analysis tool for Groovy source code, enabling monitoring and enforcement of many coding standards and best practices.


<h2 id="haskell">Haskell</h2>

- [brittany](https://github.com/lspitzner/brittany) — Haskell source code formatter
- [HLint](https://github.com/ndmitchell/hlint) — HLint is a tool for suggesting possible improvements to Haskell code.
- [Liquid Haskell](https://ucsd-progsys.github.io/liquidhaskell-blog/) — Liquid Haskell is a refinement type checker for Haskell programs.
- [Stan](https://kowainik.github.io/projects/stan) — Stan is a command-line tool for analysing Haskell projects and outputting discovered vulnerabilities in a helpful way with possible solutions for detected problems.
- [Weeder](https://github.com/ocharles/weeder) — A tool for detecting dead exports or package imports in Haskell code.


<h2 id="haxe">Haxe</h2>

- [Haxe Checkstyle](http://haxecheckstyle.github.io/docs/haxe-checkstyle/home.html) — A static analysis tool to help developers write Haxe code that adheres to a coding standard.


<h2 id="java">Java</h2>

- [Checker Framework](https://checkerframework.org) — Pluggable type-checking for Java.
- [checkstyle](https://checkstyle.org) — Checking Java source code for adherence to a Code Standard or set of validation rules (best practices).
- [ck](https://github.com/mauricioaniche/ck) — Calculates Chidamber and Kemerer object-oriented metrics by processing the source Java files.
- [ckjm](http://www.spinellis.gr/sw/ckjm) — Calculates Chidamber and Kemerer object-oriented metrics by processing the bytecode of compiled Java files.
- [CogniCrypt](https://www.eclipse.org/cognicrypt) — Checks Java source and byte code for incorrect uses of cryptographic APIs.
- [DesigniteJava](http://www.designite-tools.com/designitejava) :copyright: — DesigniteJava supports detection of various architecture, design, and implementation smells along with computation of various code quality metrics.
- [Doop](https://bitbucket.org/yanniss/doop) — Doop is a declarative framework for static analysis of Java/Android programs, centered on pointer analysis algorithms. Doop provides a large variety of analyses and also the surrounding scaffolding to run an analysis end-to-end (fact generation, processing, statistics, etc.).
- [Error-prone](https://errorprone.info) — Catch common Java mistakes as compile-time errors.
- [fb-contrib](http://fb-contrib.sourceforge.net) — A plugin for FindBugs with additional bug detectors.
- [forbidden-apis](https://github.com/policeman-tools/forbidden-apis) — Detects and forbids invocations of specific method/class/field (like reading from a text stream without a charset). Maven/Gradle/Ant compatible.
- [google-java-format](https://github.com/google/google-java-format) — Google Style Reformat.
- [HuntBugs](https://github.com/amaembo/huntbugs) :warning: — Bytecode static analyzer tool based on Procyon Compiler Tools aimed to supersede FindBugs.
- [IntelliJ IDEA](https://www.jetbrains.com/idea) :copyright: — Comes bundled with a lot of inspections for Java and Kotlin and includes tools for refactoring, formatting and more.
- [JArchitect](https://www.jarchitect.com) :copyright: — Measure, query and visualize your code and avoid unexpected issues, technical debt and complexity.
- [JBMC](https://www.cprover.org/jbmc) — Bounded model-checker for Java (bytecode), verifies user-defined assertions, standard assertions, several coverage metric analyses.
- [NullAway](https://github.com/uber/NullAway) — Type-based null-pointer checker with low build-time overhead; an [Error Prone](http://errorprone.info/) plugin.
- [OWASP Dependency Check](https://owasp.org/www-project-dependency-check) — Checks dependencies for known, publicly disclosed, vulnerabilities.
- [qulice](https://www.qulice.com) — Combines a few (pre-configured) static analysis tools (checkstyle, PMD, Findbugs, ...).
- [Reshift](https://www.reshiftsecurity.com) :copyright: — A source code analysis tool for detecting and managing Java security vulnerabilities.
- [Soot](https://soot-oss.github.io/soot) — A framework for analyzing and transforming Java and Android applications.
- [Spoon](http://spoon.gforge.inria.fr) — Spoon is a metaprogramming library to analyze and transform Java source code (incl Java 9, 10, 11, 12, 13, 14). It parses source files to build a well-designed AST with powerful analysis and transformation API. Can be integrated in Maven and Gradle.
- [SpotBugs](https://spotbugs.github.io) — SpotBugs is FindBugs' successor. A tool for static analysis to look for bugs in Java code.
- [Violations Lib](https://github.com/tomasbjerre/violations-lib) — Java library for parsing report files from static code analysis. Used by a bunch of Jenkins, Maven and Gradle plugins.


<h2 id="javascript">JavaScript</h2>

- [aether](http://aetherjs.com) :warning: — Lint, analyze, normalize, transform, sandbox, run, step through, and visualize user JavaScript, in node or the browser.
- [Closure Compiler](https://developers.google.com/closure/compiler) — A compiler tool to increase efficiency, reduce size, and provide code warnings in JavaScript files.
- [ClosureLinter](https://github.com/google/closure-linter) :warning: — Ensures that all of your project's JavaScript code follows the guidelines in the Google JavaScript Style Guide. It can also automatically fix many common errors.
- [complexity-report](https://github.com/escomplex/complexity-report) :warning: — Software complexity analysis for JavaScript projects.
- [DeepScan](https://deepscan.io) :copyright: — An analyzer for JavaScript which targets runtime errors and quality issues rather than coding conventions.
- [es6-plato](https://github.com/the-simian/es6-plato) — Visualize JavaScript (ES6) source complexity.
- [escomplex](https://github.com/jared-stilwell/escomplex) :warning: — Software complexity analysis of JavaScript-family abstract syntax trees.
- [Esprima](https://esprima.org) — ECMAScript parsing infrastructure for multipurpose analysis.
- [flow](https://flow.org) — A static type checker for JavaScript.
- [hegel](https://hegel.js.org) — A static type checker for JavaScript with a bias on type inference and strong type systems.
- [jshint](https://jshint.com/about) [:information_source:](<https://github.com/analysis-tools-dev/static-analysis/issues/223>) — Detect errors and potential problems in JavaScript code and enforce your team's coding conventions.
- [JSLint](https://github.com/douglascrockford/JSLint) [:information_source:](<https://github.com/analysis-tools-dev/static-analysis/issues/223>) — The JavaScript Code Quality Tool.
- [JSPrime](http://dpnishant.github.io/jsprime) :warning: — Static security analysis tool.
- [NodeJSScan](https://opensecurity.in) — A static security code scanner for Node.js applications powered by libsast and semgrep that builds on the njsscan cli tool. It features a UI with various dashboards about an application's security status.
- [plato](https://github.com/es-analysis/plato) :warning: — Visualize JavaScript source complexity.
- [Polymer-analyzer](https://github.com/Polymer/tools/tree/master/packages/analyzer) — A static analysis framework for Web Components.
- [retire.js](http://retirejs.github.io/retire.js) — Scanner detecting the use of JavaScript libraries with known vulnerabilities.
- [RSLint](http://rslint.org/) — A (WIP) JavaScript linter written in Rust designed to be as fast as possible, customizable, and easy to use.
- [standard](http://standardjs.com) — An npm module that checks for Javascript Styleguide issues.
- [tern](https://ternjs.net) — A JavaScript code analyzer for deep, cross-editor language support.
- [TypL](https://typl.dev) — With TypL, you just write completely standard JS, and the tool figures out your types via powerful inferencing.
- [xo](https://github.com/xojs/xo) — Opinionated but configurable ESLint wrapper with lots of goodies included. Enforces strict and readable code.
- [yardstick](https://github.com/calmh/yardstick) :warning: — Javascript code metrics.


<h2 id="julia">Julia</h2>

- [StaticLint](https://github.com/julia-vscode/StaticLint.jl) — Static Code Analysis for Julia


<h2 id="kotlin">Kotlin</h2>

- [detekt](https://detekt.github.io/detekt) — Static code analysis for Kotlin code.
- [diktat](https://analysis-dev.github.io/diktat) — Strict coding standard for Kotlin and a linter that detects and auto-fixes code smells.
- [ktlint](https://ktlint.github.io) — An anti-bikeshedding Kotlin linter with built-in formatter.


<h2 id="lua">Lua</h2>

- [luacheck](https://github.com/mpeterv/luacheck) :warning: — A tool for linting and static analysis of Lua code.
- [lualint](https://github.com/philips/lualint) — lualint performs luac-based static analysis of global variable usage in Lua source code.
- [Luanalysis](https://plugins.jetbrains.com/plugin/14698-luanalysis) :warning: — An IDE for statically typed Lua development.


<h2 id="matlab">MATLAB</h2>

- [mlint](https://mathworks.com/help/matlab/ref/mlint.html) :copyright: — Check MATLAB code files for possible problems.


<h2 id="nim">Nim</h2>

- [DrNim](https://nim-lang.org/docs/drnim.html) — DrNim combines the Nim frontend with the Z3 proof engine in order to allow verify / validate software written in Nim.
- [nimfmt](https://github.com/FedericoCeratto/nimfmt) — Nim code formatter / linter / style checker


<h2 id="ocaml">Ocaml</h2>

- [Sys](https://github.com/PLSysSec/sys) — A static/symbolic Tool for finding bugs in (browser) code. It uses the LLVM AST to find bugs like uninitialized memory access.
- [VeriFast](https://github.com/verifast/verifast) — A tool for modular formal verification of correctness properties of single-threaded and multithreaded  C and Java programs annotated with preconditions and postconditions written in separation logic.  To express rich specifications, the programmer can define inductive datatypes,  primitive recursive pure functions over these datatypes, and abstract separation logic predicates.


<h2 id="php">PHP</h2>

- [churn-php](https://github.com/bmitch/churn-php) — Helps discover good candidates for refactoring.
- [dephpend](https://github.com/mihaeu/dephpend) — Dependency analysis tool.
- [deprecation-detector](https://github.com/sensiolabs-de/deprecation-detector) — Finds usages of deprecated (Symfony) code.
- [deptrac](https://github.com/sensiolabs-de/deptrac) — Enforce rules for dependencies between software layers.
- [DesignPatternDetector](https://github.com/Halleck45/DesignPatternDetector) — Detection of design patterns in PHP code.
- [EasyCodingStandard](https://www.tomasvotruba.com/blog/2017/05/03/combine-power-of-php-code-sniffer-and-php-cs-fixer-in-3-lines) — Combine [PHP_CodeSniffer](https://github.com/squizlabs/PHP_CodeSniffer) and [PHP-CS-Fixer](https://github.com/FriendsOfPHP/PHP-CS-Fixer).
- [Enlightn](https://www.laravel-enlightn.com/) — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.
- [exakat](https://www.exakat.io) — An automated code reviewing engine for PHP.
- [GrumPHP](https://github.com/phpro/grumphp) — Checks code on every commit.
- [larastan](https://github.com/nunomaduro/larastan) — Adds static analysis to Laravel improving developer productivity and code quality. It is a wrapper around PHPStan.
- [Mondrian](http://trismegiste.github.io/Mondrian) :warning: — A set of static analysis and refactoring tools which use graph theory.
- [Nitpick CI](https://nitpick-ci.com) :copyright: — Automated PHP code review.
- [parallel-lint](https://github.com/php-parallel-lint/PHP-Parallel-Lint) — This tool checks syntax of PHP files faster than serial check with a fancier output.
- [Parse](https://github.com/psecio/parse) — A Static Security Scanner.
- [pdepend](https://pdepend.org) — Calculates software metrics like cyclomatic complexity for PHP code.
- [phan](https://github.com/phan/phan/wiki) — A modern static analyzer from etsy.
- [PHP Architecture Tester](https://github.com/carlosas/phpat) — Easy to use architecture testing tool for PHP.
- [PHP Assumptions](https://github.com/rskuipers/php-assumptions) — Checks for weak assumptions.
- [PHP Coding Standards Fixer](https://cs.symfony.com) — Fixes your code according to standards like PSR-1, PSR-2, and the Symfony standard.
- [PHP Insights](https://phpinsights.com) — Instant PHP quality checks from your console. Analysis of code quality and coding style as well as overview of code architecture and its complexity.
- [Php Inspections (EA Extended)](https://plugins.jetbrains.com/plugin/7622-php-inspections-ea-extended-) — A Static Code Analyzer for PHP.
- [PHP Refactoring Browser](http://qafoolabs.github.io/php-refactoring-browser) — Refactoring helper.
- [PHP Semantic Versioning Checker](https://github.com/tomzx/php-semver-checker) — Suggests a next version according to semantic versioning.
- [PHP-Parser](https://github.com/nikic/PHP-Parser) — A PHP parser written in PHP.
- [php-speller](https://github.com/mekras/php-speller) — PHP spell check library.
- [PHP-Token-Reflection](https://github.com/Andrewsville/PHP-Token-Reflection) :warning: — Library emulating the PHP internal reflection.
- [php7cc](https://github.com/sstalle/php7cc) :warning: — PHP 7 Compatibility Checker.
- [php7mar](https://github.com/Alexia/php7mar) :warning: — Assist developers in porting their code quickly to PHP 7.
- [PHP_CodeSniffer](https://pear.php.net/package/PHP_CodeSniffer) — Detects violations of a defined set of coding standards.
- [phpca](https://github.com/wapmorgan/PhpCodeAnalyzer) :warning: — Finds usage of non-built-in extensions.
- [phpcpd](https://github.com/sebastianbergmann/phpcpd) — Copy/Paste Detector for PHP code.
- [phpdcd](https://github.com/sebastianbergmann/phpdcd) :warning: — Dead Code Detector (DCD) for PHP code.
- [PhpDependencyAnalysis](https://mamuz.github.io/PhpDependencyAnalysis) :warning: — Builds a dependency graph for a project.
- [PhpDeprecationDetector](https://github.com/wapmorgan/PhpDeprecationDetector) — Analyzer of PHP code to search issues with deprecated functionality in newer interpreter versions.  It finds removed objects (functions, variables, constants and ini-directives),  deprecated functions functionality, and usage of forbidden names or tricks (e.g. reserved identifiers in newer versions).
- [phpdoc-to-typehint](https://github.com/dunglas/phpdoc-to-typehint) :warning: — Add scalar type hints and return types to existing PHP projects using PHPDoc annotations.
- [phpDocumentor](https://www.phpdoc.org) — Analyzes PHP source code to generate documentation.
- [phploc](https://github.com/sebastianbergmann/phploc) — A tool for quickly measuring the size and analyzing the structure of a PHP project.
- [PHPMD](https://phpmd.org) — Finds possible bugs in your code.
- [PhpMetrics](http://www.phpmetrics.org) — Calculates and visualizes various code quality metrics.
- [phpmnd](https://github.com/povils/phpmnd) — Helps to detect magic numbers.
- [PHPQA](https://edgedesigncz.github.io/phpqa) — A tool for running QA tools (phploc, phpcpd, phpcs, pdepend, phpmd, phpmetrics).
- [phpqa - jakzal](https://github.com/jakzal/phpqa) — Many tools for PHP static analysis in one container.
- [phpqa - jmolivas](https://github.com/jmolivas/phpqa) — PHPQA all-in-one Analyzer CLI tool.
- [phpsa](https://github.com/ovr/phpsa) :warning: — Static analysis tool for PHP.
- [PHPStan](https://phpstan.org) — PHP Static Analysis Tool - discover bugs in your code without running it!
- [Progpilot](https://github.com/designsecurity/progpilot) — A static analysis tool for security purposes.
- [Psalm](https://psalm.dev) — Static analysis tool for finding type errors in PHP applications.
- [Qafoo Quality Analyzer](https://github.com/Qafoo/QualityAnalyzer) :warning: — Visualizes metrics and source code.
- [Symfony Insight](https://insight.symfony.com/) :copyright: — Detect security risks, find bugs and provide actionable metrics for PHP projects.
- [Tuli](https://github.com/ircmaxell/Tuli) — A static analysis engine.
- [twig-lint](https://github.com/asm89/twig-lint) — twig-lint is a lint tool for your twig files.
- [WAP](https://securityonline.info/owasp-wap-web-application-protection-project) — Tool to detect and correct input validation vulnerabilities in PHP (4.0 or higher) web applications and predicts false positives by combining static analysis and data mining.


<h2 id="plsql">PL/SQL</h2>

- [ZPA](https://felipezorzo.com.br/zpa/) — Z PL/SQL Analyzer (ZPA) is an extensible code analyzer for PL/SQL and Oracle SQL. It can be integrated with SonarQube.


<h2 id="perl">Perl</h2>

- [Perl::Critic](https://metacpan.org/pod/Perl::Critic) — Critique Perl source code for best-practices.


<h2 id="python">Python</h2>

- [bandit](https://bandit.readthedocs.io/en/latest) — A tool to find common security issues in Python code.
- [bellybutton](https://github.com/hchasestevens/bellybutton) — A linting engine supporting custom project-specific rules.
- [Black](https://black.readthedocs.io/en/stable) — The uncompromising Python code formatter.
- [Bowler](https://pybowler.io/) — Safe code refactoring for modern Python.  Bowler is a refactoring tool for manipulating Python at the syntax tree level.  It enables safe, large scale code modifications while guaranteeing that the  resulting code compiles and runs. It provides both a simple command line interface  and a fluent API in Python for generating complex code modifications in code.
- [ciocheck](https://github.com/ContinuumIO/ciocheck) :warning: — Linter, formatter and test suite helper. As a linter, it is a wrapper around `pep8`, `pydocstyle`, `flake8`, and `pylint`.
- [cohesion](https://github.com/mschwager/cohesion) :warning: — A tool for measuring Python class cohesion.
- [Dlint](https://github.com/dlint-py/dlint) — A tool for ensuring Python code is secure.
- [fixit](https://pypi.org/project/fixit) — A framework for creating lint rules and corresponding auto-fixes for source code.
- [flake8](https://github.com/PyCQA/flake8) — A wrapper around `pyflakes`, `pycodestyle` and `mccabe`.
- [InspectorTiger](https://github.com/thg-consulting/it) :warning: — IT, Inspector Tiger, is a modern python code review tool / framework. It comes with bunch of pre-defined handlers which warns you about improvements and possible bugs. Beside these handlers, you can write your own or use community ones.
- [jedi](https://jedi.readthedocs.io/en/latest) — Autocompletion/static analysis library for Python.
- [linty fresh](https://github.com/lyft/linty_fresh) — Parse lint errors and report them to Github as comments on a pull request.
- [mccabe](https://pypi.org/project/mccabe) — Check McCabe complexity.
- [multilint](https://github.com/adamchainz/multilint) :warning: — A wrapper around `flake8`, `isort` and `modernize`.
- [mypy](http://www.mypy-lang.org) — A static type checker that aims to combine the benefits of duck typing and static typing, frequently used with [MonkeyType](https://github.com/Instagram/MonkeyType).
- [prospector](https://github.com/PyCQA/prospector) — A wrapper around `pylint`, `pep8`, `mccabe` and others.
- [py-find-injection](https://github.com/uber/py-find-injection) :warning: — Find SQL injection vulnerabilities in Python code.
- [pyanalyze](https://pyanalyze.readthedocs.io/en/latest/) — A tool for programmatically detecting common mistakes in Python code, such as references to undefined variables and type errors. It can be extended to add additional rules and perform checks specific to particular functions.
- [PyCodeQual](https://pycodequ.al) :copyright: — PyCodeQual gives you insights into complexity and bug risks. It adds automatic reviews to your pull requests.
- [pycodestyle](https://pycodestyle.pycqa.org/en/latest) — (Formerly `pep8`) Check Python code against some of the style conventions in PEP 8.
- [pydocstyle](http://www.pydocstyle.org) — Check compliance with Python docstring conventions.
- [pyflakes](https://pypi.org/project/pyflakes) — Check Python source files for errors.
- [pylint](http://pylint.pycqa.org/en/latest) — Looks for programming errors, helps enforcing a coding standard and sniffs for some code smells. It additionally includes `pyreverse` (an UML diagram generator) and `symilar` (a similarities checker).
- [pyre-check](https://pyre-check.org) — A fast, scalable type checker for large Python codebases.
- [pyright](https://github.com/Microsoft/pyright) — Static type checker for Python, created to address gaps in existing tools like mypy.
- [pyroma](https://github.com/regebro/pyroma) — Rate how well a Python project complies with the best practices of the Python packaging ecosystem, and list issues that could be improved.
- [Pysa](https://pyre-check.org/docs/pysa-basics.html) — A tool based on Facebook's pyre-check to identify potential security issues in Python code identified with taint analysis.
- [PyT - Python Taint](https://github.com/python-security/pyt) :warning: — A static analysis tool for detecting security vulnerabilities in Python web applications.
- [pytype](https://google.github.io/pytype) — A static type analyzer for Python code.
- [QuantifiedCode](https://github.com/quantifiedcode/quantifiedcode) :warning: — Automated code review & repair. It helps you to keep track of issues and metrics in your software projects, and can be easily extended to support new types of analyses.
- [radon](https://radon.readthedocs.io/en/latest) — A Python tool that computes various metrics from the source code.
- [unimport](https://unimport.hakancelik.dev) — A linter, formatter for finding and removing unused import statements.
- [vulture](https://github.com/jendrikseipp/vulture) — Find unused classes, functions and variables in Python code.
- [wemake-python-styleguide](https://wemake-python-stylegui.de) — The strictest and most opinionated python linter ever.
- [wily](https://github.com/tonybaloney/wily) — A command-line tool for archiving, exploring and graphing the complexity of Python source code.
- [xenon](https://xenon.readthedocs.io) — Monitor code complexity using [`radon`](https://github.com/rubik/radon).


<h2 id="r">R</h2>

- [cyclocomp](https://github.com/MangoTheCat/cyclocomp) — Quantifies the cyclomatic complexity of R functions / expressions.
- [goodpractice](http://mangothecat.github.io/goodpractice) — Analyses the source code for R packages and provides best-practice recommendations.
- [lintr](https://github.com/jimhester/lintr) — Static Code Analysis for R.
- [styler](https://styler.r-lib.org) — Formatting of R source code files and pretty-printing of R code.


<h2 id="ruby">Ruby</h2>

- [brakeman](https://brakemanscanner.org) — A static analysis security vulnerability scanner for Ruby on Rails applications.
- [bundler-audit](https://github.com/rubysec/bundler-audit) — Audit Gemfile.lock for gems with security vulnerabilities reported in [Ruby Advisory Database](https://github.com/rubysec/ruby-advisory-db).
- [cane](https://github.com/square/cane) :warning: — Code quality threshold checking as part of your build.
- [dawnscanner](https://github.com/thesp0nge/dawnscanner) — A static analysis security scanner for ruby written web applications. It supports Sinatra, Padrino and Ruby on Rails frameworks.
- [ERB Lint](https://github.com/Shopify/erb-lint) — Lint your ERB or HTML files
- [Fasterer](https://github.com/DamirSvrtan/fasterer) — Common Ruby idioms checker.
- [flay](https://ruby.sadi.st/Flay.html) — Flay analyzes code for structural similarities.
- [flog](https://ruby.sadi.st/Flog.html) — Flog reports the most tortured code in an easy to read pain report. The higher the score, the more pain the code is in.
- [Fukuzatsu](https://github.com/CoralineAda/fukuzatsu) — A tool for measuring code complexity in Ruby class files. Its analysis generates scores based on cyclomatic complexity algorithms with no added "opinions".
- [laser](https://github.com/michaeledgar/laser) :warning: — Static analysis and style linter for Ruby code.
- [pelusa](https://github.com/codegram/pelusa) — Static analysis Lint-type tool to improve your OO Ruby code.
- [quality](https://github.com/apiology/quality) :warning: — Runs quality checks on your code using community tools, and makes sure your numbers don't get any worse over time.
- [Querly](https://github.com/soutaro/querly) — Pattern Based Checking Tool for Ruby.
- [Railroader](https://railroader.org) :warning: — An open source static analysis security vulnerability scanner for Ruby on Rails applications.
- [rails_best_practices](https://rails-bestpractices.com) — A code metric tool for Rails projects
- [reek](https://github.com/troessner/reek) — Code smell detector for Ruby.
- [RuboCop](https://docs.rubocop.org/rubocop) — A Ruby static code analyzer, based on the community Ruby style guide.
- [Rubrowser](https://github.com/blazeeboy/rubrowser) — Ruby classes interactive dependency graph generator.
- [ruby-lint](http://code.yorickpeterse.com/ruby-lint/latest) :warning: — Static code analysis for Ruby.
- [rubycritic](https://github.com/whitesmith/rubycritic) — A Ruby code quality reporter.
- [rufo](https://github.com/ruby-formatter/rufo) — An opinionated ruby formatter, intended to be used via the command line as a text-editor plugin, to autoformat files on save or on demand.
- [Saikuro](https://metricfu.github.io/Saikuro) :warning: — A Ruby cyclomatic complexity analyzer.
- [SandiMeter](https://rubygems.org/gems/sandi_meter) :warning: — Static analysis tool for checking Ruby code for Sandi Metz' rules.
- [Sorbet](https://sorbet.org) — A fast, powerful type checker designed for Ruby.
- [Standard Ruby](https://github.com/testdouble/standard) — Ruby Style Guide, with linter & automatic code fixer
- [Steep](https://github.com/soutaro/steep) — Gradual Typing for Ruby.


<h2 id="rust">Rust</h2>

- [C2Rust](https://c2rust.com) — C2Rust helps you migrate C99-compliant code to Rust. The translator (or transpiler) produces unsafe Rust code that closely mirrors the input C code.
- [cargo udeps](https://github.com/est31/cargo-udeps) — Find unused dependencies in Cargo.toml. It either prints out a "unused crates" line listing the crates,  or it prints out a line saying that no crates were unused.
- [cargo-audit](https://rustsec.org) — Audit Cargo.lock for crates with security vulnerabilities reported to the [RustSec Advisory Database](https://github.com/RustSec/advisory-db/).
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) — Find out what takes most of the space in your executable. supports ELF (Linux, BSD), Mach-O (macOS) and PE (Windows) binaries.
- [cargo-deny](https://embarkstudios.github.io/cargo-deny) — A cargo plugin for linting your dependencies. It can be used either as a command line too, a Rust crate, or a Github action for CI. It checks for valid license information, duplicate crates, security vulnerabilities, and more.
- [cargo-expand](https://github.com/dtolnay/cargo-expand) — Cargo subcommand to show result of macro expansion  and #[derive] expansion applied to the current crate.  This is a wrapper around a more verbose compiler command.
- [cargo-inspect](https://github.com/mre/cargo-inspect) — Inspect Rust code without syntactic sugar to see what the compiler does behind the curtains.
- [cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck) — Checks all your documentation for spelling and grammar mistakes  with hunspell (ready) and languagetool (preview)
- [clippy](https://rust-lang.github.io/rust-clippy) — A code linter to catch common mistakes and improve your Rust code.
- [dylint](https://www.trailofbits.com/post/write-rust-lints-without-forking-clippy) — A tool for running Rust lints from dynamic libraries. Dylint makes it easy for developers to maintain their own personal lint collections.
- [electrolysis](http://kha.github.io/electrolysis) :warning: — A tool for formally verifying Rust programs by transpiling them into definitions in the Lean theorem prover.
- [herbie](https://github.com/mcarton/rust-herbie-lint) :warning: — Adds warnings or errors to your crate when using a numerically unstable floating point expression.
- [linter-rust](https://github.com/AtomLinter/linter-rust) :warning: — Linting your Rust-files in Atom, using rustc and cargo.
- [MIRAI](https://github.com/facebookexperimental/MIRAI) — And abstract interpreter operating on Rust's mid-level intermediate language, and providing warnings based on taint analysis.
- [prae](https://github.com/teenjuna/prae) — Provides a convenient macro that allows you to generate type wrappers  that promise to always uphold arbitrary invariants that you specified. 
- [Prusti](https://www.pm.inf.ethz.ch/research/prusti.html) — A static verifier for Rust, based on the Viper verification infrastructure. By default Prusti verifies absence of panics by proving that statements such as unreachable!() and panic!() are unreachable.
- [Rudra](https://github.com/sslab-gatech/Rudra) — Rust Memory Safety & Undefined Behavior Detection. It is capable of analyzing single Rust packages as well as all the packages on crates.io.
- [Rust Language Server](https://github.com/rust-lang-nursery/rls) — Supports functionality such as 'goto definition', symbol search, reformatting, and code completion, and enables renaming and refactorings.
- [rust-analyzer](https://rust-analyzer.github.io) — Supports functionality such as 'goto definition', type inference, symbol search, reformatting, and code completion, and enables renaming and refactorings.
- [rust-audit](https://github.com/Shnatsel/rust-audit) — Audit Rust binaries for known bugs or security vulnerabilities. This works by embedding data about the dependency tree (Cargo.lock) in JSON format into a dedicated linker section of the compiled executable.
- [rustfix](https://github.com/rust-lang/rustfix) — Read and apply the suggestions made by rustc (and third-party lints, like those offered by clippy).
- [rustfmt](https://github.com/rust-lang/rustfmt) — A tool for formatting Rust code according to style guidelines.
- [RustViz](https://github.com/rustviz/rustviz) — RustViz is a tool that generates visualizations  from simple Rust programs to assist users in better  understanding the Rust Lifetime and Borrowing mechanism. It generates SVG files with graphical indicators that integrate  with mdbook to render visualizations of data-flow in Rust programs.
- [warnalyzer](https://github.com/est31/warnalyzer) — Show unused code from multi-crate Rust projects


<h2 id="sql">SQL</h2>

- [dbcritic](https://github.com/channable/dbcritic) — dbcritic finds problems in a database schema, such as a missing primary key constraint in a table.
- [sqlcheck](https://github.com/jarulraj/sqlcheck) — Automatically identify anti-patterns in SQL queries.
- [SQLFluff](https://www.sqlfluff.com/) — Multiple dialect SQL linter and formatter.
- [sqlint](https://github.com/purcell/sqlint) — Simple SQL linter.
- [squawk](https://squawkhq.com) — Linter for PostgreSQL, focused on migrations. Prevents unexpected downtime caused by database migrations and encourages best practices around Postgres schemas and SQL.
- [tsqllint](https://github.com/tsqllint/tsqllint) — T-SQL-specific linter.
- [TSqlRules](https://github.com/ashleyglee/TSqlRules) :warning: — TSQL Static Code Analysis Rules for SQL Server.
- [Visual Expert](https://www.visual-expert.com) :copyright: — Code analysis for PowerBuilder, Oracle, and SQL Server Explores, analyzes, and documents Code 


<h2 id="scala">Scala</h2>

- [linter](https://github.com/HairyFotr/linter) :warning: — Linter is a Scala static analysis compiler plugin which adds compile-time checks for various possible bugs, inefficiencies, and style problems.
- [Scalastyle](http://www.scalastyle.org) — Scalastyle examines your Scala code and indicates potential problems with it.
- [scapegoat](https://github.com/sksamuel/scapegoat) — Scala compiler plugin for static code analysis.
- [WartRemover](https://www.wartremover.org) — A flexible Scala code linting tool.


<h2 id="shell">Shell</h2>

- [bashate](https://github.com/openstack/bashate) — Code style enforcement for bash programs. The output format aims to follow pycodestyle (pep8) default output format.
- [i-Code CNES for Shell](https://github.com/lequal/i-CodeCNES) :warning: — An open source static code analysis tool for Shell and Fortran (77 and 90).
- [kmdr](https://kmdr.sh) :warning: — CLI tool for learning commands from your terminal. kmdr delivers a break down of commands with every attribute explained.
- [sh](https://pkg.go.dev/mvdan.cc/sh/v3) — A shell parser, formatter, and interpreter with bash support; includes shfmt
- [shellcheck](https://www.shellcheck.net) — ShellCheck, a static analysis tool that gives warnings and suggestions for bash/sh shell scripts.
- [shellharden](https://github.com/anordal/shellharden) — A syntax highlighter and a tool to semi-automate the rewriting of scripts to ShellCheck conformance, mainly focused on quoting.


<h2 id="swift">Swift</h2>

- [SwiftFormat](https://github.com/nicklockwood/SwiftFormat) — A library and command-line formatting tool for reformatting Swift code.
- [SwiftLint](https://realm.github.io/SwiftLint) — A tool to enforce Swift style and conventions.
- [Tailor](https://sleekbyte.github.io/tailor) :warning: — A static analysis and lint tool for source code written in Apple's Swift programming language.


<h2 id="tcl">Tcl</h2>

- [Frink](http://catless.ncl.ac.uk/Programs/Frink) — A Tcl formatting and static check program (can prettify the program, minimise, obfuscate or just sanity check it).
- [Nagelfar](https://sourceforge.net/projects/nagelfar) — A static syntax checker for Tcl.
- [tclchecker](https://github.com/ActiveState/tdk/blob/master/docs/3.0/TDK_3.0_Checker.txt) — A static syntax analysis module (as part of [TDK](https://github.com/ActiveState/tdk)).


<h2 id="typescript">TypeScript</h2>

- [Angular ESLint](https://github.com/angular-eslint/angular-eslint#readme) — Linter for Angular projects
- [Codelyzer](http://codelyzer.com) — A set of tslint rules for static code analysis of Angular 2 TypeScript projects.
- [tslint-clean-code](https://www.npmjs.com/package/tslint-clean-code) — A set of TSLint rules inspired by the Clean Code handbook.
- [tslint-microsoft-contrib](https://github.com/Microsoft/tslint-microsoft-contrib) :warning: — A set of tslint rules for static code analysis of TypeScript projects maintained by Microsoft.
- [TypeScript Call Graph](https://github.com/whyboris/TypeScript-Call-Graph) — CLI to generate an interactive graph of functions and calls from your TypeScript files
- [TypeScript ESLint](https://github.com/typescript-eslint/typescript-eslint) — TypeScript language extension for eslint.


<h2 id="verilog">Verilog/SystemVerilog</h2>

- [Icarus Verilog](https://github.com/steveicarus/iverilog) — A Verilog simulation and synthesis tool that operates by compiling source code written in IEEE-1364 Verilog into some target format
- [svls](https://github.com/dalance/svls) — A Language Server Protocol implementation for Verilog and SystemVerilog, including lint capabilities.
- [verible-linter-action](https://github.com/chipsalliance/verible-linter-action) — Automatic SystemVerilog linting in github actions with the help of Verible Used to lint Verilog and SystemVerilog source files and comment erroneous lines  of code in Pull Requests automatically.
- [Verilator](https://www.veripool.org/verilator) — A tool which converts Verilog to a cycle-accurate behavioral model in C++ or SystemC. Performs lint code-quality checks.
- [vscode-verilog-hdl-support](https://github.com/mshr-h/vscode-verilog-hdl-support) — Verilog HDL/SystemVerilog/Bluespec SystemVerilog support for VS Code. Provides syntax highlighting and Linting support from Icarus Verilog, Vivado Logical Simulation, Modelsim and Verilator


<h2 id="vim-script">Vim Script</h2>

- [vint](https://github.com/Kuniwak/vint) :warning: — Fast and Highly Extensible Vim script Language Lint implemented by Python.


## Multiple languages

- [ale](https://github.com/w0rp/ale) — Asynchronous Lint Engine for Vim and NeoVim with support for many languages.
- [Android Studio](https://developer.android.com/studio) — Based on IntelliJ IDEA, and comes bundled with tools for Android including Android Lint.
- [AppChecker](https://npo-echelon.ru/en/solutions/appchecker.php) :copyright: — Static analysis for C/C++/C#, PHP and Java.
- [Application Inspector](https://www.ptsecurity.com/ww-en/products/ai) :copyright: — Commercial Static Code Analysis which generates exploits to verify vulnerabilities.
- [ApplicationInspector](https://github.com/microsoft/ApplicationInspector) — Creates reports of over 400 rule patterns for feature detection (e.g. the use of cryptography or version control in apps).
- [APPscreener](https://solarappscreener.com) :copyright: — Static code analysis for binary and source code - Java/Scala, PHP, Javascript, C#, PL/SQL, Python, T-SQL, C/C++, ObjectiveC/Swift, Visual Basic 6.0, Ruby, Delphi, ABAP, HTML5 and Solidity.
- [ArchUnit](https://www.archunit.org) — Unit test your Java or Kotlin architecture.
- [Atom-Beautify](https://atom.io/packages/atom-beautify) :warning: — Beautify HTML, CSS, JavaScript, PHP, Python, Ruby, Java, C, C++, C#, Objective-C, CoffeeScript, TypeScript, Coldfusion, SQL, and more in Atom editor.
- [Axivion Bauhaus Suite](https://www.axivion.com/en/products-services-9#products_bauhaussuite) :copyright: — Tracks down error-prone code locations, style violations, cloned or dead code, cyclic dependencies and more for C/C++, C#/.NET, Java and Ada 83/Ada 95.
- [Better Code Hub](https://bettercodehub.com) :copyright: — Better Code Hub checks your GitHub codebase against 10 engineering guidelines devised by the authority in software quality, Software Improvement Group.
- [callGraph](https://github.com/koknat/callGraph) — Statically generates a call graph image and displays it on screen.
- [CAST Highlight](https://www.castsoftware.com/products/highlight) :copyright: — Commercial Static Code Analysis which runs locally, but uploads the results to its cloud for presentation.
- [Checkmarx CxSAST](https://www.checkmarx.com/products/static-application-security-testing) :copyright: — Commercial Static Code Analysis which doesn't require pre-compilation.
- [ClassGraph](https://github.com/classgraph/classgraph) — A classpath and module path scanner for querying or visualizing class metadata or class relatedness.
- [Clayton](https://www.getclayton.com/) :copyright: — AI-powered code reviews for Salesforce. Secure your developments, enforce best practice and control your technical debt in real-time.
- [coala](https://coala.io) — Language independent framework for creating code analysis - supports [over 60 languages](https://coala.io/languages) by default.
- [Cobra](https://spinroot.com/cobra) :copyright: — Structural source code analyzer by NASA's Jet Propulsion Laboratory.
- [Codacy](https://www.codacy.com) :copyright: — Code Analysis to ship Better Code, Faster.
- [Code Intelligence](https://www.code-intelligence.com) :copyright: — CI/CD-agnostic DevSecOps platform which combines industry-leading fuzzing engines for finding bugs and visualizing code coverage
- [Codeac](https://www.codeac.io/?ref=awesome-static-analysis) :copyright: — Automated code review tool integrates with GitHub, Bitbucket and GitLab (even self-hosted). Available for JavaScript, TypeScript, Python, Ruby, Go, PHP, Java, Docker, and more. (open-source free)
- [codeburner](http://groupon.github.io/codeburner) — Provides a unified interface to sort and act on the issues it finds.
- [codechecker](https://codechecker.readthedocs.io/en/latest) — A defect database and viewer extension for the Clang Static Analyzer with web GUI.
- [CodeFactor](https://codefactor.io) :copyright: — Automated Code Analysis for repos on GitHub or BitBucket.
- [CodeFlow](https://www.getcodeflow.com) :copyright: — Automated code analysis tool to deal with technical depth. Integrates with Bitbucket and Gitlab. (free for Open Source Projects)
- [CodeIt.Right](https://submain.com/products/codeit.right.aspx) :copyright: — CodeIt.Right&trade; provides a fast, automated way to ensure that your source code adheres to (your) predefined design and style guidelines as well as best coding practices.
- [CodePatrol](https://cyber-security.claranet.fr/en/codepatrol) :copyright: — Automated SAST code reviews driven by security, supports 15+ languages and includes security training.
- [codeql](https://github.com/github/codeql) — Deep code analysis - semantic queries and dataflow for several languages with VSCode plugin support.
- [Coderrect](https://coderrect.com/) :copyright: — Advanced static analyzer for multi-threaded software. Supports OpenMP, Pthreads, std::thread, and GPU/CUDA.
- [CodeRush](https://www.devexpress.com/products/coderush) :copyright: — Code creation, debugging, navigation, refactoring, analysis and visualization tools that use the Roslyn engine in Visual Studio 2015 and up.
- [CodeScan](https://www.codescan.io/) :copyright: — Code Quality and Security for Salesforce Developers. Made exclusively for the Salesforce platform, CodeScan’s code analysis solutions provide you with total visibility into your code health.
- [CodeScene](https://codescene.com) :copyright: — CodeScene is a quality visualization tool for software. Prioritize technical debt, detect delivery risks, and measure organizational aspects. Fully automated.
- [CodeSonar from GrammaTech](https://www.grammatech.com/products/codesonar) :copyright: — Advanced, whole program, deep path, static analysis of C, C++, Java and C# with easy-to-understand explanations and code and path visualization.
- [Codiga](https://www.codiga.io) :copyright: — Automated Code Reviews and Technical Debt management platform that supports 12+ languages.
- [Corrode](https://github.com/jameysharp/corrode) :warning: — Semi-automatic translation from C to Rust. Could reveal bugs in the original implementation by showing Rust compiler warnings and errors. Superseded by C2Rust.
- [Coverity](https://www.synopsys.com/software-integrity/security-testing/static-analysis-sast.html) :copyright: — Synopsys Coverity supports 20 languages and over 70 frameworks including Ruby on rails, Scala, PHP, Python, JavaScript, TypeScript, Java, Fortran, C, C++, C#, VB.NET.
- [cqc](https://github.com/xcatliu/cqc) :warning: — Check your code quality for js, jsx, vue, css, less, scss, sass and styl files.
- [DeepCode](https://www.deepcode.ai) :copyright: — DeepCode finds bugs, security vulnerabilities, performance and API issues based on AI. DeepCode's speed of analysis allow us to analyse your code in real time and deliver results when you hit the save button in your IDE. Supported languages are Java, C/C++, JavaScript, Python, and TypeScript. Integrations with GitHub, BitBucket and Gitlab.
- [DeepSource](https://deepsource.io) :copyright: — In-depth static analysis to find issues in verticals of bug risks, security, anti-patterns, performance, documentation and style. Native integrations with GitHub, GitLab and Bitbucket. Less than 5% false positives.
- [Depends](https://github.com/multilang-depends/depends) — Analyses the comprehensive dependencies of code elements for Java, C/C++, Ruby.
- [DevSkim](https://github.com/microsoft/devskim) — Regex-based static analysis tool for Visual Studio, VS Code, and Sublime Text - C/C++, C#, PHP, ASP, Python, Ruby, Java, and others.
- [Embold](https://embold.io) :copyright: — Intelligent software analytics platform that identifies design issues, code issues, duplication and metrics. Supports Java, C, C++, C#, JavaScript, TypeScript, Python, Go, Kotlin and more.
- [ESLint](https://github.com/eslint/eslint) — An extensible linter for JS, following the ECMAScript standard.
- [Find Security Bugs](https://find-sec-bugs.github.io) — The SpotBugs plugin for security audits of Java web applications and Android applications. (Also work with Kotlin, Groovy and Scala projects)
- [Fortify](https://software.microfocus.com/en-us/products/static-code-analysis-sast/overview) :copyright: — A commercial static analysis platform that supports the scanning of C/C++, C#, VB.NET, VB6, ABAP/BSP, ActionScript, Apex, ASP.NET, Classic ASP, VB Script, Cobol, ColdFusion, HTML, Java, JS, JSP, MXML/Flex, Objective-C, PHP, PL/SQL, T-SQL, Python (2.6, 2.7), Ruby (1.9.3), Swift, Scala, VB, and XML.
- [Goodcheck](https://sider.github.io/goodcheck) — Regexp based customizable linter.
- [goone](https://github.com/masibw/goone) :warning: — Finds N+1 queries (SQL calls in a for loop) in go code
- [graudit](http://www.justanotherhacker.com) — Grep rough audit - source code auditing tool.
- [HCL AppScan Source](https://www.hcltechsw.com/products/appscan) :copyright: — Commercial Static Code Analysis.
- [Hopper](https://github.com/cuplv/hopper) :warning: — A static analysis tool written in scala for languages that run on JVM.
- [Hound CI](https://houndci.com) — Comments on style violations in GitHub pull requests. Supports Coffeescript, Go, HAML, JavaScript, Ruby, SCSS and Swift.
- [imhotep](https://github.com/justinabrahms/imhotep) — Comment on commits coming into your repository and check for syntactic errors and general lint warnings.
- [include-gardener](https://github.com/feddischson/include_gardener) :warning: — A multi-language static analyzer for C/C++/Obj-C/Python/Ruby to create a graph (in dot or graphml format) which shows all `#include` relations of a given set of files.
- [Infer](https://fbinfer.com) — A static analyzer for Java, C and Objective-C
- [InsiderSec](https://insidersec.io) :warning: — A open source Static Application Security Testing tool (SAST) written in GoLang for Java (Maven and Android), Kotlin (Android), Swift (iOS), .NET Full Framework, C# and Javascript (Node.js).
- [Kiuwan](https://www.kiuwan.com/code-security-sast) :copyright: — Identify and remediate cyber threats in a blazingly fast, collaborative environment, with seamless integration in your SDLC. Python, C\C++, Java, C#, PHP and more.
- [Klocwork](https://www.perforce.com/products/klocwork) :copyright: — Quality and Security Static analysis for C/C++, Java and C#.
- [LGTM](https://lgtm.com/) :copyright: — Find security vulnerabilities, variants, and critical code quality issues using queries over source code. Automatic PR code review; free for open source. Formerly semmle.
- [LGTM.com](https://lgtm.com) :copyright: — Deep code analysis for GitHub and Bitbucket to find security vulnerabilities and critical code quality issues (using Semmle QL). Automatic code review for pull requests; free for public repositories.
- [lizard](https://github.com/terryyin/lizard) — Lizard is an extensible Cyclomatic Complexity Analyzer for many programming languages  including C/C++ (doesn't require all the header files or Java imports).  It also does copy-paste detection (code clone detection/code duplicate detection) and many other forms of static code analysis. Counts lines of code without comments, CCN (cyclomatic complexity number), token count of functions, parameter count of functions.
- [Mega-Linter](https://nvuillam.github.io/mega-linter/) — Mega-Linter can handle any type of project thanks to its 70+ embedded Linters,
 its advanced reporting, runnable on any CI system or locally,
 with assisted installation and configuration, able to apply formatting and fixes
- [oclint](http://oclint.org) — A static source code analysis tool to improve quality and reduce defects for C, C++ and Objective-C.
- [ocular](https://www.shiftleft.io/ocular/) :copyright: — Enables code auditors and security teams to interactively investigate their unique code bases  to find business logic flaws and technical vulnerabilities that traditional SASTs cannot. This is done by enabling the analyst to write their own custom queries. Can find hard-coded secrets, authentication issues, and malicious code like rootkits and backdoors.
- [Offensive 360](https://offensive360.com/) :copyright: — Commercial Static Code Analysis system doesn't require building the source code or pre-compilation.
- [parasoft](https://www.parasoft.com/) :copyright: — Automated Software Testing Solutions for unit-, API-, and web UI testing. Complies with MISRA, OWASP, and others.
- [pfff](https://github.com/facebookarchive/pfff/wiki/Main) — Facebook's tools for code analysis, visualizations, or style-preserving source transformation for many languages.
- [PMD](https://pmd.github.io) — A source code analyzer for Java, Salesforce Apex, Javascript, PLSQL, XML, XSL and others.
- [pre-commit](https://pre-commit.com) — A framework for managing and maintaining multi-language pre-commit hooks.
- [Prettier](https://prettier.io) — An opinionated code formatter.
- [Pronto](https://github.com/prontolabs/pronto) — Quick automated code review of your changes. Supports more than 40 runners for various languages, including Clang, Elixir, JavaScript, PHP, Ruby and more.
- [PT.PM](https://github.com/PositiveTechnologies/PT.PM) :warning: — An engine for searching patterns in the source code, based on Unified AST or UST. At present time C#, Java, PHP, PL/SQL, T-SQL, and JavaScript are supported. Patterns can be described within the code or using a DSL.
- [Putout](https://github.com/coderaiser/putout) — Pluggable and configurable code transformer with built-in eslint, babel plugins support for js, jsx typescript, flow, markdown, yaml and json.
- [PVS-Studio](https://pvs-studio.com) :copyright: — A ([conditionally free](https://pvs-studio.com/en/order/open-source-license) for FOSS and individual developers) static analysis of C, C++, C# and Java code. For advertising purposes [you can propose a large FOSS project for analysis by PVS employees](https://github.com/viva64/pvs-studio-check-list). Supports CWE mapping, MISRA and CERT coding standards.
- [pylama](https://klen.github.io/pylama/) — Code audit tool for Python and JavaScript. Wraps pycodestyle, pydocstyle, PyFlakes, Mccabe, Pylint, and more
- [Refactoring Essentials](https://marketplace.visualstudio.com/items?itemName=SharpDevelopTeam.RefactoringEssentialsforVisualStudio) :warning: — The free Visual Studio 2015 extension for C# and VB.NET refactorings, including code best practice analyzers.
- [relint](https://github.com/codingjoe/relint) — A static file linter that allows you to write custom rules using regular expressions (RegEx).
- [ReSharper](https://www.jetbrains.com/resharper) :copyright: — Extends Visual Studio with on-the-fly code inspections for C#, VB.NET, ASP.NET, JavaScript, TypeScript and other technologies.
- [RIPS](https://www.ripstech.com) :copyright: — A static source code analyser for vulnerabilities in PHP scripts.
- [Rome](https://rome.tools/) — Rome is a linter, compiler, bundler, and [more](https://rome.tools/#development-status) for JavaScript, TypeScript, JSON, HTML, Markdown, and CSS.
- [Roslyn Analyzers](https://github.com/dotnet/roslyn-analyzers) — Roslyn-based implementation of FxCop analyzers.
- [Roslyn Security Guard](https://security-code-scan.github.io) — Project that focuses on the identification of potential vulnerabilities such as SQL injection, cross-site scripting (XSS), CSRF, cryptography weaknesses, hardcoded passwords and many more.
- [Scanmycode CE (Community Edition)](http://www.scanmycode.today) — Scanmycode - Code Scanning/SAST/Linting using many tools/Scanners with One Report
- [Scrutinizer](https://scrutinizer-ci.com) :copyright: — A proprietary code quality checker that can be integrated with GitHub.
- [Security Code Scan](https://security-code-scan.github.io) — Security code analyzer for C# and VB.NET. Detects various security vulnerability patterns: SQLi, XSS, CSRF, XXE, Open Redirect, etc. Integrates into Visual Studio 2015 and newer. Detects various security vulnerability patterns: SQLi, XSS, CSRF, XXE, Open Redirect, etc.
- [Semgrep](https://semgrep.dev) — A fast, open-source, static analysis tool for finding bugs and enforcing code standards at editor, commit, and CI time. Its rules look like the code you already write;  no abstract syntax trees or regex wrestling. Supports 17+ languages.
- [ShiftLeft](https://www.shiftleft.io) :copyright: — Identify vulnerabilities that are unique to your code base before they reach production. Leverages the Code Property Graph (CPG) to run its analyses concurrently in a single graph of graphs. Automatically finds business logic flaws in dev like hardcoded secrets and logic bombs
- [ShiftLeft Scan](https://slscan.io) — Scan is a free open-source DevSecOps platform for detecting security issues in source code and dependencies. It supports a broad range of languages and CI/CD pipelines.
- [shipshape](https://github.com/google/shipshape) :warning: — Static program analysis platform that allows custom analyzers to plug in through a common interface.
- [Sider](https://sider.review) :copyright: — An automated code reviewing tool. Improving developers' productivity.
- [Similarity Tester](https://dickgrune.com/Programs/similarity_tester/) — A tool that finds similarities between or within files to support you encountering DRY principle violations.
- [Snyk](https://snyk.io) :copyright: — Vulnerability scanner for dependencies of node.js apps (free for Open Source Projects).
- [SonarCloud](https://sonarcloud.io) :copyright: — Multi-language cloud-based static code analysis. History, trends, security hot-spots, pull request analysis and more. Free for open source.
- [SonarLint for Visual Studio](https://vs.sonarlint.org) — SonarLint is an extension for Visual Studio 2015 and 2017 that provides on-the-fly feedback to developers on new bugs and quality issues injected into .NET code.
- [SonarQube](http://www.sonarqube.org) — SonarQube is an open platform to manage code quality.
- [Sonatype](https://www.sonatype.com) :copyright: — Reports known vulnerabilities in common dependencies and recommends updated packages to minimize breaking changes
- [Soto Platform](https://www.hello2morrow.com/products/sotograph) :copyright: — Suite of static analysis tools consisting of the three components Sotoarc (Architecture Analysis), Sotograph (Quality Analysis), and Sotoreport (Quality report). Helps find differences between architecture and implementation, interface violations (e.g. external access of private parts of subsystems, detection of all classes, files, packages and subsystems which are strongly coupled by cyclical relationships and more. The Sotograph product family runs on Windows and Linux. 
- [SourceMeter](https://www.sourcemeter.com/) :copyright: — Static Code Analysis for C/C++, Java, C#, Python, and RPG III and RPG IV versions (including free-form).
- [sqlvet](https://github.com/houqp/sqlvet) — Performs static analysis on raw SQL queries in your Go code base to surface potential runtime errors. It checks for SQL syntax error, identifies unsafe queries that could potentially lead to SQL injections makes sure column count matches value count in INSERT statements and validates table- and column names.
- [Super-Linter](https://github.com/github/super-linter) — Combination of multiple linters to install as a GitHub Action.
- [Synopsys](https://www.synopsys.com/software-integrity/security-testing/static-analysis-sast.html) :copyright: — A commercial static analysis platform that allows for scanning of multiple languages (C/C++, Android, C#, Java, JS, PHP, Python, Node.JS, Ruby, Fortran, and Swift).
- [Teamscale](https://www.cqse.eu/en/teamscale/overview/) :copyright: — Static and dynamic analysis tool supporting more than 25 languages and direct IDE integration. Free hosting for Open Source projects available on request. Free academic licenses available.
- [ThreatMapper](https://github.com/deepfence/ThreatMapper) — Vulnerability Scanner and Risk Evaluation for containers, serverless and hosts at runtime. ThreatMapper generates runtime BOMs from dependencies and operating system packages, matches against multiple threat feeds, scans for unprotected secrets, and scores issues based on severity and risk-of-exploit.
- [todocheck](https://github.com/preslavmihaylov/todocheck) — Linter for integrating annotated TODOs with your issue trackers
- [trivy](https://github.com/aquasecurity/trivy) — A Simple and Comprehensive Vulnerability Scanner for Containers and other Artifacts, Suitable for CI. Trivy detects vulnerabilities of OS packages (Alpine, RHEL, CentOS, etc.) and application dependencies (Bundler, Composer, npm, yarn, etc.). Checks containers and filesystems.

- [trunk](https://trunk.io) :copyright: — Modern repositories include many technologies, each with its own set of linters. With 30+ linters and counting, Trunk makes it dead-simple to identify, install, configure, and run the right linters, static analyzers, and formatters for all your repos.
- [TscanCode](https://github.com/Tencent/TscanCode) — A fast and accurate static analysis solution for C/C++, C#, Lua codes provided by Tencent. Using GPLv3 license.
- [Undebt](https://github.com/Yelp/undebt) — Language-independent tool for massive, automatic, programmable refactoring based on simple pattern definitions.
- [Understand](https://www.scitools.com) :copyright: — Code visualization tool that provides code analysis, standards testing, metrics, graphing, dependency analysis and more for Ada, VHDL, and others.
- [Unibeautify](https://unibeautify.com) — Universal code beautifier with a GitHub app. Supports HTML, CSS, JavaScript, TypeScript, JSX, Vue, C++, Go, Objective-C, Java, Python, PHP, GraphQL, Markdown, and more.
- [Upsource](https://www.jetbrains.com/upsource) :copyright: — Code review tool with static code analysis and code-aware navigation for Java, PHP, JavaScript and Kotlin.
- [Veracode](http://www.veracode.com/products/static-analysis-sast/static-code-analysis) :copyright: — Find flaws in binaries and bytecode without requiring source. Support all major programming languages: Java, .NET, JavaScript, Swift, Objective-C, C, C++ and more.
- [Viezly](https://viezly.com) :copyright: — Code review tool with dependency diagrams. Improve your team's code reviews with better navigation and code analysis
- [WALA](https://github.com/wala/WALA) — Static analysis capabilities for Java bytecode and related languages and for JavaScript.
- [weggli](https://github.com/googleprojectzero/weggli) — A fast and robust semantic search tool for C and C++ codebases. It is designed to help security researchers identify interesting functionality in large codebases.
- [WhiteHat Application Security Platform](https://www.whitehatsec.com/platform/static-application-security-testing) :copyright: — WhiteHat Scout (for Developers) combined with WhiteHat Sentinel Source (for Operations) supporting WhiteHat Top 40 and OWASP Top 10.
- [Wotan](https://github.com/fimbullinter/wotan) — Pluggable TypeScript and JavaScript linter.
- [XCode](https://developer.apple.com/xcode) :copyright: — XCode provides a pretty decent UI for [Clang's](http://clang-analyzer.llvm.org/xcode.html) static code analyzer (C/C++, Obj-C).


## Other



<h2 id="angular">Angular</h2>

- [Angular ESLint](https://github.com/angular-eslint/angular-eslint#readme) — Linter for Angular projects


<h2 id="ansible">Ansible</h2>

- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible


<h2 id="arm">Azure Resource Manager</h2>

- [AzSK](https://azsk.azurewebsites.net/) — Secure DevOps kit for Azure (AzSK) provides security IntelliSense, Security Verification Tests (SVTs), CICD scan vulnerabilities, compliance issues, and infrastructure misconfiguration in your infrastructure-as-code. Supports Azure via ARM.


<h2 id="binary">Binaries</h2>

- [angr](https://github.com/angr/angr) — Binary code analysis tool that also supports symbolic execution.
- [binbloom](https://github.com/quarkslab/binbloom) — Analyzes a raw binary firmware and determines features like endianness or the loading address.  The tool is compatible with all architectures.
Loading address: binbloom can parse a raw binary firmware and determine its loading address. Endianness: binbloom can use heuristics to determine the endianness of a firmware. UDS Database: binbloom can parse a raw binary firmware and check if it contains an array containing UDS command IDs.
- [BinSkim](https://github.com/Microsoft/binskim) — A binary static analysis tool that provides security and correctness results for Windows portable executables.
- [Black Duck](https://www.blackducksoftware.com) :copyright: — Tool to analyze source code and binaries for reusable code, necessary licenses and potential security aspects.
- [bloaty](https://github.com/google/bloaty) — Ever wondered what's making your binary big? Bloaty McBloatface will show you a size profile of the binary so you can understand what's taking up space inside. Bloaty performs a deep analysis of the binary. Using custom ELF, DWARF, and Mach-O parsers,  Bloaty aims to accurately attribute every byte of the binary to the symbol or compileunit that produced it.  It will even disassemble the binary looking for references to anonymous data. F
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) — Find out what takes most of the space in your executable. supports ELF (Linux, BSD), Mach-O (macOS) and PE (Windows) binaries.
- [cwe_checker](https://github.com/fkie-cad/cwe_checker) — cwe_checker finds vulnerable patterns in binary executables.
- [Ghidra](https://ghidra-sre.org) — A software reverse engineering (SRE) suite of tools developed by NSA's Research Directorate in support of the Cybersecurity mission
- [IDA Free](https://www.hex-rays.com/products/ida/support/download_freeware) :copyright: — Binary code analysis tool.
- [Jakstab](https://github.com/jkinder/jakstab) — Jakstab is an Abstract Interpretation-based, integrated disassembly and static analysis framework for designing analyses on executables and recovering reliable control flow graphs.
- [JEB Decompiler](https://www.pnfsoftware.com/) :copyright: — Decompile and debug binary code. Break down and analyze document files. Android Dalvik, MIPS, ARM, Intel x86, Java, WebAssembly & Ethereum Decompilers.
- [Manalyze](https://github.com/JusticeRage/Manalyze) — A static analyzer, which checks portable executables for malicious content.
- [mcsema](https://github.com/lifting-bits/mcsema) — Framework for lifting x86, amd64, aarch64, sparc32, and sparc64 program binaries to LLVM bitcode. It translates ("lifts") executable binaries from native machine code to LLVM bitcode, which is very useful for performing program analysis methods.
- [Nauz File Detector](https://github.com/horsicq/Nauz-File-Detector) — Static Linker/Compiler/Tool detector for Windows, Linux and MacOS.
- [Twiggy](https://rustwasm.github.io/twiggy) — Analyzes a binary's call graph to profile code size. The goal is to slim down wasm binary size.
- [VMware chap](https://github.com/vmware/chap) — chap analyzes un-instrumented ELF core files for leaks, memory growth, and corruption.  It is sufficiently reliable that it can be used in automation to catch leaks before  they are committed. As an interactive tool, it helps explain memory growth,  can identify some forms of corruption, and supplements a debugger  by giving the status of various memory locations. 
- [zydis](https://zydis.re) — Fast and lightweight x86/x86-64 disassembler library


<h2 id="buildtool">Build tools</h2>

- [checkmake](https://github.com/mrtazz/checkmake) — Linter / Analyzer for Makefiles.
- [portlint](https://www.freebsd.org/cgi/man.cgi?query=portlint&sektion=1&manpath=FreeBSD+8.1-RELEASE+and+Ports) — A verifier for FreeBSD and DragonFlyBSD port directories.


<h2 id="css">CSS/SASS/SCSS</h2>

- [CSS Stats](https://cssstats.com) — Potentially interesting stats on stylesheets.
- [CSScomb](https://github.com/csscomb/csscomb.js) — A coding style formatter for CSS. Supports own configurations to make style sheets beautiful and consistent.
- [CSSLint](http://csslint.net) — Does basic syntax checking and finds problematic patterns or signs of inefficiency.
- [GraphMyCSS.com](https://graphmycss.com) — CSS Specificity Graph Generator.
- [Nu Html Checker](https://validator.github.io/validator/) — Helps you catch problems in your HTML/CSS/SVG
- [Parker](https://github.com/katiefenn/parker) :warning: — Stylesheet analysis tool.
- [PostCSS](https://postcss.org) — A tool for transforming styles with JS plugins. These plugins can lint your CSS, support variables and mixins, transpile future CSS syntax, inline images, and more.
- [Project Wallace CSS Analyzer](https://www.projectwallace.com) — Analytics for CSS, part of [Project Wallace](https://www.projectwallace.com).
- [sass-lint](https://github.com/sasstools/sass-lint) :warning: — A Node-only Sass linter for both sass and scss syntax.
- [scsslint](https://github.com/brigade/scss-lint) — Linter for SCSS files.
- [Specificity Graph](https://jonassebastianohlsson.com/specificity-graph) — CSS Specificity Graph Generator.
- [Stylelint](http://stylelint.io) — Linter for SCSS/CSS files.


<h2 id="configfile">Config Files</h2>

- [dotenv-linter](https://dotenv-linter.readthedocs.io/en/latest) — Linting dotenv files like a charm.
- [dotenv-linter (Rust)](https://dotenv-linter.github.io/#/) — Lightning-fast linter for .env files. Written in Rust
- [gixy](https://github.com/yandex/gixy) :warning: — A tool to analyze Nginx configuration. The main goal is to prevent misconfiguration and automate flaw detection.


<h2 id="configmanagement">Configuration Management</h2>

- [ansible-lint](https://docs.ansible.com/ansible-lint) — Checks playbooks for practices and behaviour that could potentially be improved.
- [AWS CloudFormation Guard](https://github.com/aws-cloudformation/cloudformation-guard) — Check local CloudFormation templates against policy-as-code rules  and generate rules from existing templates.
- [AzSK](https://azsk.azurewebsites.net/) — Secure DevOps kit for Azure (AzSK) provides security IntelliSense, Security Verification Tests (SVTs), CICD scan vulnerabilities, compliance issues, and infrastructure misconfiguration in your infrastructure-as-code. Supports Azure via ARM.
- [cfn-lint](https://github.com/awslabs/cfn-python-lint) — AWS Labs CloudFormation linter.
- [cfn_nag](https://github.com/stelligent/cfn_nag) — A linter for AWS CloudFormation templates.
- [chart-testing](https://github.com/helm/chart-testing) — ct is the the tool for testing Helm charts.  It is meant to be used for linting and testing pull requests.  It automatically detects charts changed against the target branch.
- [checkov](https://www.checkov.io) — Static analysis tool for Terraform files (tf>=v0.12), preventing cloud misconfigs at build time.
- [clusterlint](https://github.com/digitalocean/clusterlint) — Clusterlint queries live Kubernetes clusters for resources, executes common and  platform specific checks against these resources and provides actionable feedback to cluster operators.  It is a non invasive tool that is run externally. Clusterlint does not alter the resource configurations.
- [cookstyle](https://docs.chef.io/cookstyle.html) — Cookstyle is a linting tool based on the RuboCop Ruby linting tool for Chef cookbooks.
- [Datree](https://datree.io/) — A CLI tool to prevent Kubernetes misconfigurations by ensuring that manifests and Helm charts follow best practices as well as your organization’s policies
- [foodcritic](http://www.foodcritic.io) — A lint tool that checks Chef cookbooks for common problems.
- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible
- [kube-lint](https://github.com/viglesiasce/kube-lint) — A linter for Kubernetes resources with a customizable rule set. You define a list of rules that you would like to validate against your  resources and kube-lint will evaluate those rules against them.
- [kube-linter](https://github.com/stackrox/kube-linter) — KubeLinter is a static analysis tool that checks Kubernetes YAML files  and Helm charts to ensure the applications represented in them adhere to best practices. 
- [kubeval](https://kubeval.instrumenta.dev) — Validates your Kubernetes configuration files and supports multiple Kubernetes versions.
- [metadata-json-lint](https://github.com/voxpupuli/metadata-json-lint) — Tool to check the validity of Puppet metadata.json files.
- [Puppet Lint](https://github.com/rodjek/puppet-lint) — Check that your Puppet manifests conform to the style guide.
- [terraform-compliance](https://terraform-compliance.com) — A lightweight, compliance- and security focused, BDD test framework against Terraform.
- [terrascan](https://github.com/cesar-rodriguez/terrascan) — Collection of security and best practice tests for static code analysis of Terraform templates.
- [tflint](https://github.com/wata727/tflint) — A Terraform linter for detecting errors that can not be detected by `terraform plan`.
- [tfsec](https://github.com/tfsec/tfsec) — Terraform static analysis tool that prevents potential security issues by checking cloud misconfigurations at build time and directly integrates with the HCL parser for better results. Checks for violations of AWS, Azure and GCP security best practice recommendations.


<h2 id="container">Containers</h2>

- [anchore](https://anchore.io) — Discover, analyze, and certify container images. A service that analyzes Docker images and applies user-defined acceptance policies  to allow automated container image validation and certification 
- [chart-testing](https://github.com/helm/chart-testing) — ct is the the tool for testing Helm charts.  It is meant to be used for linting and testing pull requests.  It automatically detects charts changed against the target branch.
- [clair](https://github.com/coreos/clair) — Vulnerability Static Analysis for Containers.
- [clusterlint](https://github.com/digitalocean/clusterlint) — Clusterlint queries live Kubernetes clusters for resources, executes common and  platform specific checks against these resources and provides actionable feedback to cluster operators.  It is a non invasive tool that is run externally. Clusterlint does not alter the resource configurations.
- [collector](https://github.com/banyanops/collector) :warning: — Run arbitrary scripts inside containers, and gather useful information.
- [dagda](https://github.com/eliasgranderubio/dagda) — Perform static analysis of known vulnerabilities in docker images/containers.
- [Datree](https://datree.io/) — A CLI tool to prevent Kubernetes misconfigurations by ensuring that manifests and Helm charts follow best practices as well as your organization’s policies
- [Docker Label Inspector](https://github.com/garethr/docker-label-inspector) :warning: — Lint and validate Dockerfile labels.
- [Haskell Dockerfile Linter](https://github.com/lukasmartinelli/hadolint) — A smarter Dockerfile linter that helps you build best practice Docker images.
- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible
- [kube-lint](https://github.com/viglesiasce/kube-lint) — A linter for Kubernetes resources with a customizable rule set. You define a list of rules that you would like to validate against your  resources and kube-lint will evaluate those rules against them.
- [kube-linter](https://github.com/stackrox/kube-linter) — KubeLinter is a static analysis tool that checks Kubernetes YAML files  and Helm charts to ensure the applications represented in them adhere to best practices. 
- [kube-score](https://kube-score.com) — Static code analysis of your Kubernetes object definitions.
- [KubeLinter](https://github.com/stackrox/kube-linter) — KubeLinter is a static analysis tool that checks Kubernetes YAML files and Helm charts to ensure the applications represented in them adhere to best practices.
- [kubeval](https://kubeval.instrumenta.dev) — Validates your Kubernetes configuration files and supports multiple Kubernetes versions.
- [OpenSCAP](https://www.open-scap.org/) — Suite of automated audit tools to examine the configuration and  known vulnerabilities following the NIST-certified Security  Content Automation Protocol (SCAP).
- [Qualys Container Security](https://www.qualys.com/apps/container-security) :copyright: — Container native application protection to provide visibility and control of containerized applications.
- [sysdig](https://sysdig.com/) :copyright: — A secure DevOps platform for cloud and container forensics. Built on an open source stack, Sysdig provides Docker image scanning and created Falco, the open standard for runtime threat detection for containers, Kubernetes and cloud. 
- [Vuls](https://vuls.io/) — Agent-less Linux vulnerability scanner based on information from NVD, OVAL, etc.  It has some container image support, although is not a container specific tool.


<h2 id="ci">Continuous Integration</h2>

- [actionlint](https://rhysd.github.io/actionlint) — Static checker for GitHub Actions workflow files. Provides an online version.
- [AzSK](https://azsk.azurewebsites.net/) — Secure DevOps kit for Azure (AzSK) provides security IntelliSense, Security Verification Tests (SVTs), CICD scan vulnerabilities, compliance issues, and infrastructure misconfiguration in your infrastructure-as-code. Supports Azure via ARM.
- [Code Climate](https://codeclimate.com) — The open and extensible static analysis platform, for everyone.
- [exakat](https://www.exakat.io) — An automated code reviewing engine for PHP.
- [Nitpick CI](https://nitpick-ci.com) :copyright: — Automated PHP code review.
- [PullRequest](https://www.pullrequest.com) :copyright: — Code review as a service with built-in static analysis.  Increase velocity and reduce technical debt through quality code review by expert engineers backed by best-in-class automation.
- [quality](https://github.com/apiology/quality) :warning: — Runs quality checks on your code using community tools, and makes sure your numbers don't get any worse over time.
- [QuantifiedCode](https://github.com/quantifiedcode/quantifiedcode) :warning: — Automated code review & repair. It helps you to keep track of issues and metrics in your software projects, and can be easily extended to support new types of analyses.
- [Reviewdog](https://github.com/haya14busa/reviewdog) — A tool for posting review comments from any linter in any code hosting service.
- [Symfony Insight](https://insight.symfony.com/) :copyright: — Detect security risks, find bugs and provide actionable metrics for PHP projects.
- [Violations Lib](https://github.com/tomasbjerre/violations-lib) — Java library for parsing report files from static code analysis. Used by a bunch of Jenkins, Maven and Gradle plugins.


<h2 id="deno">Deno</h2>

- [deno_lint](https://github.com/denoland/deno_lint) — Official linter for Deno.


<h2 id="embedded">Embedded</h2>

- [oelint-adv](https://github.com/priv-kweihmann/oelint-adv) — Linter for bitbake recipes used in open-embedded and YOCTO


<h2 id="erb">Embedded Ruby (a.k.a. ERB, eRuby)</h2>

- [ERB Lint](https://github.com/Shopify/erb-lint) — Lint your ERB or HTML files


<h2 id="gherkin">Gherkin</h2>

- [gherkin-lint](https://github.com/vsiakka/gherkin-lint) — A linter for the Gherkin-Syntax written in Javascript.


<h2 id="html">HTML</h2>

- [Angular ESLint](https://github.com/angular-eslint/angular-eslint#readme) — Linter for Angular projects
- [Bootlint](https://github.com/twbs/bootlint) — An HTML linter for Bootstrap projects.
- [ERB Lint](https://github.com/Shopify/erb-lint) — Lint your ERB or HTML files
- [grunt-bootlint](https://github.com/twbs/grunt-bootlint) — A Grunt wrapper for [Bootlint](https://github.com/twbs/bootlint), the HTML linter for Bootstrap projects.
- [gulp-bootlint](https://github.com/tschortsch/gulp-bootlint) — A gulp wrapper for [Bootlint](https://github.com/twbs/bootlint), the HTML linter for Bootstrap projects.
- [HTML Inspector](https://github.com/philipwalton/html-inspector) :warning: — HTML Inspector is a code quality tool to help you and your team write better markup.
- [HTML Tidy](http://www.html-tidy.org) — Corrects and cleans up HTML and XML documents by fixing markup errors and upgrading legacy code to modern standards.
- [HTML-Validate](https://html-validate.org/) — Offline HTML5 validator.
- [HTMLHint](https://htmlhint.com) — A Static Code Analysis Tool for HTML.
- [Nu Html Checker](https://validator.github.io/validator/) — Helps you catch problems in your HTML/CSS/SVG
- [Polymer-analyzer](https://github.com/Polymer/tools/tree/master/packages/analyzer) — A static analysis framework for Web Components.


<h2 id="json">JSON</h2>

- [jsonlint](https://jsonlint.com/) — A JSON parser and validator with a CLI. Standalone version of jsonlint.com
- [Spectral](https://stoplight.io/open-source/spectral) — A flexible JSON/YAML linter, with out-of-the-box support for OpenAPI v2/v3 and AsyncAPI v2.


<h2 id="kubernetes">Kubernetes</h2>

- [chart-testing](https://github.com/helm/chart-testing) — ct is the the tool for testing Helm charts.  It is meant to be used for linting and testing pull requests.  It automatically detects charts changed against the target branch.
- [clusterlint](https://github.com/digitalocean/clusterlint) — Clusterlint queries live Kubernetes clusters for resources, executes common and  platform specific checks against these resources and provides actionable feedback to cluster operators.  It is a non invasive tool that is run externally. Clusterlint does not alter the resource configurations.
- [Datree](https://datree.io/) — A CLI tool to prevent Kubernetes misconfigurations by ensuring that manifests and Helm charts follow best practices as well as your organization’s policies
- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible
- [kube-lint](https://github.com/viglesiasce/kube-lint) — A linter for Kubernetes resources with a customizable rule set. You define a list of rules that you would like to validate against your  resources and kube-lint will evaluate those rules against them.
- [kube-linter](https://github.com/stackrox/kube-linter) — KubeLinter is a static analysis tool that checks Kubernetes YAML files  and Helm charts to ensure the applications represented in them adhere to best practices. 
- [kubeval](https://kubeval.instrumenta.dev) — Validates your Kubernetes configuration files and supports multiple Kubernetes versions.


<h2 id="latex">LaTeX</h2>

- [ChkTeX](http://www.nongnu.org/chktex) — A linter for LaTex which catches some typographic errors LaTeX oversees.
- [lacheck](https://www.ctan.org/pkg/lacheck) — A tool for finding common mistakes in LaTeX documents.
- [TeXLab](https://texlab.netlify.app) — A Language Server Protocol implementation for TeX/LaTeX, including lint capabilities.


<h2 id="laravel">Laravel</h2>

- [Enlightn](https://www.laravel-enlightn.com/) — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.


<h2 id="make">Makefiles</h2>

- [checkmake](https://github.com/mrtazz/checkmake) — Linter / Analyzer for Makefiles.
- [portlint](https://www.freebsd.org/cgi/man.cgi?query=portlint&sektion=1&manpath=FreeBSD+8.1-RELEASE+and+Ports) — A verifier for FreeBSD and DragonFlyBSD port directories.


<h2 id="markdown">Markdown</h2>

- [markdownlint](https://github.com/DavidAnson/markdownlint) — Node.js -based style checker and lint tool for Markdown/CommonMark files.
- [mdformat](https://mdformat.rtfd.io) — CommonMark compliant Markdown formatter
- [mdl](https://github.com/mivok/markdownlint) — A tool to check Markdown files and flag style issues.
- [remark-lint](https://remark.js.org) — Pluggable Markdown code style linter written in JavaScript.


<h2 id="meta">Metalinter</h2>

- [ciocheck](https://github.com/ContinuumIO/ciocheck) :warning: — Linter, formatter and test suite helper. As a linter, it is a wrapper around `pep8`, `pydocstyle`, `flake8`, and `pylint`.
- [flake8](https://github.com/PyCQA/flake8) — A wrapper around `pyflakes`, `pycodestyle` and `mccabe`.
- [Go Meta Linter](https://github.com/alecthomas/gometalinter) :warning: — Concurrently run Go lint tools and normalise their output. Use `golangci-lint` for new projects.
- [goreporter](https://github.com/360EntSecGroup-Skylar/goreporter) — Concurrently runs many linters and normalises their output to a report.
- [multilint](https://github.com/adamchainz/multilint) :warning: — A wrapper around `flake8`, `isort` and `modernize`.
- [prospector](https://github.com/PyCQA/prospector) — A wrapper around `pylint`, `pep8`, `mccabe` and others.


<h2 id="mobile">Mobile</h2>

- [Android Lint](http://tools.android.com/tips/lint) — Run static analysis on Android projects.
- [android-lint-summary](https://passy.github.io/android-lint-summary) :warning: — Combines lint errors of multiple projects into one output, check lint results of multiple sub-projects at once.
- [FlowDroid](https://github.com/secure-software-engineering/FlowDroid) — Static taint analysis tool for Android applications.
- [iblessing](https://www.kitploit.com/2020/08/iblessing-ios-security-exploiting.html) — iblessing is an iOS security exploiting toolkit. It can be used for reverse engineering, binary analysis and vulnerability mining.
- [Oversecured](https://oversecured.com) :copyright: — Enterprise vulnerability scanner for Android and iOS apps. It allows app owners and developers to secure each new version of a mobile app by integrating Oversecured into the development process.
- [paprika](https://github.com/GeoffreyHecht/paprika) :warning: — A toolkit to detect some code smells in analyzed Android applications.
- [qark](https://github.com/linkedin/qark) :warning: — Tool to look for several security related Android application vulnerabilities.
- [redex](https://fbredex.com) — Redex provides a framework for reading, writing, and analyzing .dex files, and a set of optimization passes  that use this framework to improve the bytecode. An APK optimized by Redex should be smaller and faster.


<h2 id="nix">Nix</h2>

- [deadnix](https://github.com/astro/deadnix) — Scan Nix files for dead code (unused variable bindings)
- [statix](https://git.peppe.rs/languages/statix/about/) — Lints and suggestions for the Nix programming language. "statix check" highlights antipatterns in Nix code. "statix fix" can fix several such occurrences.


<h2 id="nodejs">Node.js</h2>

- [lockfile-lint](https://github.com/lirantal/lockfile-lint) — Lint an npm or yarn lockfile to analyze and detect security issues
- [njsscan](https://opensecurity.in) — A static application testing (SAST) tool that can find insecure code patterns in your node.js applications using simple pattern matcher from libsast and syntax-aware semantic code pattern search tool semgrep.
- [NodeJSScan](https://opensecurity.in) — A static security code scanner for Node.js applications powered by libsast and semgrep that builds on the njsscan cli tool. It features a UI with various dashboards about an application's security status.
- [standard](http://standardjs.com) — An npm module that checks for Javascript Styleguide issues.


<h2 id="package">Packages</h2>

- [lintian](https://lintian.debian.org) — Static analysis tool for Debian packages.
- [rpmlint](https://github.com/rpm-software-management/rpmlint) — Tool for checking common errors in rpm packages.


<h2 id="protobuf">Protocol Buffers</h2>

- [buf](https://buf.build) — Provides a CLI linter that enforces good API design choices and structure
- [protolint](https://github.com/yoheimuta/protolint) — Pluggable linter and fixer to enforce Protocol Buffer style and conventions.


<h2 id="puppet">Puppet</h2>

- [metadata-json-lint](https://github.com/voxpupuli/metadata-json-lint) — Tool to check the validity of Puppet metadata.json files.


<h2 id="rails">Rails</h2>

- [dawnscanner](https://github.com/thesp0nge/dawnscanner) — A static analysis security scanner for ruby written web applications. It supports Sinatra, Padrino and Ruby on Rails frameworks.


<h2 id="security">Security/SAST</h2>

- [AzSK](https://azsk.azurewebsites.net/) — Secure DevOps kit for Azure (AzSK) provides security IntelliSense, Security Verification Tests (SVTs), CICD scan vulnerabilities, compliance issues, and infrastructure misconfiguration in your infrastructure-as-code. Supports Azure via ARM.
- [brakeman](https://brakemanscanner.org) — A static analysis security vulnerability scanner for Ruby on Rails applications.
- [Datree](https://datree.io/) — A CLI tool to prevent Kubernetes misconfigurations by ensuring that manifests and Helm charts follow best practices as well as your organization’s policies
- [Enlightn](https://www.laravel-enlightn.com/) — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.
- [Gitleaks](https://github.com/zricethezav/gitleaks) — A SAST tool for detecting hardcoded secrets like passwords, api keys, and tokens in git repos.
- [gokart](https://github.com/praetorian-inc/gokart) — Golang security analysis with a focus on minimizing false positives. It is capable of tracing the source of variables and function arguments  to determine whether input sources are safe.
- [iblessing](https://www.kitploit.com/2020/08/iblessing-ios-security-exploiting.html) — iblessing is an iOS security exploiting toolkit. It can be used for reverse engineering, binary analysis and vulnerability mining.
- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible
- [lockfile-lint](https://github.com/lirantal/lockfile-lint) — Lint an npm or yarn lockfile to analyze and detect security issues
- [njsscan](https://opensecurity.in) — A static application testing (SAST) tool that can find insecure code patterns in your node.js applications using simple pattern matcher from libsast and syntax-aware semantic code pattern search tool semgrep.
- [NodeJSScan](https://opensecurity.in) — A static security code scanner for Node.js applications powered by libsast and semgrep that builds on the njsscan cli tool. It features a UI with various dashboards about an application's security status.
- [Oversecured](https://oversecured.com) :copyright: — Enterprise vulnerability scanner for Android and iOS apps. It allows app owners and developers to secure each new version of a mobile app by integrating Oversecured into the development process.
- [PT Application Inspector](https://www.ptsecurity.com) :copyright: — Identifies code flaws and detects vulnerabilities to prevent web attacks. Demonstrates remote code execution by presenting possible exploits.
- [Qualys Container Security](https://www.qualys.com/apps/container-security) :copyright: — Container native application protection to provide visibility and control of containerized applications.
- [QuantifiedCode](https://github.com/quantifiedcode/quantifiedcode) :warning: — Automated code review & repair. It helps you to keep track of issues and metrics in your software projects, and can be easily extended to support new types of analyses.
- [Reshift](https://www.reshiftsecurity.com) :copyright: — A source code analysis tool for detecting and managing Java security vulnerabilities.
- [scorecard](https://github.com/ossf/scorecard) — Security Scorecards - Security health metrics for Open Source
- [SearchDiggity](https://resources.bishopfox.com/resources/tools/google-hacking-diggity/attack-tools/) :copyright: — Identifies vulnerabilities in open source code projects  hosted on Github, Google Code, MS CodePlex, SourceForge, and more.  The tool comes with over 130 default searches that identify SQL injection,  cross-site scripting (XSS), insecure remote and local file includes, hard-coded passwords, etc. 
- [Symfony Insight](https://insight.symfony.com/) :copyright: — Detect security risks, find bugs and provide actionable metrics for PHP projects.
- [tfsec](https://github.com/tfsec/tfsec) — Terraform static analysis tool that prevents potential security issues by checking cloud misconfigurations at build time and directly integrates with the HCL parser for better results. Checks for violations of AWS, Azure and GCP security best practice recommendations.
- [Tsunami Security Scanner](https://github.com/google/tsunami-security-scanner) — A general purpose network security scanner with an extensible plugin system for  detecting high severity RCE-like vulnerabilities with high confidence. Custom detectors for finding vulnerabilities (e.g. open APIs) can be added.


<h2 id="smart-contracts">Smart Contracts</h2>

- [mythril](https://github.com/ConsenSys/mythril) — A symbolic execution framework with batteries included, can be used to find and exploit vulnerabilities in smart contracts automatically.
- [MythX](https://mythx.io) :copyright: — MythX is an easy to use analysis platform which integrates several analysis methods like fuzzing, symbolic execution and static analysis to find vulnerabilities with high precision. It can be integrated with toolchains like Remix or VSCode or called from the command-line.
- [slither](https://github.com/trailofbits/slither) — Static analysis framework that runs a suite of vulnerability detectors, prints visual information about contract details, and provides an API to easily write custom analyses.
- [solhint](https://protofire.github.io/solhint) — Solhint is an open source project created by https://protofire.io. Its goal is to provide a linting utility for Solidity code.
- [solium](https://ethlint.readthedocs.io/en/latest) — Solium is a linter to identify and fix style and security issues in Solidity smart contracts.


<h2 id="support">Support</h2>

- [LibVCS4j](https://github.com/uni-bremen-agst/libvcs4j) — A Java library that allows existing tools to analyse the evolution of software systems by providing a common API for different version control systems and issue trackers.
- [Violations Lib](https://github.com/tomasbjerre/violations-lib) — Java library for parsing report files from static code analysis. Used by a bunch of Jenkins, Maven and Gradle plugins.


<h2 id="template">Template-Languages</h2>

- [ember-template-lint](https://github.com/ember-template-lint/ember-template-lint) — Linter for Ember or Handlebars templates.
- [haml-lint](https://github.com/sds/haml-lint) — Tool for writing clean and consistent HAML.
- [slim-lint](https://github.com/sds/slim-lint) — Configurable tool for analyzing Slim templates.
- [yamllint](https://yamllint.readthedocs.io) — Checks YAML files for syntax validity, key repetition and cosmetic problems such as lines length, trailing spaces, and indentation.


<h2 id="terraform">Terraform</h2>

- [kics](https://kics.io/) — Find security vulnerabilities, compliance issues, and infrastructure misconfigurations in your infrastructure-as-code. Supports Terraform, Kubernetes, Docker, AWS CloudFormation and Ansible
- [shisho](https://docs.shisho.dev/) — A lightweight static code analyzer designed for developers and security teams. It allows you to analyze and transform source code with an intuitive DSL similar to sed, but for code.


<h2 id="translation">Translation</h2>

- [dennis](https://github.com/willkg/dennis) :warning: — A set of utilities for working with PO files to ease development and improve quality.


<h2 id="vue">Vue.js</h2>

- [HTML-Validate](https://html-validate.org/) — Offline HTML5 validator.
- [Vetur](https://marketplace.visualstudio.com/items?itemName=octref.vetur) — Vue tooling for VS Code, powered by vls (vue language server). Vetur has support for formatting embedded HTML, CSS, SCSS, JS, TypeScript, and more. Vetur only has a "whole document formatter" and cannot format arbitrary ranges.


<h2 id="wasm">Webassembly</h2>

- [Twiggy](https://rustwasm.github.io/twiggy) — Analyzes a binary's call graph to profile code size. The goal is to slim down wasm binary size.


<h2 id="writing">Writing</h2>

- [After the Deadline](https://open.afterthedeadline.com) :warning: — Spell, style and grammar checker.
- [alex](https://alexjs.com) — Catch insensitive, inconsiderate writing
- [codespell](https://github.com/codespell-project/codespell) — Check code for common misspellings.
- [languagetool](https://languagetool.org) — Style and grammar checker for 25+ languages. It finds many errors that a simple spell checker cannot detect.
- [misspell-fixer](https://github.com/vlajos/misspell-fixer) — Quick tool for fixing common misspellings, typos in source code.
- [Misspelled Words In Context](https://jwilk.net/software/mwic) — A spell-checker that groups possible misspellings and shows them in their contexts.
- [proselint](http://proselint.com) — A linter for English prose with a focus on writing style instead of grammar.
- [vale](https://docs.errata.ai/vale/about) — A syntax-aware linter for prose built with speed and extensibility in mind.
- [write-good](https://github.com/btford/write-good) — A linter with a focus on eliminating "weasel words".


<h2 id="yaml">YAML</h2>

- [Spectral](https://stoplight.io/open-source/spectral) — A flexible JSON/YAML linter, with out-of-the-box support for OpenAPI v2/v3 and AsyncAPI v2.
- [yamllint](https://yamllint.readthedocs.io) — Checks YAML files for syntax validity, key repetition and cosmetic problems such as lines length, trailing spaces, and indentation.


<h2 id="git">git</h2>

- [commitlint](https://commitlint.js.org) — checks if your commit messages meet the conventional commit format


## More Collections

- [Clean code linters](https://github.com/collections/clean-code-linters) — A collection of linters in github collections
- [Code Quality Checker Tools For PHP Projects](https://github.com/collections/code-quality-in-php) — A collection of PHP linters in github collections
- [go-tools](https://github.com/dominikh/go-tools) — A collection of tools and libraries for working with Go code, including linters and static analysis
- [linters](https://github.com/mcandre/linters) — An introduction to static code analysis
- [OWASP Source Code Analysis Tools](https://owasp.org/www-community/Source_Code_Analysis_Tools) — List of tools maintained by the Open Web Application Security Project
- [php-static-analysis-tools](https://github.com/exakat/php-static-analysis-tools) — A reviewed list of useful PHP static analysis tools
- [Wikipedia](http://en.wikipedia.org/wiki/List_of_tools_for_static_code_analysis) — A list of tools for static code analysis.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.


Title image [Designed by Freepik](http://www.freepik.com).