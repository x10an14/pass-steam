{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self
    , utils
    , naersk
    , nixpkgs
  }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      naersk-lib = pkgs.callPackage naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage ./.;

      defaultApp = utils.lib.mkApp {
        drv = self.defaultPackage."${system}";
      };

      devShell = with pkgs; mkShell {
        buildInputs = [
          cargo
          cargo-edit
          rustc
          rustfmt
          rustPackages.clippy
        ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
    }
  );
}
