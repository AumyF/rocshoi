{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.hello-world = naersk-lib.buildPackage {
            pname = "hello-world";
            root = ./.;
          };

          # `nix run`
          apps.hello-world = flake-utils.lib.mkApp {
            drv = packages.hello-world;
          };

          # `nix develop`
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo rustfmt clippy rust-analyzer ];
          };
        }
    );
}
