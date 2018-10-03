# tmux-hints

`tmux-hints` is an application to find matches, ie. urls and navigate them
with the keyboard.

It's a rewrite of a previous tool written by me in go [tmux-url-nav][1].

Inspired by another tool in perl [tmux-url-select][2].

[1]: https://github.com/roger/tmux-url-nav
[2]: https://github.com/dequis/tmux-url-select

## Is it any good?

[Yes][3]

[3]: https://news.ycombinator.com/item?id=3067434

## Requirements

Depends on `rust`, `tmux` and `stty`.

## Installation

Run cargo build and copy/link `target/release/tmux-hints` to somewhere in your
path.

Add this to your `.tmux.conf`:

    bind some-key-here run tmux-hints

Where some-key-here is any key you want to use to start url selection.

## Usage

Once you're inside tmux-hints, keybindings:

 * `j`: down
 * `k`: up
 * `p`: paste (insert text into the tmux window)
 * `o`: open link
 * `O`: open link without closing
 * `q`: quit

## FAQ

Q: Why rust?

A: Needed a excuse to do something on it.
