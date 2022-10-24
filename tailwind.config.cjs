module.exports = {
    content: ['./src/**/*.{svelte,js,ts}'],
    plugins: [require('daisyui')],
    extend: {
        spacing: {
            '128': '32rem',
        }
    }
};