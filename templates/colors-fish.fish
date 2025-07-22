set -l foreground {color11.strip}
set -l selection {color11.strip}
set -l comment {color3.strip}
set -l red {color3.strip}
set -l orange {color8.strip}
set -l yellow {foreground.strip}
set -l green {color7.strip}
set -l purple {foreground.strip}
set -l cyan {color1.strip}
set -l pink {color6.strip}

set -g fish_color_normal $foreground
set -g fish_color_command $red
set -g fish_color_keyword $pink
set -g fish_color_quote $yellow
set -g fish_color_redirection $foreground
set -g fish_color_end $orange
set -g fish_color_error $red
set -g fish_color_param $purple
set -g fish_color_comment $comment
set -g fish_color_selection --background=$selection
set -g fish_color_search_match --background=$selection
set -g fish_color_operator $green
set -g fish_color_escape $pink
set -g fish_color_autosuggestion $comment


set -g fish_pager_color_progress $comment
set -g fish_pager_color_prefix $cyan
set -g fish_pager_color_completion $foreground
set -g fish_pager_color_description $comment
set -g fish_pager_color_selected_background --background=$selection
