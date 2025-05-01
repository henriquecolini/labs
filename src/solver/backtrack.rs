/// Used to solve a backtracking problem
pub trait State<'a>: Sized {
    type Context: 'a;

    /// Get all the successive states from the current state
    fn successors(&self, ctx: &Self::Context) -> impl Iterator<Item = Self>;

    /// Check if the state is the final/goal state. It must also be valid.
    fn is_goal(&self, ctx: &Self::Context) -> bool;
}

/// Solve the backtracking problem using the specified state (recursive)
pub fn solve<'a, C: State<'a>>(ctx: &C::Context, state: C) -> Option<C> {
    // Return once we found the goal
    if state.is_goal(ctx) {
        return Some(state);
    }

    // Search the successors for the goal
    for child in state.successors(ctx) {
        // Check if the state leads to a solution
        match solve(ctx, child) {
            Some(solution) => return Some(solution),
            None => continue,
        }
    }

    // No solution found, prune this tree
    None
}
