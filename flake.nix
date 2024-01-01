{
  description = "A static-site generator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    crate2nix.url = "github:kolloch/crate2nix";
    overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    nixpkgs,
    crate2nix,
    overlay,
    systems,
    ...
  }:
  let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    pkgsFn = system: import nixpkgs {
      inherit system;
      overlays = [ (import overlay) ];
    };

    manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
  in {
    devShells = eachSystem (system: let
      pkgs = (pkgsFn system);
    in { default = pkgs.mkShell { packages = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ]; }; });
    package = params: let
      pkgs = (pkgsFn params.system);
      crate = pkgs.callPackage "${crate2nix}/tools.nix" { inherit pkgs; };
    in (import (crate.generatedCargoNix {
      name = manifest.name;
      src = ./.;
    }) { inherit pkgs; }).rootCrate.build.override {
      features =
        if params.dev then ["dev"] else [];
    };
    templates.default = {
      path = ./template;
      description = "A starter project using Notchka";
      welcomeText = ''
        # Notchka Template

        Welcome to Notchka! Run `notchka dev` from a dev shell to start a development server.
      '';
    };
  };
}
