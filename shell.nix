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
    niv
    nixpkgs-fmt
  ];

  buildInputs = with pkgs; [
    alsaLib
    SDL2
    libGL
    libxkbcommon
    wayland
    xlibs.libX11
  ];

  APPEND_LIBRARY_PATH = with pkgs; stdenv.lib.makeLibraryPath [
    SDL2
    libGL
    vulkan-loader
    wayland
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
    libxkbcommon
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
