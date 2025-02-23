# easy-neovim-wm-nav

Quickly navigate window manager windows and Neovim splits with the same keybindings. 
Supports [AeroSpace](https://nikitabobko.github.io/AeroSpace/guide), [Sway](https://swaywm.org/), and [Hyprland](https://hyprland.org/).

This utility was heavily inspired by 

* [easy-i3-neovim-nav](https://github.com/tom-anders/easy-i3-neovim-nav)

* [i3-vim-focus](https://github.com/jwilm/i3-vim-focus)

## Installation

```
cargo install --git https://github.com/erauer/easy-neovim-wm-nav.git
```

## Usage

### Neovim configuration

Add this to your `init.lua`:

```
local servername = vim.api.nvim_get_vvar("servername")
vim.opt.title = true
vim.opt.titlestring = string.format("nvim %s -- [%s] ", vim.fn.getcwd(),
```

> **_Note:_** `easy-neovim-wm-nav` uses the window's titlestring in order to extract the server name
used for communicating with Neovim. The default regex assumes that the servername is contained in
square brackets at the very end of your `titlestring`. 

### Hyprland key bindings
```
# Arrow Keys
bind = $mainMod, left, exec, easy-neovim-wm-nav left --backend hyprland
bind = $mainMod, right, exec, easy-neovim-wm-nav right --backend hyprland 
bind = $mainMod, up, exec, easy-neovim-wm-nav up --backend hyprland 
bind = $mainMod, down, exec, easy-neovim-wm-nav down --backend hyprland 

# HJKL
bind = $mainMod, H, exec, easy-neovim-wm-nav left --backend hyprland 
bind = $mainMod, L, exec, easy-neovim-wm-nav right --backend hyprland
bind = $mainMod, K, exec, easy-neovim-wm-nav up --backend hyprland
bind = $mainMod, J, exec, easy-neovim-wm-nav down --backend hyprland 

```

### Sway key bindings
```
bindsym Mod4+h exec easy-neovim-wm-nav -b sway left
bindsym Mod4+j exec easy-neovim-wm-nav -b sway down
bindsym Mod4+k exec easy-neovim-wm-nav -b sway up
bindsym Mod4+l exec easy-neovim-wm-nav -b sway right

```

### Skhd key binding for use with Aerospace 
```
shift + alt - h : easy-neovim-wm-nav --backend aerospace left
shift + alt - j : easy-neovim-wm-nav --backend aerospace down
shift + alt - k : easy-neovim-wm-nav --backend aerospace up
shift + alt - l : easy-neovim-wm-nav --backend aerospace right
```

## Status

Let's be honest, at this point it `seems to work on my machine.`  I'm new to rust and by no means is the code production ready or of good quality. That being said, I'm open to issues and PRs and will continue to maintain this code as long as it is of use to me.


