{ pkgs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.exercism pkgs.ghc pkgs.ghcid pkgs.haskell-language-server ];

  enterShell = "";

#  starship.enable = true;

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;
  languages.elm.enable = true;
  languages.haskell.enable = true;

  # https://devenv.sh/scripts/
  # scripts.hello.exec = "echo hello from $GREET";

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks.shellcheck.enable = true;
  pre-commit.hooks.rustfmt.enable = true;
  pre-commit.hooks.clippy.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";
}
