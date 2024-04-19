{
  description = "A flake for a Rust program";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy
            ];
          };
        }
      );

      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "my-rust-program";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );

      defaultPackage = forAllSystems (system: self.packages.${system}.default);
    };
}

