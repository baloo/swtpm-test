let
  pkgs = import <nixpkgs> {};
  editor =
    if (builtins.tryEval (builtins.toString <editors>)).success
    then (import <editors/rust>).nativeBuildInputs
    else [];
  #build = import ./default.nix;
in pkgs.mkShell {
  #inherit (build) buildInputs;
  nativeBuildInputs =
    (/*build.nativeBuildInputs or */[])
    ++ (with pkgs; [ rustfmt cargo-watch cargo-license cargo pkg-config openssl opensc zlint clippy cargo-fuzz rustPlatform.bindgenHook tpm2-tss swtpm ])
    ++ editor;
}
