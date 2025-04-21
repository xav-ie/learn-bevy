{
  lib,
  pkgs,
  ...
}:

let
  devPkgs = with pkgs; [
    alsa-lib
    libxkbcommon
    pkg-config
    udev.dev
    wayland.dev
  ];
  inherit (pkgs.stdenv) isDarwin isLinux;
in
{
  packages =
    with pkgs;
    [ ]
    ++ lib.optionals isLinux devPkgs
    ++ lib.optionals isDarwin [
      llvmPackages.libcxxStdenv
      llvmPackages.libcxxClang
    ];

  languages.rust = {
    enable = true;
    mold.enable = isLinux;
  };

  # TODO: verify on darwin machine
  stdenv = if isDarwin then pkgs.llvmPackages.stdenv else pkgs.stdenv;

  env =
    { }
    // (lib.optionalAttrs isLinux {
      LD_LIBRARY_PATH =
        with pkgs;
        lib.makeLibraryPath [
          vulkan-loader
        ];
      PKG_CONFIG_PATH = lib.concatMapStringsSep ":" (pkg: "${lib.getDev pkg}/lib/pkgconfig") devPkgs;
    });

  scripts.workaround = {
    exec = ''
      cargo build && touch success
    '';
  };

  scripts.download-assets = {
    exec = builtins.readFile ./download-assets.nu;
    package = pkgs.nushell;
    binary = "nu";
    description = "Download game assets";
  };
}
