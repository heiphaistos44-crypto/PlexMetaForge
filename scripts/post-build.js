#!/usr/bin/env node
// Post-build: rename and move Setup + Portable executables to /dist

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, '..');
const NSIS_DIR = path.join(ROOT, 'src-tauri', 'target', 'release', 'bundle', 'nsis');
const RELEASE_DIR = path.join(ROOT, 'src-tauri', 'target', 'release');
const DIST_DIR = path.join(ROOT, 'dist');
const LOG_FILE = path.join(ROOT, '.logs', 'build.log');

function log(level, msg) {
  const ts = new Date().toISOString();
  const line = `[${ts}] [${level}] ${msg}`;
  console.log(line);
  fs.appendFileSync(LOG_FILE, line + '\n');
}

function findExe(dir) {
  if (!fs.existsSync(dir)) return null;
  return fs.readdirSync(dir).find((f) => f.endsWith('.exe')) ?? null;
}

fs.mkdirSync(DIST_DIR, { recursive: true });
fs.mkdirSync(path.join(ROOT, '.logs'), { recursive: true });

log('INFO', 'post-build started');

// Setup installer (NSIS)
const setupFile = findExe(NSIS_DIR);
if (setupFile) {
  const src = path.join(NSIS_DIR, setupFile);
  const dst = path.join(DIST_DIR, 'PlexMetaForge_Setup.exe');
  fs.copyFileSync(src, dst);
  log('INFO', `Setup -> dist/PlexMetaForge_Setup.exe (${setupFile})`);
} else {
  log('WARN', `No NSIS installer found in ${NSIS_DIR}`);
}

// Portable binary
const portableFile = fs.existsSync(RELEASE_DIR)
  ? fs.readdirSync(RELEASE_DIR).find(
      (f) =>
        f.endsWith('.exe') &&
        !f.includes('setup') &&
        !f.includes('Setup') &&
        !f.includes('plugin') &&
        !fs.statSync(path.join(RELEASE_DIR, f)).isDirectory()
    )
  : null;

if (portableFile) {
  const src = path.join(RELEASE_DIR, portableFile);
  const dst = path.join(DIST_DIR, 'PlexMetaForge_Portable.exe');
  fs.copyFileSync(src, dst);
  log('INFO', `Portable -> dist/PlexMetaForge_Portable.exe (${portableFile})`);
} else {
  log('WARN', `No portable exe found in ${RELEASE_DIR}`);
}

log('INFO', `dist ready at ${DIST_DIR}`);
