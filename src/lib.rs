extern crate proc_macro;

use std::vec::IntoIter;

use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};

/// Strips any outer `Delimiter::None` groups from the input,
/// returning a `TokenStream` consisting of the innermost
/// non-empty-group `TokenTree`.
/// This is used to handle a proc macro being invoked
/// by a `macro_rules!` expansion.
/// See https://github.com/rust-lang/rust/issues/72545 for background
fn ignore_groups(mut input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();
    loop {
        if let Some(TokenTree::Group(group)) = tokens.next() {
            if group.delimiter() == Delimiter::None {
                input = group.stream();
                continue;
            }
        }
        return input;
    }
}

struct TokenTreeIter {
    buf: IntoIter<char>,
    is_punct: bool,
}

impl TokenTreeIter {
    /// Constructs a new `TokenTreeIter` from a given `proc_macro::Literal`.
    ///
    /// # Panics
    /// This panics if the given `Literal` is not a string literal, or if it is not of the correct
    /// length.
    ///
    fn new(input: Literal) -> Self {
        let mut buf: Vec<char> = input.to_string().chars().collect();

        match buf.as_slice() {
            ['"', .., '"'] => (),
            _ => panic!("expected string literal, got `{}`", input),
        };
        buf.pop();
        // Remove the leading '"'
        buf.remove(0);

        // Check to see if this begins with a # character
        if let Some(&c) = buf.first() {
            // Skip it for parsing
            if c == '#' {
                buf.remove(0);
            }
        }

        Self {
            buf: buf.into_iter(),
            is_punct: false,
        }
    }

    /// Parses a single hex character (a-f/A-F/0-9) as a `u8` from the `TokenTreeIter`'s
    /// internal buffer, ignoring whitespace.
    ///
    /// # Panics
    /// This panics if a non-hex, non-whitespace character is encountered.
    fn next_hex_val(&mut self) -> Option<u8> {
        loop {
            let v = self.buf.next()?;
            let n = match v {
                '0'..='9' => v as u8 - 48,
                'A'..='F' => v as u8 - 55,
                'a'..='f' => v as u8 - 87,
                ' ' | '\r' | '\n' | '\t' => continue,
                c if c.is_ascii() => panic!("encountered invalid character: `{}`", v as char),
                _ => panic!("encountered invalid non-ASCII character"),
            };
            return Some(n);
        }
    }
}

impl Iterator for TokenTreeIter {
    type Item = TokenTree;

    /// Produces hex values (as `u8` literals) parsed from the `TokenTreeIter`'s
    /// internal buffer, alternating with commas to separate the elements of the
    /// generated array of bytes.
    ///
    /// The byte array can either be 3 elements long for RGB, or 4 for RGBA
    ///
    /// # Panics
    /// This panics if the internal buffer contains an even number of hex characters
    fn next(&mut self) -> Option<TokenTree> {
        let v = if self.is_punct {
            TokenTree::Punct(Punct::new(',', Spacing::Alone))
        } else {
            let p1 = self.next_hex_val()?;
            let p2 = match self.next_hex_val() {
                Some(v) => v,
                None => panic!("expected even number of hex characters"),
            };
            let val = (p1 << 4) + p2;
            TokenTree::Literal(Literal::u8_suffixed(val))
        };
        self.is_punct = !self.is_punct;
        Some(v)
    }
}

/// Macro for converting a string literal containing hex-encoded color data
/// into an array of bytes.
#[proc_macro]
pub fn color_from_hex(input: TokenStream) -> TokenStream {
    let mut out_ts = TokenStream::new();

    let mut in_ts = ignore_groups(input).into_iter();

    // Consume only one string literal
    let tt = in_ts.next().expect("macro requires a string literal");
    match tt {
        TokenTree::Literal(literal) => {
            let mut tokens = Vec::new();

            let iter = TokenTreeIter::new(literal);

            for token in iter {
                tokens.push(token);

                if tokens.len() > 8 {
                    panic!("expected a maximum of 8 characters for RGBA, ex: #4c4c4cff");
                }
            }

            if tokens.len() < 6 {
                panic!(
                    "expected a minimum of 6 characters for RGB, ex: #4c4c4c. Tokens: {:#?}",
                    tokens
                );
            }

            out_ts.extend(tokens.into_iter());
        }
        unexpected => panic!("expected string literal, got `{}`", unexpected),
    };

    // Create the final array by grouping the tokens with brackets
    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Bracket, out_ts)))
}
