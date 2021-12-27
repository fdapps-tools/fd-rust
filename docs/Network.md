# Network

The network layer run as simple HTTP routes for communication between nodes.

The first implementation was the expressJS routes and now I'm rewriting this rules with Rust.

For works in P2P, is necessary tunnel to resolve DNS and network bypass.

On NodeJs, when setup_server() is executed, this application create a web server with routes to node manager and make reverse proxy to another routes to main application.

## Enviroment details

The `PORT` process variable is usage to make reverse proxy from all request to nodeJs application

The `PORT_FD` process variable is usage to running fd server.

The `TUNNEL_URL` process variable is usage to return on stats route to check node health.
## TODO - RIIR

Some routes need more implementation to fully works, like DB implementation to node manager and tunneling module.

The `Done Concept` in here is simplify way to next implementations, running basic routes and setup webserver.

1. Write Route Implementation - "/stats" - DONE
2. TDD - "/stats"
3. Write Route Implementation - "/nodes" - DEPENDS DB Implementation
4. TDD - "/stats"
5. Write Route Implementation - "/join-request" - DONE/ DEPENDS DB Implementation
6. TDD - "/join-request"
7. Write Route Implementation - "/update-node-info"  - DEPENDS DB Implementation
8. TDD - "/update-node-info"
