const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: {
    index: './js/index.js'
  },
  output: {
    filename: '[name]-[hash].js',
    path: path.resolve(__dirname, 'dist')
  },
  devtool: 'cheap-source-map',
  module: {
    rules: [
      {
        test: /target\/wasm32-unknown-unknown\/(debug|release)\/minimal\.js$/,
        loader: 'string-replace-loader',
        options: {
          search: 'fetch( "minimal.wasm"',
          replace: 'fetch( require("./minimal.wasm")',
        }
      },
      {
        test: /\.wasm$/,
        type: 'javascript/auto',
        loader: 'file-loader'
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin()
  ],
  node: {
    fs: "empty"
  }
};
