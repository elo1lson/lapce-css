# Lapce plugin for Css/Scss

## Prerequisites

Install `vscode-css-language-server` via `npm` or your system package manager  
Server needs to be in one of the paths included in `PATH` environment variable

```shell
npm i --global vscode-css-language-server
```

## Available configuration

```toml
[lapce-css.volt]
serverPath = "<custom executable>"
serverArgs = ["--stdio"] # --stdio is required for all LSPs written in nodejs
```
