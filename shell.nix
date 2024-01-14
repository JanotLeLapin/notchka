{ mkShell
, rust-bin
, pkg-config
, openssl
}: mkShell {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
    openssl
  ];
}
