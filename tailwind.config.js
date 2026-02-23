/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./style/**/*.css", "./src/**/*.rs"],
  theme: {
    extend: {
      screens: {
        // Mobile-first: base styles for mobile, then sm/md/lg
        sm: "640px",
        md: "768px",
        lg: "1024px",
        xl: "1280px",
      },
      spacing: {
        "thumb-zone": "env(safe-area-inset-bottom, 0px)",
      },
    },
  },
  plugins: [],
};
