#[macro_export]
macro_rules! fmt_indent {
    ($n:ident, $f:literal, $($arg:tt)*) => {
        format!(
            concat!("{}{}", $f, (0..$n).map(|_| "\t").collect::<String>()),
            $($arg)*)
    }
}

#[macro_export]
macro_rules! to_string {
    ($slf:ident, $($arg:tt)*) => {
        pub fn to_string($slf, n: u8) -> String {
            let indent = (0..n).map(|_| "\t").collect::<String>();
            format!("{}{}\n", indent, $($arg)*);
        }
    }
}

#[macro_export]
macro_rules! keyword {
    ($name:literal, $tok:ident, $pos:ident, $($kw:tt)*) => {
        match $tok.pop_front()?.keyword() {
            Some($($kw)*) => (),
            other => {
                warn!(
                    "{} expected keyword '{}', got '{:?}' {}",
                    $name, Keyword::to_string(&$($kw)*), other, $pos.near()
                );
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! keyword_back {
    ($name:literal, $tok:ident, $pos:ident, $($kw:tt)*) => {
        match $tok.pop_back()?.keyword() {
            Some($($kw)*) => (),
            other => {
                warn!(
                    "{} expected keyword '{}', got '{:?}' {}",
                    $name, Keyword::to_string(&$($kw)*).unwrap(), other, $pos.near()
                );
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! sym {
    ($name:literal, $tok:ident, $pos:ident, $($sym:tt)*) => {
        match $tok.pop_front()?.sym() {
            Some($($sym)*) => (),
            other => {
                warn!(
                    "{} expected '{}', got '{:?}' {}",
                    $name, Sym::to_char(&$($sym)*).unwrap(), other, $pos.near()
                );
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! sym_back {
    ($name:literal, $tok:ident, $pos:ident, $($sym:tt)*) => {
        match $tok.pop_back()?.sym() {
            Some($($sym)*) => (),
            other => {
                warn!(
                    "{} expected '{}', got '{:?}' {}",
                    $name, Sym::to_char(&$($sym)*).unwrap(), other, $pos.near()
                );
                return None;
            }
        }
    };
}
