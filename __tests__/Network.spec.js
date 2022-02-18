const { setup_server } = require('../index')
const axios = require('axios');

describe('Network - Ensure internal routes are works', () => {

  let server = null
  const path = `http://localhost:${process.env.PORT_FD}`

  beforeAll(async () => {
    server = setup_server()
  });

  it(`ensure that route /stats works`, async () => {
    const response = await axios(`${path}/stats`);
    expect('TEST_TUNNEL_URL')
      .toEqual(expect.stringContaining(response.data.url));
    expect(200).toEqual(response.status)
  })

  it(`ensure that route /nodes works and return array with node-1`, async () => {
    const response = await axios(`${path}/nodes`);
    expect(200).toEqual(response.status)
    // @todo: assert array of nodes
  })

  it(`ensure that route /join-request works`, async () => {
    const response = await axios(`${path}/join-request`);
    expect(200).toEqual(response.status)
    expect('PENDING').toEqual(expect.stringContaining(response.data.status));
  })

  it(`ensure that route /update-node-info works`, async () => {
    const response = await axios(`${path}/update-node-info`);
    expect(200).toEqual(response.status)
    expect('true').toEqual(expect.stringContaining(response.data.status));
  })

})