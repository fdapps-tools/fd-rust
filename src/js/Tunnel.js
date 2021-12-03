let tryNumber = 0

module.exports = {

  async startTunnel(port) {
    const availables = [
      { name: 'localTunnel', start: require('localtunnel') }
    ]
    
    const config = { port }
    
    const tunnel = await availables[tryNumber || 0].start(config);
    process.env.TUNNEL_URL = tunnel.url
    
    tunnel.on('close', () => {
      tryNumber = (availables[tryNumber+1]) ? tryNumber += 1 : 0
      startTunnel()
    });

    return tunnel
  },

}