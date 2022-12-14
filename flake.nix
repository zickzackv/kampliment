{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage rec {
          src = ./.;
          wrapperPath = with pkgs; lib.makeBinPath [
            bat fzf gawk fd ripgrep kakoune coreutils
          ];
          buildInputs = [
            pkgs.makeWrapper
          ];
          postInstall = ''
             for s in scripts/*
             do
               install -m755 -t $out/bin/ "$s"
             done
          '';
          postFixup = ''
            for script in $out/bin/kamp-*
            do
              printf "wrapping program %s\n" $script
              wrapProgram "$script" --prefix PATH : "${wrapperPath}"
            done
          '';
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
