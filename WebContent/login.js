// Forms
const loginForm = document.getElementById("login-form");
const createUserForm = document.getElementById("create-user-form");

// Existing user input boxes
const passwordInput = document.getElementById("password-input");
const usernameInput = document.getElementById("username-input");

// New user input boxes
const newUsernameInput = document.getElementById("new-username-input");
const newPasswordInput = document.getElementById("new-password-input");
const confirmPasswordInput = document.getElementById("confirm-password-input");


console.log("login.js loaded");

loginForm.addEventListener("submit", (e) => {
    e.preventDefault();

    // Create JSON object
    const userInfo = {
        username: usernameInput.value,
        password: passwordInput.value
    };

    // Sends JSON to backend at the /login route
    fetch('http://127.0.0.1:3000/login', {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(userInfo)
    })
        .then(async response => {
            const data = await response.json().catch(() => null); // fallback if not JSON
            if (response.ok) {
                console.log(`Success (${response.status})`, data);
                window.location.href = "index.html";
            } else {
                console.warn(`Error (${response.status})`, data || response.statusText);
            }
        })
});