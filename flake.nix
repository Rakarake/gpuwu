{
  description = "gpuwu is very UwU";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          #runDerivation = nixpkgs.mkShell { shellHook = "cargo run"; };
        in {
          devShells.default = with pkgs; mkShell rec {
            nativeBuildInputs = [
              pkg-config
            ];
            buildInputs = [
              udev alsa-lib vulkan-loader
              # To use the x11 feature
              xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr 
              # To use the wayland feature
              libxkbcommon wayland
              # Cmake
              cmake
              # Fontconfig
              fontconfig
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
          #apps.default = { type = "app"; program = "${runDerivation}/bin/???????"; };
        }
      );
}

