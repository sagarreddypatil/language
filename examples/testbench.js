function id(x) {
    return x
}

function data(d, ...fields) {
    return [ d, fields ]
}

function desc(data) {
    return data[0]
}

function field(data, i) {
    return data[1][i]
}

function halt(x) {
    const util = require('util')
    console.log(util.inspect(x, { depth: null }))
}