{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          rustVersion = "latest"; # using a specific version: "1.62.0"
          rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
            extensions = [
              "rust-src" # for rust-analyzer
              "rust-analyzer" # usable by IDEs like zed-editor
              "clippy"
            ];
          };
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.pkg-config
              pkgs.openssl
              rust
            ];
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "commitoria-web";
            srcs = [
              ./lib
              ./web
            ];
            sourceRoot = "./web";
            cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./web/Cargo.lock;
            };
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
          };
        }
      );
    };
}
