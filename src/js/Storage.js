const fs = require('fs').promises;
const PATH = './src/js/localDB'

module.exports = {

  async getFile(filename) {
    let data = "[]"

    try {
      data = await fs.readFile(`${PATH}/${filename}.state`, 'utf8')

    } catch (error) {
      return []
    }

    return JSON.parse(data)
  },

  async updateFile(data, filename) {
    await fs.writeFile(`${PATH}/${filename}.state`, JSON.stringify(data))
    return true
  }
}