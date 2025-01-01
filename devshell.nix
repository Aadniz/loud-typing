{
  mkShell,
  lib,
  rust-analyzer-unwrapped,
  rustfmt,
  clippy,
  cargo,
  rustc,
  rustPlatform,
  openssl,
  pkg-config,
  python311Packages,
  xorg,
  libxkbcommon,
  alsa-utils,
  alsa-lib
}:
mkShell {
  strictDeps = true;

  buildInputs = [
    alsa-utils
    alsa-lib
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXtst
    xorg.libXrandr
    libxkbcommon
  ];

  # causes redefinition of _FORTIFY_SOURCE
  hardeningDisable = [ "all" ];

  nativeBuildInputs = [
    cargo
    rustc

    rust-analyzer-unwrapped
    rustfmt
    clippy
    openssl
    pkg-config
  ];


  env = {
    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  };
}
