const socket = new WebSocket("ws://localhost:3000/ws");

const form = document.getElementById("chat-form");

form.addEventListener("submit", (e) =>{
    e.preventDefault();

    const text = document.getElementById("chat-input");
    const name = document.getElementById("name-input");

    const message = {
        text: text.value,
        name: name.value
    };

    socket.send(JSON.stringify(message));
})