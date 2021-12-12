const { setup_server } = require('../index')

describe('src/Network', () => {

  it(`ensure that correct routes can be setuped on service`, async () => {
    const server = setup_server(process.env.PORT || 3001)
    console.log(server)
  })

})