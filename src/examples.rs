/// Provides Neo4j's Movies example graph.
#[cfg(feature = "examples-movies")]
pub mod movie_graph;

/// Provides a "Scotland Yard" game type map of London.
#[cfg(feature = "examples-london")]
pub mod london_graph;

#[cfg(test)]
mod tests {
    use super::london_graph::*;
    use super::movie_graph::*;

    #[test]
    #[cfg(feature = "examples-movies")]
    fn movie_examples() {
        let graph = movie_graph();
        println!("{:?}", graph);
    }

    #[test]
    #[cfg(feature = "examples-london")]
    fn london_examples() {
        let graph = london_graph();
        println!("{:?}", graph);
    }
}
