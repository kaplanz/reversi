//! # Monte Carlo Tree Search
//!
//! `mcts` is a library for running the MCTS algorithm for turn based games.

use std::f64::INFINITY;
use std::rc::{Rc, Weak};

/// Trait to define methods needed by MCTS.
pub trait Game: Clone {
    type Player;
    type Turn: Clone;

    /// Play a turn of the game.
    fn play(&mut self, turn: &Self::Turn);

    /// Get all actions for the current state.
    fn get_actions(&self) -> Vec<Self::Turn>;

    /// Get the current player.
    fn get_player(&self) -> Self::Player;

    /// Check if the game is over.
    fn is_over(&self) -> bool;

    /// Get the winnner of the current state.
    fn get_winner(&self) -> Option<Self::Player>;

    /// Propose an action through MCTS.
    fn mcts(&self) {}
}

/// The game tree from the current state.
struct Tree<G: Game + ?Sized> {
    root: Node<G>,
}

impl<G:Game + ?Sized> Tree<G> {
    /// Create a new `Tree` initialized with a root.
    fn new(state: Box<G>) -> Tree<G> {
        Tree {
            root: Node::new(state, None, Weak::new()),
        }
    }

    /// Explore the game tree.
    fn select(&self) -> &Node<G> {
        let node = &self.root; // start at the root

        // Loop until `node` has no children
        while !node.children.is_empty() {
            // Get the child with the highest initiative
            let mut next = &node.children[0];
            for child in node.children.iter() {
                if child.initiative > next.initiative {
                    next = &child;
                }
            }
        }

        node // return leaf node
    }

}

/// A single state in the game tree.
struct Node<G: Game + ?Sized> {
    // Position
    parent: Weak<Self>,
    children: Vec<Rc<Self>>,
    // State
    state: Box<G>,
    action: Option<G::Turn>,
    // Statistics
    wins: i32,
    sims: i32,
    initiative: f64,
}

impl<G: Game + ?Sized> Node<G> {
    /// Create a new `Node`.
    fn new(state: Box<G>, action: Option<G::Turn>, parent: Weak<Node<G>>) -> Node<G> {
        Node {
            parent,
            children: Vec::new(),
            state,
            action,
            wins: 0,
            sims: 0,
            initiative: INFINITY,
        }
    }

    /// Expand self to create children in game tree.
    fn expand(&mut self) {
        // Iterate through actions to create children
        for action in self.state.get_actions() {
            // Clone state and play action
            let mut state: G = *self.state.clone();
            state.play(&action);

            // Create a reference to parent
            let parent: Weak<Self> = self; // need to create a weak reference from `self`

            self.children
                .push(Rc::from(Node::new(Box::new(state), Some(action), parent)));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
