document.addEventListener("DOMContentLoaded", async () => {
    const application = {
        statusBar: document.getElementById("status"),
        inputBox: document.getElementById("input"),
        submitButton: document.getElementById("submit"),
        oldStatus: undefined,
        showError: function(message) {
            this.oldStatus = this.statusBar.innerText
            this.statusBar.innerText = message
            this.statusBar.classList.add("error")
            setTimeout(this.clearError, 3000)
        },
        clearError: function() {
            if (this.statusBar.classList.contains("error") && this.oldStatus) {
                this.statusBar.classList.remove("error")
                this.statusBar.textContent = this.oldStatus
                this.oldStatus = null
            }
        }
    }
    try {
        const response = await fetch("/init")
        if (response.ok) {
            try {
                const result = await response.json()
                if ('text' in result) {
                    application.statusBar.innerText = result.text
                } else {
                    application.showError('Сервер не дал описания текущего шага.')
                }
            } catch (error) {
                application.showError(error)
            }
        } else {
            application.showError("Ошибка инициализации, ответ сервера: " + response.status + (response.statusText ? " " + response.statusText : ""))
        }
    } catch (error) {
        application.showError(error)
    }
})