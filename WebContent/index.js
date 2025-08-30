const socket = new WebSocket("ws://127.0.0.1:3000/ws");
const form = document.getElementById("chat-form");
const messageBoard = document.getElementById("message-board");
const textInput = document.getElementById("chat-input");

form.addEventListener("submit", (e) => {
    e.preventDefault();
    if (socket.readyState !== WebSocket.OPEN) return;

    const msg = {
        type: "chat",
        text: textInput.value
    };

    socket.send(JSON.stringify(msg));
    textInput.value = "";
    textInput.focus();
});

socket.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    console.log(msg);


    if (msg.type === "chat") {
        const newMessage = document.createElement("tr");
        const strong = document.createElement("strong");
        newMessage.innerHTML = `<b>${msg.username}:</b> ${msg.text}`;
        messageBoard.appendChild(newMessage);
    } else if (msg.type === "system") {
        const newMessage = document.createElement("tr");
        const strong = document.createElement("strong");
        newMessage.innerHTML = `<b>System:</b> ${msg.message}`;
        messageBoard.appendChild(newMessage);
    }
};