{
  inputs = rec {
    crane.url = "github:ipetkov/crane";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    crane,
    utils,
    nixpkgs,
    ...
  }: let
    supportedSystems = ["x86_64-linux" "x86_64-darwin" "aarch64-linux"];
  in
    utils.lib.eachSystem supportedSystems
    (
      system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
        packages.oura = crane.lib.${system}.buildPackage {
          src = self;
          cargoExtraArgs = "--features \"kafkasink\"";
          nativeBuildInputs = with pkgs; [ openssl openssl.dev pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_BACKTRACE = "full";
        };
        packages.default = self.packages.${system}.oura;
      }
    );
}
