local C = {}

local function read_file(path)
	local file = io.open(path, "rb") -- r read mode and b binary mode
	if not file then return nil end
	local content = file:read "*a"   -- *a or *all reads the whole file
	file:close()
	return content
end
local safe_file = os.getenv("PALETTE_SAVE_FILE")
if safe_file == nil then safe_file = "palette.txt" end
local file = read_file(safe_file)

function Split(s, delimiter)
	local result = {};
	for match in (s .. delimiter):gmatch("(.-)" .. delimiter) do
		table.insert(result, match);
	end
	return result;
end

local splited = Split(file, "\n")
local color1 = splited[1]
local color2 = splited[2]
local color3 = splited[3]
local color4 = splited[4]
local color5 = splited[5]
local color6 = splited[6]
local color7 = splited[7]
local color8 = splited[8]


C = {
	color1 = color1,
	color2 = color2,
	color3 = color3,
	color4 = color4,
	color5 = color5,
	color6 = color6,
	color7 = color7,
	color8 = color8,
	colors = {
		-- The default text color
		foreground = color7,
		-- The default background color
		background = color1,

		-- Overrides the cell background color when the current cell is occupied by the
		-- cursor and the cursor style is set to Block
		cursor_bg = color3,
		-- Overrides the text color when the current cell is occupied by the cursor
		cursor_fg = color4,
		-- Specifies the border color of the cursor when the cursor style is set to Block,
		-- or the color of the vertical or horizontal bar when the cursor style is set to
		-- Bar or Underline.
		cursor_border = color8,

		-- the foreground color of selected text
		selection_fg = color6,
		-- the background color of selected text
		selection_bg = color2,

		-- The color of the scrollbar "thumb"; the portion that represents the current viewport
		scrollbar_thumb = color4,

		-- The color of the split lines between panes
		split = color5,

		-- Arbitrary colors of the palette in the range from 16 to 255
		indexed = { [136] = '#af8700' },

		-- Since: 20220319-142410-0fcdea07
		-- When the IME, a dead key or a leader key are being processed and are effectively
		-- holding input pending the result of input composition, change the cursor
		-- to this color to give a visual cue about the compose state.
		compose_cursor = 'orange',

		-- Colors for copy_mode and quick_select
		-- available since: 20220807-113146-c2fee766
		-- In copy_mode, the color of the active text is:
		-- 1. copy_mode_active_highlight_* if additional text was selected using the mouse
		-- 2. selection_* otherwise
		copy_mode_active_highlight_bg = { Color = '#000000' },
		-- use `AnsiColor` to specify one of the ansi color palette values
		-- (index 0-15) using one of the names "Black", "Maroon", "Green",
		--  "Olive", "Navy", "Purple", "Teal", "Silver", "Grey", "Red", "Lime",
		-- "Yellow", "Blue", "Fuchsia", "Aqua" or "White".
		copy_mode_active_highlight_fg = { AnsiColor = 'Black' },
		copy_mode_inactive_highlight_bg = { Color = '#52ad70' },
		copy_mode_inactive_highlight_fg = { AnsiColor = 'White' },

		quick_select_label_bg = { Color = 'peru' },
		quick_select_label_fg = { Color = '#ffffff' },
		quick_select_match_bg = { AnsiColor = 'Navy' },
		quick_select_match_fg = { Color = '#ffffff' },
	}
}
return C
