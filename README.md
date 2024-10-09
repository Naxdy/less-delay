# Less Input Delay

This is a **software modification** to Super Smash Bros. Ultimate that removes 1 frame of input
delay.

This is performed by disabling graphical vsync and manually timing the game's core logic to run on
the vsync timer.

Read more about how it works in the [original README](README_orig.md)

## Building

The project can be reproducibly built using only the Nix package manager.

The quickest way for you to get set up is to [download and install Nix](https://nixos.org/download) (the package manager, not necessarily the entire NixOS distribution), then proceed to [enable Nix flakes](https://nixos.wiki/wiki/Flakes), and run this command to build the package (no need to clone this repo beforehand):

```shell
nix build github:Naxdy/less-delay
```

Nix will take care of the rest, that is downloading the necessary toolchain, build tools, and dependencies, as well as compile everything for you. The resulting library will be put in `./result/lib/libless_delay.nro`, which you can then upload to your Switch, or copy to your SD card as usual.

# About this Repo

The purpose of this repo is to exist, or more accurately, to not be removed in the future. I will keep the code of this plugin up to date (though I sincerely doubt that another patch is headed our way, ever) and available, just in case the original repository happens to magically disappear.
