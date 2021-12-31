const colors = require('tailwindcss/colors')

module.exports = {
  mode: 'jit',
  purge: {
    content: [
      "./src/**/*.rs"
    ],
  },
  theme: {
    extend: {},
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: colors.black,
      red: colors.red,
      gray: colors.gray,
      orange: colors.orange,
      amber: colors.amber,
      yellow: colors.yellow,
      white: colors.white,
    }
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
