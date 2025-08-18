const socket = new WebSocket("ws://localhost:3000/ws");
const form = document.getElementById("chat-form");
const message_list = document.getElementById("message-list");
const textInput = document.getElementById("chat-input");
const userInput = document.getElementById("name-input");

form.addEventListener("submit", (e) => {
    e.preventDefault();
    if (socket.readyState !== WebSocket.OPEN) return;

    const msg = {
        type: "chat",
        user: userInput.value || "Anonymous",
        text: textInput.value
    };

    socket.send(JSON.stringify(msg));
    textInput.value = "";
    textInput.focus();
});



socket.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    if (msg.type === "chat") {
        const new_message = document.createElement("li");
        new_message.textContent = msg.text;
        message_list.appendChild(new_message);
    } else if (msg.type === "system") {
        const new_message = document.createElement("li");
        new_message.textContent = msg.text;
        message_list.appendChild(new_message);
    }
};