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
              verbose: false,
              features: ""
            }
          }
        ]
      },
      {
        test: /\.wasm$/,
        loader: 'file-loader',
        options: {
          name: 'static/wasm/[name].[hash:8].[ext]'
        }
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin()
  ]
};
