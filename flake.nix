{

  # 2. Override the flake-utils default to your version
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, ... }:
    let
      systems = [ "x86_64-linux" "aarch64" ];
    in
    utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; lib.makeLibraryPath [
        ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "easy-neovim-wm-nav";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = with pkgs; [
            iconv
            pkg-config
          ];

          postInstall = ''
            wrapProgram "$out/bin/easy-neovim-wm-nav" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

        };

        apps.default = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            iconv
            rustc
            rustfmt
            rust-analyzer
            pre-commit
            rustPackages.clippy
            pkg-config
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
