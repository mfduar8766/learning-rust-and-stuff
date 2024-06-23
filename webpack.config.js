const path = require('path');

module.exports = {
  entry: './rust-ssr-app/templates/js/index.js',
  devtool: 'inline-source-map', // This option controls if and how source maps are generated.
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
    filename: 'bundle.js',
    path: path.resolve(__dirname, './rust-ssr-app/dist'),
  },
  devServer: {
    contentBase: './dist',
  },
};
