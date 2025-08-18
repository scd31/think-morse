{
  lib,
  rustPlatform,
}:
let
  ignoredPaths = [
    "nix"
    "flake.nix"
    "flake.lock"
  ];

  cargoManifest = lib.importTOML ../Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = cargoManifest.package.name;
  version = cargoManifest.package.version;

  src = lib.cleanSourceWith {
    filter = name: _: !(builtins.elem (baseNameOf name) ignoredPaths);
    src = lib.cleanSource ../.;
  };

  cargoLock.lockFile = ../Cargo.lock;

  # skips rebuilding the whole thing with debug info
  doCheck = false;
}
