
## Outline graph:
![t](graph.svg)
To create an up-to-date version, install Graphviz and run
```rust
cargo install cargo-depgraph
cargo depgraph --all-deps | dot -Tsvg > .docs/graph.svg
```