const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');

module.exports = {
  entry: './index.ts',
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../../"),
      extraArgs: '--target bundler',
      outDir: "./pkg",
      outName: "youtube_player_api",
    }),
    new HtmlWebpackPlugin({
      inject: 'head',
      template: 'index.html',
      scriptLoading: 'module'
    })
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
    outputModule: true,
    topLevelAwait: true
  }
};
