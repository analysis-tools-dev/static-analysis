# Awesome static analysis [![Awesome](https://cdn.rawgit.com/sindresorhus/awesome/d7305f38d29fed78fa85652e3a63e154dd8e8829/media/badge.svg)](https://github.com/sindresorhus/awesome)

> Static program analysis is the analysis of computer software that is performed without actually executing programs (analysis performed on executing programs is known as dynamic analysis). — Definition by [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

This is a collection of static analysis tools and code quality checkers for all programming languages.  
Explanation: [OSS] stands for Open-Source-Software, [PROPRIETARY] stands for proprietary software.  
**Pull requests are very welcome!**

Table of Contents
-----------------

- [C/C++](#cc)
- [C#](#c)
- [Containers](#containers)
- [Elixir](#elixir)
- [Go](#go)
- [Groovy](#groovy)
- [Haskell](#haskell)
- [Haxe](#haxe)
- [Html](#html)
- [Java](#java)
- [JavaScript](#javascript)
- [Lua](#lua)
- [Python](#python)
- [PHP](#php)
- [R](#r)
- [Ruby](#ruby)
- [Rust](#rust)
- [Scala](#scala)
- [Shell](#shell)
- [Swift](#swift)
- [Meta](#meta)
- [Multiple Languages](#multiple-languages)
- [Web-Services](#web-services)

## C/C++

* [CMetrics](https://github.com/MetricsGrimoire/CMetrics) [OSS] - Measures size and complexity for C files
* [cqmetrics](https://github.com/dspinellis/cqmetrics) [OSS] - quality metrics for C code
* [clang-tidy](http://clang.llvm.org/extra/clang-tidy/) [OSS] - clang static analyser
* [cppcheck](https://github.com/danmar/cppcheck) [OSS] - static analysis of C/C++ code
* [flawfinder](http://www.dwheeler.com/flawfinder/) [OSS] - finds possible security weaknesses
* [oclint](http://oclint.org/) [OSS] - static analysis of C/C++ code
* [splint](http://www.splint.org/) [OSS] - static analysis of C/C++ code
* [tis-interpreter](https://github.com/TrustInSoft/tis-interpreter) [OSS] - An interpreter for finding subtle bugs in programs written in standard C 
* [vera++](https://bitbucket.org/verateam/vera/wiki/Introduction) [OSS] - Vera++ is a programmable tool for verification, analysis and transformation of C++ source code.

## C# #

* [Code Analysis Rule Collection](https://carc.codeplex.com/) [OSS] - Contains a set of diagnostics, code fixes and refactorings built on the Microsoft .NET Compiler Platform "Roslyn".
* [code-cracker](https://github.com/code-cracker/code-cracker) [OSS] - An analyzer library for C# and VB that uses Roslyn to produce refactorings, code analysis, and other niceties.
* [CSharpEssentials](https://github.com/DustinCampbell/CSharpEssentials) [OSS] - C# Essentials is a collection of Roslyn diagnostic analyzers, code fixes and refactorings that make it easy to work with C# 6 language features.
* [Designite](http://www.designite-tools.com) [PROPRIETARY] - Designite is a software design quality assessment tool. It supports detection of implementation and design smells, computation of various code quality metrics, and trend analysis.
* [Gendarme](http://www.mono-project.com/docs/tools+libraries/tools/gendarme/)[OSS] - Gendarme inspects programs and libraries that contain code in ECMA CIL format (Mono and .NET) and looks for common problems with the code, problems that compiler do not typically check or have not historically checked.
* [.NET Analyzers](https://github.com/DotNetAnalyzers) [OSS] - An organization for the development of analyzers (diagnostics, code fixes, and refactorings) using the .NET Compiler Platform.
* [SonarLint for Visual Studio](https://github.com/SonarSource/sonarlint-vs) [OSS] - SonarLint is a Visual Studio 2015 extension that provides on-the-fly feedback to developers on new bugs and quality issues injected into .NET code.
* [Refactoring Essentials](http://vsrefactoringessentials.com/) [OSS] - The premier free Visual Studio 2015 extension for C# and VB.NET refactorings, including code best practice analyzers to improve your projects.
* [ReSharper](https://www.jetbrains.com/resharper/) [PROPRIETARY] - Extends Visual Studio with on-the-fly code inspections for C#, VB.NET, ASP.NET, JavaScript, TypeScript and other technologies.
* [VSDiagnostics](https://github.com/Vannevelj/VSDiagnostics) [OSS] - A collection of static analyzers based on Roslyn that integrate with VS.
* [Wintellect.Analyzers](https://github.com/Wintellect/Wintellect.Analyzers) [OSS] - .NET Compiler Platform ("Roslyn") diagnostic analyzers and code fixes written by Wintellect.

## Containers

* [clair](https://github.com/coreos/clair) [OSS] - Vulnerability Static Analysis for Containers 
* [collector](https://github.com/banyanops/collector) [OSS] - Run arbitrary scripts inside containers, and gather useful information
* [Haskell Dockerfile Linter](https://github.com/lukasmartinelli/hadolint) [OSS] - A smarter Dockerfile linter that helps you build best practice Docker images

## Elixir

* [credo](https://github.com/rrrene/credo) [OSS] -  A static code analysis tool with a focus on code consistency and teaching. 

## Go

* [dingo-hunter](https://github.com/nickng/dingo-hunter) [OSS] - Static analyser for finding Deadlocks in Go
* [flen](https://github.com/lafolle/flen) [OSS] - Get info on length of functions in a Go package
* [go/ast](https://golang.org/pkg/go/ast/) [OSS] - Package ast declares the types used to represent syntax trees for Go packages.
* [gocyclo](https://github.com/fzipp/gocyclo) [OSS] - Calculate cyclomatic complexities of functions in Go source code
* [Go Meta Linter](https://github.com/alecthomas/gometalinter) [OSS] - Concurrently run Go lint tools and normalise their output
* [go vet](https://godoc.org/golang.org/x/tools/cmd/vet) [OSS] - Examines Go source code and reports suspicious constructs
* [ineffassign](https://github.com/gordonklaus/ineffassign) - Detect ineffectual assignments in Go code
* [safesql](https://github.com/stripe/safesql) [OSS] - Static analysis tool for Golang that protects against SQL injections 

## Groovy

* [CodeNarc](https://github.com/CodeNarc/CodeNarc) [OSS] - a static analysis tool for Groovy source code, enabling monitoring and enforcement of many coding standards and best practices

## Haskell

* [HLint](https://github.com/ndmitchell/hlint) [OSS] - HLint is a tool for suggesting possible improvements to Haskell code.

## Haxe

* [Haxe Checkstyle](https://github.com/HaxeCheckstyle/haxe-checkstyle) [OSS] - A static analysis tool to help developers write Haxe code that adheres to a coding standard.

## HTML

* [HTMLHint](https://github.com/yaniswang/HTMLHint) [OSS] - A Static Code Analysis Tool for HTML
* [HTML Inspector](https://github.com/philipwalton/html-inspector) [OSS] - HTML Inspector is a code quality tool to help you and your team write better markup. 

## Java

* [checkstyle](https://github.com/checkstyle/checkstyle) [OSS] - checking Java source code for adherence to a Code Standard or set of validation rules (best practices)
* [ckjm](http://www.spinellis.gr/sw/ckjm/) [OSS] - calculates Chidamber and Kemerer object-oriented metrics by processing the bytecode of compiled Java files
* [Error-prone](https://github.com/google/error-prone) [OSS] - Catch common Java mistakes as compile-time errors·
* [fb-contrib](https://github.com/mebigfatguy/fb-contrib) [OSS] - A plugin for FindBugs with additional bug detectors
* [Findbugs](https://github.com/findbugsproject/findbugs) [OSS] - FindBugs is a program to find bugs in Java programs. It looks for patterns are likely to be errors.
* [find-sec-bugs](https://find-sec-bugs.github.io/) [OSS] - IDE/Sonarcube plugin for security audits of Java web applications.
* [PMD](https://pmd.github.io/) [OSS] - A Java source code analyzer

## JavaScript

* [aether](https://github.com/codecombat/aether) [OSS] - Lint, analyze, normalize, transform, sandbox, run, step through, and visualize user JavaScript, in node or the browser.
* [ClosureLinter](https://github.com/google/closure-linter) [OSS] - ensures that all of your project's JavaScript code follows the guidelines in the Google JavaScript Style Guide. It can also automatically fix many common errors
* [complexity-report](https://github.com/jared-stilwell/complexity-report) [OSS] - Software complexity analysis for JavaScript projects
* [escomplex](https://github.com/jared-stilwell/escomplex) [OSS] - Software complexity analysis of JavaScript-family abstract syntax trees.
* [eslint](https://github.com/eslint/eslint) [OSS] - A fully pluggable tool for identifying and reporting on patterns in JavaScript
* [Esprima](https://github.com/jquery/esprima) [OSS] - ECMAScript parsing infrastructure for multipurpose analysis
* [quality](https://github.com/jden/quality) [OSS] - zero configuration code and module linting 
* [jshint](https://github.com/jshint/jshint) [OSS] - detect errors and potential problems in JavaScript code and enforce your team's coding conventions
* [JSLint](https://github.com/douglascrockford/JSLint) [PROPRIETARY] - The JavaScript Code Quality Tool
* [plato](https://github.com/es-analysis/plato) [OSS] - Visualize JavaScript source complexity
* [yardstick](https://github.com/calmh/yardstick) [OSS] - Javascript code metrics
* [XO](https://github.com/sindresorhus/xo) [OSS] - Enforce strict code style. Never discuss code style on a pull request again!

## Lua

* [luacheck](https://github.com/mpeterv/luacheck) [OSS] -  A tool for linting and static analysis of Lua code. 

## Python

* [flake8](https://pypi.python.org/pypi/flake8) [OSS] - the modular source code checker: pep8, pyflakes and co
* [jedi](http://jedi.jedidjah.ch/en/latest/) [OSS] - autocompletion/static analysis library for Python
* [Linty fresh](https://github.com/lyft/linty_fresh) [OSS] - Surface lint errors during code review
* [mypy](http://mypy-lang.org/) [OSS] - an experimental optional static type checker for Python that aims to combine the benefits of dynamic (or "duck") typing and static typing
* [prospector](https://github.com/landscapeio/prospector) [OSS] - output information about errors, potential problems, convention violations and complexity in Python code
* [pyflakes](https://github.com/pyflakes/pyflakes/) [OSS] - A simple program which checks Python source files for errors.
* [pylint](https://github.com/PyCQA/pylint) [OSS] - Looks for programming errors, helps enforcing a coding standard and sniffs for some code smells 
* [Pytype](https://github.com/google/pytype) [OSS] - A static type inferencer for Python code

## PHP

* [DesignPatternDetector](https://github.com/Halleck45/DesignPatternDetector) [OSS] - detection of design patterns in PHP code
* [deptrac](https://github.com/sensiolabs-de/deptrac) [OSS] - Enforce rules for dependencies between software layers.
* [exakat](https://github.com/exakat/exakat) [OSS] - An automated code reviewing engine for PHP
* [GrumPHP](https://github.com/phpro/grumphp) [OSS] - checks code on every commit
* [phan](https://github.com/etsy/phan) [OSS] - a modern static analyzer from etsy
* [php7cc](https://github.com/sstalle/php7cc) [OSS] - PHP 7 Compatibility Checker 
* [php7mar](https://github.com/Alexia/php7mar) [OSS] - assist developers in porting their code quickly to PHP 7
* [phpcpd](https://github.com/sebastianbergmann/phpcpd) [OSS] - Copy/Paste Detector (CPD) for PHP code. 
* [PHP_CodeSniffer](https://github.com/squizlabs/PHP_CodeSniffer) [OSS] - detects violations of a defined set of coding standards
* [phpdcd](https://github.com/sebastianbergmann/phpdcd) [OSS] - Dead Code Detector (DCD) for PHP code. 
* [PhpDependencyAnalysis](https://github.com/mamuz/PhpDependencyAnalysis) [OSS] - builds a dependency graph for a project
* [phpsa](https://github.com/ovr/phpsa) [OSS] - Static analysis tool for PHP. 
* [PHPMD](http://phpmd.org/) [OSS] - finds possible bugs in your code
* [PhpMetrics](https://github.com/Halleck45/PhpMetrics) [OSS] - calculates code complexity metrics
* [PHP Refactoring Browser](https://github.com/QafooLabs/php-refactoring-browser) [OSS] - Refactoring helper
* [PHP-Token-Reflection](https://github.com/Andrewsville/PHP-Token-Reflection) [OSS] - Library emulating the PHP internal reflection
* [PHP-Parser](https://github.com/nikic/PHP-Parser) [OSS] - A PHP parser written in PHP
* [RIPS](https://github.com/ripsscanner/rips) [OSS] -  A static source code analyser for vulnerabilities in PHP scripts
* [Tuli](https://github.com/ircmaxell/Tuli) [OSS] - A static analysis engine 
* [twig-lint](https://github.com/asm89/twig-lint) [OSS] - twig-lint is a lint tool for your twig files.

## R

* [lintr](https://github.com/jimhester/lintr) [PROPRIETARY] - Static Code Analysis for R

## Ruby

* [brakeman](https://github.com/presidentbeef/brakeman) [OSS] - A static analysis security vulnerability scanner for Ruby on Rails applications
* [cane](https://github.com/square/cane) [OSS] - Code quality threshold checking as part of your build 
* [dawnscanner](https://github.com/thesp0nge/dawnscanner) [OSS] - a static analysis security scanner for ruby written web applications. It supports Sinatra, Padrino and Ruby on Rails frameworks. 
* [flay](https://github.com/seattlerb/flay) [OSS] - Flay analyzes code for structural similarities.
* [flog](https://github.com/seattlerb/flog) [OSS] - Flog reports the most tortured code in an easy to read pain
report. The higher the score, the more pain the code is in.
* [laser](https://github.com/michaeledgar/laser) [OSS] - Static analysis and style linter for Ruby code.
* [Mondrian](http://trismegiste.github.io/Mondrian/) [OSS] - a set of static analysis and refactoring tools for more abstraction
* [pelusa](https://github.com/codegram/pelusa) [OSS] -  Static analysis Lint-type tool to improve your OO Ruby code
* [quality](https://github.com/apiology/quality) [OSS] -  Runs quality checks on your code using community tools, and makes sure your numbers don't get any worse over time. 
* [reek](https://github.com/troessner/reek) [OSS] - Code smell detector for Ruby
* [rubocop](https://github.com/bbatsov/rubocop) [OSS] - A Ruby static code analyzer, based on the community Ruby style guide. 
* [rubycritic](https://github.com/whitesmith/rubycritic) [OSS] - A Ruby code quality reporter 
* [ruby-lint](https://github.com/YorickPeterse/ruby-lint) [OSS] - Static code analysis for Ruby
* [SandyMeter](https://github.com/makaroni4/sandi_meter) [OSS] - Static analysis tool for checking Ruby code for Sandi Metz' rules.

## Rust

* [clippy](https://github.com/Manishearth/rust-clippy) [OSS] - a code linter to catch common mistakes and improve your Rust code
* [linter-rust](https://github.com/AtomLinter/linter-rust) [OSS] -  Linting your Rust-files in Atom, using rustc and cargo

## Scala

* [linter](https://github.com/HairyFotr/linter) [OSS] - Linter is a Scala static analysis compiler plugin which adds compile-time checks for various possible bugs, inefficiencies, and style problems.
* [ScalaStyle](https://github.com/scalastyle/scalastyle/wiki) [OSS] - Scalastyle examines your Scala code and indicates potential problems with it.
* [scapegoat](https://github.com/sksamuel/scapegoat) [OSS] - Scala compiler plugin for static code analysis 
* [WartRemover](https://github.com/puffnfresh/wartremover) [OSS] - a flexible Scala code linting tool.


## Shell

* [shellcheck](https://github.com/koalaman/shellcheck) [OSS] - ShellCheck, a static analysis tool that gives warnings and suggestions for bash/sh shell scripts 

## Swift

* [SwiftLint](https://github.com/realm/SwiftLint) [OSS] - A tool to enforce Swift style and conventions
* [Tailor](https://github.com/sleekbyte/tailor) [OSS] - A static analysis and lint tool for source code written in Apple's Swift programming language.

# Meta

## Multiple languages

* [codeburner](https://github.com/groupon/codeburner) [OSS] - Provides a unified interface to sort and act on the issues it finds
* [Coverity Save](http://www.coverity.com/products/coverity-save/) [PROPRIETARY] - Static analysis for  C/C++, Java and C#
* [imhotep](https://github.com/justinabrahms/imhotep) [OSS] - Comment on commits coming into your repository and check for syntactic errors and general lint warnings.
* [Infer](https://github.com/facebook/infer) [OSS] -  A static analyzer for Java, C and Objective-C
* [oclint](https://github.com/oclint/oclint) [OSS] - A static source code analysis tool to improve quality and reduce defects for C, C++ and Objective-C
* [pfff](https://github.com/facebook/pfff) [OSS] - Facebook's tools for code analysis, visualizations, or style-preserving source transformation for many languages
* [PVS-Studio](http://www.viva64.com/en/pvs-studio/) [PROPRIETARY] - static analysis of C/C++ and C# code
* [shipshape](https://github.com/google/shipshape) [OSS] - Static program analysis platform that allows custom analyzers to plug in through a common interface
* [STOKE](https://github.com/StanfordPL/stoke-release) [OSS] - a programming-language agnosti stochastic optimizer for the x86_64 instruction set. It uses random search to explore the extremely high-dimensional space of all possible program transformations
* [XCode](https://developer.apple.com/xcode/) [PROPRIETARY/OSS] - XCode provides a pretty decend UI for [Clang's](http://clang-analyzer.llvm.org/xcode.html) static code analyzer (C/C++, Obj-C)


## Web-Services

* [Codacy](https://www.codacy.com/) [PROPRIETARY] - Code Analysis to ship Better Code, Faster.
* [Code Climate](https://codeclimate.com/) [PROPRIETARY] - The open and extensible static analysis platform, for everyone.
* [ConQAT](http://www.conqat.org/) [OSS] - a toolkit for rapid development and execution of software quality analyses.
* [Functor Prevent](http://www.functor.se/products/prevent/) [PROPRIETARY] - Static code analysis for C code.
* [kiuwan](https://www.kiuwan.com/) [PROPRIETARY] - Software Analytics in the Cloud supporting more than 22 programming languages.
* [Landscape](https://landscape.io/) [PROPRIETARY] - Static code analysis for Python
* [Nitpick CI](https://nitpick-ci.com) [PROPRIETARY] - Automated PHP code review
* [Node Security Platform](https://nodesecurity.io/) [PROPRIETARY] - Continuous Security monitoring for your node apps (free for Open Source Projects)
* [QuantifiedCode](https://www.quantifiedcode.com/) [PROPRIETARY] - Automated code review & repair
* [Scrutinizer](https://scrutinizer-ci.com/) [PROPRIETARY] - A proprietery code quality checker that can be integrated with GitHub
* [SensioLabs Insight](https://insight.sensiolabs.com/) [PROPRIETARY] - Detect security risks, find bugs and provide actionable metrics for PHP projects
* [Snyk](https://snyk.io/) [PROPRIETARY] - Vulnerability scanner for dependencies of node.js apps (free for Open Source Projects)
* [Teamscale](http://www.teamscale.com/) [PROPRIETARY] - analyze, monitor, and improve the quality of your code.


## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](http://matthias-endler.de) has waived all copyright and related or neighboring rights to this work.
