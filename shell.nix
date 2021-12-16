{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustup
    rust-analyzer
    llvmPackages_latest.lld
    llvmPackages_latest.llvm
    llvmPackages_latest.bintools
    libiconv
  ];
  RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
}
