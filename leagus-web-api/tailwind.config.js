const colors = require("tailwindcss/colors");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{html,js}", "node_modules/preline/dist/*.js"],
  colors: {
    transparent: "transparent",
    current: "currentColor",
    black: colors.black,
    white: colors.white,
    gray: colors.gray,
    slate: colors.slate,
    emerald: colors.emerald,
    indigo: colors.indigo,
    yellow: colors.yellow,
  },
  plugins: [require("@tailwindcss/forms"), require("preline/plugin")],
};
