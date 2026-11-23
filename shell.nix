{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    pkg-config
    sdl3
    sdl3-ttf
  ];
}
