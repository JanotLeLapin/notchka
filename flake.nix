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
  in {
    devShells.default = pkgs.mkShell {
      packages = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
    };
  });
}
