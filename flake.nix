{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      self,
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
          rustVersion = "latest";
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
            src = ./.;

            meta.mainProgram = "web";
            cargoTestFlags = "--all-features";
            cargoBuildFlags = "--package web --workspace";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            cargoCheckHook = ''
              echo test
              exit 1
            '';

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
          };

          docker = pkgs.dockerTools.buildImage {
            name = "commitoria-web";

            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [
                pkgs.openssl
                pkgs.cacert
              ];
            };

            config = {
              Cmd = [ "${self.packages.${pkgs.system}.default}/bin/web" ];
            };
          };
        }
      );
    };
}
