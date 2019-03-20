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
  module: {
    rules: [
      {
        test: /Cargo.toml$/,
        loaders: [
          {
            loader: "cargo-web-loader",
            options: {
              flags: '',
              bin: false,
              release: true,
              verbose: true,
              features: ""
            }
          }
        ]
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
