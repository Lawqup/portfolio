/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require('@shrutibalasa/tailwind-grid-auto-fit'),
  ],
}
