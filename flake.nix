{
  description = "gpuwu is very UwU";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          #runDerivation = nixpkgs.mkShell { shellHook = "cargo run"; };
        in {
          devShells.default = import ./shell.nix { inherit pkgs; };
          #apps.default = { type = "app"; program = "${runDerivation}/bin/???????"; };
        }
      );
}
