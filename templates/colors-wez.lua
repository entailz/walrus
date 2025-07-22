local fg = "{color11}"
local bg = "{color0}"
local bright_fg = "{color6}"
local bright_bg = "{color1}"
local white = "{color8}"

return {
  background = bg,
  foreground = fg,
  cursor_bg = fg,
  cursor_fg = black,
  cursor_border = fg,
  selection_fg = black,
  selection_bg = fg,
  scrollbar_thumb = fg,
  split = white,
  ansi = {
    bright_bg,
    "{color1}",
    "{color2}",
    "{color3}",
    "{color4}",
    "{color5}",
    "{color6}",
    bright_fg,
  },
  brights = {
    white,
    "{color7}",
    "{color8}",
    "{color9}",
    "{color10}",
    "{color11}",
    "{color12}",
    fg,
  },
}
