/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: true,
    safelist: [
        {
            pattern: /./, // the "." means "everything"
        },
    ],
    plugins: [],
};
