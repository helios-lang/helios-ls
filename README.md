<p align="center">
  <img src="assets/logo.svg" alt="logo" align="center">
</p>

# Helios Language Server (Helios-LS) [![CI][badge]][ci]

The Helios Language Server is an intermediary between an editor or IDE and the
Helios compiler. It implements the [Language Server Protocol][language-server-protocol],
which allows us to provide Helios with common editor functionality such as
autocomplete, go-to definitions and find-all-references in one centralised
location.

This project is still in its early stages of development. So far, only a limited
functionality is provided. You may see the progress of the project under the
[Progress](#Progress) section.

## Building and testing

Building and testing is as easy as the following:

```shell
$ cargo build # building
$ cargo test # testing
```

## Usage

Once this crate has been built, the executable produced will be called
`helios-ls`. For now, the server can only be communicated via standard input and
output (`stdin` and `stdout`).

You should not need to invoke this executable manually – that is what clients
are for. At the moment, there is only one client implementation available:
[Helios for Visual Studio Code][vscode-helios-github]). This client (essentially
a VS Code language extension) will handle the communication between it and this
language server for you as well as provide editor-specific features previously
mentioned.

## Progress

- [x] `initialize`
- [x] `textdocument/didOpen`
- [x] `textdocument/didChange`
- [ ] `textdocument/didSave`
- [ ] `textdocument/completion`
- [ ] `textdocument/hover`
- [ ] `textdocument/rename`

## License

Unless explicitly stated otherwise, all files in this directory are licensed
under the [Apache License, Version 2.0][apache-license].

The Helios logo (the "hand-drawn" sun) is licensed under a [Creative Commons
Attribution 4.0 International License][cc-license].

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[badge]: https://github.com/helios-lang/helios-ls/workflows/CI/badge.svg
[cc-license]: http://creativecommons.org/licenses/by/4.0/
[ci]: https://github.com/helios-lang/helios-ls/actions?query=workflow:%22CI%22
[language-server-protocol]: https://microsoft.github.io/language-server-protocol/
[vscode-helios-github]: https://github.com/helios-lang/vscode-helios
