var fs = require('fs')

const spreadsheet = fs.readFileSync('./input', 'utf8')

function getChecksum() {
  return spreadsheet
    .split(/\r\n|\r|\n/g)
    .map(row => row.split(/\s+/).map(Number))
    .map(row => Math.max(...row) - Math.min(...row))
    .reduce((a, b) => a + b)
}

console.log(`Checksum: ${getChecksum()}`)
