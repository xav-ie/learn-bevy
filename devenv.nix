{
  pkgs,
  ...
}:

{
  packages = with pkgs; [
    # required by bevy
    alsa-lib
    libxkbcommon
    udev
    wayland

    # pkg-config
    # libGL
    # lld
    # clang
    # linuxPackages.nvidia_x11
  ];
  languages.rust = {
    enable = true;
    mold.enable = true;
  };
  enterShell = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      with pkgs;
      lib.makeLibraryPath [
        vulkan-loader
      ]
    }"
  '';
}
