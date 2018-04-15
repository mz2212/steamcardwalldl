# Steam Card Wallpaper DL

A quick and easy way to download all the wallpapers from a steam game.

It first scrapes steamcardexchange for all the big images for the cards, then it downloads the big images in the folder that you ran the program in.  
Check the releases page for a pre-built binary (May not be up to date).

## Usage:  
`steamcardwalldl <APPID>`  
with APPID being the id of the game you want the images from.

## Building:
 - Clone the repo: `git clone https://github.com/mz2212/steamcardwalldl`
 - Build with cargo: `cargo build --release`