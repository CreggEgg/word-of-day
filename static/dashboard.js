let current_password = document.querySelector("#current-password")
let new_password = document.querySelector("#new-password")
let new_password_submit = document.querySelector("#submit-new-password")
let new_word = document.querySelector("#new-word")
let new_word_submit = document.querySelector("#submit-new-word")

new_password_submit.onclick = () => {
    alert(window.location.host)
    alert(window.location.origin)
    let password = current_password.value ?? " "

    let response = httpGet(`/set-password?new=${new_password.value}&old=${password}`)

    console.log(response.password)
    if (response["Unauthenticated"] != undefined) {
        window.location.href = `/cooldown.html?cooldown=${response["Unauthenticated"].cooldown}`
    }
    if (response["NewPassword"] != undefined) {
        current_password.value = response["NewPassword"].password
        new_password.value = ""
    }
    // if (response == {"NewPassword": {"password": "test"}})

    alert(JSON.stringify(response))
}
new_word_submit.onclick = () => {
    let current_password = current_password.value
}
