enum GlyphAttribute {
	ATTR_NULL      = 0,
	ATTR_REVERSE   = 1,
	ATTR_UNDERLINE = 2,
	ATTR_BOLD      = 4,
	ATTR_GFX       = 8,
	ATTR_ITALIC    = 16,
	ATTR_BLINK     = 32,
};

enum CursorMovement {
	CURSOR_UP,
	CURSOR_DOWN,
	CURSOR_LEFT,
	CURSOR_RIGHT,
	CURSOR_SAVE,
	CURSOR_LOAD
};

enum CursorState {
	CURSOR_DEFAULT  = 0,
	CURSOR_HIDE     = 1,
	CURSOR_WRAPNEXT = 2
};

enum GlyphState {
	GLYPH_SET   = 1,
	GLYPH_DIRTY = 2
};

enum TermMode {
	MODE_WRAP	= 1,
	MODE_INSERT      = 2,
	MODE_APPKEYPAD   = 4,
	MODE_ALTSCREEN   = 8,
	MODE_CRLF	= 16,
	MODE_MOUSEBTN    = 32,
	MODE_MOUSEMOTION = 64,
	MODE_MOUSE       = 32|64,
	MODE_REVERSE     = 128,
	MODE_KBDLOCK     = 256
};

enum EscapeState {
	ESC_START      = 1,
	ESC_CSI	= 2,
	ESC_STR	= 4, /* DSC, OSC, PM, APC */
	ESC_ALTCHARSET = 8,
	ESC_STR_END    = 16, /* a final string was encountered */
	ESC_TEST       = 32, /* Enter in test mode */
};

enum WindowState {
	WIN_VISIBLE = 1,
	WIN_REDRAW  = 2,
	WIN_FOCUSED = 4
};

enum { B0=1, B1=2, B2=4, B3=8, B4=16, B5=32, B6=64, B7=128 };




fn main() {
    println!("Hello, world!");

    let g_term = Term::new();


    tty_new();
    sdl_new();

    run();


}
