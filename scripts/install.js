#!/usr/bin/env node
'use strict';

// Postinstall: download the prebuilt GhostNet CLI binary for this platform from
// the matching GitHub Release and drop it in scripts/bin/.

const fs = require('fs');
const path = require('path');
const https = require('https');
const { version } = require('../package.json');

const REPO = 'mehboii/GHOSTNET-CLI';

// Map Node's platform-arch to the release asset name produced by CI.
const ASSETS = {
  'win32-x64': 'ghostnet-win32-x64.exe',
  'linux-x64': 'ghostnet-linux-x64',
  'linux-arm64': 'ghostnet-linux-arm64',
  'darwin-x64': 'ghostnet-darwin-x64',
  'darwin-arm64': 'ghostnet-darwin-arm64',
};

function resolveAsset() {
  const key = `${process.platform}-${process.arch}`;
  const asset = ASSETS[key];
  if (!asset) {
    throw new Error(
      `unsupported platform "${key}". Build from source instead: https://github.com/${REPO}`,
    );
  }
  return asset;
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { 'User-Agent': 'ghostnet-cli-installer' } }, (res) => {
        // Follow GitHub's redirect to the asset storage.
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          res.resume();
          return resolve(download(res.headers.location, dest));
        }
        if (res.statusCode !== 200) {
          res.resume();
          return reject(new Error(`download failed (HTTP ${res.statusCode}) for ${url}`));
        }
        const file = fs.createWriteStream(dest, { mode: 0o755 });
        res.pipe(file);
        file.on('finish', () => file.close(() => resolve()));
        file.on('error', reject);
      })
      .on('error', reject);
  });
}

async function main() {
  const asset = resolveAsset();
  const binDir = path.join(__dirname, 'bin');
  fs.mkdirSync(binDir, { recursive: true });

  const ext = process.platform === 'win32' ? '.exe' : '';
  const dest = path.join(binDir, `ghostnet${ext}`);
  const url = `https://github.com/${REPO}/releases/download/v${version}/${asset}`;

  process.stdout.write(`Downloading GhostNet CLI v${version} (${asset})…\n`);
  await download(url, dest);
  if (process.platform !== 'win32') {
    fs.chmodSync(dest, 0o755);
  }
  process.stdout.write('GhostNet CLI installed. Run `ghostnet --help` to get started.\n');
}

main().catch((err) => {
  console.error(`GhostNet CLI install failed: ${err.message}`);
  process.exit(1);
});
