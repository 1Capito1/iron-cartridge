{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.SDL2
    pkgs.SDL2_image    # Optional: For loading images
    pkgs.SDL2_ttf      # Optional: For font rendering
    pkgs.SDL2_mixer    # Optional: For audio
  ];
}
