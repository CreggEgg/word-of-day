let current_password = document.querySelector("#current-password")
let new_password = document.querySelector("#new-password")
let new_password_submit = document.querySelector("#submit-new-password")
let new_word = document.querySelector("#new-word")
let new_word_submit = document.querySelector("#submit-new-word")

new_password_submit.onclick = () => {
    // alert(window.location.host)
    // alert(window.location.origin)
    let password = current_password.value ?? " "

    let response = httpGet(`/set-password?new=${new_password.value}&old=${password}`)

    console.log(response.password)
    if (response["Unauthenticated"] != undefined) {
        window.location.href = `/word-of-day/cooldown?cooldown=${response["Unauthenticated"].cooldown}`
    }
    if (response["CooldownActive"] != undefined) {
        window.location.href = `/word-of-day/cooldown?cooldown=${response["CooldownActive"].cooldown}`
    }
    if (response["NewPassword"] != undefined) {
        current_password.value = response["NewPassword"].password
        new_password.value = ""
    }
    // if (response == {"NewPassword": {"password": "test"}})

    // alert(JSON.stringify(response))
}
new_word_submit.onclick = () => {
    let password = current_password.value ?? " "
    let response = httpGet(`/set-today?password=${password}&new_word=${new_word.value}`)
    if (response["Unauthenticated"] != undefined) {
        window.location.href = `/word-of-day/cooldown?cooldown=${response["Unauthenticated"].cooldown}`
    }
    alert("success")
}
