{ mkShell
, inputs
, system
, pkgs
, ...
}:

mkShell {
  name = "rust";

  inherit (inputs.self.checks.${system}.pre-commit) shellHook;
  buildInputs = inputs.self.checks.${system}.pre-commit.enabledPackages;

  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
    glib
    pango
  ];

  packages = with pkgs; [
    rofi-wayland
  ];

  CARGO_HOME = "cargo";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUSTFLAGS = "--cfg rofi_next";
}
