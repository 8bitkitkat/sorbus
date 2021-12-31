pub extern crate rowan;

use {
    rowan::{GreenNode, SyntaxNode},
    std::fmt::Debug,
};

pub use {
    marker::{CompletedMarker, Marker},
    rowan::TextRange,
    sink::Sink,
    source::Source,
};

pub mod marker;
pub mod sink;
pub mod source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<K: TokenKindTrait, E: PartialEq> {
    StartNode {
        kind: K,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(E),
    Placeholder,
}

/// ```
/// use sorbus::TextRange;
///
/// struct Token {
///     kind: TokenKind,
///     span: TextRange,
/// }
///
/// enum TokenKind {
///     // ...
/// }
/// ```
pub trait TokenTrait<K: TokenKindTrait>: Sized + Clone {
    fn new(kind: K, text_range: TextRange) -> Self;

    fn kind(&self) -> K;

    fn text_range(&self) -> &TextRange;
}

pub trait TokenKindTrait: Sized + Clone + Copy + PartialEq + std::fmt::Debug {
    #[allow(clippy::wrong_self_convention)]
    fn is_trivial(self) -> bool;
}

pub trait ParserTrait<K: TokenKindTrait, E: Debug + PartialEq>: Sized {
    fn events(&mut self) -> &mut Vec<Event<K, E>>;

    fn node_start(&mut self) -> Marker {
        let pos = self.events().len();
        self.events().push(Event::Placeholder);

        Marker::new(pos)
    }

    fn node_complete(&mut self, m: Marker, kind: K) -> CompletedMarker {
        m.complete(self, kind)
    }
}

#[derive(Debug, Clone)]
pub struct ParseResult<E> {
    pub green_node: GreenNode,
    pub errors: Option<Vec<E>>,
}

impl<E> ParseResult<E> {
    pub fn debug_tree<L: rowan::Language>(&self) -> String {
        let syntax_node: rowan::SyntaxNode<L> =
            rowan::SyntaxNode::new_root(self.green_node.clone());
        format!("{:#?}", syntax_node)
    }

    pub fn syntax<L: rowan::Language>(&self) -> SyntaxNode<L> {
        SyntaxNode::new_root(self.green_node.clone())
    }
}
