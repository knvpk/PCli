{ pkgs, ... }:
let
  rootDir = ../..;
  cargoToml = builtins.fromTOML (builtins.readFile "${toString rootDir}/cli/Cargo.toml");
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = rootDir;

  cargoLock = {
    lockFile = "${rootDir}/Cargo.lock";
  };

  buildType = "release"; # build

  # buildInputs = with pkgs; [
  #   openssl
  # ];

  OPENSSL_DIR = "${pkgs.openssl.out}";
  RUST_BACKTRACE = 1;
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

  nativeBuildInputs = with pkgs; [
    openssl
    pkg-config
  ];

  meta = with pkgs.lib; {
    description = "An cli tool to use with different softwares in seneca global.";
    license = licenses.mit;
    maintainers = cargoToml.package.authors;
    platforms = platforms.linux;
  };
}
