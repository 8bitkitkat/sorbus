use {
    crate::{TextRange, TokenKindTrait, TokenTrait},
    std::marker::PhantomData,
};

pub struct Source<'t, 's, K: TokenKindTrait, T: TokenTrait<K>> {
    tokens: &'t [T],
    src: &'s str,
    cursor: usize,
    _p: PhantomData<K>,
}

impl<'t, 's, K: TokenKindTrait, T: TokenTrait<K>> Source<'t, 's, K, T> {
    pub fn new(tokens: &'t [T], src: &'s str) -> Self {
        Self {
            tokens,
            src,
            cursor: 0,
            _p: PhantomData,
        }
    }

    pub fn next_token(&mut self) -> Option<&T> {
        self.eat_trivia();

        let tok = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(tok)
    }

    pub fn peek_kind(&mut self) -> Option<K> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub fn peek_token(&mut self) -> Option<&T> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub fn last_token_range(&mut self) -> Option<&TextRange> {
        self.tokens.last().map(|t| t.text_range())
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, K::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<K> {
        self.tokens.get(self.cursor).map(|t| t.kind())
    }

    fn peek_token_raw(&mut self) -> Option<&T> {
        self.tokens.get(self.cursor)
    }

    pub fn into_parts(self) -> (&'t [T], &'s str) {
        (self.tokens, self.src)
    }
}
