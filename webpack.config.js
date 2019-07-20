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
        test: /.*minimal\.js$/,
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
      },
      {
        test: /\.s?css$/,
        use: [
          "style-loader",       // creates style nodes from JS strings
          "css-loader",         // translates CSS into CommonJS
          "sass-loader" // compiles Sass to CSS, using Node Sass by default
        ]
      }
    ]
  },
  resolve: {
    extensions: [ '.js', '.sass', '.scss', 'css', '.rs', '.png' ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'CoC',
      template: 'src/index.html'
    }),
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
