/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/**/*.html', './src/main.rs'],
  theme: {
    fontFamily: {
      'fira-code': ['Fira Code Nerd Font', 'monospace'],
    },
    extend: {
      colors: {
        primary: '#1793d1',
      },
    },
  },
  plugins: [],
}
