{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
      };
      rustPlatform = pkgs.makeRustPlatform {
        # inherit (rustToolchain) cargo rustc;
        cargo = rustToolchain.cargo;
        rustc = rustToolchain.rustc;
      };
    in
    {

      packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;

      packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = (with pkgs; [

        ]) ++ [
          rustToolchain
          rustPlatform.bindgenHook
        ];
      };

      formatter.${system} = pkgs.nixpkgs-fmt;

    };

}
