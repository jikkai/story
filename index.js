require('babel-core/register')(
  {
    presets: ['stage-3', 'es2016-node5']
  }
)

require('babel-polyfill')

require('./app.js')