{
  perSystem =
    {
      config,
      lib,
      craneLib,
      pkgs,
      ...
    }:
    let
      cfg = config.pre-commit;

      # some of this was written by ai :(
      # nixpkgs has no wasi-sdk package, but its wasi32 cross toolchain
      # provides clang and wasi-libc built for WebAssembly.
      wasi = pkgs.pkgsCross.wasi32;
      wasiCC = "${wasi.stdenv.cc}/bin/${wasi.stdenv.cc.targetPrefix}clang";
      wasiCXX = "${wasi.stdenv.cc}/bin/${wasi.stdenv.cc.targetPrefix}clang++";
      wasiAR = "${wasi.stdenv.cc}/bin/${wasi.stdenv.cc.targetPrefix}ar";
      wasiLibDir = "${wasi.stdenv.cc.libc}/lib/wasm32-wasip1";
    in
    {
      # Adapted from
      # https://github.com/cachix/git-hooks.nix/blob/dcf5072734cb576d2b0c59b2ac44f5050b5eac82/flake-module.nix#L66-L78
      devShells.default = craneLib.devShell {
        packages = lib.flatten [
          cfg.settings.enabledPackages
          cfg.settings.package

          pkgs.rust-analyzer
          # Native dependencies for the `comp` binary (smithay/winit).
          pkgs.pkg-config
          pkgs.udev
          pkgs.seatd
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs.libX11
          pkgs.libxcb
          pkgs.libdrm
          pkgs.libgbm
          pkgs.libdisplay-info
          pkgs.alsa-lib
        ];

        shellHook = builtins.concatStringsSep "\n" [
          cfg.installationScript
          ''
            # cc-rs must use the wasm cross compiler for the wasm targets;
            # the global CC/CXX from stdenv point at the host.
            export CC_wasm32_wasip2="${wasiCC}"
            export CXX_wasm32_wasip2="${wasiCXX}"
            export AR_wasm32_wasip2="${wasiAR}"
            export CC_wasm32_wasip3="${wasiCC}"
            export CXX_wasm32_wasip3="${wasiCXX}"
            export AR_wasm32_wasip3="${wasiAR}"

            # lua-src links wasi-libc's setjmp and wasi-emulated-signal, and
            # Lua's setjmp/longjmp needs wasm exception handling.
            #
            # Only do this for wasip2: the nixpkgs wasi-libc is built for
            # wasip1, and adding it to the wasip3 link shadows Rust's own
            # cooperative-threading libc.
            export CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS="-Lnative=${wasiLibDir} -Ctarget-feature=+exception-handling"

            # Older versions of this shell exported this for wasip3 too, which
            # shadowed Rust's wasip3 libc with the wasip1 one.
            unset CARGO_TARGET_WASM32_WASIP3_RUSTFLAGS
          ''
        ];
      };
    };
}
