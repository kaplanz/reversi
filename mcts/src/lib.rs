//! # Monte Carlo Tree Search
//!
//! `mcts` is a library for running the MCTS algorithm for turn based games.

use rand::seq::SliceRandom;
use std::f64::INFINITY;
use std::time::Instant;

const DURATION: u128 = 995;
const THRESHOLD: u32 = 3;
const EXPLORE: f64 = 0.5;

pub trait Mcts: Clone {
    type Player: PartialEq;
    type Turn: Clone;

    /// Get the current player.
    fn player(&self) -> Self::Player;

    /// Get all legal turns.
    fn turns(&self) -> Vec<Self::Turn>;

    /// Play a turn of the game.
    fn play(&mut self, turn: Self::Turn);

    /// Check if the game is over.
    fn over(&self) -> bool;

    /// Get the winner of the game.
    fn winner(&self) -> Option<Self::Player>;

    /// Run MCTS to select a turn.
    fn mcts(&self) -> Self::Turn {
        // Record time MCTS was started
        let now = Instant::now();

        // Create the game tree
        let mut tree = Tree::new(Box::new(self.clone()));
        tree.expand(tree.root); // expand at root

        // Return immediately if only one valid turn
        if tree.borrow_node(tree.root).children.len() == 1 {
            let root = tree.borrow_node(tree.root);
            return tree.borrow_node(root.children[0]).action.clone().unwrap();
        }

        let mut round = 0;
        while now.elapsed().as_millis() < DURATION {
            // Select a leaf node to expand
            let mut leaf = tree.select();

            // Expand `leaf` if it's been simulated more than `THRESHOLD`
            if tree.borrow_node(leaf).sims > THRESHOLD {
                tree.expand(leaf);
                leaf = *tree
                    .borrow_node_mut(leaf)
                    .children
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&leaf);
            }

            // Simulate at `leaf`
            let winner = tree.borrow_node(leaf).simulate();

            // Backpropagate the winner
            tree.backpropagate(leaf, winner, round);

            // Increment the round number
            round += 1;
        }

        // Find most simulated node
        let root = tree.borrow_node(tree.root);
        let mut best = root.children[0];
        for child in root.children.iter() {
            if tree.borrow_node(*child).sims > tree.borrow_node(best).sims {
                best = *child;
            }
        }

        // Play most simulated node
        if let Some(turn) = tree.borrow_node(best).action.clone() {
            turn
        } else {
            panic!("Error: could not find most simulated node.")
        }
    }
}

/// The game tree from the current position.
struct Tree<G: Mcts> {
    arena: Vec<Node<G>>,
    root: usize,
}

/// A single state in the game tree.
struct Node<G: Mcts> {
    // Position
    idx: usize,
    parent: usize,
    children: Vec<usize>,
    // State
    state: Box<G>,
    action: Option<G::Turn>,
    // Statistics
    wins: u32,
    sims: u32,
    initiative: f64,
}

impl<G: Mcts> Node<G> {
    /// Create a new Node.
    fn new(idx: usize, parent: usize, state: Box<G>, action: Option<G::Turn>) -> Node<G> {
        Node {
            idx,
            parent,
            children: Vec::new(),
            state,
            action,
            wins: 0,
            sims: 0,
            initiative: INFINITY,
        }
    }

    /// Simulate the game form this node.
    fn simulate(&self) -> Option<G::Player> {
        // Create a copy of the current state to simulate
        let mut state = self.state.clone();

        while !state.over() {
            // Policy: select a random move
            let action = state
                .turns()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            state.play(action);
        }

        state.winner()
    }

    /// Update this node's initiative
    fn update_initiative(&mut self, round: usize) {
        let expliotation = (self.wins as f64) / (self.sims as f64);
        let exploration = EXPLORE * ((round as f64).log10() / self.sims as f64).sqrt();
        self.initiative = expliotation + exploration;
    }
}

impl<G: Mcts> Tree<G> {
    /// Create a new Tree initialized with a root.
    fn new(state: Box<G>) -> Tree<G> {
        Tree {
            arena: vec![Node::new(0, 0, state, None)],
            root: 0,
        }
    }

    /// Borrow a `Node` from the tree.
    fn borrow_node(&self, idx: usize) -> &Node<G> {
        &self.arena[idx]
    }

    /// Borrow a `Node` from the tree mutably.
    fn borrow_node_mut(&mut self, idx: usize) -> &mut Node<G> {
        &mut self.arena[idx]
    }

    /// Explore the game tree.
    fn select(&self) -> usize {
        let mut node = &self.arena[self.root]; // start at the root

        // Loop until `node` has no children
        while !node.children.is_empty() {
            // Get the child with the highest initiative
            node = &self.arena[node.children[0]];
            for child in node.children.iter() {
                let child = &self.arena[*child];
                if child.initiative > node.initiative {
                    node = &child;
                }
            }
        }

        node.idx
    }

    /// Expand a node to create children in the game tree.
    fn expand(&mut self, idx: usize) {
        // Iterate through actions to create children
        for action in self.arena[idx].state.turns() {
            // Clone state and play action
            let mut state: G = *self.arena[idx].state.clone();
            state.play(action.clone());

            // Add the new child
            self.arena.push(Node::new(
                self.arena.len(),
                idx,
                Box::new(state),
                Some(action),
            ));
            // Parent stores index of child
            let child = self.arena.last().unwrap().idx;
            self.arena[idx].children.push(child);
        }
    }

    /// Backpropagate the result of a simulation.
    fn backpropagate(&mut self, mut idx: usize, winner: Option<G::Player>, round: usize) {
        // Backpropagate until the root
        while idx != 0 {
            let node = &mut self.arena[idx];

            // Update statistics of node
            if Some(node.state.player()) == winner {
                node.wins += 1;
            }
            node.sims += 1;

            // Update this node's initiative
            node.update_initiative(round);

            // Ascend to parent
            idx = node.parent;
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
