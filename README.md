# tmux-hints [![Build Status](https://travis-ci.org/Roger/tmux-hints.svg?branch=master)](https://travis-ci.org/Roger/tmux-hints)

`tmux-hints` is an application to find matches, ie. urls and navigate them
with the keyboard.

It's a rewrite of a previous tool written by me in go [tmux-url-nav][1].

Inspired by another tool in perl [tmux-url-select][2].

[1]: https://github.com/roger/tmux-url-nav
[2]: https://github.com/dequis/tmux-url-select

## Is it any good?

[Yes][3]

[3]: https://news.ycombinator.com/item?id=3067434

## Demo

![](https://d.fsck.com.ar/cm0Pv.gif)

## Requirements

Depends on `rust`, `tmux` and `stty`.

## Installation

From crates.io, just `cargo install tmux-hints` and make sure that cargo bin
it's in your path.

Or from source, runn cargo build and copy/link `target/release/tmux-hints`
to somewhere in your path.

In both cases, you need to a key binding to your `.tmux.conf`:

    bind some-key-here run tmux-hints

Where some-key-here is any key you want to use to start hints selection.

## Usage

Once you're inside tmux-hints, keybindings:

 * `j`: down
 * `k`: up
 * `0`-`9`: select by number
 * `p`: paste (insert text into the tmux window)
 * `o`: open link
 * `O`: open link without closing
 * `c`: print current configuration
 * `q`: quit

## Configuration

The configuration uses toml format, you need to create a new file in your $XDG_CONFIG_HOME (in most of the *nix ~/.config/) called tmux-hints.toml

### default config

```
opener = 'xdg-open'
show_position = true

[hint.unselected]
background = 0
foreground = 6
bold = false
dim = false
blink = false
reverse = false
hidden = false
underlined = false

[hint.selected]
background = 6
foreground = 0
bold = false
dim = false
blink = false
reverse = false
hidden = false
underlined = false

[position.unselected]
background = 0
foreground = 6
bold = false
dim = false
blink = false
reverse = false
hidden = false
underlined = false

[position.selected]
background = 6
foreground = 0
bold = false
dim = false
blink = false
reverse = false
hidden = false
underlined = false
```

## FAQ

Q: Why rust?

A: Needed a excuse to do something on it.
