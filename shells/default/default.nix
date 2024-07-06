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

  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUSTFLAGS = "--cfg rofi_next";
  EDITOR = "code -w";
}
