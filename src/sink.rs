use {
    crate::{rowan::GreenNodeBuilder, Event, ParseRes, TokenKindTrait, TokenTrait},
    std::error::Error,
};

pub struct Sink<'t, 's, K: TokenKindTrait, T: TokenTrait<K>, E: Error + Clone + PartialEq> {
    tokens: &'t [T],
    src: &'s str,
    cursor: usize,
    builder: GreenNodeBuilder<'static>,
    events: Vec<Event<K, E>>,
    errors: Vec<E>,
}

impl<'t, 's, K: TokenKindTrait, T: TokenTrait<K>, E: Error + Clone + PartialEq>
    Sink<'t, 's, K, T, E>
{
    pub fn new(tokens: &'t [T], src: &'s str, events: Vec<Event<K, E>>) -> Self {
        Self {
            tokens,
            src,
            cursor: 0,
            builder: GreenNodeBuilder::new(),
            events,
            errors: Vec::new(),
        }
    }

    pub fn finish<Lang: rowan::Language<Kind = K>>(mut self) -> ParseRes<E> {
        for idx in 0..self.events.len() {
            match std::mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;
                    let mut forward_parent = forward_parent;

                    // walk though the forward events until
                    // we reach one without a forward event
                    while let Some(fp) = forward_parent {
                        idx += fp;

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            std::mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        // self.builder.start_node(kind.into())
                        self.builder.start_node(Lang::kind_to_raw(kind))
                    }
                }
                Event::AddToken => self.token::<Lang>(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(e) => self.errors.push(e),
                Event::Placeholder => {}
            }

            self.eat_trivia::<Lang>();
        }

        ParseRes {
            green_node: self.builder.finish(),
            errors: if self.errors.is_empty() {
                None
            } else {
                Some(self.errors)
            },
        }
    }

    fn token<Lang: rowan::Language<Kind = K>>(&mut self) {
        let token = self.tokens[self.cursor].clone();
        let kind = token.kind();
        let text_range = *token.text_range();

        // self.builder.token(kind.into(), self.src[text_range].into());
        self.builder
            .token(Lang::kind_to_raw(kind), self.src[text_range].into());
        self.cursor += 1;
    }

    fn eat_trivia<Lang: rowan::Language<Kind = K>>(&mut self) {
        while let Some(lexeme) = self.tokens.get(self.cursor) {
            if !lexeme.kind().is_trivia() {
                break;
            }

            self.token::<Lang>();
        }
    }
}
