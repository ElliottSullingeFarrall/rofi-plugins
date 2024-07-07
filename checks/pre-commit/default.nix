{ lib
, system
, ...
}:

lib.pre-commit-hooks.${system}.run {
  src = ../..;

  hooks = {
    gptcommit = {
      enable = true;
    };

    editorconfig-checker = {
      enable = true;
      types_or = [ "nix" "shell" "rust" ];
    };

    nil.enable = true;
    nixpkgs-fmt.enable = true;
    deadnix.enable = true;

    rustfmt.enable = true;

    taplo.enable = true;
  };
}
