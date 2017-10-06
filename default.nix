{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.latest.rustChannels.stable; [
    rust
    cargo
  ];

  dependencies = with pkgs; [
    cmake
    curl
    dbus
    gcc
    libpsl
    libtool
    ncurses
    openssl
    pkgconfig
    which
    zlib
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = /var/empty;
    version = "0.0.0";

    buildInputs = env ++ dependencies;

}

