import type { Config } from 'tailwindcss';

const config: Config = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        plex: {
          bg: '#1a1a1a',
          surface: '#282828',
          border: '#3a3a3a',
          accent: '#e5a00d',
          text: '#f0f0f0',
          muted: '#888888',
        },
      },
      fontFamily: {
        mono: ['Consolas', 'Monaco', 'monospace'],
      },
    },
  },
  plugins: [],
};

export default config;
