const kvs = [
    ["df", {
        tlt: "default article title",
        ctt: "article content list something about the source of website, again choose me for you, hahaha..."
    }],
    ["rust", {
        tlt: "rust learning",
        ctt: "rust is a language"
    }],
    ["java", {
        tlt: "java learning",
        ctt: "java is a language"
    }],
];
const m = new Map(kvs);
const tmc = document.getElementById("tmc")
setInterval(function () {
    const date = new Date()
    tmc.innerText = date.toUTCString()
}, 0)

const t = document.getElementById("tlt")
const c = document.getElementById("ctt")

const df = m.get("df")
t.innerText = df.tlt
c.innerText = df.ctt

const dms = document.getElementsByClassName("att")
for (let index = 0; index < dms.length; index++) {
    const e = dms[index];
    const k = e.textContent.trim().toLowerCase()
    e.id = k
    const v = m.get(k)
    e.addEventListener("click", (event) => {
        t.innerText = v.tlt
        c.innerText = v.ctt
    })
}

const mode = document.getElementById("darkmode")
const arl = document.getElementById("arl")
let darkmode = false
mode.addEventListener("click", (event) => {
    mode.style.transition = "2s"
    if (darkmode) {
        document.body.style.backgroundColor = "#FFFFFF"
        document.body.style.color = "#1A1A1D"
        mode.src = "images/sun.svg"
        darkmode = false
    } else {
        document.body.style.backgroundColor = "#0B192C"
        document.body.style.color = "#FFFFFF"
        mode.src = "images/moon.svg"
        darkmode = true
    }
})