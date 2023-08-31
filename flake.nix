{
  description = "gpuwu is very UwU";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; }; in {
        devShell = pkgs.mkShell {
          packages = [ pkgs.wasm-bindgen-cli ];
        };
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "gpuwu";
          version = "0.0.1";
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
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

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
