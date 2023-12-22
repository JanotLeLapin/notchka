{
  description = "My note-taking repository";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:Numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    overlay,
    flake-utils,
    ...
  }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import overlay) ];
    };

    manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
    generator = pkgs.rustPlatform.buildRustPackage {
      name = manifest.name;
      pname = manifest.name;
      version = manifest.version;
      src = ./.;

      cargoBuildFlags = "--release";
      cargoLock = { lockFile = ./Cargo.lock; };
    };
  in {
    devShells.default = pkgs.mkShell {
      packages = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
    };
    packages.generator = generator;
    packages.default = pkgs.stdenv.mkDerivation {
      name = "uni";
      version = manifest.version;
      src = ./.;

      buildInputs = [ generator ];
      buildPhase = "generator";
      installPhase = "cp -r dist $out";
    };
  });
}
