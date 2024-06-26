/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./static/js/*.js",
    // "./templates/**/*.go.tmpl",
    "./templates/**/*.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
}
