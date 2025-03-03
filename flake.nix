{
  description = "Dev shell for Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ...}: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
        pkgs.cargo
        pkgs.rustc
        pkgs.libpcap         # Library for packet capture (required by pnet)
        pkgs.openssl
    ];

    shellHook = ''
        echo "Rust dev shell"
        echo "Rust version: $(rustc --version)"
    '';

    };
  };
}