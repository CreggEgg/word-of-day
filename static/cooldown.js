let cooldown = document.querySelector("#cooldown")
let cooldown_amount = (new URLSearchParams(window.location.search)).get('cooldown');

if (cooldown_amount != undefined) {
    alert("hi")
    cooldown.innerText = `${parseInt(cooldown_amount) / 1000}s`
    let total = Math.floor(parseInt(cooldown_amount) / 1000);
    for (let i = 0; i <= total; i++) {
        // console.log(i)
        setTimeout(() => {
            cooldown.innerText = `${total - i}s`
        }, i * 1000);
    }
}
