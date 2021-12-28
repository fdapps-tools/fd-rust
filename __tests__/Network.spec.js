const { setup_server } = require('../index')
const axios = require('axios');

describe('Network - Node Manager routes', () => {

  let server = null
  const path = `http://localhost:${process.env.PORT_FD}`

  beforeAll(async () => {
    server = setup_server()
  });

  it(`ensure that route /stats works`, async () => {
    const { data } = await axios(`${path}/stats`);
    expect('TEST_TUNNEL_URL')
      .toEqual(expect.stringContaining(data.url));
  })

  it(`ensure that route /nodes works and return array with node-1`, async () => {
    const { data } = await axios(`${path}/nodes`);
    expect(['node-1']).toEqual(expect.arrayContaining(data.nodes));
  })

  it(`ensure that route /join-request works`, async () => {
    const  { data } = await axios(`${path}/join-request`);
    expect('PENDING').toEqual(expect.stringContaining(data.status));
  })

  it(`ensure that route /update-node-info works`, async () => {
    const  { data } = await axios(`${path}/update-node-info`);
    expect('true').toEqual(expect.stringContaining(data.status));
  })

})