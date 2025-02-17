let cooldown = document.querySelector("#cooldown")
let cooldown_amount = (new URLSearchParams(window.location.search)).get('cooldown');

if (cooldown_amount != undefined) {
    cooldown.innerText = `${parseInt(cooldown_amount) / 1000}s`
}
