
let todays_word = document.querySelector("#todays-word")

let word = httpGet(`/get-today`)
if (word == "NothingToday") {
    todays_word.innerText = "No word today"
} else {
    todays_word.innerText = word["Today"].word
}
