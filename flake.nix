{
  description = "A static-site generator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    overlay,
    ...
  }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ] (system: let
      overlays = [ (import overlay) ];
      pkgs = (import nixpkgs { inherit system overlays; });
    in (fn { inherit system pkgs; }));
  in {
    devShells = eachSystem ({ pkgs, ... }: { default = pkgs.callPackage ./shell.nix {}; });
    packages = eachSystem ({ pkgs, ... }: { default = pkgs.callPackage ./default.nix {}; });
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
