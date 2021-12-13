const { setup_server } = require('../index')
const axios = require('axios');

// @todo: test routes
describe('Network - Node Manager routes', () => {

  const server = null
  const path = 'http://localhost:3000'

  beforeAll(() => {
    server = setup_server(process.env.PORT || 3001)
  });

  it(`ensure that route /stats works`, async () => {
    const res = await axios(`${path}/stats`);
    console.log('res -------', res)
  })

  it(`ensure that route /nodes works`, async () => {
    const res = await axios(`${path}/nodes`);
    console.log('res -------', res)
  })

  it(`ensure that route /join-request works`, async () => {
    const res = await axios(`${path}/join-request`);
    console.log('res -------', res)
  })

  it(`ensure that route /update-node-info works`, async () => {
    const res = await axios(`${path}/update-node-info`);
    console.log('res -------', res)
  })

})