module.exports = {
  mode: 'jit',
  purge: {
    content: [
      "./src/**/*.rs"
    ],
  },
  darkMode: 'media',
  theme: {
    // extend: {},
    colors: {
      "black": "#2e3440",
      "dark-gray": "#3b4252",
      "white": "#eceff4",

      "red": "#bf616a",

    //   transparent: 'transparent',
    //   current: 'currentColor',
    //   black: colors.black,
    //   red: '#f00',
    //   gray: colors.gray,
    //   orange: colors.orange,
    //   amber: colors.amber,
    //   yellow: colors.yellow,
    //   white: colors.white,
    }
  },
  plugins: [
    // require('@tailwindcss/forms'),
    // require('@tailwindcss/aspect-ratio'),
    // require('@tailwindcss/typography'),
    // require('tailwindcss-children'),
  ],
}
