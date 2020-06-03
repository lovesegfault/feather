let
  pkgs = import ./nix {};
in
pkgs.mkShell {
  name = "feather";
  nativeBuildInputs = with pkgs; [
    cargo
    llvmPackages_latest.clang
    llvmPackages_latest.lld

    rust-analyzer
    cargo-edit
    cargo-tree

    cargo-flamegraph
    linuxPackages_latest.perf

    niv
    nixpkgs-fmt
  ];
}
