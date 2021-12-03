const rustFd = require('./index.node');

const Node = require('./src/js/Node');
const Network = require('./src/js/Network');
const Tunnel = require('./src/js/Tunnel');

module.exports = {
  ...rustFd,
  init: Node.init,
  checkNodesIsUp: Node.checkNodesIsUp,
  syncJoinRequests: Node.syncJoinRequests,
  attachRoutes: Network.attachRoutes,
  startTunnel: Tunnel.startTunnel,
}