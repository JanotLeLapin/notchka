{
  description = "My note-taking repository";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    crate2nix.url = "github:kolloch/crate2nix";
    overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:Numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crate2nix,
    overlay,
    flake-utils,
    ...
  }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import overlay) ];
    };

    manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
    generator =
      let
        crate = pkgs.callPackage "${crate2nix}/tools.nix" { inherit pkgs; };
      in import (crate.generatedCargoNix {
        name = manifest.name;
        src = ./.;
      }) {
        inherit pkgs;
      };
  in rec {
    devShells.default = pkgs.mkShell {
      packages = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
    };
    packages.generator = generator.rootCrate.build;
    packages.default = pkgs.stdenv.mkDerivation {
      name = "uni";
      version = manifest.version;
      src = ./.;

      buildInputs = [ packages.generator ];
      buildPhase = "generator";
      installPhase = "cp -r dist $out";
    };
  });
}
