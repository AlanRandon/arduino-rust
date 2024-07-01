with import <nixpkgs> { };
mkShell {
  buildInputs = [
    avrdude
    ravedude
    pkgsCross.avr.buildPackages.binutils
    pkgsCross.avr.buildPackages.gcc
  ];
}
