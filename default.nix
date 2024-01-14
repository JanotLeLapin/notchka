{ rustPlatform
, pkg-config
, openssl
}: let
  manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
in rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  src = ./.;
  cargoLock = { lockFile = ./Cargo.lock; };
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}
