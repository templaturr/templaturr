{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.cargo
  ];

  shellHook = ''
    export RUST_BACKTRACE=1

    echo "Rust environment setup"
    echo "$(rustc --version)"
    echo "$(cargo --version)"
  '';
}
