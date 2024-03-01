#this .nix code is from a nix fan (that's how they said to be credited instead of nicks/realname wtw) (so, this code is not from me)

#use 'nix-build .', assuming this file is in an empty dir and file is named default.nix
#or 'nix-build default.nix' as a command.

{ pkgs? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
pname = "glutin_simple1";
version = "1.0.0";

src = pkgs.fetchFromGitHub {
owner = "correabuscar";
repo = pname;
rev = "master";
hash = "sha256-Yior+F4bLEosumNNWftC5gNAQUwLyif6NYYdlBg2Mws=";
};

propagatedBuildInputs = with pkgs; [
xorg.libX11
xorg.libXcursor
xorg.libXrandr
xorg.libXi
xorg.libXext
xorg.libXxf86vm
xorg.libXft
xorg.libXinerama
xorg.libXmu
xorg.libXtst
xorg.libXrender
xorg.libXpresent
xorg.libXScrnSaver
xorg.libXt
libGL
];

postFixup = ''
patchelf --set-rpath "${pkgs.lib.makeLibraryPath propagatedBuildInputs}" $out/bin/glutin_simple1
'';

cargoHash = "sha256-B+jso8x700GUOnXEJK2oIJwak+5knhA8kalBRK1xDCg=";

}
