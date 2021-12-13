# Network

The network layer run as simple HTTP routes for communication between nodes.

The first implementation was the expressJS routes and now I'm rewriting this rules with Rust.

For works in P2P, is necessary tunnel to resolve DNS and network bypass.

On NodeJs, when setup_server() is executed, this application create a web server with routes to node manager and make reverse proxy to another routes to main application.

## TODO - RIIR

1. Write Route Implementation - "/stats"
2. TDD - "/stats"
3. Write Route Implementation - "/nodes"
4. TDD - "/stats"
5. Write Route Implementation - "/join-request"
6. TDD - "/join-request"
7. Write Route Implementation - "/update-node-info"
8. TDD - "/update-node-info"
