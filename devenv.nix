{
  config,
  lib,
  pkgs,
  ...
}:

{
  packages =
    with pkgs;
    [
      # necessary?
      # pkg-config
      # libGL
      # lld
      # clang
      # linuxPackages.nvidia_x11
    ]
    ++ lib.optionals pkgs.stdenv.isLinux [
      # required by bevy on linux
      alsa-lib
      libxkbcommon
      udev
      wayland
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      llvmPackages.libcxxStdenv
      llvmPackages.libcxxClang
    ];
  languages.rust = {
    enable = true;
    mold.enable = pkgs.stdenv.isLinux;
  };
  stdenv = pkgs.stdenvNoCC;
  enterShell = builtins.concatStringsSep "\n" [
    (lib.getExe config.scripts.download-assets.scriptPackage)
    (lib.optionalString pkgs.stdenv.isLinux # sh
      ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
          with pkgs;
          lib.makeLibraryPath [
            vulkan-loader
          ]
        }"
      ''
    )
  ];

  scripts.download-assets = {
    exec = # nu
      ''
        # Download game assets
        def main [] {
          let tempDir = "temp"
          mkdir $tempDir
          let outBase = "bevy-ball-game/assets"
          let audioFolder = $outBase | path join "audio"
          let spritesFolder = $outBase | path join "sprites"
          mkdir $audioFolder $spritesFolder

          let archives = [
             # https://kenney.nl/assets/rolling-ball-assets
            [
              "https://kenney.nl/media/pages/assets/rolling-ball-assets/2f4cda9784-1677495701/kenney_rolling-ball-assets.zip"
              "ball-assets.zip"
              [
                "PNG/Default/ball_blue_large.png"
                "PNG/Default/ball_red_large.png"
                "PNG/Default/star.png"
              ]
            ]
            # https://kenney.nl/assets/sci-fi-sounds
            [
              "https://kenney.nl/media/pages/assets/sci-fi-sounds/81d6323e0a-1677589334/kenney_sci-fi-sounds.zip"
              "sci-fi.zip"
              [
                "Audio/laserLarge_000.ogg"
                "Audio/explosionCrunch_000.ogg"
                "Audio/impactMetal_000.ogg"
                "Audio/impactMetal_001.ogg"
                "Audio/impactMetal_002.ogg"
                "Audio/impactMetal_003.ogg"
                "Audio/impactMetal_004.ogg"
              ]
            ]
            # https://kenney.nl/assets/interface-sounds
            [
              "https://kenney.nl/media/pages/assets/interface-sounds/f3134a7c4c-1677589452/kenney_interface-sounds.zip"
              "interface.zip"
              [
                # "Audio/pluck_001.ogg"
                # "Audio/pluck_002.ogg"
              ]
            ]
          ]

          $archives | par-each {|archive| match $archive {
            [$url $zipName $assets] => {
                let zipPath = $"($tempDir)/($zipName)"

                # 1. download
                if (not ($zipPath | path exists)) {
                  print $"Downloading ($zipName)..."
                  http get $url | save -f $zipPath
                }

                # 2. extract
                $assets | par-each {|asset|
                  let assetFilename = $asset | path basename
                  let assetOutPath = $"($tempDir)/($assetFilename)"
                  let extension = $asset | path parse | get extension
                  let assetInstallPath = match $extension {
                    "ogg" => { $audioFolder | path join $assetFilename },
                    "png" => { $spritesFolder | path join $assetFilename },
                    _ => { error make {msg: $"Unknown asset type: ($assetFilename)"} },
                  }

                  if (not ($assetOutPath | path exists)) {
                    print $"Extracting ($assetOutPath)..."
                    unzip -q -j $zipPath $asset -d $tempDir
                  }

                  # 3. install
                  if (not ($assetInstallPath | path exists)) {
                    cp $assetOutPath $assetInstallPath
                  }
                }

              },
            _ => { error make {msg: "Fix archive format."} },
          }}
          return;
        }
      '';
    package = pkgs.nushell;
    binary = "nu";
    description = "Download game assets";
  };

}
