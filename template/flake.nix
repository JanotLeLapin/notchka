{
  description = "My site made with Notchka";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    systems.url = "github:nix-systems/default";
    rust-notchka.url = "git+file:///home/josephd/programs/notchka";
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
  in {
    devShells = eachSystem (system: {
      default = (pkgs system).mkShell {
        packages = [ (rust-notchka.package { inherit system; dev = true; }) ];
      };
    });
    packages = eachSystem (system: {
      default = (pkgs system).stdenv.mkDerivation {
        name = "my-site";
        version = "0.1.0";
        src = ./.;

        buildInputs = [ (rust-notchka.package { inherit system; dev = false; }) ];
        buildPhase = "notchka build";
        installPhase = "cp -r build $out";
      };
    });
  };
}
