{
  description = "My site made with Notchka";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    systems.url = "github:nix-systems/default";
    rust-notchka.url = "github:JanotLeLapin/notchka";
  };

  outputs = {
    self,
    nixpkgs,
    systems,
    rust-notchka,
    ...
  }:
  let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    pkgs = system: import nixpkgs { inherit system; };
    notchka = system: rust-notchka.packages.${system}.default;
  in {
    devShells = eachSystem (system: { default = (pkgs system).mkShell { packages = [ (notchka system) ]; }; });
    packages = eachSystem (system: {
      default = (pkgs system).stdenv.mkDerivation {
        name = "my-site";
        version = "0.1.0";
        src = ./.;

        buildInputs = [ (notchka system) ];
        buildPhase = "notchka build";
        installPhase = "cp -r build $out";
      };
    });
  };
}
