# Rofi-Plugins

This repository contains several plugins for [rofi](https://github.com/davatorium/rofi) made in rust using [rofi-mode.rs](https://github.com/SabrinaJewson/rofi-mode.rs).

### rofi-power

Provides a **power** mode that gives options for logging off, rebooting, shutting down, etc.

# Installation

The plugins are availble as nix packages via flakes. Simply include
```nix
rofi-plugins.url = "github:ElliottSullingeFarrall/rofi-plugins";
```
The packages are availble at `packages.${system}.${plugin-name}` and overlays for use with **nixpkgs** are availble at `overlays."package/${plugin-name}$` and `overlays.default`.
