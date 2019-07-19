const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
  entry: {
    index: './js/index.js'
  },
  output: {
    filename: '[name]-[hash].js',
    path: path.resolve(__dirname, 'dist'),
    publicPath: '/'
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
      },
      {
        test: /\.(png|jpg|gif|eot|woff|ttf|svg|ico)$/,
        loader: 'file-loader'
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin(),
    new webpack.HotModuleReplacementPlugin()
  ],
  node: {
    fs: "empty"
  },
  devServer: {
    contentBase: path.join(__dirname, 'public'),
    proxy: {
      '/__': 'http://localhost:5000',
    },
    historyApiFallback: {
      rewrites: [
        { from: /./, to: '/' }
      ]
    },
    hot: true
  }
};
