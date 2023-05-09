module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        shake: {
          '25%': {transform: 'translate(-5px)'},
          '75%': {transform: 'translate(5px)'},
        },
      },
      animation: {
        shake: 'shake 0.35s linear 2',
      },
    },
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
  plugins: [
  ],
}
