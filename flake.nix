{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils?ref=main";
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
      in
      {
        packages = rec {
          think-morse = pkgs.callPackage ./nix/package.nix { };
          default = think-morse;
        };
      }
    )
    // {
      nixosModules = rec {
        think-morse = import ./nix/nixos.nix inputs;
        default = think-morse;
      };
    };
}
