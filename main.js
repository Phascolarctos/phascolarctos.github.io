const theme = document.getElementById("mode")
const sun = document.getElementById('mode_sun')
const moon = document.getElementById('mode_moon')

theme.addEventListener('click', (e) => {
    const htmlElement = document.documentElement;
    
    if (htmlElement.getAttribute('mode') === 'light') {
        htmlElement.setAttribute('mode', 'dark');
        sun.classList.toggle('right_mode')
        moon.classList.toggle('right_mode')
    } else if (htmlElement.getAttribute('mode') === 'dark') {
        htmlElement.setAttribute('mode', 'light');
        sun.classList.toggle('right_mode')
        moon.classList.toggle('right_mode')
    } else {
        htmlElement.setAttribute('mode', 'dark')
        sun.classList.toggle('right_mode')
        moon.classList.toggle('right_mode')
    }
})
