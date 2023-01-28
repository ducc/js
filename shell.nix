{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = [ 
      pkgs.buildPackages.pkg-config
      pkgs.buildPackages.gcc
      pkgs.buildPackages.bintools
    ];
}
