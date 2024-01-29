{
  description = "gpuwu is very UwU";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-utils, nixpkgs, naersk, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = with fenix.packages.${system}; combine [
          minimal.cargo
          minimal.rustc
          latest.clippy
          latest.rust-src
          latest.rustfmt
        ];
        neededPkgs = with pkgs; [
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
      in
      {
        defaultPackage = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage
          {
            src = ./.;
            nativeBuildInputs = neededPkgs;
          };

        devShell = pkgs.mkShell {
          packages = with pkgs; [ wasm-bindgen-cli wasm-pack python3 toolchain ] ++ neededPkgs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath neededPkgs;
        };
      });
}
