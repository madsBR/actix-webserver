const path = require('path');
const fs = require('fs');

const appsDir = path.resolve(__dirname, 'static');

module.exports = {
  entry: getEntries(),
  mode: 'production',
  output: {
    path: appsDir,
    filename: '[name].js',
  },
  resolve: {
    extensions: ['.ts'], // File extensions to resolve
  },
  module: {
    rules: [
      {
        test: /\.ts$/, // Match .ts files
        use: 'ts-loader', // Use ts-loader to transpile TypeScript
        exclude: /node_modules/,
      },
    ],
  },
};

function getEntries() {
    const entries = {};
  
    const apps = fs.readdirSync(appsDir);
    apps.forEach(app => {
      const appPath = path.join(appsDir, app, 'js', 'entry.ts');
      const appKey = path.join(app, 'js', `${app}-entry`);      
  
      if (fs.existsSync(appPath)) {
        entries[appKey] = appPath;
      }
    });
  
    return entries;
  }