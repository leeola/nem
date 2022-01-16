module.exports = {
  mode: 'jit',
  purge: {
    content: [
      "index.html",
      "design.html",
      "./src/**/*.rs"
    ],
  },
  darkMode: 'media',
  theme: {
    // extend: {},
    colors: {
      // Nord colors, using as a base for sane starting defaults
      //
      // polar night
      "dark-1": "#2e3440",
      "dark-2": "#3b4252",
      "dark-3": "#434c5e",
      "dark-4": "#4c566a",
      // snow storm
      "light-1": "#d8dee9",
      "light-2": "#e5e9f0",
      "light-3": "#eceff4",
      // frost
      "light-green": "#8fbcbb",
      "teal": "#88c0d0",
      "light-blue": "#81a1c1",
      "blue": "#5e81ac",
      // aurora
      "red": "#bf616a",
      "red": "#bf616a",
      "orange": "#d08770",
      "yellow": "#ebcb8b",
      "green": "#a3be8c",
      "light-purple": "#b48ead",

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
