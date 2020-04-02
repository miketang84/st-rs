// ============================================

/* appearance */
const FONT: &'static str = "./LiberationMono-Regular.ttf";
const FONTSIZE: isize = 12;
const BORDERPX: isize = 2;
const SHELL: &str = "/bin/sh";

/* TERM value */
const TERMNAME: &str = "st-256color";

const TABSPACES: usize = 8;
const WORD_BREAK: &str = " ";

/* Terminal colors (16 first used in escape sequence) */
const COLORMAP: Vec<sdl::Color> = [
    /* 8 normal colors */
    sdl::Color::RGBA { 0,   0,   0, 0 },    // black
    sdl::Color::RGBA { 128,   0,   0, 0 },  // red3
    sdl::Color::RGBA {    0, 128,   0, 0 }, // green3
    sdl::Color::RGBA { 128, 128,   0, 0  }, // yellow3
    sdl::Color::RGBA {   0,   0, 128, 0  }, // blue2
    sdl::Color::RGBA { 128,   0, 128, 0  }, // magenta3
    sdl::Color::RGBA {   0, 128, 128, 0  }, // cyan3
    sdl::Color::RGBA { 192, 192, 192, 0  }, // gray90

    /* 8 bright colors */
    sdl::Color::RGBA {  128, 128, 128, 0},  // gray50
    sdl::Color::RGBA { 255,   0,   0, 0  }, // red
    sdl::Color::RGBA {   0, 255,   0, 0  }, // green
    sdl::Color::RGBA {  255, 255,   0, 0 }, // yellow
    sdl::Color::RGBA {    0,   0, 255, 0 }, // #0000ff
    sdl::Color::RGBA { 255,   0, 255, 0  }, // magenta
    sdl::Color::RGBA {   0, 255, 255, 0  }, // cyan
    sdl::Color::RGBA { 255, 255, 255, 0  }, // white
];

const DEFAULTFG: usize = 7;
const DEFAULTBG: usize = 0;
const DEFAULTCS: usize = 256;
const DEFAULTUCS: usize = 257;



/*
 * Special keys (change & recompile st.info accordingly)
 * Keep in mind that kpress() in st.c hardcodes some keys.
 *
 * Mask value:
 * * Use XK_ANY_MOD to match the key no matter modifiers state
 * * Use XK_NO_MOD to match the key alone (no modifiers)
 */

/* key, mask, output */
// const key: Vec<Key> = [
//     { SDLK_LEFT,      KMOD_ALT,  "\033[1;3D" },
//     { SDLK_RIGHT,     KMOD_ALT,  "\033[1;3C" },

//     { SDLK_BACKSPACE, 0, "\177" },
//     { SDLK_INSERT,    0, "\033[2~" },
//     { SDLK_DELETE,    0, "\033[3~" },
//     { SDLK_HOME,      0, "\033[1~" },
//     { SDLK_END,       0, "\033[4~" },
//     { SDLK_PAGEUP,    0, "\033[5~" },
//     { SDLK_PAGEDOWN,  0, "\033[6~" },
//     { SDLK_F1,        0, "\033OP"   },
//     { SDLK_F2,        0, "\033OQ"   },
//     { SDLK_F3,        0, "\033OR"   },
//     { SDLK_F4,        0, "\033OS"   },
//     { SDLK_F5,        0, "\033[15~" },
//     { SDLK_F6,        0, "\033[17~" },
//     { SDLK_F7,        0, "\033[18~" },
//     { SDLK_F8,        0, "\033[19~" },
//     { SDLK_F9,        0, "\033[20~" },
//     { SDLK_F10,       0, "\033[21~" },
//     { SDLK_F11,       0, "\033[23~" },
//     { SDLK_F12,       0, "\033[24~" },
// ];



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
    MODE_WRAP	     = 1,
    MODE_INSERT      = 2,
    MODE_APPKEYPAD   = 4,
    MODE_ALTSCREEN   = 8,
    MODE_CRLF	     = 16,
    MODE_MOUSEBTN    = 32,
    MODE_MOUSEMOTION = 64,
    MODE_MOUSE       = 32|64,
    MODE_REVERSE     = 128,
    MODE_KBDLOCK     = 256
};

enum EscapeState {
    ESC_START      = 1,
    ESC_CSI	   = 2,
    ESC_STR	   = 4, /* DSC, OSC, PM, APC */
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

const UTF_SIZ: usize = 4;
const ESC_BUF_SIZ: usize = 256;
const ESC_ARG_SIZE: usize = 16;
const STR_BUF_SIZ: usize = 256;
const STR_ARG_SIZ: usize = 16;
const DRAW_BUF_SIZ: usize = 20*1024;



struct Glyph {
    c: [0u8;UTF_SIZ],
    mode: usize,
    fg: u16,
    bg: u16,
    state: u8,
}

struct TCursor {
    attr: Glyph,
    x: isize,
    y: iszie,
    state: i8,
}

struct CSIEscape {
    buf: [0u8;ESC_BUF_SIZ],
    len: isize,
    priv: i8,
    arg: [0u8;ESC_ARG_SIZE],
    narg: isize,
    mode: i8,
}

struct STREscape {
    type: i8,
    buf: [0u8;STR_BUF_SIZ],
    len: isize,
    args: [0u8;STR_ARG_SIZ],
    narg: isize,
}

struct Term {
    row: isize,
    col: isize,
    line: Vec<Vec<Glyph>>,  // screen
    alt: Vec<Vec<Glyph>>,  // alternate screen
    dirty: Vec<bool>,
    c: TCursor,
    top: isize,
    bot: isize,
    mode: isize,
    esc: isize,
    tabs: Vec<bool>,
}

struct Window {
    win: sdl::Surface,
    scr: isize,
    isfixed: bool,
    fx: isize,
    fy: isize,
    fw: isize,
    fh: isize,
    tw: isize,
    th: isize,
    w: isize,
    h: isize,
    ch: isize,
    cw: isize,
    state: i8,
}

struct Key {
    k: sdl::Key,
    mask: sdl::Mod,
    s: [0u8;ESC_BUF_SIZ]
}

// typedef union {
//	int i;
//	unsigned int ui;
//	float f;
//	const void *v;
// } Arg;

// typedef struct {
//	SDLMod mod;
//	SDLKey keysym;
//	void (*func)(const Arg *);
//	const Arg arg;
// } Shortcut;

struct DC {
    colors: Vec<sdl::Color>,
    font: sdl::ttf::Font,
    ifont: sdl::ttf::Font,
    bfont: sdl::ttf::Font,
    ibfont: sdl::ttf::Font,
}


fn limit(x: isize, a: isize, b: iszie) -> isize {
    if x < a {
	a
    }
    else if x > b {
	b
    }
    else {
	x
    }
}



tnew(int col, int row) {
	/* set screen size */
	term.row = row;
	term.col = col;
	term.line = xmalloc(term.row * sizeof(Line));
	term.alt  = xmalloc(term.row * sizeof(Line));
	term.dirty = xmalloc(term.row * sizeof(*term.dirty));
	term.tabs = xmalloc(term.col * sizeof(*term.tabs));

	for(row = 0; row < term.row; row++) {
		term.line[row] = xmalloc(term.col * sizeof(Glyph));
		term.alt [row] = xmalloc(term.col * sizeof(Glyph));
		term.dirty[row] = 0;
	}
	memset(term.tabs, 0, term.col * sizeof(*term.tabs));
	/* setup screen */
	treset();

void
treset(void) {
	uint i;

	term.c = (TCursor){{
		.mode = ATTR_NULL,
		.fg = defaultfg,
		.bg = defaultbg
	}, .x = 0, .y = 0, .state = CURSOR_DEFAULT};

	memset(term.tabs, 0, term.col * sizeof(*term.tabs));
	for(i = tabspaces; i < term.col; i += tabspaces)
		term.tabs[i] = 1;
	term.top = 0;
	term.bot = term.row - 1;
	term.mode = MODE_WRAP;

	tclearregion(0, 0, term.col-1, term.row-1);
}



impl Term {

    fn reset(&self) {
	let glyph = Glyph {
	    mode: ATTR_NULL,
	    fg: DEFAULTFG,
	    bg: DEFAULTBG,
	    c: Default::default(),
	    state: Default::default(),
	};
	let cursor = TCursor {
	    glyph,
	    x: 0,
	    y: 0,
	    state: CURSOR_DEFAULT
	};

	for i in 1..(col/TABSPACES) {
	    self.tabs[i*TABSPACES] = 1;
	}

	self.top = 0;
	self.bot = self.row - 1;
	self.mode = MODE_WRAP;

	self.clear_region(0, 0, self.row-1, self.col - 1);

    }

    fn clear_region(&mut self, mut x1: isize, mut y1: isize, mut x2: isize, mut y2: isize) {
	if x1 > x2 {
	    (x1, x2) = (x2, x1);
	}
	if y1 > y2 {
	    (y1, y2) = (y2, y1);
	}

	x1 = limit(x1, 0, self.col - 1);
	x2 = limit(x2, 0, self.col - 1);
	y1 = limit(y1, 0, self.row - 1);
	y2 = limit(y2, 0, self.row - 1);

	for y in y1..=y2 {
	    self.dirty[y] = 1;
	    for x in x1..=x2 {
		// TODO: how to express two dimension cell index
		self.line[y][x].state = 0;
	    }
	}
    }



    fn new() {
	let row = 24;
	let col = 80;

	let lines: Vec<Vec<Glyph>> = Vec::with_capacity(row);
	let alts: Vec<Vec<Glyph>> = Vec::with_capacity(row);
	let dirtys: Vec<bool> = Vec::with_capacity(row);
	let tabss: Vec<bool> = Vec::with_capacity(col);

	Term {
	    row,
	    col,
	    line: lines,
	    alt: alts,
	    dirty: dirtys,
	    tabs: tabss,
	    ..Default::default(),
	}
    }


}










fn main() {
    println!("Hello, world!");

    let g_term = Term::new();
    g_term.reset();

    tty_new();
    sdl_new();

    run();


}
