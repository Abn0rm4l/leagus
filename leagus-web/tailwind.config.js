const colors = require("tailwindcss/colors");

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["./templates/**/*.{html,js}", "node_modules/preline/dist/*.js"],
  colors: {
    transparent: "transparent",
    current: "currentColor",
    black: colors.black,
    white: colors.white,
    gray: colors.gray,
    slate: colors.slate,
    stone: colors.stone,
    emerald: colors.emerald,
    indigo: colors.indigo,
    amber: colors.amber,
    lime: colors.lime,
    yellow: colors.yellow,
  },
  plugins: [require("@tailwindcss/forms"), require("preline/plugin")],
};
