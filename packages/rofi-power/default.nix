{ rustPlatform
, lib
, pkgs
, ...
}:

let
  root = ../../plugins/rofi-logout;
  manifest = (lib.importTOML "${root}/Cargo.toml").package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource root;
  cargoLock.lockFile = "${root}/Cargo.lock";

  buildInputs = with pkgs; [
    pkg-config
    glib
    pango
  ];
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  RUSTFLAGS = "--cfg rofi_next";

  postInstall = ''
    mkdir -p $out/lib/rofi
    mv $out/lib/lib*.so $out/lib/rofi
  '';
}
