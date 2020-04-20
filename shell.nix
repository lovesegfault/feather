let
  pkgs = import ./nix {};
in
pkgs.mkShell {
  name = "rayzor";
  nativeBuildInputs = with pkgs; [
    cargo
    cmake
    pkgconfig

    rust-analyzer
    cargo-edit
    cargo-tree

    cargo-flamegraph
    linuxPackages_latest.perf

    niv
    nixpkgs-fmt
  ];
}
