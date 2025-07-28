# Walrus

Walrus is a highly oppinionated and minimal rewrite of Pywal in Rust. Walrus
uses a Haishoku style backend. I might add more backends in the future, for
now this works fine for me. Feed Walrus an image and automatically generate base16
color sequence for your terminal and program(s) of choice.

__Defaults & Flags:__
- Default template directory is ~/.config/walrus/templates, you may point to
a different template directory if need be using the --template, or -t flag.
See the templates directory if you would like an example of how to format
said templates.
- Default output directory is ~/.cache/walrus, use the --output, or -o flag
to change this behavior.
- Saturation  factor is modified with the -s or --saturation flag, (1.0 =
normal, 0.2 = 20% saturation)
- Walrus generates a stripped JSON file for use with my Thorn layershell.
- Like Pywal, terminal sequences are pushed to open terminals, like such:
"/dev/pts/[0-9]*".
- Generates 16 colors, labeled as {color0-15}
