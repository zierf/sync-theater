import path from 'path';
import { fileURLToPath } from 'url';

import HtmlWebpackPlugin from 'html-webpack-plugin';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import webpack from 'webpack';


const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const config = {
  entry: './index.ts',
  devtool: process.env.SOURCE_MAP ? 'inline-source-map' : 'hidden-source-map',
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
      crateDirectory: path.resolve(__dirname, '../../'),
      extraArgs: '--target bundler --features=std',
      outDir: './pkg',
      outName: 'youtube_player_api',
    }),
    new HtmlWebpackPlugin({
      inject: 'head',
      template: 'index.html',
      scriptLoading: 'module'
    })
  ],
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 4000,
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
    outputModule: true,
    topLevelAwait: true
  }
};

export default config;
