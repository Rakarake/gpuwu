{
  description = "gpuwu is very UwU";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit overlays system; };
        # This makes all targets available, WASM
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        # All packages needed for building
        packageDeps = with pkgs; [
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
      in {
        # The rust package, use `nix build` to build
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "gpuwu";
          version = "0.0.1";
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # Runtime deps
          buildInputs = packageDeps;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };

        # This makes sure we can build for WASM
        # Remember to add necessary changes made in defaultPackage to devShell
        devShell = pkgs.mkShell rec {
          buildInputs = packageDeps;
          packages = [ pkgs.wasm-bindgen-cli pkgs.wasm-pack rust ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
